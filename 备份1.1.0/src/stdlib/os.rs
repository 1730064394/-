use crate::error::{RuntimeError, RuntimeResult};
use crate::runtime::value::Value;
use std::env;
use std::fs;
use std::path::Path;
use std::process::{Command, Stdio};

pub fn builtin_get_env(args: Vec<Value>) -> RuntimeResult<Value> {
    if args.is_empty() {
        return Err(RuntimeError::ArgumentCountError {
            function: "获取环境变量".to_string(),
            expected: 1,
            actual: 0,
        });
    }

    let key = match &args[0] {
        Value::字符串(s) => s,
        _ => return Err(RuntimeError::TypeError("环境变量名必须是字符串".to_string())),
    };

    match env::var(key) {
        Ok(value) => Ok(Value::字符串(value)),
        Err(_) => Ok(Value::空值),
    }
}

pub fn builtin_set_env(args: Vec<Value>) -> RuntimeResult<Value> {
    if args.len() < 2 {
        return Err(RuntimeError::ArgumentCountError {
            function: "设置环境变量".to_string(),
            expected: 2,
            actual: args.len(),
        });
    }

    let key = match &args[0] {
        Value::字符串(s) => s,
        _ => return Err(RuntimeError::TypeError("环境变量名必须是字符串".to_string())),
    };

    let value = match &args[1] {
        Value::字符串(s) => s,
        _ => return Err(RuntimeError::TypeError("环境变量值必须是字符串".to_string())),
    };

    env::set_var(key, value);
    Ok(Value::布尔值(true))
}

pub fn builtin_remove_env(args: Vec<Value>) -> RuntimeResult<Value> {
    if args.is_empty() {
        return Err(RuntimeError::ArgumentCountError {
            function: "删除环境变量".to_string(),
            expected: 1,
            actual: 0,
        });
    }

    let key = match &args[0] {
        Value::字符串(s) => s,
        _ => return Err(RuntimeError::TypeError("环境变量名必须是字符串".to_string())),
    };

    env::remove_var(key);
    Ok(Value::布尔值(true))
}

pub fn builtin_get_cwd(_args: Vec<Value>) -> RuntimeResult<Value> {
    match env::current_dir() {
        Ok(path) => Ok(Value::字符串(path.to_string_lossy().to_string())),
        Err(e) => Err(RuntimeError::General(format!("获取当前目录失败: {}", e))),
    }
}

pub fn builtin_set_cwd(args: Vec<Value>) -> RuntimeResult<Value> {
    if args.is_empty() {
        return Err(RuntimeError::ArgumentCountError {
            function: "设置当前目录".to_string(),
            expected: 1,
            actual: 0,
        });
    }

    let path = match &args[0] {
        Value::字符串(s) => s,
        _ => return Err(RuntimeError::TypeError("路径必须是字符串".to_string())),
    };

    match env::set_current_dir(path) {
        Ok(_) => Ok(Value::布尔值(true)),
        Err(e) => Err(RuntimeError::General(format!("设置当前目录失败: {}", e))),
    }
}

pub fn builtin_execute_command(args: Vec<Value>) -> RuntimeResult<Value> {
    if args.is_empty() {
        return Err(RuntimeError::ArgumentCountError {
            function: "执行命令".to_string(),
            expected: 1,
            actual: 0,
        });
    }

    let command_str = match &args[0] {
        Value::字符串(s) => s,
        _ => return Err(RuntimeError::TypeError("命令必须是字符串".to_string())),
    };

    let output = if args.len() > 1 {
        let args_vec: Vec<String> = args[1..]
            .iter()
            .map(|arg| match arg {
                Value::字符串(s) => s.clone(),
                _ => arg.to_string(),
            })
            .collect();
        Command::new(command_str)
            .args(&args_vec)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
    } else {
        Command::new(command_str)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
    };

    match output {
        Ok(result) => {
            let stdout = String::from_utf8_lossy(&result.stdout).to_string();
            let stderr = String::from_utf8_lossy(&result.stderr).to_string();
            let exit_code = result.status.code().unwrap_or(-1);

            let mut result_dict = std::collections::HashMap::new();
            result_dict.insert("标准输出".to_string(), Value::字符串(stdout));
            result_dict.insert("标准错误".to_string(), Value::字符串(stderr));
            result_dict.insert("退出码".to_string(), Value::整数(exit_code as i64));

            Ok(Value::字典(result_dict))
        }
        Err(e) => Err(RuntimeError::General(format!("执行命令失败: {}", e))),
    }
}

