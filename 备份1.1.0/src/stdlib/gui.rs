use crate::error::{RuntimeError, RuntimeResult};
use crate::gui::{init_gui_engine, get_gui_engine};
use crate::runtime::value::Value;

pub fn builtin_创建窗口(参数: Vec<Value>) -> RuntimeResult<Value> {
    if 参数.len() < 4 {
        return Err(RuntimeError::General("创建窗口需要至少4个参数: 窗口ID, 标题, 宽度, 高度".to_string()));
    }
    
    let 窗口ID = match &参数[0] {
        Value::字符串(s) => s.clone(),
        _ => return Err(RuntimeError::General("窗口ID必须是字符串".to_string())),
    };
    
    let 标题 = match &参数[1] {
        Value::字符串(s) => s.clone(),
        _ => return Err(RuntimeError::General("标题必须是字符串".to_string())),
    };
    
    let 宽度 = match &参数[2] {
        Value::整数(n) => *n as f32,
        Value::浮点数(f) => *f as f32,
        _ => return Err(RuntimeError::General("宽度必须是数字".to_string())),
    };
    
    let 高度 = match &参数[3] {
        Value::整数(n) => *n as f32,
        Value::浮点数(f) => *f as f32,
        _ => return Err(RuntimeError::General("高度必须是数字".to_string())),
    };
    
    init_gui_engine();
    let engine = get_gui_engine();
    engine.create_window(&窗口ID, &标题, 宽度, 高度);
    
    Ok(Value::空值)
}

pub fn builtin_设置当前窗口(参数: Vec<Value>) -> RuntimeResult<Value> {
    if 参数.len() != 1 {
        return Err(RuntimeError::General("设置当前窗口需要1个参数: 窗口ID".to_string()));
    }
    
    let 窗口ID = match &参数[0] {
        Value::字符串(s) => s.clone(),
        _ => return Err(RuntimeError::General("窗口ID必须是字符串".to_string())),
    };
    
    init_gui_engine();
    let engine = get_gui_engine();
    engine.set_current_window(&窗口ID);
    
    Ok(Value::空值)
}

pub fn builtin_添加按钮(参数: Vec<Value>) -> RuntimeResult<Value> {
    if 参数.len() < 6 {
        return Err(RuntimeError::General("添加按钮需要至少6个参数: 按钮ID, 文本, X坐标, Y坐标, 宽度, 高度".to_string()));
    }
    
    let 按钮ID = match &参数[0] {
        Value::字符串(s) => s.clone(),
        _ => return Err(RuntimeError::General("按钮ID必须是字符串".to_string())),
    };
    
    let 文本 = match &参数[1] {
        Value::字符串(s) => s.clone(),
        _ => return Err(RuntimeError::General("按钮文本必须是字符串".to_string())),
    };
    
    let X坐标 = match &参数[2] {
        Value::整数(n) => *n as f32,
        Value::浮点数(f) => *f as f32,
        _ => return Err(RuntimeError::General("X坐标必须是数字".to_string())),
    };
    
    let Y坐标 = match &参数[3] {
        Value::整数(n) => *n as f32,
        Value::浮点数(f) => *f as f32,
        _ => return Err(RuntimeError::General("Y坐标必须是数字".to_string())),
    };
    
    let 宽度 = match &参数[4] {
        Value::整数(n) => *n as f32,
        Value::浮点数(f) => *f as f32,
        _ => return Err(RuntimeError::General("宽度必须是数字".to_string())),
    };
    
    let 高度 = match &参数[5] {
        Value::整数(n) => *n as f32,
        Value::浮点数(f) => *f as f32,
        _ => return Err(RuntimeError::General("高度必须是数字".to_string())),
    };
    
    init_gui_engine();
    let engine = get_gui_engine();
    engine.add_button(&按钮ID, &文本, X坐标, Y坐标, 宽度, 高度);
    
    Ok(Value::空值)
}

