use crate::error::{RuntimeError, RuntimeResult};
use crate::runtime::value::Value;
use std::collections::HashMap;

/// 访问网页并返回内容
pub fn builtin_访问网页(参数: Vec<Value>) -> RuntimeResult<Value> {
    if 参数.is_empty() {
        return Err(RuntimeError::ArgumentCountError {
            function: "访问网页".to_string(),
            expected: 1,
            actual: 0,
        });
    }
    
    let url = match &参数[0] {
        Value::字符串(s) => s.clone(),
        _ => return Err(RuntimeError::TypeError("URL必须是字符串".to_string())),
    };
    
    #[cfg(feature = "network")]
    {
        use reqwest::blocking::Client;
        
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .map_err(|e| RuntimeError::General(format!("创建客户端失败: {}", e)))?;
        
        match client.get(&url).send() {
            Ok(response) => {
                let status: i64 = response.status().as_u16() as i64;
                let headers = response.headers().clone();
                
                match response.text() {
                    Ok(text) => {
                        let mut result = HashMap::new();
                        result.insert("状态码".to_string(), Value::整数(status));
                        result.insert("内容".to_string(), Value::字符串(text));
                        
                        let mut headers_map = HashMap::new();
                        for (name, value) in headers.iter() {
                            if let Ok(value_str) = value.to_str() {
                                headers_map.insert(name.as_str().to_string(), Value::字符串(value_str.to_string()));
                            }
                        }
                        result.insert("响应头".to_string(), Value::字典(headers_map));
                        
                        Ok(Value::字典(result))
                    }
                    Err(e) => Err(RuntimeError::General(format!("读取响应失败: {}", e))),
                }
            }
            Err(e) => Err(RuntimeError::General(format!("请求失败: {}", e))),
        }
    }
    
    #[cfg(not(feature = "network"))]
    {
        Err(RuntimeError::General("网络功能未启用，请重新编译时启用 network 特性".to_string()))
    }
}

/// 获取网页状态码
pub fn builtin_获取状态码(参数: Vec<Value>) -> RuntimeResult<Value> {
    if 参数.is_empty() {
        return Err(RuntimeError::ArgumentCountError {
            function: "获取状态码".to_string(),
            expected: 1,
            actual: 0,
        });
    }
    
    let url = match &参数[0] {
        Value::字符串(s) => s.clone(),
        _ => return Err(RuntimeError::TypeError("URL必须是字符串".to_string())),
    };
    
    #[cfg(feature = "network")]
    {
        use reqwest::blocking::Client;
        
        let client = Client::new();
        match client.head(&url).send() {
            Ok(response) => {
                let status: i64 = response.status().as_u16() as i64;
                Ok(Value::整数(status))
            }
            Err(e) => Err(RuntimeError::General(format!("请求失败: {}", e))),
        }
    }
    
    #[cfg(not(feature = "network"))]
    {
        Err(RuntimeError::General("网络功能未启用，请重新编译时启用 network 特性".to_string()))
    }
}

/// 下载文件
pub fn builtin_下载文件(参数: Vec<Value>) -> RuntimeResult<Value> {
    if 参数.len() < 2 {
        return Err(RuntimeError::ArgumentCountError {
            function: "下载文件".to_string(),
            expected: 2,
            actual: 参数.len(),
        });
    }
    
    let url = match &参数[0] {
        Value::字符串(s) => s.clone(),
        _ => return Err(RuntimeError::TypeError("URL必须是字符串".to_string())),
    };
    
    let filepath = match &参数[1] {
        Value::字符串(s) => s.clone(),
        _ => return Err(RuntimeError::TypeError("文件路径必须是字符串".to_string())),
    };
    
    #[cfg(feature = "network")]
    {
        use reqwest::blocking::Client;
        use std::fs::File;
        use std::io::Write;
        
        let client = Client::new();
        match client.get(&url).send() {
            Ok(response) => {
                match response.bytes() {
                    Ok(bytes) => {
                        match File::create(&filepath) {
                            Ok(mut file) => {
                                match file.write_all(&bytes) {
                                    Ok(_) => Ok(Value::布尔值(true)),
                                    Err(e) => Err(RuntimeError::General(format!("写入文件失败: {}", e))),
                                }
                            }
                            Err(e) => Err(RuntimeError::General(format!("创建文件失败: {}", e))),
                        }
                    }
                    Err(e) => Err(RuntimeError::General(format!("读取响应失败: {}", e))),
                }
            }
            Err(e) => Err(RuntimeError::General(format!("下载失败: {}", e))),
        }
    }
    
    #[cfg(not(feature = "network"))]
    {
        Err(RuntimeError::General("网络功能未启用，请重新编译时启用 network 特性".to_string()))
    }
}

