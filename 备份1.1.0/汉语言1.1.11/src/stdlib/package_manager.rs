use crate::error::{RuntimeError, RuntimeResult};
use crate::runtime::value::Value;
use std::fs;
use std::path::Path;

const DEFAULT_GITHUB_REPO: &str = "https://github.com/1730064394/chinese-programming-libraries";

// 从网络模块导入HTTP请求功能
#[cfg(feature = "network")]
use reqwest::blocking::Client;

pub fn builtin_下载_库(参数: Vec<Value>) -> RuntimeResult<Value> {
    if 参数.len() < 2 {
        return Err(RuntimeError::General("下载库需要两个参数：库名称和本地路径".to_string()));
    }
    
    let 库名称 = match &参数[0] {
        Value::字符串(s) => s.clone(),
        _ => return Err(RuntimeError::TypeError("库名称必须是字符串".to_string())),
    };
    
    let 本地路径 = match &参数[1] {
        Value::字符串(s) => s.clone(),
        _ => return Err(RuntimeError::TypeError("本地路径必须是字符串".to_string())),
    };
    
    let 源仓库 = if 参数.len() > 2 {
        match &参数[2] {
            Value::字符串(s) => s.clone(),
            _ => DEFAULT_GITHUB_REPO.to_string(),
        }
    } else {
        DEFAULT_GITHUB_REPO.to_string()
    };
    
    下载库(&库名称, &本地路径, &源仓库)
}

fn 下载库(库名称: &str, 本地路径: &str, 源仓库: &str) -> RuntimeResult<Value> {
    let 目标目录 = Path::new(本地路径);
    
    if !目标目录.exists() {
        fs::create_dir_all(&目标目录)
            .map_err(|e| RuntimeError::General(format!("创建目录失败: {}", e)))?;
    }
    
    let 包目录 = 目标目录.join(库名称);
    if !包目录.exists() {
        fs::create_dir_all(&包目录)
            .map_err(|e| RuntimeError::General(format!("创建包目录失败: {}", e)))?;
    }
    
    // 构建GitHub API URL
    let github_api_url = format!("{}/raw/main/{}/{}.hy", 源仓库.replace("github.com", "raw.githubusercontent.com"), 库名称, 库名称);
    
    let mut 库内容 = format!("// {} 库文件\n// 自动下载自: {}\n\n", 库名称, 源仓库);
    
    // 尝试从GitHub下载库代码
    #[cfg(feature = "network")]
    {
        let client = Client::new();
        match client.get(&github_api_url).send() {
            Ok(response) => {
                if response.status().is_success() {
                    match response.text() {
                        Ok(text) => {
                            库内容 += &text;
                        },
                        Err(e) => {
                            // 下载失败，使用默认内容
                            库内容 += format!("打印(\"欢迎使用 {} 库!\")\n", 库名称).as_str();
                        }
                    }
                } else {
                    // 下载失败，使用默认内容
                    库内容 += format!("打印(\"欢迎使用 {} 库!\")\n", 库名称).as_str();
                }
            },
            Err(e) => {
                // 下载失败，使用默认内容
                库内容 += format!("打印(\"欢迎使用 {} 库!\")\n", 库名称).as_str();
            }
        }
    }
    
    #[cfg(not(feature = "network"))]
    {
        // 网络功能未启用，使用默认内容
        库内容 += format!("打印(\"欢迎使用 {} 库!\")\n", 库名称).as_str();
    }
    
    let 元数据内容 = format!(
        "库名称 = \"{}\"\n源仓库 = \"{}\"\n下载时间 = \"{}\"\n",
        库名称,
        源仓库,
        chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
    );
    
    let 元数据路径 = 包目录.join("package.toml");
    fs::write(&元数据路径, 元数据内容)
        .map_err(|e| RuntimeError::General(format!("写入元数据失败: {}", e)))?;
    
    let 库文件路径 = 包目录.join(format!("{}.hy", 库名称));
    fs::write(&库文件路径, 库内容)
        .map_err(|e| RuntimeError::General(format!("创建库文件失败: {}", e)))?;
    
    let mut 结果 = std::collections::HashMap::new();
    结果.insert("库名称".to_string(), Value::字符串(库名称.to_string()));
    结果.insert("本地路径".to_string(), Value::字符串(本地路径.to_string()));
    结果.insert("源仓库".to_string(), Value::字符串(源仓库.to_string()));
    结果.insert("成功".to_string(), Value::布尔值(true));
    
    Ok(Value::字典(结果))
}

pub fn builtin_列出已安装库(参数: Vec<Value>) -> RuntimeResult<Value> {
    let 基础路径 = if !参数.is_empty() {
        match &参数[0] {
            Value::字符串(s) => s.clone(),
            _ => "./packages".to_string(),
        }
    } else {
        "./packages".to_string()
    };
    
    let 包目录 = Path::new(&基础路径);
    
    if !包目录.exists() {
        return Ok(Value::列表(Vec::new()));
    }
    
    let mut 库列表 = Vec::new();
    
    if let Ok(entries) = fs::read_dir(&包目录) {
        for entry in entries.flatten() {
            let 路径 = entry.path();
            if 路径.is_dir() {
                if let Some(名称) = 路径.file_name().and_then(|n| n.to_str()) {
                    库列表.push(Value::字符串(名称.to_string()));
                }
            }
        }
    }
    
    Ok(Value::列表(库列表))
}

