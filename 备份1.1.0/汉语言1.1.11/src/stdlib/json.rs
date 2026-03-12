use crate::error::{RuntimeError, RuntimeResult};
use crate::runtime::value::Value;

use serde_json;

fn value_to_json(value: &Value) -> Result<serde_json::Value, RuntimeError> {
    match value {
        Value::空值 => Ok(serde_json::Value::Null),
        Value::布尔值(b) => Ok(serde_json::Value::Bool(*b)),
        Value::整数(i) => Ok(serde_json::Value::Number(serde_json::Number::from(*i))),
        Value::浮点数(f) => {
            if f.is_finite() {
                Ok(serde_json::Value::Number(serde_json::Number::from_f64(*f).ok_or_else(|| 
                    RuntimeError::General("浮点数转换失败".to_string())
                )?))
            } else {
                Err(RuntimeError::General("不支持的浮点数值".to_string()))
            }
        }
        Value::字符串(s) => Ok(serde_json::Value::String(s.clone())),
        Value::列表(list) => {
            let mut arr = vec![];
            for item in list {
                arr.push(value_to_json(item)?);
            }
            Ok(serde_json::Value::Array(arr))
        }
        Value::字典(map) => {
            let mut obj = serde_json::Map::new();
            for (key, value) in map {
                obj.insert(key.clone(), value_to_json(value)?);
            }
            Ok(serde_json::Value::Object(obj))
        }
        Value::集合(list) => {
            let mut arr = vec![];
            for item in list {
                arr.push(value_to_json(item)?);
            }
            Ok(serde_json::Value::Array(arr))
        }
        Value::内置函数 { .. } | Value::函数 { .. } | Value::异步函数 { .. } => {
            Err(RuntimeError::General("函数无法序列化为JSON".to_string()))
        }
        Value::对象 { .. } | Value::类 { .. } => {
            Err(RuntimeError::General("对象或类无法序列化为JSON".to_string()))
        }
        Value::生成器 { .. } => {
            Err(RuntimeError::General("生成器无法序列化为JSON".to_string()))
        }
        Value::Promise { .. } => {
            Err(RuntimeError::General("Promise无法序列化为JSON".to_string()))
        }
    }
}

fn json_to_value(json: &serde_json::Value) -> Value {
    match json {
        serde_json::Value::Null => Value::空值,
        serde_json::Value::Bool(b) => Value::布尔值(*b),
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Value::整数(i)
            } else if let Some(f) = n.as_f64() {
                Value::浮点数(f)
            } else {
                Value::空值
            }
        }
        serde_json::Value::String(s) => Value::字符串(s.clone()),
        serde_json::Value::Array(arr) => {
            let mut list = vec![];
            for item in arr {
                list.push(json_to_value(item));
            }
            Value::列表(list)
        }
        serde_json::Value::Object(obj) => {
            let mut map = std::collections::HashMap::new();
            for (key, value) in obj {
                map.insert(key.clone(), json_to_value(value));
            }
            Value::字典(map)
        }
    }
}

pub fn builtin_json_stringify(args: Vec<Value>) -> RuntimeResult<Value> {
    if args.is_empty() {
        return Err(RuntimeError::ArgumentCountError {
            function: "JSON序列化".to_string(),
            expected: 1,
            actual: 0,
        });
    }

    let value = &args[0];
    let json = value_to_json(value)?;

    let result = if args.len() > 1 {
        serde_json::to_string_pretty(&json)
    } else {
        serde_json::to_string(&json)
    };

    match result {
        Ok(json_str) => Ok(Value::字符串(json_str)),
        Err(e) => Err(RuntimeError::General(format!("JSON序列化失败: {}", e))),
    }
}

pub fn builtin_json_parse(args: Vec<Value>) -> RuntimeResult<Value> {
    if args.is_empty() {
        return Err(RuntimeError::ArgumentCountError {
            function: "JSON解析".to_string(),
            expected: 1,
            actual: 0,
        });
    }

    let json_str = match &args[0] {
        Value::字符串(s) => s,
        _ => return Err(RuntimeError::TypeError("JSON字符串必须是字符串".to_string())),
    };

    match serde_json::from_str(json_str) {
        Ok(json) => Ok(json_to_value(&json)),
        Err(e) => Err(RuntimeError::General(format!("JSON解析失败: {}", e))),
    }
}

pub fn builtin_json_parse_file(args: Vec<Value>) -> RuntimeResult<Value> {
    if args.is_empty() {
        return Err(RuntimeError::ArgumentCountError {
            function: "JSON解析文件".to_string(),
            expected: 1,
            actual: 0,
        });
    }

    let file_path = match &args[0] {
        Value::字符串(s) => s,
        _ => return Err(RuntimeError::TypeError("文件路径必须是字符串".to_string())),
    };

    match std::fs::read_to_string(file_path) {
        Ok(content) => {
            match serde_json::from_str(&content) {
                Ok(json) => Ok(json_to_value(&json)),
                Err(e) => Err(RuntimeError::General(format!("JSON解析失败: {}", e))),
            }
        }
        Err(e) => Err(RuntimeError::General(format!("读取文件失败: {}", e))),
    }
}

