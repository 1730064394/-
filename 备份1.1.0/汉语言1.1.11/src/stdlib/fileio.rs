use crate::error::{RuntimeError, RuntimeResult};
use crate::runtime::value::Value;
use std::fs;
use std::path::Path;

pub fn builtin_append_file(args: Vec<Value>) -> RuntimeResult<Value> {
    use std::fs::OpenOptions;
    use std::io::Write;
    
    if args.len() < 2 {
        return Err(RuntimeError::ArgumentCountError {
            function: "追加文件".to_string(),
            expected: 2,
            actual: args.len(),
        });
    }
    
    let filename = match &args[0] {
        Value::字符串(s) => s.clone(),
        _ => return Err(RuntimeError::TypeError("文件名必须是字符串".to_string())),
    };
    
    let content = args[1].to_string_value();
    
    match OpenOptions::new()
        .create(true)
        .append(true)
        .open(&filename)
    {
        Ok(mut file) => {
            file.write_all(content.as_bytes())
                .map_err(|e| RuntimeError::General(format!("追加文件失败: {}", e)))?;
            Ok(Value::布尔值(true))
        }
        Err(e) => Err(RuntimeError::General(format!("打开文件失败: {}", e))),
    }
}

pub fn builtin_delete_file(args: Vec<Value>) -> RuntimeResult<Value> {
    if args.is_empty() {
        return Err(RuntimeError::ArgumentCountError {
            function: "删除文件".to_string(),
            expected: 1,
            actual: 0,
        });
    }
    
    let filename = match &args[0] {
        Value::字符串(s) => s,
        _ => return Err(RuntimeError::TypeError("文件名必须是字符串".to_string())),
    };
    
    match fs::remove_file(filename) {
        Ok(_) => Ok(Value::布尔值(true)),
        Err(e) => Err(RuntimeError::General(format!("删除文件失败: {}", e))),
    }
}

pub fn builtin_copy_file(args: Vec<Value>) -> RuntimeResult<Value> {
    if args.len() < 2 {
        return Err(RuntimeError::ArgumentCountError {
            function: "复制文件".to_string(),
            expected: 2,
            actual: args.len(),
        });
    }
    
    let src = match &args[0] {
        Value::字符串(s) => s,
        _ => return Err(RuntimeError::TypeError("源文件名必须是字符串".to_string())),
    };
    
    let dst = match &args[1] {
        Value::字符串(s) => s,
        _ => return Err(RuntimeError::TypeError("目标文件名必须是字符串".to_string())),
    };
    
    match fs::copy(src, dst) {
        Ok(_) => Ok(Value::布尔值(true)),
        Err(e) => Err(RuntimeError::General(format!("复制文件失败: {}", e))),
    }
}

pub fn builtin_list_dir(args: Vec<Value>) -> RuntimeResult<Value> {
    if args.is_empty() {
        return Err(RuntimeError::ArgumentCountError {
            function: "列出目录".to_string(),
            expected: 1,
            actual: 0,
        });
    }
    
    let dirname = match &args[0] {
        Value::字符串(s) => s,
        _ => return Err(RuntimeError::TypeError("目录名必须是字符串".to_string())),
    };
    
    let path = Path::new(dirname);
    if !path.is_dir() {
        return Err(RuntimeError::General(format!("不是目录: {}", dirname)));
    }
    
    match fs::read_dir(path) {
        Ok(entries) => {
            let mut files = Vec::new();
            for entry in entries {
                if let Ok(entry) = entry {
                    if let Some(name) = entry.file_name().to_str() {
                        files.push(Value::字符串(name.to_string()));
                    }
                }
            }
            Ok(Value::列表(files))
        }
        Err(e) => Err(RuntimeError::General(format!("读取目录失败: {}", e))),
    }
}

pub fn builtin_create_dir(args: Vec<Value>) -> RuntimeResult<Value> {
    if args.is_empty() {
        return Err(RuntimeError::ArgumentCountError {
            function: "创建目录".to_string(),
            expected: 1,
            actual: 0,
        });
    }
    
    let dirname = match &args[0] {
        Value::字符串(s) => s,
        _ => return Err(RuntimeError::TypeError("目录名必须是字符串".to_string())),
    };
    
    let recursive = if args.len() > 1 {
        args[1].is_truthy()
    } else {
        false
    };
    
    let result = if recursive {
        fs::create_dir_all(dirname)
    } else {
        fs::create_dir(dirname)
    };
    
    match result {
        Ok(_) => Ok(Value::布尔值(true)),
        Err(e) => Err(RuntimeError::General(format!("创建目录失败: {}", e))),
    }
}

