use crate::gui::events::{Event, EventType, EventHandler};
use crate::gui::Control;


#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InputType {
    文本,
    密码,
    数字,
    邮箱,
    电话,
    网址,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ValidationMode {
    无验证,
    非空验证,
    长度验证,
    正则验证,
    自定义验证,
}

pub struct Input {
    id: String,
    占位符: String,
    文本: String,
    x: f32,
    y: f32,
    宽度: f32,
    高度: f32,
    输入类型: InputType,
    验证模式: ValidationMode,
    最大长度: Option<usize>,
    最小长度: Option<usize>,
    正则表达式: Option<String>,
    自定义验证器: Option<Box<dyn Fn(&str) -> bool + Send + Sync>>,
    可见: bool,
    启用: bool,
    只读: bool,
    字体大小: f32,
    字体颜色: String,
    背景颜色: String,
    边框颜色: String,
    #[allow(dead_code)]
    边框宽度: f32,
    #[allow(dead_code)]
    圆角半径: f32,
    光标位置: usize,
    选区开始: Option<usize>,
    选区结束: Option<usize>,
    文本改变回调: Option<Box<dyn Fn(&str) + Send + Sync>>,
    值改变回调: Option<Box<dyn Fn(&str) + Send + Sync>>,
    获得焦点回调: Option<Box<dyn Fn() + Send + Sync>>,
    失去焦点回调: Option<Box<dyn Fn() + Send + Sync>>,
}

impl Input {
    pub fn new(占位符: &str) -> Self {
        Input {
            id: format!("input_{}", std::time::Instant::now().elapsed().as_nanos()),
            占位符: 占位符.to_string(),
            文本: String::new(),
            x: 0.0,
            y: 0.0,
            宽度: 300.0,
            高度: 40.0,
            输入类型: InputType::文本,
            验证模式: ValidationMode::无验证,
            最大长度: None,
            最小长度: None,
            正则表达式: None,
            自定义验证器: None,
            可见: true,
            启用: true,
            只读: false,
            字体大小: 14.0,
            字体颜色: "#000000".to_string(),
            背景颜色: "#ffffff".to_string(),
            边框颜色: "#cccccc".to_string(),
            边框宽度: 1.0,
            圆角半径: 4.0,
            光标位置: 0,
            选区开始: None,
            选区结束: None,
            文本改变回调: None,
            值改变回调: None,
            获得焦点回调: None,
            失去焦点回调: None,
        }
    }
    
    pub fn with_position(mut self, x: f32, y: f32) -> Self {
        self.x = x;
        self.y = y;
        self
    }
    
    pub fn with_size(mut self, width: f32, height: f32) -> Self {
        self.宽度 = width;
        self.高度 = height;
        self
    }
    
    pub fn with_type(mut self, input_type: InputType) -> Self {
        self.输入类型 = input_type;
        self
    }
    
    pub fn with_validation(mut self, mode: ValidationMode) -> Self {
        self.验证模式 = mode;
        self
    }
    
    pub fn with_length_limits(mut self, min: Option<usize>, max: Option<usize>) -> Self {
        self.最小长度 = min;
        self.最大长度 = max;
        if min.is_some() || max.is_some() {
            self.验证模式 = ValidationMode::长度验证;
        }
        self
    }
    
    pub fn with_regex(mut self, pattern: &str) -> Self {
        self.正则表达式 = Some(pattern.to_string());
        self.验证模式 = ValidationMode::正则验证;
        self
    }
    
    pub fn with_custom_validator(mut self, validator: Box<dyn Fn(&str) -> bool + Send + Sync>) -> Self {
        self.自定义验证器 = Some(validator);
        self.验证模式 = ValidationMode::自定义验证;
        self
    }
    
    pub fn with_font_size(mut self, size: f32) -> Self {
        self.字体大小 = size;
        self
    }
    
    pub fn with_colors(mut self, text: &str, background: &str, border: &str) -> Self {
        self.字体颜色 = text.to_string();
        self.背景颜色 = background.to_string();
        self.边框颜色 = border.to_string();
        self
    }
    
    pub fn set_text(&mut self, 文本: &str) {
        let old_text = self.文本.clone();
        self.文本 = 文本.to_string();
        self.光标位置 = self.文本.chars().count();
        
        if let Some(ref callback) = self.文本改变回调 {
            callback(&self.文本);
        }
        
        if old_text != self.文本 {
            if let Some(ref callback) = self.值改变回调 {
                callback(&self.文本);
            }
        }
    }
    
    pub fn get_text(&self) -> &str {
        &self.文本
    }
    
