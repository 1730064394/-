use crate::error::{RuntimeError, RuntimeResult};
use crate::runtime::value::Value;
use std::collections::HashMap;

pub fn builtin_字符串分割(参数: Vec<Value>) -> RuntimeResult<Value> {
    if 参数.is_empty() {
        return Err(RuntimeError::ArgumentCountError {
            function: "字符串分割".to_string(),
            expected: 1,
            actual: 0,
        });
    }
    
    let 字符串 = match &参数[0] {
        Value::字符串(s) => s.clone(),
        _ => return Err(RuntimeError::TypeError("第一个参数必须是字符串".to_string())),
    };
    
    let 分隔符 = if 参数.len() > 1 {
        match &参数[1] {
            Value::字符串(s) => s.clone(),
            _ => return Err(RuntimeError::TypeError("分隔符必须是字符串".to_string())),
        }
    } else {
        " ".to_string()
    };
    
    let 最大分割数 = if 参数.len() > 2 {
        match &参数[2] {
            Value::整数(n) => Some(*n as usize),
            _ => return Err(RuntimeError::TypeError("最大分割数必须是整数".to_string())),
        }
    } else {
        None
    };
    
    let 结果 = if 最大分割数.is_some() {
        字符串.splitn(最大分割数.unwrap() + 1, &分隔符)
            .map(|s| Value::字符串(s.to_string()))
            .collect()
    } else {
        字符串.split(&分隔符)
            .map(|s| Value::字符串(s.to_string()))
            .collect()
    };
    
    Ok(Value::列表(结果))
}

pub fn builtin_字符串连接(参数: Vec<Value>) -> RuntimeResult<Value> {
    if 参数.len() < 2 {
        return Err(RuntimeError::ArgumentCountError {
            function: "字符串连接".to_string(),
            expected: 2,
            actual: 参数.len(),
        });
    }
    
    let 分隔符 = match &参数[0] {
        Value::字符串(s) => s.clone(),
        _ => return Err(RuntimeError::TypeError("分隔符必须是字符串".to_string())),
    };
    
    let 列表 = match &参数[1] {
        Value::列表(list) => list.clone(),
        _ => return Err(RuntimeError::TypeError("第二个参数必须是列表".to_string())),
    };
    
    let 字符串列表: Vec<String> = 列表.iter()
        .map(|v| v.to_string_value())
        .collect();
    
    Ok(Value::字符串(字符串列表.join(&分隔符)))
}

pub fn builtin_字符串替换(参数: Vec<Value>) -> RuntimeResult<Value> {
    if 参数.len() < 3 {
        return Err(RuntimeError::ArgumentCountError {
            function: "字符串替换".to_string(),
            expected: 3,
            actual: 参数.len(),
        });
    }
    
    let 字符串 = match &参数[0] {
        Value::字符串(s) => s.clone(),
        _ => return Err(RuntimeError::TypeError("第一个参数必须是字符串".to_string())),
    };
    
    let 旧字符串 = match &参数[1] {
        Value::字符串(s) => s.clone(),
        _ => return Err(RuntimeError::TypeError("旧字符串必须是字符串".to_string())),
    };
    
    let 新字符串 = match &参数[2] {
        Value::字符串(s) => s.clone(),
        _ => return Err(RuntimeError::TypeError("新字符串必须是字符串".to_string())),
    };
    
    let 替换次数 = if 参数.len() > 3 {
        match &参数[3] {
            Value::整数(n) => Some(*n as usize),
            _ => return Err(RuntimeError::TypeError("替换次数必须是整数".to_string())),
        }
    } else {
        None
    };
    
    let 结果 = if let Some(n) = 替换次数 {
        字符串.replacen(&旧字符串, &新字符串, n)
    } else {
        字符串.replace(&旧字符串, &新字符串)
    };
    
    Ok(Value::字符串(结果))
}

