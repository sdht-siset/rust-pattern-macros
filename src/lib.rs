use proc_macro::TokenStream;

mod observable;
mod simple_factory;

/// 为 trait 生成简单工厂实现的属性宏。
///
/// 此宏应用于 trait 定义，会自动生成一个对应的工厂结构体。
/// 生成的工厂结构体名为 `{TraitName}Factory`，并提供一个 `create` 方法，
/// 该方法使用全局静态工厂来创建 trait 对象的实例。
///
/// 注意：此宏生成的代码依赖于 `rust_patterns` crate，请确保在项目中添加该依赖。
///
/// # 使用方法
///
/// ```rust,ignore
/// use pattern_macros::simple_factory;
///
/// #[simple_factory]
/// pub trait MyTrait {
///     fn do_something(&self);
/// }
/// ```
///
/// 也可以指定额外的 trait bound：
///
/// ```rust,ignore
/// use pattern_macros::simple_factory;
///
/// #[simple_factory(Send + Sync)]
/// pub trait MyTrait {
///     fn do_something(&self);
/// }
/// ```
///
/// # 生成的内容
///
/// 宏会生成以下内容：
/// 1. 原始的 trait 定义
/// 2. 工厂结构体 `{TraitName}Factory`
/// 3. 工厂的 `create` 方法实现
///
/// `create` 方法签名：
/// ```rust,ignore
/// pub fn create(
///     id: impl AsRef<str>,
///     strategy: rust_patterns::FactoryFallback,
/// ) -> Result<Box<dyn MyTrait>, rust_patterns::FactoryError>
/// ```
#[proc_macro_attribute]
pub fn simple_factory(args: TokenStream, input: TokenStream) -> TokenStream {
    simple_factory::generate(args, input)
}

/// 为结构体自动实现 `Observable` trait 的属性宏。
///
/// 此宏应用于结构体定义，会自动：
/// 1. 添加 `registry: ObserverRegistry<Self>` 字段
/// 2. 实现 `Observable` trait，包括 `State` 和 `Error` 关联类型
/// 3. 提供 `attach` 和 `detach` 方法的默认实现
///
/// # 使用方法
///
/// ```rust,ignore
/// use pattern_macros::observable;
///
/// #[observable(state = u64, error = anyhow::Error)]
/// struct TemperatureSensor {
///     temperature: f64,
/// }
/// ```
///
/// 也可以使用自定义的错误类型：
///
/// ```rust,ignore
/// #[observable(state = String, error = std::io::Error)]
/// struct Logger {
///     log_level: u8,
/// }
/// ```
///
/// # 生成的内容
///
/// 宏会生成以下内容：
/// 1. 添加 `registry: ObserverRegistry<Self>` 字段到结构体
/// 2. 实现 `Observable` trait，设置 `State` 和 `Error` 关联类型
/// 3. 实现 `attach` 和 `detach` 方法，委托给内部的 `registry`
#[proc_macro_attribute]
pub fn observable(args: TokenStream, input: TokenStream) -> TokenStream {
    observable::generate(args, input)
}
