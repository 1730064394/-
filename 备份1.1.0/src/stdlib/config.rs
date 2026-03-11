use crate::error::{RuntimeError, RuntimeResult};
use crate::runtime::value::Value;
use std::collections::HashMap;
use std::fs;

pub fn builtin_读取INI配置(参数: Vec<Value>) -> RuntimeResult<Value> {
    if 参数.is_empty() {
        return Err(RuntimeError::General("读取INI配置需要一个参数：文件路径".to_string()));
    }
    
    let filepath = match &参数[0] {
        Value::字符串(s) => s.clone(),
        _ => return Err(RuntimeError::TypeError("文件路径必须是字符串".to_string())),
    };
    
    let content = fs::read_to_string(&filepath)
        .map_err(|e| RuntimeError::General(format!("读取配置文件失败: {}", e)))?;
    
    let mut result = HashMap::new();
    let mut current_section = "默认".to_string();
    
    for line in content.lines() {
        let line = line.trim();
        
        if line.is_empty() || line.starts_with(';') || line.starts_with('#') {
            continue;
        }
        
        if line.starts_with('[') && line.ends_with(']') {
            current_section = line[1..line.len()-1].to_string();
            result.insert(current_section.clone(), HashMap::new());
        } else if let Some(pos) = line.find('=') {
            let key = line[..pos].trim().to_string();
            let value = line[pos+1..].trim().to_string();
            
            if let Some(section) = result.get_mut(&current_section) {
                section.insert(key, Value::字符串(value));
            }
        }
    }
    
    let dict_result: HashMap<String, Value> = result.into_iter()
        .map(|(k, v)| (k, Value::字典(v)))
        .collect();
    
    Ok(Value::字典(dict_result))
}

pub fn builtin_写入INI配置(参数: Vec<Value>) -> RuntimeResult<Value> {
    if 参数.len() < 2 {
        return Err(RuntimeError::General("写入INI配置需要两个参数：文件路径和配置数据".to_string()));
    }
    
    let filepath = match &参数[0] {
        Value::字符串(s) => s.clone(),
        _ => return Err(RuntimeError::TypeError("文件路径必须是字符串".to_string())),
    };
    
    let config = match &参数[1] {
        Value::字典(map) => map.clone(),
        _ => return Err(RuntimeError::TypeError("配置数据必须是字典".to_string())),
    };
    
    let mut content = String::new();
    
    for (section, section_data) in &config {
        content.push_str(&format!("[{}]\n", section));
        
        if let Value::字典(items) = section_data {
            for (key, value) in items {
                content.push_str(&format!("{}={}\n", key, value.to_string_value()));
            }
        }
        content.push('\n');
    }
    
    fs::write(&filepath, content)
        .map_err(|e| RuntimeError::General(format!("写入配置文件失败: {}", e)))?;
    
    Ok(Value::布尔值(true))
}

pub fn builtin_读取TOML配置(参数: Vec<Value>) -> RuntimeResult<Value> {
    if 参数.is_empty() {
        return Err(RuntimeError::General("读取TOML配置需要一个参数：文件路径".to_string()));
    }
    
    let filepath = match &参数[0] {
        Value::字符串(s) => s.clone(),
        _ => return Err(RuntimeError::TypeError("文件路径必须是字符串".to_string())),
    };
    
    let content = fs::read_to_string(&filepath)
        .map_err(|e| RuntimeError::General(format!("读取配置文件失败: {}", e)))?;
    
    parse_toml(&content)
}

pub fn builtin_写入TOML配置(参数: Vec<Value>) -> RuntimeResult<Value> {
    if 参数.len() < 2 {
        return Err(RuntimeError::General("写入TOML配置需要两个参数：文件路径和配置数据".to_string()));
    }
    
    let filepath = match &参数[0] {
        Value::字符串(s) => s.clone(),
        _ => return Err(RuntimeError::TypeError("文件路径必须是字符串".to_string())),
    };
    
    let config = match &参数[1] {
        Value::字典(map) => map.clone(),
        _ => return Err(RuntimeError::TypeError("配置数据必须是字典".to_string())),
    };
    
    let content = toml_to_string(&config);
    
    fs::write(&filepath, content)
        .map_err(|e| RuntimeError::General(format!("写入配置文件失败: {}", e)))?;
    
    Ok(Value::布尔值(true))
}