pub fn builtin_字符串去除空白(参数: Vec<Value>) -> RuntimeResult<Value> {
    if 参数.is_empty() {
        return Err(RuntimeError::ArgumentCountError {
            function: "字符串去除空白".to_string(),
            expected: 1,
            actual: 0,
        });
    }
    
    let 字符串 = match &参数[0] {
        Value::字符串(s) => s.clone(),
        _ => return Err(RuntimeError::TypeError("参数必须是字符串".to_string())),
    };
    
    let 模式 = if 参数.len() > 1 {
        match &参数[1] {
            Value::字符串(s) => s.as_str(),
            _ => "两端",
        }
    } else {
        "两端"
    };
    
    let 结果 = match 模式 {
        "左" | "前" | "开头" => 字符串.trim_start().to_string(),
        "右" | "后" | "结尾" => 字符串.trim_end().to_string(),
        _ => 字符串.trim().to_string(),
    };
    
    Ok(Value::字符串(结果))
}

pub fn builtin_字符串转大写(参数: Vec<Value>) -> RuntimeResult<Value> {
    if 参数.is_empty() {
        return Err(RuntimeError::ArgumentCountError {
            function: "字符串转大写".to_string(),
            expected: 1,
            actual: 0,
        });
    }
    
    let 字符串 = match &参数[0] {
        Value::字符串(s) => s.clone(),
        _ => return Err(RuntimeError::TypeError("参数必须是字符串".to_string())),
    };
    
    Ok(Value::字符串(字符串.to_uppercase()))
}

pub fn builtin_字符串转小写(参数: Vec<Value>) -> RuntimeResult<Value> {
    if 参数.is_empty() {
        return Err(RuntimeError::ArgumentCountError {
            function: "字符串转小写".to_string(),
            expected: 1,
            actual: 0,
        });
    }
    
    let 字符串 = match &参数[0] {
        Value::字符串(s) => s.clone(),
        _ => return Err(RuntimeError::TypeError("参数必须是字符串".to_string())),
    };
    
    Ok(Value::字符串(字符串.to_lowercase()))
}

pub fn builtin_字符串查找(参数: Vec<Value>) -> RuntimeResult<Value> {
    if 参数.len() < 2 {
        return Err(RuntimeError::ArgumentCountError {
            function: "字符串查找".to_string(),
            expected: 2,
            actual: 参数.len(),
        });
    }
    
    let 字符串 = match &参数[0] {
        Value::字符串(s) => s.clone(),
        _ => return Err(RuntimeError::TypeError("第一个参数必须是字符串".to_string())),
    };
    
    let 子串 = match &参数[1] {
        Value::字符串(s) => s.clone(),
        _ => return Err(RuntimeError::TypeError("子串必须是字符串".to_string())),
    };
    
    let 起始位置 = if 参数.len() > 2 {
        match &参数[2] {
            Value::整数(n) => *n as usize,
            _ => return Err(RuntimeError::TypeError("起始位置必须是整数".to_string())),
        }
    } else {
        0
    };
    
    match 字符串[起始位置..].find(&子串) {
        Some(pos) => Ok(Value::整数((起始位置 + pos) as i64)),
        None => Ok(Value::整数(-1)),
    }
}

pub fn builtin_字符串包含(参数: Vec<Value>) -> RuntimeResult<Value> {
    if 参数.len() < 2 {
        return Err(RuntimeError::ArgumentCountError {
            function: "字符串包含".to_string(),
            expected: 2,
            actual: 参数.len(),
        });
    }
    
    let 字符串 = match &参数[0] {
        Value::字符串(s) => s.clone(),
        _ => return Err(RuntimeError::TypeError("第一个参数必须是字符串".to_string())),
    };
    
    let 子串 = match &参数[1] {
        Value::字符串(s) => s.clone(),
        _ => return Err(RuntimeError::TypeError("子串必须是字符串".to_string())),
    };
    
    Ok(Value::布尔值(字符串.contains(&子串)))
}

