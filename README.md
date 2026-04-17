# Rust Pattern Macros

[![Crates.io](https://img.shields.io/crates/v/rust-pattern-macros.svg)](https://crates.io/crates/rust-pattern-macros)
[![Documentation](https://docs.rs/rust-pattern-macros/badge.svg)](https://docs.rs/rust-pattern-macros)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Build Status](https://github.com/sdht-siset/rust-pattern-macros/actions/workflows/rust.yml/badge.svg)](https://github.com/sdht-siset/rust-pattern-macros/actions)

[![Crates.io](https://img.shields.io/crates/v/rust-pattern-macros.svg)](https://crates.io/crates/rust-pattern-macros)
[![Documentation](https://docs.rs/rust-pattern-macros/badge.svg)](https://docs.rs/rust-pattern-macros)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Build Status](https://github.com/sdht-siset/rust-pattern-macros/actions/workflows/rust.yml/badge.svg)](https://github.com/sdht-siset/rust-pattern-macros/actions)

`rust-pattern-macros` 是 `rust-patterns` 库的过程宏扩展，提供了属性宏来简化 Rust 中设计模式的实现。

## 特性

- **零成本抽象**：生成的代码经过优化，几乎没有运行时开销
- **线程安全**：所有生成的代码都是线程安全的
- **易于使用**：简洁的 API 设计，提供丰富的文档和示例
- **生产就绪**：经过充分测试，包含单元测试和文档测试

## 安装

在 `Cargo.toml` 中添加：

```toml
[dependencies]
rust-patterns = "0.1.1"
rust-pattern-macros = "0.1.3"
```

## 包含的宏

### 1. `#[simple_factory]` - 简单工厂模式

为 trait 生成简单工厂实现，自动创建工厂结构体和创建方法。

#### 基本用法

```rust
use rust_pattern_macros::simple_factory;

#[simple_factory]
pub trait Product {
    fn name(&self) -> &str;
    fn price(&self) -> f64;
}

// 宏会生成 ProductFactory 结构体
// 可以使用 ProductFactory::create() 方法创建产品
```

#### 带 trait bound 的用法

```rust
#[simple_factory(Send + Sync)]
pub trait Service {
    fn execute(&self) -> Result<String, String>;
}

// 生成的工厂会确保创建的对象满足 Send + Sync 约束
```

#### 生成的代码

宏会生成以下内容：
1. 原始的 trait 定义
2. 工厂结构体 `{TraitName}Factory`
3. `create` 方法实现，使用全局静态工厂实例

`create` 方法签名：
```rust
pub fn create(
    id: &str,
    strategy: rust_patterns::FactoryFallback,
) -> Result<(&str, Box<dyn Trait>), rust_patterns::FactoryError>
```

### 2. `#[observable]` - 观察者模式

为结构体自动实现 `Observable` trait，添加观察者管理功能。

#### 基本用法

```rust
use rust_pattern_macros::observable;

#[observable(state = u64, error = String)]
struct Counter {
    value: u64,
}

// 手动初始化（需要提供 registry 字段）
let mut counter = Counter {
    value: 0,
    registry: rust_patterns::ObserverRegistry::new(),
};

// 现在可以使用 attach、detach、notify 等方法
```

#### 自定义错误类型

```rust
#[derive(Debug)]
enum SensorError {
    InvalidReading,
    CommunicationFailed,
}

impl std::fmt::Display for SensorError { /* ... */ }
impl std::error::Error for SensorError { /* ... */ }

#[observable(state = f64, error = SensorError)]
struct TemperatureSensor {
    temperature: f64,
}
```

#### 生成的代码

宏会为结构体生成：
1. 添加 `registry: ObserverRegistry<Self>` 字段
2. 实现 `Observable` trait，设置 `State` 和 `Error` 关联类型
3. 实现 `attach` 和 `detach` 方法
4. 提供两个通知方法：
   - `notify(&self, state: &State)` - 使用 StopOnError 策略
   - `notify_ignore_error(&self, state: &State)` - 使用 IgnoreError 策略

#### 设计特点
- `registry` 字段使用结构体的可见性
- 不提供 `Default` 实现，结构体初始化由用户负责
- 只支持具名字段的结构体

## 示例

查看 `examples/` 目录获取完整的使用示例：

```bash
# 运行 observable 示例
cargo run --example observable_example

# 运行 simple_factory 示例
cargo run --example simple_factory_example
```

## API 文档

完整的 API 文档可在 [docs.rs](https://docs.rs/rust-pattern-macros) 查看。

## 运行测试

```bash
# 运行所有测试
cargo test

# 运行文档测试
cargo test --doc

# 运行特定宏的测试
cargo test --test observable_test
cargo test --test simple_factory_test
```

## 系统要求

- Rust 1.70 或更高版本（需要 `std::sync::LazyLock`）
- `rust-patterns` crate 必须在你的项目中可用

## 贡献指南

欢迎贡献！请遵循以下步骤：

1. Fork 项目
2. 创建功能分支 (`git checkout -b feature/amazing-feature`)
3. 提交更改 (`git commit -m 'Add some amazing feature'`)
4. 推送到分支 (`git push origin feature/amazing-feature`)
5. 打开 Pull Request

## 许可证

本项目基于 MIT 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情。

## 作者

- **Linshan Yang** - [yanglsh@yeah.net](mailto:yanglsh@yeah.net)

## 致谢

- 感谢所有贡献者和用户
- 灵感来源于经典的设计模式书籍和 Rust 社区的最佳实践

## 版本信息

查看 [crates.io](https://crates.io/crates/rust-pattern-macros) 获取最新版本，或访问 [GitHub Releases](https://github.com/sdht-siset/rust-pattern-macros/releases) 查看版本历史。

## 相关项目

- [rust-patterns](https://github.com/sdht-siset/rust-patterns) - 统一的设计模式库
- [inventory](https://crates.io/crates/inventory) - 编译时注册系统
- [thiserror](https://crates.io/crates/thiserror) - 错误处理库

---

**提示**: 本项目正在积极开发中，API 可能会发生变化。建议在生产环境中使用时锁定特定版本。