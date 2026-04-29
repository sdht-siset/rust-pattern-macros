use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{Field, FieldMutability, Fields, Ident, ItemStruct, Type, Visibility, parse_macro_input};

/// 解析 observable 宏的参数
///
/// 此结构体用于解析 `#[observable(state = Type, error = ErrorType)]` 中的参数。
/// 它包含两个必需的参数：状态类型和错误类型。
struct ObservableArgs {
    state_type: Type,
    error_type: Type,
}

impl ObservableArgs {
    /// 解析 observable 宏的参数
    ///
    /// # 参数
    /// * `args` - 宏的参数 TokenStream，格式为 "state = Type, error = ErrorType"
    ///
    /// # 返回值
    /// * `Ok(ObservableArgs)` - 成功解析的参数
    /// * `Err(syn::Error)` - 解析失败时返回错误
    ///
    /// # 参数格式
    /// - 必须包含两个参数：`state` 和 `error`
    /// - 参数格式：`key = value`
    /// - 多个参数用逗号分隔
    /// - 值必须是有效的 Rust 类型表达式
    fn parse(args: TokenStream) -> syn::Result<Self> {
        let args_str = args.to_string();
        let parts = args_str.split(',').map(|s| s.trim());

        let mut state_type = None;
        let mut error_type = None;

        for part in parts {
            if let Some((key, value)) = part.split_once('=') {
                let key = key.trim();
                let value = value.trim();

                match key {
                    "state" => {
                        state_type = Some(syn::parse_str::<Type>(value)?);
                    }
                    "error" => {
                        error_type = Some(syn::parse_str::<Type>(value)?);
                    }
                    _ => {
                        return Err(syn::Error::new(
                            Span::call_site(),
                            format!("unknown parameter '{}', expected 'state' or 'error'", key),
                        ));
                    }
                }
            } else {
                return Err(syn::Error::new(
                    Span::call_site(),
                    format!(
                        "invalid parameter format '{}', expected 'key = value'",
                        part
                    ),
                ));
            }
        }

        Ok(Self {
            state_type: state_type
                .ok_or_else(|| syn::Error::new(Span::call_site(), "missing 'state' parameter"))?,
            error_type: error_type
                .ok_or_else(|| syn::Error::new(Span::call_site(), "missing 'error' parameter"))?,
        })
    }
}

/// 生成 observable 宏的实现
///
/// 此函数接收宏的参数和输入，解析结构体定义和 observable 参数，
/// 为结构体添加 `ObserverRegistry` 字段并实现 `Observable` trait。
///
/// # 参数
/// * `args` - 宏的参数 TokenStream，格式为 "state = Type, error = ErrorType"
/// * `input` - 宏的输入 TokenStream，包含要为其实现 Observable 的结构体定义
///
/// # 返回值
/// 生成的代码的 `TokenStream`，包含：
/// 1. 修改后的结构体定义（添加了 `registry` 字段）
/// 2. `Observable` trait 的实现
/// 3. 两个通知方法：`notify` 和 `notify_ignore_error`
///
/// # 生成的内容
/// - 添加 `registry: ObserverRegistry<Self>` 字段到结构体
/// - 实现 `Observable` trait，设置 `State` 和 `Error` 关联类型
/// - 实现 `attach` 和 `detach` 方法，委托给内部的 `registry`
/// - 提供 `notify` 方法（使用 StopOnError 策略）
/// - 提供 `notify_ignore_error` 方法（使用 IgnoreError 策略）
///
/// # 注意事项
/// - `registry` 字段使用结构体的可见性
/// - 不提供 `Default` 实现，结构体初始化由用户负责
/// - 只支持具名字段的结构体（不支持元组结构体和单元结构体）
pub fn generate(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = match ObservableArgs::parse(args) {
        Ok(args) => args,
        Err(e) => return e.to_compile_error().into(),
    };

    let mut input_struct = parse_macro_input!(input as ItemStruct);
    let struct_name = &input_struct.ident;

    // 添加 registry 字段，使用结构体的可见性
    let registry_field = Field {
        attrs: Vec::new(),
        vis: Visibility::Inherited,
        mutability: FieldMutability::None,
        ident: Some(Ident::new("registry", Span::call_site())),
        colon_token: Some(syn::token::Colon(Span::call_site())),
        ty: syn::parse_str::<Type>(&format!(
            "::rust_patterns::ObserverRegistry<{}>",
            struct_name
        ))
        .unwrap(),
    };

    match &mut input_struct.fields {
        Fields::Named(fields) => {
            fields.named.push(registry_field);
        }
        Fields::Unnamed(_fields) => {
            return syn::Error::new_spanned(
                struct_name,
                "#[observable] can only be applied to structs with named fields",
            )
            .to_compile_error()
            .into();
        }
        Fields::Unit => {
            return syn::Error::new_spanned(
                struct_name,
                "#[observable] can only be applied to structs with named fields",
            )
            .to_compile_error()
            .into();
        }
    }

    // 生成 Observable trait 实现
    let state_type = &args.state_type;
    let error_type = &args.error_type;

    let expanded = quote! {
        #input_struct

        impl ::rust_patterns::Observable for #struct_name {
            type State = #state_type;
            type Error = #error_type;

            fn attach(&mut self, observer: ::std::sync::Arc<dyn ::rust_patterns::Observer<Subject = Self>>) {
                self.registry.attach(observer);
            }

            fn detach(&mut self, observer: ::std::sync::Arc<dyn ::rust_patterns::Observer<Subject = Self>>) {
                self.registry.detach(observer);
            }
        }

        impl #struct_name {
            /// 通知所有观察者状态变化
            ///
            /// 当一个观察者处理更新失败时，立即停止并返回错误。
            #[inline]
            fn notify(&self, state: &#state_type) -> Result<(), #error_type> {
                self.registry.notify(state)
            }

            /// 通知所有观察者状态变化
            ///
            /// 即使某个观察者失败，也继续通知其他观察者。
            #[inline]
            fn notify_ignore_error(&self, state: &#state_type) {
                self.registry.notify_ignore_error(state)
            }
        }

    };

    expanded.into()
}