/// 解析HTML中的链接
pub fn builtin_提取链接(参数: Vec<Value>) -> RuntimeResult<Value> {
    if 参数.is_empty() {
        return Err(RuntimeError::ArgumentCountError {
            function: "提取链接".to_string(),
            expected: 1,
            actual: 0,
        });
    }
    
    let html = match &参数[0] {
        Value::字符串(s) => s.clone(),
        _ => return Err(RuntimeError::TypeError("HTML内容必须是字符串".to_string())),
    };
    
    // 简单的正则表达式提取链接
    let mut links = Vec::new();
    let re = regex::Regex::new(r#"href=["']([^"']+)["']"#).unwrap();
    
    for cap in re.captures_iter(&html) {
        if let Some(link) = cap.get(1) {
            links.push(Value::字符串(link.as_str().to_string()));
        }
    }
    
    Ok(Value::列表(links))
}

/// 解析HTML中的图片
pub fn builtin_提取图片(参数: Vec<Value>) -> RuntimeResult<Value> {
    if 参数.is_empty() {
        return Err(RuntimeError::ArgumentCountError {
            function: "提取图片".to_string(),
            expected: 1,
            actual: 0,
        });
    }
    
    let html = match &参数[0] {
        Value::字符串(s) => s.clone(),
        _ => return Err(RuntimeError::TypeError("HTML内容必须是字符串".to_string())),
    };
    
    let mut images = Vec::new();
    let re = regex::Regex::new(r#"<img[^>]+src=["']([^"']+)["']"#).unwrap();
    
    for cap in re.captures_iter(&html) {
        if let Some(src) = cap.get(1) {
            images.push(Value::字符串(src.as_str().to_string()));
        }
    }
    
    Ok(Value::列表(images))
}

/// 解析HTML中的标题
pub fn builtin_提取标题(参数: Vec<Value>) -> RuntimeResult<Value> {
    if 参数.is_empty() {
        return Err(RuntimeError::ArgumentCountError {
            function: "提取标题".to_string(),
            expected: 1,
            actual: 0,
        });
    }
    
    let html = match &参数[0] {
        Value::字符串(s) => s.clone(),
        _ => return Err(RuntimeError::TypeError("HTML内容必须是字符串".to_string())),
    };
    
    // 提取<title>标签内容
    let re = regex::Regex::new(r#"<title[^>]*>([^<]+)</title>"#).unwrap();
    
    if let Some(cap) = re.captures(&html) {
        if let Some(title) = cap.get(1) {
            return Ok(Value::字符串(title.as_str().trim().to_string()));
        }
    }
    
    Ok(Value::字符串("".to_string()))
}

/// 发送POST请求
pub fn builtin_POST请求(参数: Vec<Value>) -> RuntimeResult<Value> {
    if 参数.len() < 2 {
        return Err(RuntimeError::ArgumentCountError {
            function: "POST请求".to_string(),
            expected: 2,
            actual: 参数.len(),
        });
    }
    
    let url = match &参数[0] {
        Value::字符串(s) => s.clone(),
        _ => return Err(RuntimeError::TypeError("URL必须是字符串".to_string())),
    };
    
    let body = match &参数[1] {
        Value::字符串(s) => s.clone(),
        Value::字典(d) => {
            // 将字典转换为JSON字符串
            let mut json_obj = serde_json::Map::new();
            for (k, v) in d.iter() {
                let json_value = match v {
                    Value::字符串(s) => serde_json::Value::String(s.clone()),
                    Value::整数(n) => serde_json::Value::Number(serde_json::Number::from(*n)),
                    Value::浮点数(f) => serde_json::Value::Number(serde_json::Number::from_f64(*f).unwrap_or(serde_json::Number::from(0))),
                    Value::布尔值(b) => serde_json::Value::Bool(*b),
                    _ => serde_json::Value::Null,
                };
                json_obj.insert(k.clone(), json_value);
            }
            serde_json::Value::Object(json_obj).to_string()
        }
        _ => return Err(RuntimeError::TypeError("请求体必须是字符串或字典".to_string())),
    };
    
    #[cfg(feature = "network")]
    {
        use reqwest::blocking::Client;
        
        let client = Client::new();
        match client.post(&url).body(body).send() {
            Ok(response) => {
                let status: i64 = response.status().as_u16() as i64;
                match response.text() {
                    Ok(text) => {
                        let mut result = HashMap::new();
                        result.insert("状态码".to_string(), Value::整数(status));
                        result.insert("内容".to_string(), Value::字符串(text));
                        Ok(Value::字典(result))
                    }
                    Err(e) => Err(RuntimeError::General(format!("读取响应失败: {}", e))),
                }
            }
            Err(e) => Err(RuntimeError::General(format!("请求失败: {}", e))),
        }
    }
    
    #[cfg(not(feature = "network"))]
    {
        Err(RuntimeError::General("网络功能未启用，请重新编译时启用 network 特性".to_string()))
    }
}