pub fn builtin_spawn_process(args: Vec<Value>) -> RuntimeResult<Value> {
    if args.is_empty() {
        return Err(RuntimeError::ArgumentCountError {
            function: "启动进程".to_string(),
            expected: 1,
            actual: 0,
        });
    }

    let command_str = match &args[0] {
        Value::字符串(s) => s,
        _ => return Err(RuntimeError::TypeError("命令必须是字符串".to_string())),
    };

    let child = if args.len() > 1 {
        let args_vec: Vec<String> = args[1..]
            .iter()
            .map(|arg| match arg {
                Value::字符串(s) => s.clone(),
                _ => arg.to_string(),
            })
            .collect();
        Command::new(command_str).args(&args_vec).spawn()
    } else {
        Command::new(command_str).spawn()
    };

    match child {
        Ok(process) => {
            let pid = process.id();
            Ok(Value::整数(pid as i64))
        }
        Err(e) => Err(RuntimeError::General(format!("启动进程失败: {}", e))),
    }
}

pub fn builtin_list_env(_args: Vec<Value>) -> RuntimeResult<Value> {
    let env_vars: Vec<Value> = env::vars()
        .map(|(key, value)| {
            let mut pair = std::collections::HashMap::new();
            pair.insert("键".to_string(), Value::字符串(key));
            pair.insert("值".to_string(), Value::字符串(value));
            Value::字典(pair)
        })
        .collect();

    Ok(Value::列表(env_vars))
}

pub fn builtin_path_exists(args: Vec<Value>) -> RuntimeResult<Value> {
    if args.is_empty() {
        return Err(RuntimeError::ArgumentCountError {
            function: "路径存在".to_string(),
            expected: 1,
            actual: 0,
        });
    }

    let path = match &args[0] {
        Value::字符串(s) => s,
        _ => return Err(RuntimeError::TypeError("路径必须是字符串".to_string())),
    };

    Ok(Value::布尔值(Path::new(path).exists()))
}

pub fn builtin_path_is_file(args: Vec<Value>) -> RuntimeResult<Value> {
    if args.is_empty() {
        return Err(RuntimeError::ArgumentCountError {
            function: "是文件".to_string(),
            expected: 1,
            actual: 0,
        });
    }

    let path = match &args[0] {
        Value::字符串(s) => s,
        _ => return Err(RuntimeError::TypeError("路径必须是字符串".to_string())),
    };

    Ok(Value::布尔值(Path::new(path).is_file()))
}

pub fn builtin_path_is_dir(args: Vec<Value>) -> RuntimeResult<Value> {
    if args.is_empty() {
        return Err(RuntimeError::ArgumentCountError {
            function: "是目录".to_string(),
            expected: 1,
            actual: 0,
        });
    }

    let path = match &args[0] {
        Value::字符串(s) => s,
        _ => return Err(RuntimeError::TypeError("路径必须是字符串".to_string())),
    };

    Ok(Value::布尔值(Path::new(path).is_dir()))
}

pub fn builtin_get_temp_dir(_args: Vec<Value>) -> RuntimeResult<Value> {
    match env::temp_dir().to_str() {
        Some(path) => Ok(Value::字符串(path.to_string())),
        None => Err(RuntimeError::General("无法获取临时目录".to_string())),
    }
}

pub fn builtin_get_home_dir(_args: Vec<Value>) -> RuntimeResult<Value> {
    match env::var("HOME").or_else(|_| env::var("USERPROFILE")) {
        Ok(path) => Ok(Value::字符串(path)),
        Err(_) => Err(RuntimeError::General("无法获取用户主目录".to_string())),
    }
}

pub fn builtin_join_path(args: Vec<Value>) -> RuntimeResult<Value> {
    if args.len() < 2 {
        return Err(RuntimeError::ArgumentCountError {
            function: "连接路径".to_string(),
            expected: 2,
            actual: args.len(),
        });
    }

    let base = match &args[0] {
        Value::字符串(s) => s,
        _ => return Err(RuntimeError::TypeError("路径必须是字符串".to_string())),
    };

    let mut path = Path::new(base).to_path_buf();

    for arg in &args[1..] {
        let component = match arg {
            Value::字符串(s) => s,
            _ => return Err(RuntimeError::TypeError("路径组件必须是字符串".to_string())),
        };
        path = path.join(component);
    }

    Ok(Value::字符串(path.to_string_lossy().to_string()))
}

pub fn builtin_get_absolute_path(args: Vec<Value>) -> RuntimeResult<Value> {
    if args.is_empty() {
        return Err(RuntimeError::ArgumentCountError {
            function: "获取绝对路径".to_string(),
            expected: 1,
            actual: 0,
        });
    }

    let path = match &args[0] {
        Value::字符串(s) => s,
        _ => return Err(RuntimeError::TypeError("路径必须是字符串".to_string())),
    };

    match fs::canonicalize(path) {
        Ok(abs_path) => Ok(Value::字符串(abs_path.to_string_lossy().to_string())),
        Err(e) => Err(RuntimeError::General(format!("获取绝对路径失败: {}", e))),
    }
}

pub fn builtin_get_path_separator(_args: Vec<Value>) -> RuntimeResult<Value> {
    let separator = if cfg!(windows) { "\\" } else { "/" };
    Ok(Value::字符串(separator.to_string()))
}