pub fn builtin_解析配置字符串(参数: Vec<Value>) -> RuntimeResult<Value> {
    if 参数.is_empty() {
        return Err(RuntimeError::General("解析配置字符串需要一个参数：配置字符串".to_string()));
    }
    
    let content = match &参数[0] {
        Value::字符串(s) => s.clone(),
        _ => return Err(RuntimeError::TypeError("配置字符串必须是字符串".to_string())),
    };
    
    let format_type = if 参数.len() > 1 {
        match &参数[1] {
            Value::字符串(s) => s.to_lowercase(),
            _ => "ini".to_string(),
        }
    } else {
        "ini".to_string()
    };
    
    if format_type == "toml" {
        parse_toml(&content)
    } else {
        let mut result = HashMap::new();
        let mut current_section = "默认".to_string();
        
        for line in content.lines() {
            let line = line.trim();
            
            if line.is_empty() || line.starts_with(';') || line.starts_with('#') {
                continue;
            }
            
            if line.starts_with('[') && line.ends_with(']') {
                current_section = line[1..line.len()-1].to_string();
                result.insert(current_section.clone(), HashMap::new());
            } else if let Some(pos) = line.find('=') {
                let key = line[..pos].trim().to_string();
                let value = line[pos+1..].trim().to_string();
                
                if let Some(section) = result.get_mut(&current_section) {
                    section.insert(key, Value::字符串(value));
                }
            }
        }
        
        let dict_result: HashMap<String, Value> = result.into_iter()
            .map(|(k, v)| (k, Value::字典(v)))
            .collect();
        
        Ok(Value::字典(dict_result))
    }
}

fn parse_toml(content: &str) -> RuntimeResult<Value> {
    let mut result = HashMap::new();
    let mut current_section = "默认".to_string();
    result.insert(current_section.clone(), HashMap::new());
    
    for line in content.lines() {
        let line = line.trim();
        
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        
        if line.starts_with('[') && line.ends_with(']') {
            current_section = line[1..line.len()-1].to_string();
            result.insert(current_section.clone(), HashMap::new());
        } else if let Some(pos) = line.find('=') {
            let key = line[..pos].trim().to_string();
            let value_str = line[pos+1..].trim();
            
            let value = parse_toml_value(value_str);
            
            if let Some(section) = result.get_mut(&current_section) {
                section.insert(key, value);
            }
        }
    }
    
    let dict_result: HashMap<String, Value> = result.into_iter()
        .map(|(k, v)| (k, Value::字典(v)))
        .collect();
    
    Ok(Value::字典(dict_result))
}

fn parse_toml_value(s: &str) -> Value {
    let s = s.trim();
    
    if s == "true" {
        return Value::布尔值(true);
    }
    if s == "false" {
        return Value::布尔值(false);
    }
    
    if s.starts_with('"') && s.ends_with('"') {
        return Value::字符串(s[1..s.len()-1].to_string());
    }
    
    if s.starts_with('\'') && s.ends_with('\'') {
        return Value::字符串(s[1..s.len()-1].to_string());
    }
    
    if let Ok(n) = s.parse::<i64>() {
        return Value::整数(n);
    }
    
    if let Ok(n) = s.parse::<f64>() {
        return Value::浮点数(n);
    }
    
    Value::字符串(s.to_string())
}

fn toml_to_string(config: &HashMap<String, Value>) -> String {
    let mut result = String::new();
    
    for (section, section_data) in config {
        result.push_str(&format!("[{}]\n", section));
        
        if let Value::字典(items) = section_data {
            for (key, value) in items {
                let value_str = match value {
                    Value::字符串(s) => format!("\"{}\"", s),
                    Value::整数(n) => n.to_string(),
                    Value::浮点数(n) => n.to_string(),
                    Value::布尔值(b) => b.to_string(),
                    _ => value.to_string_value(),
                };
                result.push_str(&format!("{} = {}\n", key, value_str));
            }
        }
        result.push('\n');
    }
    
    result
}
