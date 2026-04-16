# rust-pattern-macros

[![Crates.io](https://img.shields.io/crates/v/rust-pattern-macros.svg)](https://crates.io/crates/rust-pattern-macros)
[![Documentation](https://docs.rs/rust-pattern-macros/badge.svg)](https://docs.rs/rust-pattern-macros)
[![License](https://img.shields.io/crates/l/rust-pattern-macros.svg)](https://crates.io/crates/rust-pattern-macros)

`rust-patterns` 库的过程宏扩展。这个 crate 提供了属性宏来简化 Rust 中设计模式的实现。

## 安装

在 `Cargo.toml` 中添加：

```toml
[dependencies]
rust-pattern-macros = "0.1"
rust-patterns = "0.1"  # 生成的代码需要这个依赖
```

## 提供的宏

### `#[simple_factory]`

为 trait 生成简单工厂实现。

#### 使用方法

```rust
use rust_pattern_macros::simple_factory;

#[simple_factory]
pub trait MyTrait {
    fn do_something(&self);
}

// 这会生成：
// 1. 原始的 trait 定义
// 2. 名为 `MyTraitFactory` 的工厂结构体
// 3. 工厂的 `create` 方法实现
```

你也可以指定额外的 trait bound：

```rust
use rust_pattern_macros::simple_factory;

#[simple_factory(Send + Sync)]
pub trait MyTrait {
    fn do_something(&self);
}
```

#### 生成的代码

宏会生成一个工厂结构体，包含以下 `create` 方法：

```rust,ignore
pub fn create(
    id: &str,
    strategy: rust_patterns::FactoryFallback,
) -> Result<(&str, Box<dyn MyTrait>), rust_patterns::FactoryError>
```

工厂使用全局静态工厂实例，通过 `LazyLock` 实现线程安全的懒加载初始化。

## 系统要求

- Rust 1.70 或更高版本（需要 `std::sync::LazyLock`）
- `rust-patterns` crate 必须在你的项目中可用

## 许可证

本项目采用 MIT 许可证。

- MIT 许可证 ([LICENSE](LICENSE) 或 http://opensource.org/licenses/MIT)

## 贡献

除非你明确声明，否则根据 MIT 许可证的定义，你提交的任何贡献都将按上述许可证授权，无需任何附加条款或条件。
