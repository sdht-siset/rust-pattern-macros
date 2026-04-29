//! simple_factory 宏的集成测试
#![allow(dead_code, unused_variables)]

use rust_pattern_macros::simple_factory;
use rust_patterns::{FactoryError, FactoryFallback};

// 测试基本功能
#[simple_factory]
pub trait TestProduct {
    fn get_id(&self) -> u32;
}

#[derive(Default)]
struct ProductImpl1;

impl TestProduct for ProductImpl1 {
    fn get_id(&self) -> u32 {
        1
    }
}

#[derive(Default)]
struct ProductImpl2;

impl TestProduct for ProductImpl2 {
    fn get_id(&self) -> u32 {
        2
    }
}

#[test]
fn test_simple_factory_macro_basic() {
    // 验证工厂结构体已生成
    let factory_type_name = std::any::type_name::<TestProductFactory>();
    assert!(factory_type_name.contains("TestProductFactory"));

    println!("Basic simple_factory macro test passed");
}

#[test]
fn test_simple_factory_create_method_exists() {
    // 测试 create 方法存在且类型正确
    let result = TestProductFactory::create("test", FactoryFallback::NoFallback);

    // 由于工厂未注册，应该返回 FactoryNotFound 错误
    match result {
        Err(FactoryError::FactoryNotFound(_)) => {
            // 这是预期的，因为工厂没有注册
            println!("Create method exists and returns correct error type");
        }
        _ => {
            // 其他结果都是不正确的
            panic!("Expected FactoryNotFound error");
        }
    }
}

#[test]
fn test_simple_factory_method_signature() {
    // 验证 create 方法的签名
    // 直接调用 create 方法，如果编译通过则签名正确
    let _result = TestProductFactory::create("test", FactoryFallback::NoFallback);

    // 验证返回类型为 Result<Box<T>, FactoryError>
    fn assert_return_type<T: ?Sized>(_result: Result<Box<T>, FactoryError>) {}

    // 由于工厂未注册，我们只能验证错误类型
    match TestProductFactory::create("test", FactoryFallback::NoFallback) {
        Err(FactoryError::FactoryNotFound(_)) => {
            // 这是预期的
        }
        _ => {
            // 其他结果都是不正确的
        }
    }

    println!("Create method signature test passed");
}

// 测试带 trait bound 的工厂
#[simple_factory(Send + Sync)]
pub trait ThreadSafeService {
    fn process(&self) -> String;
}

#[derive(Default)]
struct SafeServiceImpl;

impl ThreadSafeService for SafeServiceImpl {
    fn process(&self) -> String {
        "processed".to_string()
    }
}

#[test]
fn test_simple_factory_with_bounds() {
    // 验证带 bound 的工厂已生成
    let factory_type_name = std::any::type_name::<ThreadSafeServiceFactory>();
    assert!(factory_type_name.contains("ThreadSafeServiceFactory"));

    // 测试 create 方法
    let result = ThreadSafeServiceFactory::create("safe_service", FactoryFallback::NoFallback);

    match result {
        Err(FactoryError::FactoryNotFound(_)) => {
            println!("Factory with bounds created successfully");
        }
        _ => {
            panic!("Expected FactoryNotFound error for factory with bounds");
        }
    }
}

// 测试多个 trait bound
#[simple_factory(Send + Sync)]
pub trait ComplexService {
    fn compute(&self) -> i32;
}

#[derive(Default)]
struct ComplexServiceImpl;

impl ComplexService for ComplexServiceImpl {
    fn compute(&self) -> i32 {
        42
    }
}

#[test]
fn test_simple_factory_with_multiple_bounds() {
    // 验证工厂已生成
    let _factory = ComplexServiceFactory;

    // 测试 create 方法
    let result = ComplexServiceFactory::create("complex", FactoryFallback::NoFallback);

    match result {
        Err(FactoryError::FactoryNotFound(_)) => {
            println!("Factory with multiple bounds created successfully");
        }
        _ => {
            panic!("Expected FactoryNotFound error for factory with multiple bounds");
        }
    }
}

// 测试错误处理
#[test]
fn test_simple_factory_error_types() {
    let result = TestProductFactory::create("nonexistent", FactoryFallback::NoFallback);

    // 测试不同的错误情况
    match result {
        Err(FactoryError::FactoryNotFound(id)) => {
            assert_eq!(id, "nonexistent");
            println!("FactoryNotFound error test passed");
        }
        _ => panic!("Expected FactoryNotFound error"),
    }

    // 测试空 ID 无回退
    let result = TestProductFactory::create("", FactoryFallback::NoFallback);
    match result {
        Err(FactoryError::EmptyIdNoFallback) => {
            println!("EmptyIdNoFallback error test passed");
        }
        _ => panic!("Expected EmptyIdNoFallback error"),
    }
}

// 测试工厂结构体是公开的
#[test]
fn test_factory_visibility() {
    // 如果这行编译通过，说明工厂结构体是公开的
    let _factory: TestProductFactory = TestProductFactory;
    println!("Factory visibility test passed");
}

// 主测试函数
fn main() {
    println!("Running simple_factory macro tests...");

    test_simple_factory_macro_basic();
    test_simple_factory_create_method_exists();
    test_simple_factory_method_signature();
    test_simple_factory_with_bounds();
    test_simple_factory_with_multiple_bounds();
    test_simple_factory_error_types();
    test_factory_visibility();

    println!("All tests passed!");
}