pub fn builtin_json_stringify_file(args: Vec<Value>) -> RuntimeResult<Value> {
    if args.len() < 2 {
        return Err(RuntimeError::ArgumentCountError {
            function: "JSON序列化文件".to_string(),
            expected: 2,
            actual: args.len(),
        });
    }

    let file_path = match &args[0] {
        Value::字符串(s) => s,
        _ => return Err(RuntimeError::TypeError("文件路径必须是字符串".to_string())),
    };

    let value = &args[1];
    let json = value_to_json(value)?;

    let content = if args.len() > 2 {
        serde_json::to_string_pretty(&json)
    } else {
        serde_json::to_string(&json)
    };

    match content {
        Ok(content) => {
            match std::fs::write(file_path, content) {
                Ok(_) => Ok(Value::布尔值(true)),
                Err(e) => Err(RuntimeError::General(format!("写入文件失败: {}", e))),
            }
        }
        Err(e) => Err(RuntimeError::General(format!("JSON序列化失败: {}", e))),
    }
}

pub fn builtin_json_validate(args: Vec<Value>) -> RuntimeResult<Value> {
    if args.is_empty() {
        return Err(RuntimeError::ArgumentCountError {
            function: "JSON验证".to_string(),
            expected: 1,
            actual: 0,
        });
    }

    let json_str = match &args[0] {
        Value::字符串(s) => s,
        _ => return Err(RuntimeError::TypeError("JSON字符串必须是字符串".to_string())),
    };

    match serde_json::from_str::<serde_json::Value>(json_str) {
        Ok(_) => Ok(Value::布尔值(true)),
        Err(_) => Ok(Value::布尔值(false)),
    }
}

pub fn builtin_json_get(args: Vec<Value>) -> RuntimeResult<Value> {
    if args.len() < 2 {
        return Err(RuntimeError::ArgumentCountError {
            function: "JSON获取".to_string(),
            expected: 2,
            actual: args.len(),
        });
    }

    let json_str = match &args[0] {
        Value::字符串(s) => s,
        _ => return Err(RuntimeError::TypeError("JSON字符串必须是字符串".to_string())),
    };

    let path = match &args[1] {
        Value::字符串(s) => s,
        _ => return Err(RuntimeError::TypeError("路径必须是字符串".to_string())),
    };

    match serde_json::from_str::<serde_json::Value>(json_str) {
        Ok(json) => {
            let mut current = &json;
            let parts: Vec<&str> = path.split(".").collect();
            
            for part in parts {
                current = match current {
                    serde_json::Value::Object(obj) => obj.get(part).ok_or_else(|| 
                        RuntimeError::General(format!("路径不存在: {}", part))
                    )?,
                    _ => return Err(RuntimeError::General("路径格式错误".to_string())),
                };
            }
            
            Ok(json_to_value(current))
        }
        Err(e) => Err(RuntimeError::General(format!("JSON解析失败: {}", e))),
    }
}

pub fn builtin_json_set(args: Vec<Value>) -> RuntimeResult<Value> {
    if args.len() < 3 {
        return Err(RuntimeError::ArgumentCountError {
            function: "JSON设置".to_string(),
            expected: 3,
            actual: args.len(),
        });
    }

    let json_str = match &args[0] {
        Value::字符串(s) => s,
        _ => return Err(RuntimeError::TypeError("JSON字符串必须是字符串".to_string())),
    };

    let path = match &args[1] {
        Value::字符串(s) => s,
        _ => return Err(RuntimeError::TypeError("路径必须是字符串".to_string())),
    };

    let value = &args[2];
    let new_value = value_to_json(value)?;

    match serde_json::from_str::<serde_json::Value>(json_str) {
        Ok(mut json) => {
            let parts: Vec<&str> = path.split(".").collect();
            let last_index = parts.len() - 1;
            
            let mut current = &mut json;
            for (i, part) in parts.iter().enumerate() {
                if i == last_index {
                    if let serde_json::Value::Object(obj) = current {
                        obj.insert(part.to_string(), new_value.clone());
                    } else {
                        return Err(RuntimeError::General("路径格式错误".to_string()));
                    }
                } else {
                    current = match current {
                        serde_json::Value::Object(obj) => {
                            if !obj.contains_key(*part) {
                                obj.insert(part.to_string(), serde_json::Value::Object(serde_json::Map::new()));
                            }
                            obj.get_mut(*part).unwrap()
                        }
                        _ => return Err(RuntimeError::General("路径格式错误".to_string())),
                    };
                }
            }
            
            match serde_json::to_string(&json) {
                Ok(updated_json) => Ok(Value::字符串(updated_json)),
                Err(e) => Err(RuntimeError::General(format!("JSON序列化失败: {}", e))),
            }
        }
        Err(e) => Err(RuntimeError::General(format!("JSON解析失败: {}", e))),
    }
}
