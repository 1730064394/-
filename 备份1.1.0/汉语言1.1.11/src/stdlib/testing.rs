use crate::error::{RuntimeError, RuntimeResult};
use crate::runtime::value::Value;
use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;

thread_local! {
    static TEST_RESULTS: RefCell<TestResults> = RefCell::new(TestResults::new());
}

struct TestResults {
    passed: usize,
    failed: usize,
    errors: Vec<String>,
}

impl TestResults {
    fn new() -> Self {
        Self {
            passed: 0,
            failed: 0,
            errors: Vec::new(),
        }
    }
}

pub fn builtin_断言相等(参数: Vec<Value>) -> RuntimeResult<Value> {
    if 参数.len() != 2 {
        return Err(RuntimeError::General("断言相等需要两个参数".to_string()));
    }
    
    let left = &参数[0];
    let right = &参数[1];
    
    if left == right {
        TEST_RESULTS.with(|tr| {
            tr.borrow_mut().passed += 1;
        });
        Ok(Value::布尔值(true))
    } else {
        let msg = format!("断言失败: 期望 {:?}, 实际 {:?}", right, left);
        TEST_RESULTS.with(|tr| {
            tr.borrow_mut().failed += 1;
            tr.borrow_mut().errors.push(msg.clone());
        });
        Err(RuntimeError::General(msg))
    }
}

pub fn builtin_断言不相等(参数: Vec<Value>) -> RuntimeResult<Value> {
    if 参数.len() != 2 {
        return Err(RuntimeError::General("断言不相等需要两个参数".to_string()));
    }
    
    let left = &参数[0];
    let right = &参数[1];
    
    if left != right {
        TEST_RESULTS.with(|tr| {
            tr.borrow_mut().passed += 1;
        });
        Ok(Value::布尔值(true))
    } else {
        let msg = format!("断言失败: 两个值不应该相等: {:?}", left);
        TEST_RESULTS.with(|tr| {
            tr.borrow_mut().failed += 1;
            tr.borrow_mut().errors.push(msg.clone());
        });
        Err(RuntimeError::General(msg))
    }
}

pub fn builtin_断言为真(参数: Vec<Value>) -> RuntimeResult<Value> {
    if 参数.len() != 1 {
        return Err(RuntimeError::General("断言为真需要一个参数".to_string()));
    }
    
    match &参数[0] {
        Value::布尔值(true) => {
            TEST_RESULTS.with(|tr| {
                tr.borrow_mut().passed += 1;
            });
            Ok(Value::布尔值(true))
        }
        _ => {
            let msg = format!("断言失败: 期望为真, 实际 {:?}", 参数[0]);
            TEST_RESULTS.with(|tr| {
                tr.borrow_mut().failed += 1;
                tr.borrow_mut().errors.push(msg.clone());
            });
            Err(RuntimeError::General(msg))
        }
    }
}

pub fn builtin_断言为假(参数: Vec<Value>) -> RuntimeResult<Value> {
    if 参数.len() != 1 {
        return Err(RuntimeError::General("断言为假需要一个参数".to_string()));
    }
    
    match &参数[0] {
        Value::布尔值(false) => {
            TEST_RESULTS.with(|tr| {
                tr.borrow_mut().passed += 1;
            });
            Ok(Value::布尔值(true))
        }
        _ => {
            let msg = format!("断言失败: 期望为假, 实际 {:?}", 参数[0]);
            TEST_RESULTS.with(|tr| {
                tr.borrow_mut().failed += 1;
                tr.borrow_mut().errors.push(msg.clone());
            });
            Err(RuntimeError::General(msg))
        }
    }
}

pub fn builtin_断言为空(参数: Vec<Value>) -> RuntimeResult<Value> {
    if 参数.len() != 1 {
        return Err(RuntimeError::General("断言为空需要一个参数".to_string()));
    }
    
    match &参数[0] {
        Value::空值 => {
            TEST_RESULTS.with(|tr| {
                tr.borrow_mut().passed += 1;
            });
            Ok(Value::布尔值(true))
        }
        _ => {
            let msg = format!("断言失败: 期望为空, 实际 {:?}", 参数[0]);
            TEST_RESULTS.with(|tr| {
                tr.borrow_mut().failed += 1;
                tr.borrow_mut().errors.push(msg.clone());
            });
            Err(RuntimeError::General(msg))
        }
    }
}

