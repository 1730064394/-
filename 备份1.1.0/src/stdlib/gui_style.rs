use crate::error::{RuntimeError, RuntimeResult};
use crate::runtime::value::Value;
use std::collections::HashMap;

/// 颜色结构
#[derive(Clone, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Color { r, g, b, a }
    }
    
    pub fn from_hex(hex: &str) -> Option<Self> {
        let hex = hex.trim_start_matches('#');
        match hex.len() {
            6 => {
                let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
                let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
                let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
                Some(Color::new(r, g, b, 255))
            }
            8 => {
                let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
                let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
                let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
                let a = u8::from_str_radix(&hex[6..8], 16).ok()?;
                Some(Color::new(r, g, b, a))
            }
            _ => None,
        }
    }
    
    pub fn to_hex(&self) -> String {
        format!("#{:02X}{:02X}{:02X}{:02X}", self.r, self.g, self.b, self.a)
    }
}

/// 样式结构
#[derive(Clone, Debug)]
pub struct Style {
    pub background_color: Option<Color>,
    pub text_color: Option<Color>,
    pub border_color: Option<Color>,
    pub border_width: f32,
    pub border_radius: f32,
    pub shadow_color: Option<Color>,
    pub shadow_offset: (f32, f32),
    pub shadow_blur: f32,
    pub font_size: f32,
    pub font_weight: u16,
    pub padding: (f32, f32, f32, f32),
    pub margin: (f32, f32, f32, f32),
}

impl Style {
    pub fn new() -> Self {
        Style {
            background_color: None,
            text_color: None,
            border_color: None,
            border_width: 0.0,
            border_radius: 0.0,
            shadow_color: None,
            shadow_offset: (0.0, 0.0),
            shadow_blur: 0.0,
            font_size: 14.0,
            font_weight: 400,
            padding: (0.0, 0.0, 0.0, 0.0),
            margin: (0.0, 0.0, 0.0, 0.0),
        }
    }
}

/// 预定义颜色方案
pub fn get_color_scheme(scheme_name: &str) -> HashMap<String, Color> {
    let mut scheme = HashMap::new();
    
    match scheme_name {
        "海洋蓝" => {
            scheme.insert("主色".to_string(), Color::from_hex("#1890FF").unwrap());
            scheme.insert("辅助色".to_string(), Color::from_hex("#52C41A").unwrap());
            scheme.insert("强调色".to_string(), Color::from_hex("#FAAD14").unwrap());
            scheme.insert("危险色".to_string(), Color::from_hex("#F5222D").unwrap());
            scheme.insert("背景色".to_string(), Color::from_hex("#F0F2F5").unwrap());
            scheme.insert("卡片背景".to_string(), Color::from_hex("#FFFFFF").unwrap());
            scheme.insert("文本主色".to_string(), Color::from_hex("#262626").unwrap());
            scheme.insert("文本次要".to_string(), Color::from_hex("#595959").unwrap());
            scheme.insert("边框色".to_string(), Color::from_hex("#D9D9D9").unwrap());
        }
        "暗夜紫" => {
            scheme.insert("主色".to_string(), Color::from_hex("#722ED1").unwrap());
            scheme.insert("辅助色".to_string(), Color::from_hex("#13C2C2").unwrap());
            scheme.insert("强调色".to_string(), Color::from_hex("#EB2F96").unwrap());
            scheme.insert("危险色".to_string(), Color::from_hex("#CF1322").unwrap());
            scheme.insert("背景色".to_string(), Color::from_hex("#141414").unwrap());
            scheme.insert("卡片背景".to_string(), Color::from_hex("#1F1F1F").unwrap());
            scheme.insert("文本主色".to_string(), Color::from_hex("#E8E8E8").unwrap());
            scheme.insert("文本次要".to_string(), Color::from_hex("#A6A6A6").unwrap());
            scheme.insert("边框色".to_string(), Color::from_hex("#434343").unwrap());
        }
        "自然绿" => {
            scheme.insert("主色".to_string(), Color::from_hex("#27AE60").unwrap());
            scheme.insert("辅助色".to_string(), Color::from_hex("#2980B9").unwrap());
            scheme.insert("强调色".to_string(), Color::from_hex("#E67E22").unwrap());
            scheme.insert("危险色".to_string(), Color::from_hex("#C0392B").unwrap());
            scheme.insert("背景色".to_string(), Color::from_hex("#F8F9FA").unwrap());
            scheme.insert("卡片背景".to_string(), Color::from_hex("#FFFFFF").unwrap());
            scheme.insert("文本主色".to_string(), Color::from_hex("#2C3E50").unwrap());
            scheme.insert("文本次要".to_string(), Color::from_hex("#7F8C8D").unwrap());
            scheme.insert("边框色".to_string(), Color::from_hex("#BDC3C7").unwrap());
        }
        _ => {
            // 默认颜色方案
            scheme.insert("主色".to_string(), Color::from_hex("#1890FF").unwrap());
            scheme.insert("辅助色".to_string(), Color::from_hex("#52C41A").unwrap());
            scheme.insert("强调色".to_string(), Color::from_hex("#FAAD14").unwrap());
            scheme.insert("危险色".to_string(), Color::from_hex("#F5222D").unwrap());
            scheme.insert("背景色".to_string(), Color::from_hex("#F0F2F5").unwrap());
            scheme.insert("卡片背景".to_string(), Color::from_hex("#FFFFFF").unwrap());
            scheme.insert("文本主色".to_string(), Color::from_hex("#262626").unwrap());
            scheme.insert("文本次要".to_string(), Color::from_hex("#595959").unwrap());
            scheme.insert("边框色".to_string(), Color::from_hex("#D9D9D9").unwrap());
        }
    }
    
    scheme
}

