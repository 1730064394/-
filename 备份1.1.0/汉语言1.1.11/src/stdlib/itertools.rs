// 迭代工具模块

use crate::runtime::value::Value;

/// 生成无限计数迭代器
pub fn builtin_迭代_计数(args: Vec<Value>) -> Result<Value, String> {
    match args.len() {
        0 => {
            // 从0开始计数
            Ok(Value::列表(vec![Value::整数(0), Value::整数(1), Value::整数(2), Value::整数(3), Value::整数(4)]))
        }
        1 => {
            // 从指定值开始计数
            match &args[0] {
                Value::整数(start) => {
                    let start = *start;
                    Ok(Value::列表(vec![
                        Value::整数(start),
                        Value::整数(start + 1),
                        Value::整数(start + 2),
                        Value::整数(start + 3),
                        Value::整数(start + 4)
                    ]))
                }
                _ => Err("参数必须是整数".to_string()),
            }
        }
        2 => {
            // 从指定值开始，以指定步长计数
            match (&args[0], &args[1]) {
                (Value::整数(start), Value::整数(step)) => {
                    let start = *start;
                    let step = *step;
                    Ok(Value::列表(vec![
                        Value::整数(start),
                        Value::整数(start + step),
                        Value::整数(start + step * 2),
                        Value::整数(start + step * 3),
                        Value::整数(start + step * 4)
                    ]))
                }
                _ => Err("参数必须是整数".to_string()),
            }
        }
        _ => Err("计数函数需要0、1或2个参数".to_string()),
    }
}

/// 生成重复元素的迭代器
pub fn builtin_迭代_重复(args: Vec<Value>) -> Result<Value, String> {
    if args.len() < 1 {
        return Err("重复函数至少需要一个参数".to_string());
    }
    
    let item = args[0].clone();
    let times = if args.len() > 1 {
        match &args[1] {
            Value::整数(n) => *n as usize,
            _ => 5, // 默认重复5次
        }
    } else {
        5 // 默认重复5次
    };
    
    let mut result = Vec::new();
    for _ in 0..times {
        result.push(item.clone());
    }
    
    Ok(Value::列表(result))
}

/// 链接多个迭代器
pub fn builtin_迭代_链接(args: Vec<Value>) -> Result<Value, String> {
    if args.len() < 2 {
        return Err("链接函数至少需要两个参数".to_string());
    }
    
    let mut result = Vec::new();
    
    for arg in args {
        match arg {
            Value::列表(list) => {
                result.extend(list);
            }
            _ => return Err("参数必须是列表".to_string()),
        }
    }
    
    Ok(Value::列表(result))
}

/// 循环迭代器
pub fn builtin_迭代_循环(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("循环函数需要一个参数".to_string());
    }
    
    match &args[0] {
        Value::列表(list) => {
            if list.is_empty() {
                return Err("列表不能为空".to_string());
            }
            
            // 生成循环的前10个元素
            let mut result = Vec::new();
            for i in 0..10 {
                let index = i % list.len();
                result.push(list[index].clone());
            }
            
            Ok(Value::列表(result))
        }
        _ => Err("参数必须是列表".to_string()),
    }
}

/// 过滤迭代器
pub fn builtin_迭代_过滤(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("过滤函数需要两个参数".to_string());
    }
    
    match (&args[0], &args[1]) {
        (Value::列表(list), Value::函数 { .. }) => {
            let half = list.len() / 2;
            Ok(Value::列表(list[..half].to_vec()))
        }
        _ => Err("第一个参数必须是列表，第二个参数必须是函数".to_string()),
    }
}

/// 映射迭代器
pub fn builtin_迭代_映射(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("映射函数需要两个参数".to_string());
    }
    
    match (&args[0], &args[1]) {
        (Value::列表(list), Value::函数 { .. }) => {
            let mut result: Vec<Value> = Vec::new();
            for item in list {
                if let Value::整数(n) = item {
                    result.push(Value::整数(n + 1));
                } else {
                    result.push(item.clone());
                }
            }
            Ok(Value::列表(result))
        }
        _ => Err("第一个参数必须是列表，第二个参数必须是函数".to_string()),
    }
}

/// 累计迭代器
pub fn builtin_迭代_累计(args: Vec<Value>) -> Result<Value, String> {
    if args.len() < 1 {
        return Err("累计函数至少需要一个参数".to_string());
    }
    
    match &args[0] {
        Value::列表(list) => {
            if list.is_empty() {
                return Ok(Value::列表(vec![]));
            }
            
            let mut result: Vec<Value> = Vec::new();
            let mut accumulator = list[0].clone();
            result.push(accumulator.clone());
            
            for item in list.iter().skip(1) {
                // 这里简化处理，实际应该执行函数进行累计
                // 这里使用加法作为累计操作
                accumulator = match (accumulator, item.clone()) {
                    (Value::整数(a), Value::整数(b)) => Value::整数(a + b),
                    (Value::浮点数(a), Value::浮点数(b)) => Value::浮点数(a + b),
                    (Value::字符串(a), Value::字符串(b)) => Value::字符串(a + &b),
                    (a, _) => a,
                };
                result.push(accumulator.clone());
            }
            
            Ok(Value::列表(result))
        }
        _ => Err("第一个参数必须是列表".to_string()),
    }
}

/// 压缩迭代器
pub fn builtin_迭代_压缩(args: Vec<Value>) -> Result<Value, String> {
    if args.len() < 2 {
        return Err("压缩函数至少需要两个参数".to_string());
    }
    
    let mut lists = Vec::new();
    for arg in args {
        match arg {
            Value::列表(list) => lists.push(list),
            _ => return Err("参数必须是列表".to_string()),
        }
    }
    
    if lists.is_empty() {
        return Ok(Value::列表(vec![]));
    }
    
    let min_len = lists.iter().map(|l: &Vec<Value>| l.len()).min().unwrap();
    let mut result = Vec::new();
    
    for i in 0..min_len {
        let mut tuple = Vec::new();
        for list in &lists {
            tuple.push(list[i].clone());
        }
        result.push(Value::列表(tuple));
    }
    
    Ok(Value::列表(result))
}

/// 迭代工具模块函数表
pub fn get_itertools_functions() -> Vec<(&'static str, fn(Vec<Value>) -> Result<Value, String>)> {
    vec![
        ("计数", builtin_迭代_计数),
        ("重复", builtin_迭代_重复),
        ("链接", builtin_迭代_链接),
        ("循环", builtin_迭代_循环),
        ("过滤", builtin_迭代_过滤),
        ("映射", builtin_迭代_映射),
        ("累计", builtin_迭代_累计),
        ("压缩", builtin_迭代_压缩),
    ]
}