pub fn builtin_字符串开头是(参数: Vec<Value>) -> RuntimeResult<Value> {
    if 参数.len() < 2 {
        return Err(RuntimeError::ArgumentCountError {
            function: "字符串开头是".to_string(),
            expected: 2,
            actual: 参数.len(),
        });
    }
    
    let 字符串 = match &参数[0] {
        Value::字符串(s) => s.clone(),
        _ => return Err(RuntimeError::TypeError("第一个参数必须是字符串".to_string())),
    };
    
    let 前缀 = match &参数[1] {
        Value::字符串(s) => s.clone(),
        _ => return Err(RuntimeError::TypeError("前缀必须是字符串".to_string())),
    };
    
    Ok(Value::布尔值(字符串.starts_with(&前缀)))
}

pub fn builtin_字符串结尾是(参数: Vec<Value>) -> RuntimeResult<Value> {
    if 参数.len() < 2 {
        return Err(RuntimeError::ArgumentCountError {
            function: "字符串结尾是".to_string(),
            expected: 2,
            actual: 参数.len(),
        });
    }
    
    let 字符串 = match &参数[0] {
        Value::字符串(s) => s.clone(),
        _ => return Err(RuntimeError::TypeError("第一个参数必须是字符串".to_string())),
    };
    
    let 后缀 = match &参数[1] {
        Value::字符串(s) => s.clone(),
        _ => return Err(RuntimeError::TypeError("后缀必须是字符串".to_string())),
    };
    
    Ok(Value::布尔值(字符串.ends_with(&后缀)))
}

pub fn builtin_字符串长度(参数: Vec<Value>) -> RuntimeResult<Value> {
    if 参数.is_empty() {
        return Err(RuntimeError::ArgumentCountError {
            function: "字符串长度".to_string(),
            expected: 1,
            actual: 0,
        });
    }
    
    let 字符串 = match &参数[0] {
        Value::字符串(s) => s.clone(),
        _ => return Err(RuntimeError::TypeError("参数必须是字符串".to_string())),
    };
    
    Ok(Value::整数(字符串.chars().count() as i64))
}

pub fn builtin_字符串重复(参数: Vec<Value>) -> RuntimeResult<Value> {
    if 参数.len() < 2 {
        return Err(RuntimeError::ArgumentCountError {
            function: "字符串重复".to_string(),
            expected: 2,
            actual: 参数.len(),
        });
    }
    
    let 字符串 = match &参数[0] {
        Value::字符串(s) => s.clone(),
        _ => return Err(RuntimeError::TypeError("第一个参数必须是字符串".to_string())),
    };
    
    let 次数 = match &参数[1] {
        Value::整数(n) => *n as usize,
        _ => return Err(RuntimeError::TypeError("次数必须是整数".to_string())),
    };
    
    Ok(Value::字符串(字符串.repeat(次数)))
}

pub fn builtin_字符串反转(参数: Vec<Value>) -> RuntimeResult<Value> {
    if 参数.is_empty() {
        return Err(RuntimeError::ArgumentCountError {
            function: "字符串反转".to_string(),
            expected: 1,
            actual: 0,
        });
    }
    
    let 字符串 = match &参数[0] {
        Value::字符串(s) => s.clone(),
        _ => return Err(RuntimeError::TypeError("参数必须是字符串".to_string())),
    };
    
    let 反转 = 字符串.chars().rev().collect::<String>();
    Ok(Value::字符串(反转))
}

pub fn builtin_字符串截取(参数: Vec<Value>) -> RuntimeResult<Value> {
    if 参数.is_empty() {
        return Err(RuntimeError::ArgumentCountError {
            function: "字符串截取".to_string(),
            expected: 1,
            actual: 0,
        });
    }
    
    let 字符串 = match &参数[0] {
        Value::字符串(s) => s.clone(),
        _ => return Err(RuntimeError::TypeError("第一个参数必须是字符串".to_string())),
    };
    
    let chars: Vec<char> = 字符串.chars().collect();
    let 长度 = chars.len();
    
    let 起始 = if 参数.len() > 1 {
        match &参数[1] {
            Value::整数(n) => {
                let n = *n;
                if n < 0 {
                    (长度 as i64 + n) as usize
                } else {
                    n as usize
                }
            }
            _ => return Err(RuntimeError::TypeError("起始位置必须是整数".to_string())),
        }
    } else {
        0
    };
    
    let 结束 = if 参数.len() > 2 {
        match &参数[2] {
            Value::整数(n) => {
                let n = *n;
                if n < 0 {
                    (长度 as i64 + n) as usize
                } else {
                    n as usize
                }
            }
            _ => return Err(RuntimeError::TypeError("结束位置必须是整数".to_string())),
        }
    } else {
        长度
    };
    
    let 起始 = 起始.min(长度);
    let 结束 = 结束.min(长度);
    
    if 起始 > 结束 {
        return Ok(Value::字符串("".to_string()));
    }
    
    let 结果: String = chars[起始..结束].iter().collect();
    Ok(Value::字符串(结果))
}

