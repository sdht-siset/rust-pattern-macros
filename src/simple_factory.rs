use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{Ident, ItemTrait, TraitBound, punctuated::Punctuated, token::Plus};

/// 为给定的 trait 生成简单工厂实现。
///
/// 此函数接收一个 trait 定义和 trait bound，生成相应的工厂结构体。
/// 生成的工厂结构体名为 `{TraitName}Factory`，并提供一个 `create` 方法，
/// 该方法使用全局静态工厂来创建 trait 对象的实例。
///
/// # 参数
/// * `product_trait` - 要为其生成工厂的 trait 定义
/// * `product_bounds` - trait 的额外 bound，用于构建完整的 trait 对象类型
///
/// # 返回值
/// 生成的代码的 `TokenStream`，包含：
/// 1. 原始的 trait 定义
/// 2. 工厂结构体 `{TraitName}Factory`
/// 3. 工厂的 `create` 方法实现
///
/// # 生成的内容
/// - 工厂结构体：`pub struct {TraitName}Factory;`
/// - `create` 方法：使用 `LazyLock` 缓存工厂实例，通过 ID 和策略创建对象
pub fn generate(
    product_trait: ItemTrait,
    product_bounds: Punctuated<TraitBound, Plus>,
) -> TokenStream {
    let product_vis = &product_trait.vis;
    let product_ident = &product_trait.ident;
    let factory_ident = Ident::new(&format!("{product_ident}Factory"), Span::call_site());
    let bounds_iter = product_bounds.iter();
    let product_type = quote! { dyn #product_ident #( + #bounds_iter )* };

    quote! {
        #product_trait

        #product_vis struct #factory_ident;

        impl #factory_ident {
            #[inline]
            pub fn create(
                id: &str,
                strategy: rust_patterns::FactoryFallback,
            ) -> std::result::Result<(&str, Box<#product_type>), rust_patterns::FactoryError> {
                use std::sync::LazyLock;
                use rust_patterns::{FactoryRegistry, SimpleFactory};

                static FACTORY: LazyLock<SimpleFactory<#product_type>> =
                    LazyLock::new(FactoryRegistry::simple_factory);

                FACTORY.create(id, strategy)
            }
        }
    }
    .into()
}
