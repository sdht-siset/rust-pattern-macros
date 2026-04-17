//! 演示 observable 宏的使用示例
#![allow(dead_code, unused_variables)]

use rust_pattern_macros::observable;
use rust_patterns::{Observable, Observer};
use std::sync::Arc;

// 示例 1: 基本用法
#[observable(state = u64, error = String)]
struct Counter {
    value: u64,
}

impl Counter {
    // 自定义构造函数
    #[allow(dead_code)]
    pub fn with_value(value: u64) -> Self {
        Self {
            value,
            registry: rust_patterns::ObserverRegistry::new(),
        }
    }

    pub fn increment(&mut self) {
        self.value += 1;
        // 通知观察者值已更新
        let _ = self.notify(&self.value);
    }
}

// 示例 2: 使用 anyhow::Error
// 示例 2: 使用 anyhow::Error（需要添加anyhow依赖）
// #[observable(state = String, error = anyhow::Error)]
#[allow(dead_code)]
struct Logger {
    log_level: u8,
}

// 示例 3: 使用自定义错误类型
#[derive(Debug)]
#[allow(dead_code)]
enum SensorError {
    InvalidReading,
    CommunicationFailed,
}

impl std::fmt::Display for SensorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SensorError::InvalidReading => write!(f, "Invalid sensor reading"),
            SensorError::CommunicationFailed => write!(f, "Sensor communication failed"),
        }
    }
}

impl std::error::Error for SensorError {}

#[observable(state = f64, error = SensorError)]
struct TemperatureSensor {
    temperature: f64,
}

// 观察者实现
struct CounterDisplay;

impl Observer for CounterDisplay {
    type Subject = Counter;

    fn update(&self, state: &u64) -> Result<(), String> {
        println!("Counter updated: {}", state);
        Ok(())
    }
}

#[allow(dead_code)]
struct TemperatureDisplay;

struct TemperatureObserver;

impl Observer for TemperatureObserver {
    type Subject = TemperatureSensor;

    fn update(&self, state: &f64) -> Result<(), SensorError> {
        println!("Temperature: {:.1}°C", state);
        Ok(())
    }
}

fn main() {
    // 测试 Counter
    let mut counter = Counter {
        value: 0,
        registry: rust_patterns::ObserverRegistry::new(),
    };
    let display = Arc::new(CounterDisplay);

    counter.attach(display.clone());
    counter.increment(); // 输出: Counter updated: 1
    counter.increment(); // 输出: Counter updated: 2

    // 测试 TemperatureSensor
    let mut sensor = TemperatureSensor {
        temperature: 20.0,
        registry: rust_patterns::ObserverRegistry::new(),
    };

    let temp_display = Arc::new(TemperatureObserver);
    sensor.attach(temp_display.clone());

    sensor.temperature = 25.5;
    let _ = sensor.notify(&sensor.temperature); // 输出: Temperature: 25.5°C

    // 测试通知策略
    sensor.temperature = 30.0;
    sensor.notify_ignore_error(&sensor.temperature); // 输出: Temperature: 30.0°C

    println!("All examples completed successfully!");
}
