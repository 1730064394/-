use crate::error::{RuntimeError, RuntimeResult};
use crate::runtime::value::Value;

pub fn builtin_http_get(args: Vec<Value>) -> RuntimeResult<Value> {
    if args.is_empty() {
        return Err(RuntimeError::ArgumentCountError {
            function: "HTTP获取".to_string(),
            expected: 1,
            actual: 0,
        });
    }
    
    let url = match &args[0] {
        Value::字符串(s) => s.clone(),
        _ => return Err(RuntimeError::TypeError("URL必须是字符串".to_string())),
    };
    
    #[cfg(feature = "network")]
    {
        use reqwest::blocking::Client;
        
        let client = Client::new();
        match client.get(&url).send() {
            Ok(response) => {
                match response.text() {
                    Ok(text) => Ok(Value::字符串(text)),
                    Err(e) => Err(RuntimeError::General(format!("读取响应失败: {}", e))),
                }
            }
            Err(e) => Err(RuntimeError::General(format!("HTTP请求失败: {}", e))),
        }
    }
    
    #[cfg(not(feature = "network"))]
    {
        Err(RuntimeError::General("网络功能未启用，请重新编译时启用 network 特性".to_string()))
    }
}

pub fn builtin_http_post(args: Vec<Value>) -> RuntimeResult<Value> {
    if args.len() < 2 {
        return Err(RuntimeError::ArgumentCountError {
            function: "HTTP提交".to_string(),
            expected: 2,
            actual: args.len(),
        });
    }
    
    let url = match &args[0] {
        Value::字符串(s) => s.clone(),
        _ => return Err(RuntimeError::TypeError("URL必须是字符串".to_string())),
    };
    
    let body = args[1].to_string_value();
    
    #[cfg(feature = "network")]
    {
        use reqwest::blocking::Client;
        
        let client = Client::new();
        match client.post(&url).body(body).send() {
            Ok(response) => {
                match response.text() {
                    Ok(text) => Ok(Value::字符串(text)),
                    Err(e) => Err(RuntimeError::General(format!("读取响应失败: {}", e))),
                }
            }
            Err(e) => Err(RuntimeError::General(format!("HTTP请求失败: {}", e))),
        }
    }
    
    #[cfg(not(feature = "network"))]
    {
        Err(RuntimeError::General("网络功能未启用，请重新编译时启用 network 特性".to_string()))
    }
}

pub fn builtin_http_put(args: Vec<Value>) -> RuntimeResult<Value> {
    if args.len() < 2 {
        return Err(RuntimeError::ArgumentCountError {
            function: "HTTP更新".to_string(),
            expected: 2,
            actual: args.len(),
        });
    }
    
    let url = match &args[0] {
        Value::字符串(s) => s.clone(),
        _ => return Err(RuntimeError::TypeError("URL必须是字符串".to_string())),
    };
    
    let body = args[1].to_string_value();
    
    #[cfg(feature = "network")]
    {
        use reqwest::blocking::Client;
        
        let client = Client::new();
        match client.put(&url).body(body).send() {
            Ok(response) => {
                match response.text() {
                    Ok(text) => Ok(Value::字符串(text)),
                    Err(e) => Err(RuntimeError::General(format!("读取响应失败: {}", e))),
                }
            }
            Err(e) => Err(RuntimeError::General(format!("HTTP请求失败: {}", e))),
        }
    }
    
    #[cfg(not(feature = "network"))]
    {
        Err(RuntimeError::General("网络功能未启用，请重新编译时启用 network 特性".to_string()))
    }
}

pub fn builtin_http_delete(args: Vec<Value>) -> RuntimeResult<Value> {
    if args.is_empty() {
        return Err(RuntimeError::ArgumentCountError {
            function: "HTTP删除".to_string(),
            expected: 1,
            actual: 0,
        });
    }
    
    let url = match &args[0] {
        Value::字符串(s) => s.clone(),
        _ => return Err(RuntimeError::TypeError("URL必须是字符串".to_string())),
    };
    
    #[cfg(feature = "network")]
    {
        use reqwest::blocking::Client;
        
        let client = Client::new();
        match client.delete(&url).send() {
            Ok(response) => {
                match response.text() {
                    Ok(text) => Ok(Value::字符串(text)),
                    Err(e) => Err(RuntimeError::General(format!("读取响应失败: {}", e))),
                }
            }
            Err(e) => Err(RuntimeError::General(format!("HTTP请求失败: {}", e))),
        }
    }
    
    #[cfg(not(feature = "network"))]
    {
        Err(RuntimeError::General("网络功能未启用，请重新编译时启用 network 特性".to_string()))
    }
}

pub fn builtin_http_head(args: Vec<Value>) -> RuntimeResult<Value> {
    if args.is_empty() {
        return Err(RuntimeError::ArgumentCountError {
            function: "HTTP头部".to_string(),
            expected: 1,
            actual: 0,
        });
    }
    
    let url = match &args[0] {
        Value::字符串(s) => s.clone(),
        _ => return Err(RuntimeError::TypeError("URL必须是字符串".to_string())),
    };
    
    #[cfg(feature = "network")]
    {
        use reqwest::blocking::Client;
        
        let client = Client::new();
        match client.head(&url).send() {
            Ok(response) => {
                let mut headers = std::collections::HashMap::new();
                for (name, value) in response.headers().iter() {
                    if let Ok(value_str) = value.to_str() {
                        headers.insert(name.as_str().to_string(), Value::字符串(value_str.to_string()));
                    }
                }
                Ok(Value::字典(headers))
            }
            Err(e) => Err(RuntimeError::General(format!("HTTP请求失败: {}", e))),
        }
    }
    
    #[cfg(not(feature = "network"))]
    {
        Err(RuntimeError::General("网络功能未启用，请重新编译时启用 network 特性".to_string()))
    }
}
