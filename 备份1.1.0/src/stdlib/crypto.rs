use crate::error::{RuntimeError, RuntimeResult};
use crate::runtime::value::Value;
use md5;
use sha2::{Sha256, Sha512, Digest};

pub fn builtin_MD5哈希(参数: Vec<Value>) -> RuntimeResult<Value> {
    if 参数.len() != 1 {
        return Err(RuntimeError::General("MD5哈希需要一个参数".to_string()));
    }
    
    let input = match &参数[0] {
        Value::字符串(s) => s,
        _ => return Err(RuntimeError::TypeError("参数必须是字符串".to_string())),
    };
    
    let hash = md5::compute(input.as_bytes());
    let result = format!("{:x}", hash);
    
    Ok(Value::字符串(result))
}

pub fn builtin_SHA256哈希(参数: Vec<Value>) -> RuntimeResult<Value> {
    if 参数.len() != 1 {
        return Err(RuntimeError::General("SHA256哈希需要一个参数".to_string()));
    }
    
    let input = match &参数[0] {
        Value::字符串(s) => s,
        _ => return Err(RuntimeError::TypeError("参数必须是字符串".to_string())),
    };
    
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    let result = format!("{:x}", hasher.finalize());
    
    Ok(Value::字符串(result))
}

pub fn builtin_SHA512哈希(参数: Vec<Value>) -> RuntimeResult<Value> {
    if 参数.len() != 1 {
        return Err(RuntimeError::General("SHA512哈希需要一个参数".to_string()));
    }
    
    let input = match &参数[0] {
        Value::字符串(s) => s,
        _ => return Err(RuntimeError::TypeError("参数必须是字符串".to_string())),
    };
    
    let mut hasher = Sha512::new();
    hasher.update(input.as_bytes());
    let result = format!("{:x}", hasher.finalize());
    
    Ok(Value::字符串(result))
}

pub fn builtin_Base64编码(参数: Vec<Value>) -> RuntimeResult<Value> {
    if 参数.len() != 1 {
        return Err(RuntimeError::General("Base64编码需要一个参数".to_string()));
    }
    
    let input = match &参数[0] {
        Value::字符串(s) => s,
        _ => return Err(RuntimeError::TypeError("参数必须是字符串".to_string())),
    };
    
    use base64::{Engine as _, engine::general_purpose};
    let result = general_purpose::STANDARD.encode(input.as_bytes());
    
    Ok(Value::字符串(result))
}

pub fn builtin_Base64解码(参数: Vec<Value>) -> RuntimeResult<Value> {
    if 参数.len() != 1 {
        return Err(RuntimeError::General("Base64解码需要一个参数".to_string()));
    }
    
    let input = match &参数[0] {
        Value::字符串(s) => s,
        _ => return Err(RuntimeError::TypeError("参数必须是字符串".to_string())),
    };
    
    use base64::{Engine as _, engine::general_purpose};
    match general_purpose::STANDARD.decode(input) {
        Ok(bytes) => {
            match String::from_utf8(bytes) {
                Ok(s) => Ok(Value::字符串(s)),
                Err(_) => Err(RuntimeError::General("解码结果不是有效的UTF-8字符串".to_string())),
            }
        }
        Err(e) => Err(RuntimeError::General(format!("Base64解码失败: {}", e))),
    }
}

pub fn builtin_十六进制编码(参数: Vec<Value>) -> RuntimeResult<Value> {
    if 参数.len() != 1 {
        return Err(RuntimeError::General("十六进制编码需要一个参数".to_string()));
    }
    
    let input = match &参数[0] {
        Value::字符串(s) => s,
        _ => return Err(RuntimeError::TypeError("参数必须是字符串".to_string())),
    };
    
    let result: String = input.as_bytes().iter().map(|b| format!("{:02x}", b)).collect();
    
    Ok(Value::字符串(result))
}

pub fn builtin_十六进制解码(参数: Vec<Value>) -> RuntimeResult<Value> {
    if 参数.len() != 1 {
        return Err(RuntimeError::General("十六进制解码需要一个参数".to_string()));
    }
    
    let input = match &参数[0] {
        Value::字符串(s) => s,
        _ => return Err(RuntimeError::TypeError("参数必须是字符串".to_string())),
    };
    
    if input.len() % 2 != 0 {
        return Err(RuntimeError::General("十六进制字符串长度必须是偶数".to_string()));
    }
    
    let mut bytes = Vec::new();
    for i in (0..input.len()).step_by(2) {
        let byte_str = &input[i..i+2];
        match u8::from_str_radix(byte_str, 16) {
            Ok(b) => bytes.push(b),
            Err(_) => return Err(RuntimeError::General(format!("无效的十六进制字符: {}", byte_str))),
        }
    }
    
    match String::from_utf8(bytes) {
        Ok(s) => Ok(Value::字符串(s)),
        Err(_) => Err(RuntimeError::General("解码结果不是有效的UTF-8字符串".to_string())),
    }
}