pub fn builtin_字符串统计(参数: Vec<Value>) -> RuntimeResult<Value> {
    if 参数.len() < 2 {
        return Err(RuntimeError::ArgumentCountError {
            function: "字符串统计".to_string(),
            expected: 2,
            actual: 参数.len(),
        });
    }
    
    let 字符串 = match &参数[0] {
        Value::字符串(s) => s.clone(),
        _ => return Err(RuntimeError::TypeError("第一个参数必须是字符串".to_string())),
    };
    
    let 子串 = match &参数[1] {
        Value::字符串(s) => s.clone(),
        _ => return Err(RuntimeError::TypeError("子串必须是字符串".to_string())),
    };
    
    let 计数 = 字符串.matches(&子串).count();
    Ok(Value::整数(计数 as i64))
}

pub fn builtin_字符串居中(参数: Vec<Value>) -> RuntimeResult<Value> {
    if 参数.len() < 2 {
        return Err(RuntimeError::ArgumentCountError {
            function: "字符串居中".to_string(),
            expected: 2,
            actual: 参数.len(),
        });
    }
    
    let 字符串 = match &参数[0] {
        Value::字符串(s) => s.clone(),
        _ => return Err(RuntimeError::TypeError("第一个参数必须是字符串".to_string())),
    };
    
    let 总宽度 = match &参数[1] {
        Value::整数(n) => *n as usize,
        _ => return Err(RuntimeError::TypeError("总宽度必须是整数".to_string())),
    };
    
    let 填充字符 = if 参数.len() > 2 {
        match &参数[2] {
            Value::字符串(s) => {
                let chars: Vec<char> = s.chars().collect();
                if chars.is_empty() {
                    ' '
                } else {
                    chars[0]
                }
            }
            _ => ' ',
        }
    } else {
        ' '
    };
    
    let 当前宽度 = 字符串.chars().count();
    
    if 当前宽度 >= 总宽度 {
        return Ok(Value::字符串(字符串));
    }
    
    let 总填充 = 总宽度 - 当前宽度;
    let 左填充 = 总填充 / 2;
    let 右填充 = 总填充 - 左填充;
    
    let 结果 = format!(
        "{}{}{}",
        填充字符.to_string().repeat(左填充),
        字符串,
        填充字符.to_string().repeat(右填充)
    );
    
    Ok(Value::字符串(结果))
}

pub fn builtin_字符串左对齐(参数: Vec<Value>) -> RuntimeResult<Value> {
    if 参数.len() < 2 {
        return Err(RuntimeError::ArgumentCountError {
            function: "字符串左对齐".to_string(),
            expected: 2,
            actual: 参数.len(),
        });
    }
    
    let 字符串 = match &参数[0] {
        Value::字符串(s) => s.clone(),
        _ => return Err(RuntimeError::TypeError("第一个参数必须是字符串".to_string())),
    };
    
    let 总宽度 = match &参数[1] {
        Value::整数(n) => *n as usize,
        _ => return Err(RuntimeError::TypeError("总宽度必须是整数".to_string())),
    };
    
    let 填充字符 = if 参数.len() > 2 {
        match &参数[2] {
            Value::字符串(s) => {
                let chars: Vec<char> = s.chars().collect();
                if chars.is_empty() {
                    ' '
                } else {
                    chars[0]
                }
            }
            _ => ' ',
        }
    } else {
        ' '
    };
    
    let 当前宽度 = 字符串.chars().count();
    
    if 当前宽度 >= 总宽度 {
        return Ok(Value::字符串(字符串));
    }
    
    let 右填充 = 总宽度 - 当前宽度;
    let 结果 = format!("{}{}", 字符串, 填充字符.to_string().repeat(右填充));
    
    Ok(Value::字符串(结果))
}