/// 创建颜色
pub fn builtin_创建颜色(参数: Vec<Value>) -> RuntimeResult<Value> {
    if 参数.len() < 3 {
        return Err(RuntimeError::ArgumentCountError {
            function: "创建颜色".to_string(),
            expected: 3,
            actual: 参数.len(),
        });
    }
    
    let r = match &参数[0] {
        Value::整数(n) => *n as u8,
        _ => return Err(RuntimeError::TypeError("红色通道必须是整数".to_string())),
    };
    
    let g = match &参数[1] {
        Value::整数(n) => *n as u8,
        _ => return Err(RuntimeError::TypeError("绿色通道必须是整数".to_string())),
    };
    
    let b = match &参数[2] {
        Value::整数(n) => *n as u8,
        _ => return Err(RuntimeError::TypeError("蓝色通道必须是整数".to_string())),
    };
    
    let a = if 参数.len() > 3 {
        match &参数[3] {
            Value::整数(n) => *n as u8,
            _ => 255,
        }
    } else {
        255
    };
    
    let color = Color::new(r, g, b, a);
    let mut result = HashMap::new();
    result.insert("r".to_string(), Value::整数(r as i64));
    result.insert("g".to_string(), Value::整数(g as i64));
    result.insert("b".to_string(), Value::整数(b as i64));
    result.insert("a".to_string(), Value::整数(a as i64));
    result.insert("hex".to_string(), Value::字符串(color.to_hex()));
    
    Ok(Value::字典(result))
}

/// 从HEX创建颜色
pub fn builtin_HEX颜色(参数: Vec<Value>) -> RuntimeResult<Value> {
    if 参数.is_empty() {
        return Err(RuntimeError::ArgumentCountError {
            function: "HEX颜色".to_string(),
            expected: 1,
            actual: 0,
        });
    }
    
    let hex = match &参数[0] {
        Value::字符串(s) => s.clone(),
        _ => return Err(RuntimeError::TypeError("颜色代码必须是字符串".to_string())),
    };
    
    match Color::from_hex(&hex) {
        Some(color) => {
            let mut result = HashMap::new();
            result.insert("r".to_string(), Value::整数(color.r as i64));
            result.insert("g".to_string(), Value::整数(color.g as i64));
            result.insert("b".to_string(), Value::整数(color.b as i64));
            result.insert("a".to_string(), Value::整数(color.a as i64));
            result.insert("hex".to_string(), Value::字符串(color.to_hex()));
            Ok(Value::字典(result))
        }
        None => Err(RuntimeError::General(format!("无效的颜色代码: {}", hex))),
    }
}

