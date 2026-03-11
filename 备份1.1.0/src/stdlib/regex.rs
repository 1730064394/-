use crate::error::{RuntimeError, RuntimeResult};
use crate::runtime::value::Value;
use regex::Regex;

pub fn builtin_正则_编译(参数: Vec<Value>) -> RuntimeResult<Value> {
    if 参数.len() != 1 {
        return Err(RuntimeError::General("正则编译需要一个参数".to_string()));
    }
    
    let pattern = match &参数[0] {
        Value::字符串(s) => s,
        _ => return Err(RuntimeError::TypeError("参数必须是字符串".to_string())),
    };
    
    match Regex::new(pattern) {
        Ok(_) => Ok(Value::字符串(pattern.clone())),
        Err(e) => Err(RuntimeError::General(format!("正则表达式编译失败: {}", e))),
    }
}

pub fn builtin_正则_匹配(参数: Vec<Value>) -> RuntimeResult<Value> {
    if 参数.len() != 2 {
        return Err(RuntimeError::General("正则匹配需要两个参数".to_string()));
    }
    
    let pattern = match &参数[0] {
        Value::字符串(s) => s,
        _ => return Err(RuntimeError::TypeError("第一个参数必须是字符串".to_string())),
    };
    
    let text = match &参数[1] {
        Value::字符串(s) => s,
        _ => return Err(RuntimeError::TypeError("第二个参数必须是字符串".to_string())),
    };
    
    let re = Regex::new(pattern)
        .map_err(|e| RuntimeError::General(format!("正则表达式编译失败: {}", e)))?;
    
    Ok(Value::布尔值(re.is_match(text)))
}

pub fn builtin_正则_查找(参数: Vec<Value>) -> RuntimeResult<Value> {
    if 参数.len() != 2 {
        return Err(RuntimeError::General("正则查找需要两个参数".to_string()));
    }
    
    let pattern = match &参数[0] {
        Value::字符串(s) => s,
        _ => return Err(RuntimeError::TypeError("第一个参数必须是字符串".to_string())),
    };
    
    let text = match &参数[1] {
        Value::字符串(s) => s,
        _ => return Err(RuntimeError::TypeError("第二个参数必须是字符串".to_string())),
    };
    
    let re = Regex::new(pattern)
        .map_err(|e| RuntimeError::General(format!("正则表达式编译失败: {}", e)))?;
    
    let matches: Vec<Value> = re.find_iter(text)
        .map(|m| Value::字符串(m.as_str().to_string()))
        .collect();
    
    Ok(Value::列表(matches))
}

pub fn builtin_正则_查找全部(参数: Vec<Value>) -> RuntimeResult<Value> {
    builtin_正则_查找(参数)
}

pub fn builtin_正则_替换(参数: Vec<Value>) -> RuntimeResult<Value> {
    if 参数.len() < 3 {
        return Err(RuntimeError::General("正则替换需要三个参数".to_string()));
    }
    
    let pattern = match &参数[0] {
        Value::字符串(s) => s,
        _ => return Err(RuntimeError::TypeError("第一个参数必须是字符串".to_string())),
    };
    
    let replacement = match &参数[1] {
        Value::字符串(s) => s,
        _ => return Err(RuntimeError::TypeError("第二个参数必须是字符串".to_string())),
    };
    
    let text = match &参数[2] {
        Value::字符串(s) => s,
        _ => return Err(RuntimeError::TypeError("第三个参数必须是字符串".to_string())),
    };
    
    let re = Regex::new(pattern)
        .map_err(|e| RuntimeError::General(format!("正则表达式编译失败: {}", e)))?;
    
    let result = re.replace_all(text, replacement.as_str());
    Ok(Value::字符串(result.to_string()))
}

pub fn builtin_正则_分割(参数: Vec<Value>) -> RuntimeResult<Value> {
    if 参数.len() < 2 {
        return Err(RuntimeError::General("正则分割需要两个参数".to_string()));
    }
    
    let pattern = match &参数[0] {
        Value::字符串(s) => s,
        _ => return Err(RuntimeError::TypeError("第一个参数必须是字符串".to_string())),
    };
    
    let text = match &参数[1] {
        Value::字符串(s) => s,
        _ => return Err(RuntimeError::TypeError("第二个参数必须是字符串".to_string())),
    };
    
    let re = Regex::new(pattern)
        .map_err(|e| RuntimeError::General(format!("正则表达式编译失败: {}", e)))?;
    
    let parts: Vec<Value> = re.split(text)
        .map(|s| Value::字符串(s.to_string()))
        .collect();
    
    Ok(Value::列表(parts))
}

pub fn builtin_正则_提取分组(参数: Vec<Value>) -> RuntimeResult<Value> {
    if 参数.len() < 2 {
        return Err(RuntimeError::General("正则提取分组需要两个参数".to_string()));
    }
    
    let pattern = match &参数[0] {
        Value::字符串(s) => s,
        _ => return Err(RuntimeError::TypeError("第一个参数必须是字符串".to_string())),
    };
    
    let text = match &参数[1] {
        Value::字符串(s) => s,
        _ => return Err(RuntimeError::TypeError("第二个参数必须是字符串".to_string())),
    };
    
    let re = Regex::new(pattern)
        .map_err(|e| RuntimeError::General(format!("正则表达式编译失败: {}", e)))?;
    
    if let Some(caps) = re.captures(text) {
        let groups: Vec<Value> = caps.iter()
            .skip(1)
            .filter_map(|m| m.map(|s| Value::字符串(s.as_str().to_string())))
            .collect();
        Ok(Value::列表(groups))
    } else {
        Ok(Value::列表(vec![]))
    }
}

pub fn register_regex_functions(env: &mut crate::runtime::value::Environment) {
    env.define("正则编译".to_string(), Value::内置函数 {
        名称: "正则编译".to_string(),
        函数: builtin_正则_编译,
    });
    
    env.define("正则匹配".to_string(), Value::内置函数 {
        名称: "正则匹配".to_string(),
        函数: builtin_正则_匹配,
    });
    
    env.define("正则查找".to_string(), Value::内置函数 {
        名称: "正则查找".to_string(),
        函数: builtin_正则_查找,
    });
    
    env.define("正则查找全部".to_string(), Value::内置函数 {
        名称: "正则查找全部".to_string(),
        函数: builtin_正则_查找全部,
    });
    
    env.define("正则替换".to_string(), Value::内置函数 {
        名称: "正则替换".to_string(),
        函数: builtin_正则_替换,
    });
    
    env.define("正则分割".to_string(), Value::内置函数 {
        名称: "正则分割".to_string(),
        函数: builtin_正则_分割,
    });
    
    env.define("正则提取分组".to_string(), Value::内置函数 {
        名称: "正则提取分组".to_string(),
        函数: builtin_正则_提取分组,
    });
}