    pub fn append_text(&mut self, text: &str) {
        let old_text = self.文本.clone();
        self.文本.push_str(text);
        self.光标位置 = self.文本.chars().count();
        
        if let Some(ref callback) = self.文本改变回调 {
            callback(&self.文本);
        }
        
        if old_text != self.文本 {
            if let Some(ref callback) = self.值改变回调 {
                callback(&self.文本);
            }
        }
    }
    
    pub fn insert_text(&mut self, pos: usize, text: &str) {
        let old_text = self.文本.clone();
        let mut chars: Vec<char> = self.文本.chars().collect();
        let insert_pos = pos.min(chars.len());
        chars.splice(insert_pos..insert_pos, text.chars());
        self.文本 = chars.into_iter().collect();
        self.光标位置 = insert_pos + text.chars().count();
        
        if let Some(ref callback) = self.文本改变回调 {
            callback(&self.文本);
        }
        
        if old_text != self.文本 {
            if let Some(ref callback) = self.值改变回调 {
                callback(&self.文本);
            }
        }
    }
    
    pub fn delete_text(&mut self, start: usize, end: usize) {
        let old_text = self.文本.clone();
        let mut chars: Vec<char> = self.文本.chars().collect();
        let start_pos = start.min(chars.len());
        let end_pos = end.min(chars.len());
        
        if start_pos < end_pos {
            chars.splice(start_pos..end_pos, []);
            self.文本 = chars.into_iter().collect();
            self.光标位置 = start_pos;
            
            if let Some(ref callback) = self.文本改变回调 {
                callback(&self.文本);
            }
            
            if old_text != self.文本 {
                if let Some(ref callback) = self.值改变回调 {
                    callback(&self.文本);
                }
            }
        }
    }
    
    pub fn clear(&mut self) {
        self.set_text("");
    }
    
    pub fn set_cursor_position(&mut self, pos: usize) {
        self.光标位置 = pos.min(self.文本.chars().count());
    }
    
    pub fn get_cursor_position(&self) -> usize {
        self.光标位置
    }
    
    pub fn set_selection(&mut self, start: Option<usize>, end: Option<usize>) {
        self.选区开始 = start;
        self.选区结束 = end;
    }
    
    pub fn get_selection(&self) -> Option<(usize, usize)> {
        match (&self.选区开始, &self.选区结束) {
            (Some(start), Some(end)) => Some((*start, *end)),
            _ => None,
        }
    }
    
    pub fn get_selected_text(&self) -> Option<String> {
        if let Some((start, end)) = self.get_selection() {
            let chars: Vec<char> = self.文本.chars().collect();
            let start_pos = start.min(chars.len());
            let end_pos = end.min(chars.len());
            if start_pos < end_pos {
                Some(chars[start_pos..end_pos].iter().collect())
            } else {
                None
            }
        } else {
            None
        }
    }
    
    pub fn set_enabled(&mut self, enabled: bool) {
        self.启用 = enabled;
    }
    
    pub fn is_enabled(&self) -> bool {
        self.启用
    }
    
    pub fn set_readonly(&mut self, readonly: bool) {
        self.只读 = readonly;
    }
    
    pub fn is_readonly(&self) -> bool {
        self.只读
    }
    
    pub fn set_text_change_callback(&mut self, callback: Box<dyn Fn(&str) + Send + Sync>) {
        self.文本改变回调 = Some(callback);
    }
    
    pub fn set_value_change_callback(&mut self, callback: Box<dyn Fn(&str) + Send + Sync>) {
        self.值改变回调 = Some(callback);
    }
    
    pub fn set_focus_callbacks(&mut self, 
                           gain: Option<Box<dyn Fn() + Send + Sync>>,
                           lose: Option<Box<dyn Fn() + Send + Sync>>) {
        self.获得焦点回调 = gain;
        self.失去焦点回调 = lose;
    }
    
    pub fn validate(&self) -> Result<(), String> {
        match self.验证模式 {
            ValidationMode::无验证 => Ok(()),
            ValidationMode::非空验证 => {
                if self.文本.trim().is_empty() {
                    Err("输入不能为空".to_string())
                } else {
                    Ok(())
                }
            }
            ValidationMode::长度验证 => {
                let len = self.文本.chars().count();
                if let Some(min) = self.最小长度 {
                    if len < min {
                        return Err(format!("输入长度不能少于{}个字符", min));
                    }
                }
                if let Some(max) = self.最大长度 {
                    if len > max {
                        return Err(format!("输入长度不能超过{}个字符", max));
                    }
                }
                Ok(())
            }
            ValidationMode::正则验证 => {
                if let Some(ref pattern) = self.正则表达式 {
                    match regex::Regex::new(pattern) {
                        Ok(re) => {
                            if !re.is_match(&self.文本) {
                                Err("输入格式不正确".to_string())
                            } else {
                                Ok(())
                            }
                        }
                        Err(_) => Err("正则表达式格式错误".to_string())
                    }
                } else {
                    Ok(())
                }
            }
            ValidationMode::自定义验证 => {
                if let Some(ref validator) = self.自定义验证器 {
                    if validator(&self.文本) {
                        Ok(())
                    } else {
                        Err("输入验证失败".to_string())
                    }
                } else {
                    Ok(())
                }
            }
        }
    }
    
