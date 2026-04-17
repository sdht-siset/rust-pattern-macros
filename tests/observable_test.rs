//! observable 宏的集成测试
#![allow(dead_code, unused_variables)]

use rust_pattern_macros::observable;
use rust_patterns::{Observable, Observer, ObserverRegistry};
use std::sync::Arc;

// 外部模块，用于测试字段可见性
mod external {
    use rust_pattern_macros::observable;

    #[observable(state = u32, error = String)]
    pub struct ExternalObservable {
        pub data: u32,
    }
}

// 测试基本功能
#[observable(state = u32, error = String)]
struct TestObservable {
    data: u32,
}

// 测试观察者
struct TestObserver {
    id: u32,
    received_values: Vec<u32>,
}

impl Observer for TestObserver {
    type Subject = TestObservable;

    fn update(&self, state: &u32) -> Result<(), String> {
        // 在实际测试中，我们会记录接收到的值
        Ok(())
    }
}

#[test]
fn test_observable_macro_basic() {
    // 测试结构体是否被正确扩展
    let observable = TestObservable {
        data: 0,
        registry: ObserverRegistry::new(),
    };

    // 验证 registry 字段存在
    let _registry = &observable.registry;

    // 验证 Observable trait 已实现
    fn assert_observable<T: Observable<State = u32, Error = String>>(_: &T) {}
    assert_observable(&observable);

    println!("Basic observable macro test passed");
}

#[test]
fn test_observable_attach_detach() {
    let mut observable = TestObservable {
        data: 0,
        registry: rust_patterns::ObserverRegistry::new(),
    };
    let observer = Arc::new(TestObserver {
        id: 1,
        received_values: Vec::new(),
    });

    // 测试附加观察者
    observable.attach(observer.clone());

    // 测试分离观察者
    observable.detach(observer);

    println!("Attach/detach test passed");
}

#[test]
fn test_observable_notify() {
    let observable = TestObservable {
        data: 0,
        registry: ObserverRegistry::new(),
    };

    // 测试通知方法存在
    let result = observable.notify(&42);
    assert!(result.is_ok());

    println!("Notify test passed");
}

#[test]
fn test_observable_notify_ignore_error() {
    let observable = TestObservable {
        data: 0,
        registry: ObserverRegistry::new(),
    };

    // 测试忽略错误的通知方法
    let result = observable.notify_ignore_error(&42);
    assert!(result.is_ok());

    println!("Notify ignore error test passed");
}

// 测试错误类型
#[derive(Debug)]
enum TestError {
    SomeError,
}

impl std::fmt::Display for TestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TestError")
    }
}

impl std::error::Error for TestError {}

// 测试自定义错误类型
#[observable(state = String, error = TestError)]
struct CustomErrorObservable {
    message: String,
}

#[test]
fn test_custom_error_type() {
    let observable = CustomErrorObservable {
        message: String::new(),
        registry: ObserverRegistry::new(),
    };

    // 验证错误类型正确
    fn assert_custom_error_observable<T: Observable<State = String, Error = TestError>>(_: &T) {}
    assert_custom_error_observable(&observable);

    println!("Custom error type test passed");
}

// 测试多个参数
#[observable(state = f64, error = std::io::Error)]
struct MultiParamObservable {
    value: f64,
}

#[test]
fn test_multi_param_observable() {
    let observable = MultiParamObservable {
        value: 0.0,
        registry: ObserverRegistry::new(),
    };

    // 验证类型参数正确
    fn assert_multi_param_observable<T: Observable<State = f64, Error = std::io::Error>>(_: &T) {}
    assert_multi_param_observable(&observable);

    println!("Multi-parameter observable test passed");
}

#[test]
fn test_registry_field_visibility() {
    // 在同一个模块中，可以访问 registry 字段
    let observable = TestObservable {
        data: 0,
        registry: ObserverRegistry::new(),
    };
    let _registry = &observable.registry; // 这应该可以编译

    // 测试从外部模块无法直接访问 registry 字段
    // 注意：由于我们还在同一个文件中，这只是一个概念验证
    // 在实际使用中，如果 observable 被导出到其他模块，
    // registry 字段将是私有的

    println!("Registry field visibility test passed");
}

// 主测试函数
fn main() {
    println!("Running observable macro tests...");

    test_observable_macro_basic();
    test_observable_attach_detach();
    test_observable_notify();
    test_observable_notify_ignore_error();
    test_custom_error_type();
    test_multi_param_observable();
    test_registry_field_visibility();

    println!("All tests passed!");
}