pub fn builtin_字符串右对齐(参数: Vec<Value>) -> RuntimeResult<Value> {
    if 参数.len() < 2 {
        return Err(RuntimeError::ArgumentCountError {
            function: "字符串右对齐".to_string(),
            expected: 2,
            actual: 参数.len(),
        });
    }
    
    let 字符串 = match &参数[0] {
        Value::字符串(s) => s.clone(),
        _ => return Err(RuntimeError::TypeError("第一个参数必须是字符串".to_string())),
    };
    
    let 总宽度 = match &参数[1] {
        Value::整数(n) => *n as usize,
        _ => return Err(RuntimeError::TypeError("总宽度必须是整数".to_string())),
    };
    
    let 填充字符 = if 参数.len() > 2 {
        match &参数[2] {
            Value::字符串(s) => {
                let chars: Vec<char> = s.chars().collect();
                if chars.is_empty() {
                    ' '
                } else {
                    chars[0]
                }
            }
            _ => ' ',
        }
    } else {
        ' '
    };
    
    let 当前宽度 = 字符串.chars().count();
    
    if 当前宽度 >= 总宽度 {
        return Ok(Value::字符串(字符串));
    }
    
    let 左填充 = 总宽度 - 当前宽度;
    let 结果 = format!("{}{}", 填充字符.to_string().repeat(左填充), 字符串);
    
    Ok(Value::字符串(结果))
}

pub fn builtin_字符串是否数字(参数: Vec<Value>) -> RuntimeResult<Value> {
    if 参数.is_empty() {
        return Err(RuntimeError::ArgumentCountError {
            function: "字符串是否数字".to_string(),
            expected: 1,
            actual: 0,
        });
    }
    
    let 字符串 = match &参数[0] {
        Value::字符串(s) => s.clone(),
        _ => return Err(RuntimeError::TypeError("参数必须是字符串".to_string())),
    };
    
    Ok(Value::布尔值(字符串.chars().all(|c| c.is_ascii_digit())))
}

pub fn builtin_字符串是否字母(参数: Vec<Value>) -> RuntimeResult<Value> {
    if 参数.is_empty() {
        return Err(RuntimeError::ArgumentCountError {
            function: "字符串是否字母".to_string(),
            expected: 1,
            actual: 0,
        });
    }
    
    let 字符串 = match &参数[0] {
        Value::字符串(s) => s.clone(),
        _ => return Err(RuntimeError::TypeError("参数必须是字符串".to_string())),
    };
    
    Ok(Value::布尔值(字符串.chars().all(|c| c.is_alphabetic())))
}

pub fn builtin_字符串是否字母数字(参数: Vec<Value>) -> RuntimeResult<Value> {
    if 参数.is_empty() {
        return Err(RuntimeError::ArgumentCountError {
            function: "字符串是否字母数字".to_string(),
            expected: 1,
            actual: 0,
        });
    }
    
    let 字符串 = match &参数[0] {
        Value::字符串(s) => s.clone(),
        _ => return Err(RuntimeError::TypeError("参数必须是字符串".to_string())),
    };
    
    Ok(Value::布尔值(字符串.chars().all(|c| c.is_alphanumeric())))
}

pub fn builtin_字符串是否空白(参数: Vec<Value>) -> RuntimeResult<Value> {
    if 参数.is_empty() {
        return Err(RuntimeError::ArgumentCountError {
            function: "字符串是否空白".to_string(),
            expected: 1,
            actual: 0,
        });
    }
    
    let 字符串 = match &参数[0] {
        Value::字符串(s) => s.clone(),
        _ => return Err(RuntimeError::TypeError("参数必须是字符串".to_string())),
    };
    
    Ok(Value::布尔值(字符串.chars().all(|c| c.is_whitespace())))
}