/// 获取颜色方案
pub fn builtin_获取颜色方案(参数: Vec<Value>) -> RuntimeResult<Value> {
    let scheme_name = if 参数.is_empty() {
        "默认".to_string()
    } else {
        match &参数[0] {
            Value::字符串(s) => s.clone(),
            _ => "默认".to_string(),
        }
    };
    
    let scheme = get_color_scheme(&scheme_name);
    let mut result = HashMap::new();
    
    for (name, color) in scheme {
        let mut color_dict = HashMap::new();
        color_dict.insert("r".to_string(), Value::整数(color.r as i64));
        color_dict.insert("g".to_string(), Value::整数(color.g as i64));
        color_dict.insert("b".to_string(), Value::整数(color.b as i64));
        color_dict.insert("a".to_string(), Value::整数(color.a as i64));
        color_dict.insert("hex".to_string(), Value::字符串(color.to_hex()));
        result.insert(name, Value::字典(color_dict));
    }
    
    Ok(Value::字典(result))
}

/// 创建阴影效果
pub fn builtin_创建阴影(参数: Vec<Value>) -> RuntimeResult<Value> {
    if 参数.len() < 4 {
        return Err(RuntimeError::ArgumentCountError {
            function: "创建阴影".to_string(),
            expected: 4,
            actual: 参数.len(),
        });
    }
    
    let offset_x = match &参数[0] {
        Value::整数(n) => *n as f32,
        Value::浮点数(f) => *f as f32,
        _ => return Err(RuntimeError::TypeError("X偏移必须是数字".to_string())),
    };
    
    let offset_y = match &参数[1] {
        Value::整数(n) => *n as f32,
        Value::浮点数(f) => *f as f32,
        _ => return Err(RuntimeError::TypeError("Y偏移必须是数字".to_string())),
    };
    
    let blur = match &参数[2] {
        Value::整数(n) => *n as f32,
        Value::浮点数(f) => *f as f32,
        _ => return Err(RuntimeError::TypeError("模糊度必须是数字".to_string())),
    };
    
    let color_hex = match &参数[3] {
        Value::字符串(s) => s.clone(),
        _ => return Err(RuntimeError::TypeError("颜色必须是字符串".to_string())),
    };
    
    let mut result = HashMap::new();
    result.insert("offset_x".to_string(), Value::浮点数(offset_x as f64));
    result.insert("offset_y".to_string(), Value::浮点数(offset_y as f64));
    result.insert("blur".to_string(), Value::浮点数(blur as f64));
    result.insert("color".to_string(), Value::字符串(color_hex));
    
    Ok(Value::字典(result))
}

/// 创建边框样式
pub fn builtin_创建边框(参数: Vec<Value>) -> RuntimeResult<Value> {
    if 参数.len() < 3 {
        return Err(RuntimeError::ArgumentCountError {
            function: "创建边框".to_string(),
            expected: 3,
            actual: 参数.len(),
        });
    }
    
    let width = match &参数[0] {
        Value::整数(n) => *n as f32,
        Value::浮点数(f) => *f as f32,
        _ => return Err(RuntimeError::TypeError("边框宽度必须是数字".to_string())),
    };
    
    let color_hex = match &参数[1] {
        Value::字符串(s) => s.clone(),
        _ => return Err(RuntimeError::TypeError("边框颜色必须是字符串".to_string())),
    };
    
    let radius = match &参数[2] {
        Value::整数(n) => *n as f32,
        Value::浮点数(f) => *f as f32,
        _ => return Err(RuntimeError::TypeError("圆角半径必须是数字".to_string())),
    };
    
    let mut result = HashMap::new();
    result.insert("width".to_string(), Value::浮点数(width as f64));
    result.insert("color".to_string(), Value::字符串(color_hex));
    result.insert("radius".to_string(), Value::浮点数(radius as f64));
    
    Ok(Value::字典(result))
}

