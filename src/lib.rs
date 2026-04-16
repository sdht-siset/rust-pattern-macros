use proc_macro::TokenStream;
use syn::{ItemTrait, TraitBound, parse_macro_input, punctuated::Punctuated, token::Plus};

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
///     id: &str,
///     strategy: rust_patterns::FactoryFallback,
/// ) -> Result<(&str, Box<dyn MyTrait>), rust_patterns::FactoryError>
/// ```
#[proc_macro_attribute]
pub fn simple_factory(args: TokenStream, input: TokenStream) -> TokenStream {
    let product_trait = parse_macro_input!(input as ItemTrait);
    let product_bounds = if args.is_empty() {
        Punctuated::<TraitBound, Plus>::new()
    } else {
        parse_macro_input!(args with Punctuated::<TraitBound, Plus>::parse_terminated)
    };

    simple_factory::generate(product_trait, product_bounds)
}