pub fn builtin_断言不为空(参数: Vec<Value>) -> RuntimeResult<Value> {
    if 参数.len() != 1 {
        return Err(RuntimeError::General("断言不为空需要一个参数".to_string()));
    }
    
    match &参数[0] {
        Value::空值 => {
            let msg = "断言失败: 期望不为空".to_string();
            TEST_RESULTS.with(|tr| {
                tr.borrow_mut().failed += 1;
                tr.borrow_mut().errors.push(msg.clone());
            });
            Err(RuntimeError::General(msg))
        }
        _ => {
            TEST_RESULTS.with(|tr| {
                tr.borrow_mut().passed += 1;
            });
            Ok(Value::布尔值(true))
        }
    }
}

pub fn builtin_断言包含(参数: Vec<Value>) -> RuntimeResult<Value> {
    if 参数.len() != 2 {
        return Err(RuntimeError::General("断言包含需要两个参数".to_string()));
    }
    
    let container = &参数[0];
    let item = &参数[1];
    
    let contains = match container {
        Value::字符串(s) => {
            match item {
                Value::字符串(sub) => s.contains(sub),
                _ => false,
            }
        }
        Value::列表(items) => items.contains(item),
        _ => false,
    };
    
    if contains {
        TEST_RESULTS.with(|tr| {
            tr.borrow_mut().passed += 1;
        });
        Ok(Value::布尔值(true))
    } else {
        let msg = format!("断言失败: {:?} 不包含 {:?}", container, item);
        TEST_RESULTS.with(|tr| {
            tr.borrow_mut().failed += 1;
            tr.borrow_mut().errors.push(msg.clone());
        });
        Err(RuntimeError::General(msg))
    }
}

pub fn builtin_断言类型(参数: Vec<Value>) -> RuntimeResult<Value> {
    if 参数.len() != 2 {
        return Err(RuntimeError::General("断言类型需要两个参数".to_string()));
    }
    
    let value = &参数[0];
    let expected_type = match &参数[1] {
        Value::字符串(s) => s.clone(),
        _ => return Err(RuntimeError::TypeError("类型名必须是字符串".to_string())),
    };
    
    let actual_type = value.type_name();
    
    if actual_type == expected_type {
        TEST_RESULTS.with(|tr| {
            tr.borrow_mut().passed += 1;
        });
        Ok(Value::布尔值(true))
    } else {
        let msg = format!("断言失败: 期望类型 {}, 实际类型 {}", expected_type, actual_type);
        TEST_RESULTS.with(|tr| {
            tr.borrow_mut().failed += 1;
            tr.borrow_mut().errors.push(msg.clone());
        });
        Err(RuntimeError::General(msg))
    }
}

pub fn builtin_获取测试结果(参数: Vec<Value>) -> RuntimeResult<Value> {
    if !参数.is_empty() {
        return Err(RuntimeError::General("获取测试结果不需要参数".to_string()));
    }
    
    TEST_RESULTS.with(|tr| {
        let tr = tr.borrow();
        let mut map = HashMap::new();
        map.insert("通过".to_string(), Value::整数(tr.passed as i64));
        map.insert("失败".to_string(), Value::整数(tr.failed as i64));
        map.insert("总数".to_string(), Value::整数((tr.passed + tr.failed) as i64));
        
        let errors: Vec<Value> = tr.errors.iter().map(|e| Value::字符串(e.clone())).collect();
        map.insert("错误列表".to_string(), Value::列表(errors));
        
        Ok(Value::字典(map))
    })
}

pub fn builtin_重置测试结果(参数: Vec<Value>) -> RuntimeResult<Value> {
    if !参数.is_empty() {
        return Err(RuntimeError::General("重置测试结果不需要参数".to_string()));
    }
    
    TEST_RESULTS.with(|tr| {
        *tr.borrow_mut() = TestResults::new();
    });
    
    Ok(Value::布尔值(true))
}