pub fn builtin_添加标签(参数: Vec<Value>) -> RuntimeResult<Value> {
    if 参数.len() < 4 {
        return Err(RuntimeError::General("添加标签需要至少4个参数: 标签ID, 文本, X坐标, Y坐标".to_string()));
    }
    
    let 标签ID = match &参数[0] {
        Value::字符串(s) => s.clone(),
        _ => return Err(RuntimeError::General("标签ID必须是字符串".to_string())),
    };
    
    let 文本 = match &参数[1] {
        Value::字符串(s) => s.clone(),
        _ => return Err(RuntimeError::General("标签文本必须是字符串".to_string())),
    };
    
    let X坐标 = match &参数[2] {
        Value::整数(n) => *n as f32,
        Value::浮点数(f) => *f as f32,
        _ => return Err(RuntimeError::General("X坐标必须是数字".to_string())),
    };
    
    let Y坐标 = match &参数[3] {
        Value::整数(n) => *n as f32,
        Value::浮点数(f) => *f as f32,
        _ => return Err(RuntimeError::General("Y坐标必须是数字".to_string())),
    };
    
    init_gui_engine();
    let engine = get_gui_engine();
    engine.add_label(&标签ID, &文本, X坐标, Y坐标);
    
    Ok(Value::空值)
}

pub fn builtin_添加输入框(参数: Vec<Value>) -> RuntimeResult<Value> {
    if 参数.len() < 6 {
        return Err(RuntimeError::General("添加输入框需要至少6个参数: 输入框ID, 占位符, X坐标, Y坐标, 宽度, 高度".to_string()));
    }
    
    let 输入框ID = match &参数[0] {
        Value::字符串(s) => s.clone(),
        _ => return Err(RuntimeError::General("输入框ID必须是字符串".to_string())),
    };
    
    let 占位符 = match &参数[1] {
        Value::字符串(s) => s.clone(),
        _ => return Err(RuntimeError::General("占位符必须是字符串".to_string())),
    };
    
    let X坐标 = match &参数[2] {
        Value::整数(n) => *n as f32,
        Value::浮点数(f) => *f as f32,
        _ => return Err(RuntimeError::General("X坐标必须是数字".to_string())),
    };
    
    let Y坐标 = match &参数[3] {
        Value::整数(n) => *n as f32,
        Value::浮点数(f) => *f as f32,
        _ => return Err(RuntimeError::General("Y坐标必须是数字".to_string())),
    };
    
    let 宽度 = match &参数[4] {
        Value::整数(n) => *n as f32,
        Value::浮点数(f) => *f as f32,
        _ => return Err(RuntimeError::General("宽度必须是数字".to_string())),
    };
    
    let 高度 = match &参数[5] {
        Value::整数(n) => *n as f32,
        Value::浮点数(f) => *f as f32,
        _ => return Err(RuntimeError::General("高度必须是数字".to_string())),
    };
    
    init_gui_engine();
    let engine = get_gui_engine();
    engine.add_input(&输入框ID, &占位符, X坐标, Y坐标, 宽度, 高度);
    
    Ok(Value::空值)
}

pub fn builtin_显示窗口(参数: Vec<Value>) -> RuntimeResult<Value> {
    if 参数.len() != 1 {
        return Err(RuntimeError::General("显示窗口需要1个参数: 窗口ID".to_string()));
    }
    
    let 窗口ID = match &参数[0] {
        Value::字符串(s) => s.clone(),
        _ => return Err(RuntimeError::General("窗口ID必须是字符串".to_string())),
    };
    
    init_gui_engine();
    let engine = get_gui_engine();
    engine.show_window(&窗口ID);
    
    Ok(Value::空值)
}

pub fn builtin_等待(参数: Vec<Value>) -> RuntimeResult<Value> {
    let 秒数 = if 参数.len() >= 1 {
        match &参数[0] {
            Value::整数(n) => *n as f64,
            Value::浮点数(f) => *f,
            _ => return Err(RuntimeError::General("等待时间必须是数字".to_string())),
        }
    } else {
        1.0
    };
    
    std::thread::sleep(std::time::Duration::from_secs_f64(秒数));
    
    Ok(Value::空值)
}
