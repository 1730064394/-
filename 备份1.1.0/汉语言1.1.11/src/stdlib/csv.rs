use crate::error::{RuntimeError, RuntimeResult};
use crate::runtime::value::Value;

pub fn builtin_CSV读取(参数: Vec<Value>) -> RuntimeResult<Value> {
    if 参数.len() < 1 {
        return Err(RuntimeError::General("CSV读取需要一个参数：文件路径".to_string()));
    }
    
    let filepath = match &参数[0] {
        Value::字符串(s) => s,
        _ => return Err(RuntimeError::TypeError("参数必须是字符串".to_string())),
    };
    
    let content = std::fs::read_to_string(filepath)
        .map_err(|e| RuntimeError::General(format!("读取文件失败: {}", e)))?;
    
    let delimiter = if 参数.len() > 1 {
        match &参数[1] {
            Value::字符串(s) if !s.is_empty() => s.chars().next().unwrap(),
            _ => ',',
        }
    } else {
        ','
    };
    
    let has_header = 参数.len() > 2 && match &参数[2] {
        Value::布尔值(b) => *b,
        _ => true,
    };
    
    let mut result = Vec::new();
    let mut lines = content.lines();
    
    let headers: Vec<String> = if has_header {
        if let Some(first_line) = lines.next() {
            parse_csv_line(first_line, delimiter)
        } else {
            vec![]
        }
    } else {
        vec![]
    };
    
    for line in lines {
        let values = parse_csv_line(line, delimiter);
        if has_header && !headers.is_empty() {
            let mut row = std::collections::HashMap::new();
            for (i, value) in values.iter().enumerate() {
                if i < headers.len() {
                    row.insert(headers[i].clone(), Value::字符串(value.clone()));
                }
            }
            result.push(Value::字典(row));
        } else {
            result.push(Value::列表(values.iter().map(|v| Value::字符串(v.clone())).collect()));
        }
    }
    
    Ok(Value::列表(result))
}

pub fn builtin_CSV写入(参数: Vec<Value>) -> RuntimeResult<Value> {
    if 参数.len() < 2 {
        return Err(RuntimeError::General("CSV写入需要两个参数：文件路径和数据".to_string()));
    }
    
    let filepath = match &参数[0] {
        Value::字符串(s) => s,
        _ => return Err(RuntimeError::TypeError("第一个参数必须是字符串".to_string())),
    };
    
    let delimiter = if 参数.len() > 2 {
        match &参数[2] {
            Value::字符串(s) if !s.is_empty() => s.chars().next().unwrap(),
            _ => ',',
        }
    } else {
        ','
    };
    
    let mut content = String::new();
    
    match &参数[1] {
        Value::列表(rows) => {
            for (i, row) in rows.iter().enumerate() {
                match row {
                    Value::列表(values) => {
                        let line: Vec<String> = values.iter().map(|v| {
                            let s = v.to_string_value();
                            if s.contains(delimiter) || s.contains('"') || s.contains('\n') {
                                format!("\"{}\"", s.replace('"', "\"\""))
                            } else {
                                s
                            }
                        }).collect();
                        content.push_str(&line.join(&delimiter.to_string()));
                    }
                    Value::字典(map) => {
                        let values: Vec<String> = map.values().map(|v| {
                            let s = v.to_string_value();
                            if s.contains(delimiter) || s.contains('"') || s.contains('\n') {
                                format!("\"{}\"", s.replace('"', "\"\""))
                            } else {
                                s
                            }
                        }).collect();
                        content.push_str(&values.join(&delimiter.to_string()));
                    }
                    _ => {}
                }
                if i < rows.len() - 1 {
                    content.push('\n');
                }
            }
        }
        _ => return Err(RuntimeError::TypeError("第二个参数必须是列表".to_string())),
    }
    
    std::fs::write(filepath, content)
        .map_err(|e| RuntimeError::General(format!("写入文件失败: {}", e)))?;
    
    Ok(Value::布尔值(true))
}

pub fn builtin_CSV解析(参数: Vec<Value>) -> RuntimeResult<Value> {
    if 参数.len() < 1 {
        return Err(RuntimeError::General("CSV解析需要一个参数：CSV字符串".to_string()));
    }
    
    let content = match &参数[0] {
        Value::字符串(s) => s,
        _ => return Err(RuntimeError::TypeError("参数必须是字符串".to_string())),
    };
    
    let delimiter = if 参数.len() > 1 {
        match &参数[1] {
            Value::字符串(s) if !s.is_empty() => s.chars().next().unwrap(),
            _ => ',',
        }
    } else {
        ','
    };
    
    let mut result = Vec::new();
    for line in content.lines() {
        let values = parse_csv_line(line, delimiter);
        result.push(Value::列表(values.iter().map(|v| Value::字符串(v.clone())).collect()));
    }
    
    Ok(Value::列表(result))
}

pub fn builtin_CSV生成(参数: Vec<Value>) -> RuntimeResult<Value> {
    if 参数.len() < 1 {
        return Err(RuntimeError::General("CSV生成需要一个参数：数据列表".to_string()));
    }
    
    let delimiter = if 参数.len() > 1 {
        match &参数[1] {
            Value::字符串(s) if !s.is_empty() => s.chars().next().unwrap(),
            _ => ',',
        }
    } else {
        ','
    };
    
    let mut lines = Vec::new();
    
    match &参数[0] {
        Value::列表(rows) => {
            for row in rows {
                match row {
                    Value::列表(values) => {
                        let line: Vec<String> = values.iter().map(|v| {
                            let s = v.to_string_value();
                            if s.contains(delimiter) || s.contains('"') || s.contains('\n') {
                                format!("\"{}\"", s.replace('"', "\"\""))
                            } else {
                                s
                            }
                        }).collect();
                        lines.push(line.join(&delimiter.to_string()));
                    }
                    Value::字典(map) => {
                        let values: Vec<String> = map.values().map(|v| {
                            let s = v.to_string_value();
                            if s.contains(delimiter) || s.contains('"') || s.contains('\n') {
                                format!("\"{}\"", s.replace('"', "\"\""))
                            } else {
                                s
                            }
                        }).collect();
                        lines.push(values.join(&delimiter.to_string()));
                    }
                    _ => {}
                }
            }
        }
        _ => return Err(RuntimeError::TypeError("参数必须是列表".to_string())),
    }
    
    Ok(Value::字符串(lines.join("\n")))
}

fn parse_csv_line(line: &str, delimiter: char) -> Vec<String> {
    let mut values = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;
    let chars: Vec<char> = line.chars().collect();
    let mut i = 0;
    
    while i < chars.len() {
        let c = chars[i];
        
        if in_quotes {
            if c == '"' {
                if i + 1 < chars.len() && chars[i + 1] == '"' {
                    current.push('"');
                    i += 1;
                } else {
                    in_quotes = false;
                }
            } else {
                current.push(c);
            }
        } else {
            if c == '"' {
                in_quotes = true;
            } else if c == delimiter {
                values.push(current.trim().to_string());
                current = String::new();
            } else {
                current.push(c);
            }
        }
        i += 1;
    }
    
    values.push(current.trim().to_string());
    values
}
