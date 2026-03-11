use crate::error::{RuntimeError, RuntimeResult};
use crate::runtime::value::Value;
use std::collections::HashMap;

pub fn builtin_stack_new(_args: Vec<Value>) -> RuntimeResult<Value> {
    Ok(Value::列表(Vec::new()))
}

pub fn builtin_stack_push(args: Vec<Value>) -> RuntimeResult<Value> {
    if args.len() < 2 {
        return Err(RuntimeError::ArgumentCountError {
            function: "栈推入".to_string(),
            expected: 2,
            actual: args.len(),
        });
    }
    
    match &args[0] {
        Value::列表(v) => {
            let mut new_stack = v.clone();
            new_stack.push(args[1].clone());
            Ok(Value::列表(new_stack))
        }
        _ => Err(RuntimeError::TypeError("第一个参数必须是列表".to_string())),
    }
}

pub fn builtin_stack_pop(args: Vec<Value>) -> RuntimeResult<Value> {
    if args.is_empty() {
        return Err(RuntimeError::ArgumentCountError {
            function: "栈弹出".to_string(),
            expected: 1,
            actual: 0,
        });
    }
    
    match &args[0] {
        Value::列表(v) if !v.is_empty() => {
            let mut new_stack = v.clone();
            let last = new_stack.pop().unwrap();
            Ok(last)
        }
        Value::列表(_) => Err(RuntimeError::General("栈为空".to_string())),
        _ => Err(RuntimeError::TypeError("第一个参数必须是列表".to_string())),
    }
}

pub fn builtin_stack_peek(args: Vec<Value>) -> RuntimeResult<Value> {
    if args.is_empty() {
        return Err(RuntimeError::ArgumentCountError {
            function: "栈顶元素".to_string(),
            expected: 1,
            actual: 0,
        });
    }
    
    match &args[0] {
        Value::列表(v) if !v.is_empty() => Ok(v.last().unwrap().clone()),
        Value::列表(_) => Err(RuntimeError::General("栈为空".to_string())),
        _ => Err(RuntimeError::TypeError("第一个参数必须是列表".to_string())),
    }
}

pub fn builtin_queue_new(_args: Vec<Value>) -> RuntimeResult<Value> {
    Ok(Value::列表(Vec::new()))
}

pub fn builtin_queue_enqueue(args: Vec<Value>) -> RuntimeResult<Value> {
    if args.len() < 2 {
        return Err(RuntimeError::ArgumentCountError {
            function: "队列入队".to_string(),
            expected: 2,
            actual: args.len(),
        });
    }
    
    match &args[0] {
        Value::列表(v) => {
            let mut new_queue = v.clone();
            new_queue.push(args[1].clone());
            Ok(Value::列表(new_queue))
        }
        _ => Err(RuntimeError::TypeError("第一个参数必须是列表".to_string())),
    }
}

pub fn builtin_queue_dequeue(args: Vec<Value>) -> RuntimeResult<Value> {
    if args.is_empty() {
        return Err(RuntimeError::ArgumentCountError {
            function: "队列出队".to_string(),
            expected: 1,
            actual: 0,
        });
    }
    
    match &args[0] {
        Value::列表(v) if !v.is_empty() => {
            let mut new_queue = v.clone();
            let first = new_queue.remove(0);
            Ok(first)
        }
        Value::列表(_) => Err(RuntimeError::General("队列为空".to_string())),
        _ => Err(RuntimeError::TypeError("第一个参数必须是列表".to_string())),
    }
}

pub fn builtin_queue_peek(args: Vec<Value>) -> RuntimeResult<Value> {
    if args.is_empty() {
        return Err(RuntimeError::ArgumentCountError {
            function: "队列首元素".to_string(),
            expected: 1,
            actual: 0,
        });
    }
    
    match &args[0] {
        Value::列表(v) if !v.is_empty() => Ok(v.first().unwrap().clone()),
        Value::列表(_) => Err(RuntimeError::General("队列为空".to_string())),
        _ => Err(RuntimeError::TypeError("第一个参数必须是列表".to_string())),
    }
}

pub fn builtin_hashmap_new(_args: Vec<Value>) -> RuntimeResult<Value> {
    Ok(Value::字典(HashMap::new()))
}

