//! 演示 simple_factory 宏的使用示例
#![allow(dead_code, unused_variables)]

use rust_pattern_macros::simple_factory;
use rust_patterns::{FactoryError, FactoryFallback};

// 示例 1: 基本用法 - 为 trait 生成简单工厂
#[simple_factory]
pub trait Product {
    fn name(&self) -> &str;
    fn price(&self) -> f64;
}

// 产品实现 A
#[derive(Default)]
struct ProductA;

impl Product for ProductA {
    fn name(&self) -> &str {
        "Product A"
    }

    fn price(&self) -> f64 {
        19.99
    }
}

// 产品实现 B
#[derive(Default)]
struct ProductB;

impl Product for ProductB {
    fn name(&self) -> &str {
        "Product B"
    }

    fn price(&self) -> f64 {
        29.99
    }
}

// 注册工厂（在实际使用中，这些注册通常在单独的模块中）
// 注意：这里只是示例，实际注册需要使用 inventory crate
// register_factory!(dyn Product, "product_a", ProductA);
// register_factory!(dyn Product, "product_b", ProductB);

// 示例 2: 带额外 trait bound 的工厂
#[simple_factory(Send + Sync)]
pub trait Service {
    fn execute(&self) -> Result<String, String>;
}

// 服务实现
#[derive(Default)]
struct ServiceImpl;

impl Service for ServiceImpl {
    fn execute(&self) -> Result<String, String> {
        Ok("Service executed successfully".to_string())
    }
}

// 注册服务工厂
// register_factory!(dyn Service + Send + Sync, "service_impl", ServiceImpl);

fn main() {
    println!("=== Simple Factory Macro Example ===\n");

    // 测试 Product 工厂
    println!("1. Testing Product factory:");

    // 注意：由于工厂未实际注册，这些调用会失败
    // 在实际使用中，需要先注册工厂

    match ProductFactory::create("product_a", FactoryFallback::NoFallback) {
        Ok((id, product)) => {
            println!(
                "   Created product: {}, ID: {}, Price: ${:.2}",
                product.name(),
                id,
                product.price()
            );
        }
        Err(FactoryError::FactoryNotFound(id)) => {
            println!(
                "   Product factory not found for ID: '{}' (expected - factories not registered)",
                id
            );
        }
        Err(e) => {
            println!("   Error: {}", e);
        }
    }

    // 测试回退策略
    println!("\n2. Testing fallback strategies:");

    match ProductFactory::create("", FactoryFallback::First) {
        Ok((id, product)) => {
            println!("   First fallback created: {}, ID: {}", product.name(), id);
        }
        Err(FactoryError::NoFactoriesAvailable) => {
            println!("   No factories available (expected - factories not registered)");
        }
        Err(e) => {
            println!("   Error: {}", e);
        }
    }

    // 测试 Service 工厂
    println!("\n3. Testing Service factory with Send + Sync bounds:");

    match ServiceFactory::create("service_impl", FactoryFallback::NoFallback) {
        Ok((id, service)) => match service.execute() {
            Ok(result) => println!("   Service executed: {}", result),
            Err(e) => println!("   Service error: {}", e),
        },
        Err(FactoryError::FactoryNotFound(id)) => {
            println!("   Service factory not found for ID: '{}' (expected)", id);
        }
        Err(e) => {
            println!("   Error: {}", e);
        }
    }

    // 演示生成的工厂类型
    println!("\n4. Generated factory types:");
    println!(
        "   - ProductFactory: {}",
        std::any::type_name::<ProductFactory>()
    );
    println!(
        "   - ServiceFactory: {}",
        std::any::type_name::<ServiceFactory>()
    );

    println!("\n=== Example completed ===");
}

// 辅助函数演示宏生成的代码结构
#[allow(dead_code)]
fn demonstrate_generated_code() {
    // 宏会生成类似以下的代码：
    //
    // pub struct ProductFactory;
    //
    // impl ProductFactory {
    //     pub fn create(
    //         id: &str,
    //         strategy: FactoryFallback,
    //     ) -> Result<(&str, Box<dyn Product>), FactoryError> {
    //         use std::sync::LazyLock;
    //         use rust_patterns::{FactoryRegistry, SimpleFactory};
    //
    //         static FACTORY: LazyLock<SimpleFactory<dyn Product>> =
    //             LazyLock::new(FactoryRegistry::simple_factory);
    //
    //         FACTORY.create(id, strategy)
    //     }
    // }
}