/// 创建文本样式
pub fn builtin_创建文本样式(参数: Vec<Value>) -> RuntimeResult<Value> {
    if 参数.is_empty() {
        return Err(RuntimeError::ArgumentCountError {
            function: "创建文本样式".to_string(),
            expected: 1,
            actual: 0,
        });
    }
    
    let size = match &参数[0] {
        Value::整数(n) => *n as f32,
        Value::浮点数(f) => *f as f32,
        _ => return Err(RuntimeError::TypeError("字体大小必须是数字".to_string())),
    };
    
    let weight = if 参数.len() > 1 {
        match &参数[1] {
            Value::整数(n) => *n as u16,
            _ => 400,
        }
    } else {
        400
    };
    
    let color_hex = if 参数.len() > 2 {
        match &参数[2] {
            Value::字符串(s) => s.clone(),
            _ => "#262626".to_string(),
        }
    } else {
        "#262626".to_string()
    };
    
    let mut result = HashMap::new();
    result.insert("size".to_string(), Value::浮点数(size as f64));
    result.insert("weight".to_string(), Value::整数(weight as i64));
    result.insert("color".to_string(), Value::字符串(color_hex));
    
    Ok(Value::字典(result))
}

/// 创建间距（内边距或外边距）
pub fn builtin_创建间距(参数: Vec<Value>) -> RuntimeResult<Value> {
    if 参数.is_empty() {
        return Err(RuntimeError::ArgumentCountError {
            function: "创建间距".to_string(),
            expected: 1,
            actual: 0,
        });
    }
    
    let top = match &参数[0] {
        Value::整数(n) => *n as f32,
        Value::浮点数(f) => *f as f32,
        _ => return Err(RuntimeError::TypeError("间距必须是数字".to_string())),
    };
    
    let right = if 参数.len() > 1 {
        match &参数[1] {
            Value::整数(n) => *n as f32,
            Value::浮点数(f) => *f as f32,
            _ => top,
        }
    } else {
        top
    };
    
    let bottom = if 参数.len() > 2 {
        match &参数[2] {
            Value::整数(n) => *n as f32,
            Value::浮点数(f) => *f as f32,
            _ => top,
        }
    } else {
        top
    };
    
    let left = if 参数.len() > 3 {
        match &参数[3] {
            Value::整数(n) => *n as f32,
            Value::浮点数(f) => *f as f32,
            _ => right,
        }
    } else {
        right
    };
    
    let mut result = HashMap::new();
    result.insert("top".to_string(), Value::浮点数(top as f64));
    result.insert("right".to_string(), Value::浮点数(right as f64));
    result.insert("bottom".to_string(), Value::浮点数(bottom as f64));
    result.insert("left".to_string(), Value::浮点数(left as f64));
    
    Ok(Value::字典(result))
}

/// 应用样式到控件
pub fn builtin_应用样式(参数: Vec<Value>) -> RuntimeResult<Value> {
    if 参数.len() < 2 {
        return Err(RuntimeError::ArgumentCountError {
            function: "应用样式".to_string(),
            expected: 2,
            actual: 参数.len(),
        });
    }
    
    let widget_id = match &参数[0] {
        Value::字符串(s) => s.clone(),
        _ => return Err(RuntimeError::TypeError("控件ID必须是字符串".to_string())),
    };
    
    let style_dict = match &参数[1] {
        Value::字典(d) => d.clone(),
        _ => return Err(RuntimeError::TypeError("样式必须是字典".to_string())),
    };
    
    // 这里应该调用GUI引擎来应用样式
    // 目前只是返回成功
    Ok(Value::空值)
}

/// 创建渐变背景
pub fn builtin_创建渐变(参数: Vec<Value>) -> RuntimeResult<Value> {
    if 参数.len() < 2 {
        return Err(RuntimeError::ArgumentCountError {
            function: "创建渐变".to_string(),
            expected: 2,
            actual: 参数.len(),
        });
    }
    
    let direction = match &参数[0] {
        Value::字符串(s) => s.clone(),
        _ => "水平".to_string(),
    };
    
    let colors = match &参数[1] {
        Value::列表(l) => l.clone(),
        _ => return Err(RuntimeError::TypeError("颜色必须是列表".to_string())),
    };
    
    let mut result = HashMap::new();
    result.insert("direction".to_string(), Value::字符串(direction));
    result.insert("colors".to_string(), Value::列表(colors));
    
    Ok(Value::字典(result))
}