pub fn builtin_delete_dir(args: Vec<Value>) -> RuntimeResult<Value> {
    if args.is_empty() {
        return Err(RuntimeError::ArgumentCountError {
            function: "删除目录".to_string(),
            expected: 1,
            actual: 0,
        });
    }
    
    let dirname = match &args[0] {
        Value::字符串(s) => s,
        _ => return Err(RuntimeError::TypeError("目录名必须是字符串".to_string())),
    };
    
    let recursive = if args.len() > 1 {
        args[1].is_truthy()
    } else {
        false
    };
    
    let result = if recursive {
        fs::remove_dir_all(dirname)
    } else {
        fs::remove_dir(dirname)
    };
    
    match result {
        Ok(_) => Ok(Value::布尔值(true)),
        Err(e) => Err(RuntimeError::General(format!("删除目录失败: {}", e))),
    }
}

pub fn builtin_is_file(args: Vec<Value>) -> RuntimeResult<Value> {
    if args.is_empty() {
        return Err(RuntimeError::ArgumentCountError {
            function: "是文件".to_string(),
            expected: 1,
            actual: 0,
        });
    }
    
    let path_str = match &args[0] {
        Value::字符串(s) => s,
        _ => return Err(RuntimeError::TypeError("路径必须是字符串".to_string())),
    };
    
    Ok(Value::布尔值(Path::new(path_str).is_file()))
}

pub fn builtin_is_dir(args: Vec<Value>) -> RuntimeResult<Value> {
    if args.is_empty() {
        return Err(RuntimeError::ArgumentCountError {
            function: "是目录".to_string(),
            expected: 1,
            actual: 0,
        });
    }
    
    let path_str = match &args[0] {
        Value::字符串(s) => s,
        _ => return Err(RuntimeError::TypeError("路径必须是字符串".to_string())),
    };
    
    Ok(Value::布尔值(Path::new(path_str).is_dir()))
}

pub fn builtin_file_size(args: Vec<Value>) -> RuntimeResult<Value> {
    if args.is_empty() {
        return Err(RuntimeError::ArgumentCountError {
            function: "文件大小".to_string(),
            expected: 1,
            actual: 0,
        });
    }
    
    let path_str = match &args[0] {
        Value::字符串(s) => s,
        _ => return Err(RuntimeError::TypeError("路径必须是字符串".to_string())),
    };
    
    let path = Path::new(path_str);
    match fs::metadata(path) {
        Ok(metadata) => Ok(Value::整数(metadata.len() as i64)),
        Err(e) => Err(RuntimeError::General(format!("获取文件大小失败: {}", e))),
    }
}

pub fn builtin_rename_file(args: Vec<Value>) -> RuntimeResult<Value> {
    if args.len() < 2 {
        return Err(RuntimeError::ArgumentCountError {
            function: "重命名文件".to_string(),
            expected: 2,
            actual: args.len(),
        });
    }
    
    let old_name = match &args[0] {
        Value::字符串(s) => s,
        _ => return Err(RuntimeError::TypeError("旧文件名必须是字符串".to_string())),
    };
    
    let new_name = match &args[1] {
        Value::字符串(s) => s,
        _ => return Err(RuntimeError::TypeError("新文件名必须是字符串".to_string())),
    };
    
    match fs::rename(old_name, new_name) {
        Ok(_) => Ok(Value::布尔值(true)),
        Err(e) => Err(RuntimeError::General(format!("重命名文件失败: {}", e))),
    }
}

pub fn builtin_move_file(args: Vec<Value>) -> RuntimeResult<Value> {
    if args.len() < 2 {
        return Err(RuntimeError::ArgumentCountError {
            function: "移动文件".to_string(),
            expected: 2,
            actual: args.len(),
        });
    }
    
    let src = match &args[0] {
        Value::字符串(s) => s,
        _ => return Err(RuntimeError::TypeError("源路径必须是字符串".to_string())),
    };
    
    let dst = match &args[1] {
        Value::字符串(s) => s,
        _ => return Err(RuntimeError::TypeError("目标路径必须是字符串".to_string())),
    };
    
    match fs::rename(src, dst) {
        Ok(_) => Ok(Value::布尔值(true)),
        Err(e) => Err(RuntimeError::General(format!("移动文件失败: {}", e))),
    }
}