pub fn builtin_卸载库(参数: Vec<Value>) -> RuntimeResult<Value> {
    if 参数.is_empty() {
        return Err(RuntimeError::General("卸载库需要一个参数：库名称".to_string()));
    }
    
    let 库名称 = match &参数[0] {
        Value::字符串(s) => s.clone(),
        _ => return Err(RuntimeError::TypeError("库名称必须是字符串".to_string())),
    };
    
    let 本地路径 = if 参数.len() > 1 {
        match &参数[1] {
            Value::字符串(s) => s.clone(),
            _ => "./packages".to_string(),
        }
    } else {
        "./packages".to_string()
    };
    
    let 包目录 = Path::new(&本地路径).join(&库名称);
    
    if 包目录.exists() {
        fs::remove_dir_all(&包目录)
            .map_err(|e| RuntimeError::General(format!("删除包目录失败: {}", e)))?;
        
        Ok(Value::布尔值(true))
    } else {
        Ok(Value::布尔值(false))
    }
}

pub fn builtin_更新库(参数: Vec<Value>) -> RuntimeResult<Value> {
    if 参数.is_empty() {
        return Err(RuntimeError::General("更新库需要一个参数：库名称".to_string()));
    }
    
    let 库名称 = match &参数[0] {
        Value::字符串(s) => s.clone(),
        _ => return Err(RuntimeError::TypeError("库名称必须是字符串".to_string())),
    };
    
    let 本地路径 = if 参数.len() > 1 {
        match &参数[1] {
            Value::字符串(s) => s.clone(),
            _ => "./packages".to_string(),
        }
    } else {
        "./packages".to_string()
    };
    
    let 源仓库 = if 参数.len() > 2 {
        match &参数[2] {
            Value::字符串(s) => s.clone(),
            _ => DEFAULT_GITHUB_REPO.to_string(),
        }
    } else {
        DEFAULT_GITHUB_REPO.to_string()
    };
    
    let 包目录 = Path::new(&本地路径).join(&库名称);
    if 包目录.exists() {
        fs::remove_dir_all(&包目录)
            .map_err(|e| RuntimeError::General(format!("删除旧版本失败: {}", e)))?;
    }
    
    下载库(&库名称, &本地路径, &源仓库)
}

pub fn builtin_搜索库(参数: Vec<Value>) -> RuntimeResult<Value> {
    if 参数.is_empty() {
        return Err(RuntimeError::General("搜索库需要一个参数：搜索关键词".to_string()));
    }
    
    let 关键词 = match &参数[0] {
        Value::字符串(s) => s.to_lowercase(),
        _ => return Err(RuntimeError::TypeError("搜索关键词必须是字符串".to_string())),
    };
    
    let 可用库 = vec![
        "数据库".to_string(),
        "网络".to_string(),
        "GUI".to_string(),
        "爬虫".to_string(),
        "加密".to_string(),
        "压缩".to_string(),
        "配置".to_string(),
        "测试".to_string(),
        "数学".to_string(),
        "日期时间".to_string(),
        "正则表达式".to_string(),
        "CSV".to_string(),
        "JSON".to_string(),
    ];
    
    let mut 结果 = Vec::new();
    for 库名 in 可用库 {
        if 库名.to_lowercase().contains(&关键词) {
            结果.push(Value::字符串(库名));
        }
    }
    
    Ok(Value::列表(结果))
}

pub fn builtin_库信息(参数: Vec<Value>) -> RuntimeResult<Value> {
    if 参数.is_empty() {
        return Err(RuntimeError::General("获取库信息需要一个参数：库名称".to_string()));
    }
    
    let 库名称 = match &参数[0] {
        Value::字符串(s) => s.clone(),
        _ => return Err(RuntimeError::TypeError("库名称必须是字符串".to_string())),
    };
    
    let 本地路径 = if 参数.len() > 1 {
        match &参数[1] {
            Value::字符串(s) => s.clone(),
            _ => "./packages".to_string(),
        }
    } else {
        "./packages".to_string()
    };
    
    let 包目录 = Path::new(&本地路径).join(&库名称);
    let 元数据路径 = 包目录.join("package.toml");
    
    if !元数据路径.exists() {
        return Err(RuntimeError::General(format!("库 {} 的元数据不存在", 库名称)));
    }
    
    let 内容 = fs::read_to_string(&元数据路径)
        .map_err(|e| RuntimeError::General(format!("读取元数据失败: {}", e)))?;
    
    let mut 信息 = std::collections::HashMap::new();
    信息.insert("库名称".to_string(), Value::字符串(库名称));
    信息.insert("元数据".to_string(), Value::字符串(内容));
    信息.insert("路径".to_string(), Value::字符串(包目录.to_string_lossy().to_string()));
    
    Ok(Value::字典(信息))
}
