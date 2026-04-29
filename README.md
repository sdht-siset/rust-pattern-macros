# Rust Pattern Macros

[![Crates.io](https://img.shields.io/crates/v/rust-pattern-macros.svg)](https://crates.io/crates/rust-pattern-macros)
[![Documentation](https://docs.rs/rust-pattern-macros/badge.svg)](https://docs.rs/rust-pattern-macros)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

`rust-pattern-macros` 提供过程宏，为 Rust 设计模式生成样板代码。

> **注意**：宏生成的代码引用 `::rust_patterns::` 路径，因此 **必须** 依赖 [`rust-patterns`](https://crates.io/crates/rust-patterns) crate。

## 安装

```toml
[dependencies]
rust-pattern-macros = "0.1"
rust-patterns = "0.1"
```

## 提供的宏

### 1. `#[simple_factory]`

为 trait 自动生成工厂结构体，使用 `LazyLock` 缓存全局工厂实例。

#### 基本用法

```rust
use rust_pattern_macros::simple_factory;

#[simple_factory]
pub trait Product {
    fn name(&self) -> &str;
}
```

宏会生成 `ProductFactory` 结构体，包含 `create` 方法：

| 生成的项 | 说明 |
|---------|------|
| `ProductFactory` | 工厂结构体 |
| `ProductFactory::create(id, strategy)` | 通过 ID 和回退策略创建产品 |

方法签名：

```rust,ignore
impl ProductFactory {
    pub fn create(
        id: impl AsRef<str>,
        strategy: rust_patterns::FactoryFallback,
    ) -> Result<Box<dyn Product>, rust_patterns::FactoryError>
}
```

#### 指定 trait bound

```rust
use rust_pattern_macros::simple_factory;

#[simple_factory(Send + Sync)]
pub trait Service {
    fn execute(&self) -> Result<String, String>;
}

// 生成：Result<Box<dyn Service + Send + Sync>, FactoryError>
```

#### 完整示例

```rust,ignore
use rust_pattern_macros::simple_factory;
use rust_patterns::{FactoryError, FactoryFallback, register_factory};

#[simple_factory]
pub trait Product {
    fn name(&self) -> &str;
}

#[derive(Default)]
struct ProductA;
impl Product for ProductA {
    fn name(&self) -> &str { "Product A" }
}

register_factory!(dyn Product, "product_a", ProductA);

fn main() {
    match ProductFactory::create("product_a", FactoryFallback::NoFallback) {
        Ok(product) => println!("创建产品: {}", product.name()),
        Err(FactoryError::FactoryNotFound(id)) => {
            println!("未找到工厂: {}", id);
        }
        Err(e) => println!("错误: {}", e),
    }
}
```

### 2. `#[observable]`

为结构体自动添加 `ObserverRegistry` 字段并实现 `Observable` trait。

#### 参数

| 参数 | 类型 | 说明 |
|------|------|------|
| `state` | 类型 | 被观察者的状态类型（必需） |
| `error` | 类型 | 观察者返回的错误类型（必需） |

#### 基本用法

```rust
use rust_pattern_macros::observable;

#[observable(state = u64, error = String)]
struct Counter {
    value: u64,
}

// 手动初始化
let mut counter = Counter {
    value: 0,
    registry: rust_patterns::ObserverRegistry::new(),
};
```

#### 生成的内容

| 生成的项 | 说明 |
|---------|------|
| `registry` 字段 | `ObserverRegistry<Self>` 类型，使用结构体的可见性 |
| `Observable` 实现 | `attach` / `detach` 委托给 `registry` |
| `notify(&self, state)` | 遇到错误立即停止并返回 |
| `notify_ignore_error(&self, state)` | 忽略错误，通知所有观察者 |

#### 完整示例

```rust
use std::sync::Arc;
use rust_pattern_macros::observable;
use rust_patterns::{Observer, Observable};

#[observable(state = f64, error = String)]
struct TemperatureSensor {
    temperature: f64,
}

impl TemperatureSensor {
    fn new(temp: f64) -> Self {
        Self {
            temperature: temp,
            registry: rust_patterns::ObserverRegistry::new(),
        }
    }

    fn set_temperature(&mut self, temp: f64) {
        self.temperature = temp;
        let _ = self.notify(&self.temperature);
    }
}

struct Display;
impl Observer for Display {
    type Subject = TemperatureSensor;
    fn update(&self, state: &f64) -> Result<(), String> {
        println!("温度: {}°C", state);
        Ok(())
    }
}

fn main() {
    let mut sensor = TemperatureSensor::new(25.0);
    sensor.attach(Arc::new(Display));
    sensor.set_temperature(30.0);
}
```

#### 限制

- 只支持具名字段的结构体（不支持元组结构体、单元结构体）
- 需要用户手动初始化 `registry` 字段
- `registry` 字段使用与结构体相同的可见性

## 运行测试

```bash
# 运行所有测试
cargo test

# 运行特定测试
cargo test --test simple_factory_test
cargo test --test observable_test
```

## 运行示例

```bash
cargo run --example simple_factory_example
cargo run --example observable_example
```

## 许可证

MIT © Linshan Yang

## 相关项目

- [rust-patterns](https://crates.io/crates/rust-patterns) — 统一的设计模式库（推荐使用）
- [rust-pattern-components](https://crates.io/crates/rust-pattern-components) — 运行时组件库