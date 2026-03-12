use crate::error::{RuntimeError, RuntimeResult};
use crate::runtime::value::Value;
use std::fs;
use std::path::Path;

pub fn builtin_压缩文件(参数: Vec<Value>) -> RuntimeResult<Value> {
    if 参数.len() < 2 {
        return Err(RuntimeError::General("压缩文件需要两个参数：源文件路径和目标压缩文件路径".to_string()));
    }
    
    let source_path = match &参数[0] {
        Value::字符串(s) => s.clone(),
        _ => return Err(RuntimeError::TypeError("源文件路径必须是字符串".to_string())),
    };
    
    let dest_path = match &参数[1] {
        Value::字符串(s) => s.clone(),
        _ => return Err(RuntimeError::TypeError("目标文件路径必须是字符串".to_string())),
    };
    
    let content = fs::read(&source_path)
        .map_err(|e| RuntimeError::General(format!("读取文件失败: {}", e)))?;
    
    let compressed = compress_data(&content);
    
    fs::write(&dest_path, compressed)
        .map_err(|e| RuntimeError::General(format!("写入压缩文件失败: {}", e)))?;
    
    Ok(Value::布尔值(true))
}

pub fn builtin_解压文件(参数: Vec<Value>) -> RuntimeResult<Value> {
    if 参数.len() < 2 {
        return Err(RuntimeError::General("解压文件需要两个参数：压缩文件路径和目标文件路径".to_string()));
    }
    
    let source_path = match &参数[0] {
        Value::字符串(s) => s.clone(),
        _ => return Err(RuntimeError::TypeError("压缩文件路径必须是字符串".to_string())),
    };
    
    let dest_path = match &参数[1] {
        Value::字符串(s) => s.clone(),
        _ => return Err(RuntimeError::TypeError("目标文件路径必须是字符串".to_string())),
    };
    
    let content = fs::read(&source_path)
        .map_err(|e| RuntimeError::General(format!("读取压缩文件失败: {}", e)))?;
    
    let decompressed = decompress_data(&content)?;
    
    fs::write(&dest_path, decompressed)
        .map_err(|e| RuntimeError::General(format!("写入解压文件失败: {}", e)))?;
    
    Ok(Value::布尔值(true))
}

pub fn builtin_压缩数据(参数: Vec<Value>) -> RuntimeResult<Value> {
    if 参数.is_empty() {
        return Err(RuntimeError::General("压缩数据需要一个参数：数据".to_string()));
    }
    
    let data: Vec<u8> = match &参数[0] {
        Value::字符串(s) => s.as_bytes().to_vec(),
        Value::列表(bytes) => {
            bytes.iter().map(|b| {
                match b {
                    Value::整数(n) => *n as u8,
                    _ => 0u8,
                }
            }).collect()
        }
        _ => return Err(RuntimeError::TypeError("数据必须是字符串或字节列表".to_string())),
    };
    
    let compressed = compress_data(&data);
    let result: Vec<Value> = compressed.iter().map(|b| Value::整数(*b as i64)).collect();
    
    Ok(Value::列表(result))
}

pub fn builtin_解压数据(参数: Vec<Value>) -> RuntimeResult<Value> {
    if 参数.is_empty() {
        return Err(RuntimeError::General("解压数据需要一个参数：压缩数据".to_string()));
    }
    
    let data: Vec<u8> = match &参数[0] {
        Value::列表(bytes) => {
            bytes.iter().map(|b| {
                match b {
                    Value::整数(n) => *n as u8,
                    _ => 0u8,
                }
            }).collect()
        }
        _ => return Err(RuntimeError::TypeError("压缩数据必须是字节列表".to_string())),
    };
    
    let decompressed = decompress_data(&data)?;
    
    match String::from_utf8(decompressed) {
        Ok(s) => Ok(Value::字符串(s)),
        Err(_) => Err(RuntimeError::General("解压结果不是有效的UTF-8字符串".to_string())),
    }
}

pub fn builtin_计算压缩率(参数: Vec<Value>) -> RuntimeResult<Value> {
    if 参数.len() < 2 {
        return Err(RuntimeError::General("计算压缩率需要两个参数：原始大小和压缩后大小".to_string()));
    }
    
    let original_size = match &参数[0] {
        Value::整数(n) => *n as f64,
        Value::浮点数(n) => *n,
        _ => return Err(RuntimeError::TypeError("原始大小必须是数字".to_string())),
    };
    
    let compressed_size = match &参数[1] {
        Value::整数(n) => *n as f64,
        Value::浮点数(n) => *n,
        _ => return Err(RuntimeError::TypeError("压缩后大小必须是数字".to_string())),
    };
    
    if original_size == 0.0 {
        return Ok(Value::浮点数(0.0));
    }
    
    let ratio = (original_size - compressed_size) / original_size * 100.0;
    Ok(Value::浮点数(ratio))
}

fn compress_data(data: &[u8]) -> Vec<u8> {
    let mut result = Vec::new();
    let mut i = 0;
    
    while i < data.len() {
        let current = data[i];
        let mut count = 1u8;
        
        while (i + count as usize) < data.len() 
            && data[i + count as usize] == current 
            && count < 255 {
            count += 1;
        }
        
        result.push(count);
        result.push(current);
        i += count as usize;
    }
    
    result
}

fn decompress_data(data: &[u8]) -> RuntimeResult<Vec<u8>> {
    let mut result = Vec::new();
    let mut i = 0;
    
    while i + 1 < data.len() {
        let count = data[i];
        let byte = data[i + 1];
        
        for _ in 0..count {
            result.push(byte);
        }
        
        i += 2;
    }
    
    Ok(result)
}