    pub fn is_valid(&self) -> bool {
        self.validate().is_ok()
    }
    
    pub fn get_validation_error(&self) -> Option<String> {
        match self.validate() {
            Ok(()) => None,
            Err(e) => Some(e),
        }
    }
    
    pub fn handle_key_press(&mut self, keycode: u32) {
        if !self.启用 || self.只读 {
            return;
        }
        
        match keycode {
            8 => { // Backspace
                if self.光标位置 > 0 {
                    let mut chars: Vec<char> = self.文本.chars().collect();
                    chars.remove(self.光标位置 - 1);
                    self.文本 = chars.into_iter().collect();
                    self.光标位置 -= 1;
                    
                    if let Some(ref callback) = self.文本改变回调 {
                        callback(&self.文本);
                    }
                    
                    if let Some(ref callback) = self.值改变回调 {
                        callback(&self.文本);
                    }
                }
            }
            46 => { // Delete
                let mut chars: Vec<char> = self.文本.chars().collect();
                if self.光标位置 < chars.len() {
                    chars.remove(self.光标位置);
                    self.文本 = chars.into_iter().collect();
                    
                    if let Some(ref callback) = self.文本改变回调 {
                        callback(&self.文本);
                    }
                    
                    if let Some(ref callback) = self.值改变回调 {
                        callback(&self.文本);
                    }
                }
            }
            37 => { // Left Arrow
                if self.光标位置 > 0 {
                    self.光标位置 -= 1;
                }
            }
            39 => { // Right Arrow
                let max_pos = self.文本.chars().count();
                if self.光标位置 < max_pos {
                    self.光标位置 += 1;
                }
            }
            36 => { // Home
                self.光标位置 = 0;
            }
            35 => { // End
                self.光标位置 = self.文本.chars().count();
            }
            _ => {}
        }
    }
    
    pub fn render(&self) {
        if !self.可见 {
            return;
        }
        
        let display_text = if self.文本.is_empty() {
            &self.占位符
        } else {
            &self.文本
        };
        
        let validation_info = if let Some(ref error) = self.get_validation_error() {
            format!(", 验证错误: {}", error)
        } else {
            String::new()
        };
        
        let type_info = match self.输入类型 {
            InputType::密码 => ", 类型=密码",
            InputType::数字 => ", 类型=数字",
            InputType::邮箱 => ", 类型=邮箱",
            InputType::电话 => ", 类型=电话",
            InputType::网址 => ", 类型=网址",
            InputType::文本 => "",
        };
        
        let text_to_display = match self.输入类型 {
            InputType::密码 => {
                if self.文本.is_empty() {
                    display_text.to_string()
                } else {
                    "*".repeat(self.文本.chars().count())
                }
            }
            _ => display_text.to_string(),
        };
        
        println!("绘制输入框: 位置=({:.1}, {:.1}), 大小=({:.1}×{:.1}), 文本='{}', 光标={}, 验证={}{}{}", 
                 self.x, self.y, self.宽度, self.高度, text_to_display, self.光标位置, 
                 if self.验证模式 == ValidationMode::无验证 { "无" } else { "有" },
                 type_info, validation_info);
    }
}

impl Control for Input {
    fn get_id(&self) -> &str {
        &self.id
    }
    
    fn draw(&self) {
        self.render();
    }
    
    fn set_position(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }
    
    fn set_size(&mut self, width: f32, height: f32) {
        self.宽度 = width;
        self.高度 = height;
    }
    
    fn set_visible(&mut self, visible: bool) {
        self.可见 = visible;
    }
    
    fn is_visible(&self) -> bool {
        self.可见
    }
}

impl EventHandler for Input {
    fn handle_event(&mut self, event: &Event) {
        match event.事件类型 {
            EventType::键盘按下 => {
                if let Some(keycode) = event.键码 {
                    self.handle_key_press(keycode);
                }
            }
            EventType::焦点获得 => {
                if let Some(ref callback) = self.获得焦点回调 {
                    callback();
                }
            }
            EventType::焦点失去 => {
                if let Some(ref callback) = self.失去焦点回调 {
                    callback();
                }
            }
            _ => {}
        }
    }
}

impl Default for Input {
    fn default() -> Self {
        Self::new("请输入内容")
    }
}