// 随机数模块

use crate::runtime::value::Value;
use rand::prelude::*;
use std::sync::{Mutex, LazyLock};

static RNG: LazyLock<Mutex<StdRng>> = LazyLock::new(|| {
    Mutex::new(StdRng::from_entropy())
});

/// 生成随机整数
pub fn builtin_随机_整数(args: Vec<Value>) -> Result<Value, String> {
    match args.len() {
        0 => {
            // 生成任意整数
            let mut rng = RNG.lock().unwrap();
            Ok(Value::整数(rng.gen()))
        }
        1 => {
            // 生成0到max-1之间的整数
            match &args[0] {
                Value::整数(max) => {
                    if *max <= 0 {
                        return Err("最大值必须大于0".to_string());
                    }
                    let mut rng = RNG.lock().unwrap();
                    Ok(Value::整数(rng.gen_range(0..*max)))
                }
                _ => Err("参数必须是整数".to_string()),
            }
        }
        2 => {
            // 生成min到max-1之间的整数
            match (&args[0], &args[1]) {
                (Value::整数(min), Value::整数(max)) => {
                    if *min >= *max {
                        return Err("最小值必须小于最大值".to_string());
                    }
                    let mut rng = RNG.lock().unwrap();
                    Ok(Value::整数(rng.gen_range(*min..*max)))
                }
                _ => Err("参数必须是整数".to_string()),
            }
        }
        _ => Err("随机整数函数需要0、1或2个参数".to_string()),
    }
}

/// 生成随机浮点数
pub fn builtin_随机_浮点数(args: Vec<Value>) -> Result<Value, String> {
    match args.len() {
        0 => {
            // 生成0.0到1.0之间的浮点数
            let mut rng = RNG.lock().unwrap();
            Ok(Value::浮点数(rng.gen()))
        }
        1 => {
            // 生成0.0到max之间的浮点数
            match &args[0] {
                Value::浮点数(max) => {
                    if *max <= 0.0 {
                        return Err("最大值必须大于0".to_string());
                    }
                    let mut rng = RNG.lock().unwrap();
                    Ok(Value::浮点数(rng.gen_range(0.0..*max)))
                }
                Value::整数(max) => {
                    if *max <= 0 {
                        return Err("最大值必须大于0".to_string());
                    }
                    let mut rng = RNG.lock().unwrap();
                    Ok(Value::浮点数(rng.gen_range(0.0..(*max as f64))))
                }
                _ => Err("参数必须是数字".to_string()),
            }
        }
        2 => {
            // 生成min到max之间的浮点数
            match (&args[0], &args[1]) {
                (Value::浮点数(min), Value::浮点数(max)) => {
                    if *min >= *max {
                        return Err("最小值必须小于最大值".to_string());
                    }
                    let mut rng = RNG.lock().unwrap();
                    Ok(Value::浮点数(rng.gen_range(*min..*max)))
                }
                (Value::整数(min), Value::整数(max)) => {
                    if *min >= *max {
                        return Err("最小值必须小于最大值".to_string());
                    }
                    let mut rng = RNG.lock().unwrap();
                    Ok(Value::浮点数(rng.gen_range((*min as f64)..(*max as f64))))
                }
                (Value::整数(min), Value::浮点数(max)) => {
                    if (*min as f64) >= *max {
                        return Err("最小值必须小于最大值".to_string());
                    }
                    let mut rng = RNG.lock().unwrap();
                    Ok(Value::浮点数(rng.gen_range((*min as f64)..*max)))
                }
                (Value::浮点数(min), Value::整数(max)) => {
                    if *min >= (*max as f64) {
                        return Err("最小值必须小于最大值".to_string());
                    }
                    let mut rng = RNG.lock().unwrap();
                    Ok(Value::浮点数(rng.gen_range(*min..(*max as f64))))
                }
                _ => Err("参数必须是数字".to_string()),
            }
        }
        _ => Err("随机浮点数函数需要0、1或2个参数".to_string()),
    }
}

/// 从列表中随机选择元素
pub fn builtin_随机_选择(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("随机选择函数需要一个参数".to_string());
    }
    
    match &args[0] {
        Value::列表(list) => {
            if list.is_empty() {
                return Err("列表不能为空".to_string());
            }
            let mut rng = RNG.lock().unwrap();
            let index = rng.gen_range(0..list.len());
            Ok(list[index].clone())
        }
        _ => Err("参数必须是列表".to_string()),
    }
}

/// 随机打乱列表
pub fn builtin_随机_打乱(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("随机打乱函数需要一个参数".to_string());
    }
    
    match &args[0] {
        Value::列表(list) => {
            let mut shuffled = list.clone();
            let mut rng = RNG.lock().unwrap();
            shuffled.shuffle(&mut *rng);
            Ok(Value::列表(shuffled))
        }
        _ => Err("参数必须是列表".to_string()),
    }
}

/// 生成随机布尔值
pub fn builtin_随机_布尔值(_args: Vec<Value>) -> Result<Value, String> {
    let mut rng = RNG.lock().unwrap();
    Ok(Value::布尔值(rng.gen()))
}

/// 设置随机种子
pub fn builtin_随机_种子(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("随机种子函数需要一个参数".to_string());
    }
    
    match &args[0] {
        Value::整数(seed) => {
            let mut rng = RNG.lock().unwrap();
            *rng = StdRng::seed_from_u64(*seed as u64);
            Ok(Value::空值)
        }
        _ => Err("参数必须是整数".to_string()),
    }
}

/// 随机模块函数表
pub fn get_random_functions() -> Vec<(&'static str, fn(Vec<Value>) -> Result<Value, String>)> {
    vec![
        ("整数", builtin_随机_整数),
        ("浮点数", builtin_随机_浮点数),
        ("选择", builtin_随机_选择),
        ("打乱", builtin_随机_打乱),
        ("布尔值", builtin_随机_布尔值),
        ("种子", builtin_随机_种子),
    ]
}