pub fn builtin_hashmap_set(args: Vec<Value>) -> RuntimeResult<Value> {
    if args.len() < 3 {
        return Err(RuntimeError::ArgumentCountError {
            function: "哈希表设置".to_string(),
            expected: 3,
            actual: args.len(),
        });
    }
    
    match &args[0] {
        Value::字典(m) => {
            let mut new_map = m.clone();
            let key = args[1].to_string_value();
            new_map.insert(key, args[2].clone());
            Ok(Value::字典(new_map))
        }
        _ => Err(RuntimeError::TypeError("第一个参数必须是字典".to_string())),
    }
}

pub fn builtin_hashmap_get(args: Vec<Value>) -> RuntimeResult<Value> {
    if args.len() < 2 {
        return Err(RuntimeError::ArgumentCountError {
            function: "哈希表获取".to_string(),
            expected: 2,
            actual: args.len(),
        });
    }
    
    match &args[0] {
        Value::字典(m) => {
            let key = args[1].to_string_value();
            Ok(m.get(&key).cloned().unwrap_or(Value::空值))
        }
        _ => Err(RuntimeError::TypeError("第一个参数必须是字典".to_string())),
    }
}

pub fn builtin_hashmap_has(args: Vec<Value>) -> RuntimeResult<Value> {
    if args.len() < 2 {
        return Err(RuntimeError::ArgumentCountError {
            function: "哈希表包含".to_string(),
            expected: 2,
            actual: args.len(),
        });
    }
    
    match &args[0] {
        Value::字典(m) => {
            let key = args[1].to_string_value();
            Ok(Value::布尔值(m.contains_key(&key)))
        }
        _ => Err(RuntimeError::TypeError("第一个参数必须是字典".to_string())),
    }
}

pub fn builtin_hashmap_remove(args: Vec<Value>) -> RuntimeResult<Value> {
    if args.len() < 2 {
        return Err(RuntimeError::ArgumentCountError {
            function: "哈希表删除".to_string(),
            expected: 2,
            actual: args.len(),
        });
    }
    
    match &args[0] {
        Value::字典(m) => {
            let mut new_map = m.clone();
            let key = args[1].to_string_value();
            new_map.remove(&key);
            Ok(Value::字典(new_map))
        }
        _ => Err(RuntimeError::TypeError("第一个参数必须是字典".to_string())),
    }
}

pub fn builtin_hashmap_keys(args: Vec<Value>) -> RuntimeResult<Value> {
    if args.is_empty() {
        return Err(RuntimeError::ArgumentCountError {
            function: "哈希表键列表".to_string(),
            expected: 1,
            actual: 0,
        });
    }
    
    match &args[0] {
        Value::字典(m) => {
            let keys: Vec<Value> = m.keys().map(|k| Value::字符串(k.clone())).collect();
            Ok(Value::列表(keys))
        }
        _ => Err(RuntimeError::TypeError("第一个参数必须是字典".to_string())),
    }
}

pub fn builtin_hashmap_values(args: Vec<Value>) -> RuntimeResult<Value> {
    if args.is_empty() {
        return Err(RuntimeError::ArgumentCountError {
            function: "哈希表值列表".to_string(),
            expected: 1,
            actual: 0,
        });
    }
    
    match &args[0] {
        Value::字典(m) => Ok(Value::列表(m.values().cloned().collect())),
        _ => Err(RuntimeError::TypeError("第一个参数必须是字典".to_string())),
    }
}

pub fn builtin_hashmap_size(args: Vec<Value>) -> RuntimeResult<Value> {
    if args.is_empty() {
        return Err(RuntimeError::ArgumentCountError {
            function: "哈希表大小".to_string(),
            expected: 1,
            actual: 0,
        });
    }
    
    match &args[0] {
        Value::字典(m) => Ok(Value::整数(m.len() as i64)),
        _ => Err(RuntimeError::TypeError("第一个参数必须是字典".to_string())),
    }
}

pub fn builtin_hashmap_clear(args: Vec<Value>) -> RuntimeResult<Value> {
    if args.is_empty() {
        return Err(RuntimeError::ArgumentCountError {
            function: "哈希表清空".to_string(),
            expected: 1,
            actual: 0,
        });
    }
    
    match &args[0] {
        Value::字典(_) => Ok(Value::字典(HashMap::new())),
        _ => Err(RuntimeError::TypeError("第一个参数必须是字典".to_string())),
    }
}
