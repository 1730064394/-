use crate::gui::events::{Event, EventHandler};
use crate::gui::Control;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LabelAlignment {
    左对齐,
    居中对齐,
    右对齐,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TextWrap {
    不换行,
    自动换行,
    字符换行,
}

pub struct Label {
    id: String,
    文本: String,
    x: f32,
    y: f32,
    宽度: f32,
    高度: f32,
    对齐方式: LabelAlignment,
    换行方式: TextWrap,
    可见: bool,
    字体名称: String,
    字体大小: f32,
    字体粗细: u32,
    字体颜色: String,
    背景颜色: Option<String>,
    边框宽度: f32,
    边框颜色: Option<String>,
    行间距: f32,
    字间距: f32,
    下划线: bool,
    删除线: bool,
    斜体: bool,
}

impl Label {
    pub fn new(文本: &str) -> Self {
        Label {
            id: format!("label_{}", std::time::Instant::now().elapsed().as_nanos()),
            文本: 文本.to_string(),
            x: 0.0,
            y: 0.0,
            宽度: 200.0,
            高度: 30.0,
            对齐方式: LabelAlignment::左对齐,
            换行方式: TextWrap::不换行,
            可见: true,
            字体名称: "微软雅黑".to_string(),
            字体大小: 14.0,
            字体粗细: 400,
            字体颜色: "#000000".to_string(),
            背景颜色: None,
            边框宽度: 0.0,
            边框颜色: None,
            行间距: 1.2,
            字间距: 0.0,
            下划线: false,
            删除线: false,
            斜体: false,
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
    
    pub fn with_alignment(mut self, alignment: LabelAlignment) -> Self {
        self.对齐方式 = alignment;
        self
    }
    
    pub fn with_wrap(mut self, wrap: TextWrap) -> Self {
        self.换行方式 = wrap;
        self
    }
    
    pub fn with_font(mut self, name: &str, size: f32) -> Self {
        self.字体名称 = name.to_string();
        self.字体大小 = size;
        self
    }
    
    pub fn with_font_weight(mut self, weight: u32) -> Self {
        self.字体粗细 = weight;
        self
    }
    
    pub fn with_text_color(mut self, color: &str) -> Self {
        self.字体颜色 = color.to_string();
        self
    }
    
    pub fn with_background(mut self, color: &str) -> Self {
        self.背景颜色 = Some(color.to_string());
        self
    }
    
    pub fn with_border(mut self, width: f32, color: &str) -> Self {
        self.边框宽度 = width;
        self.边框颜色 = Some(color.to_string());
        self
    }
    
    pub fn with_line_spacing(mut self, spacing: f32) -> Self {
        self.行间距 = spacing;
        self
    }
    
    pub fn with_letter_spacing(mut self, spacing: f32) -> Self {
        self.字间距 = spacing;
        self
    }
    
    pub fn with_underline(mut self, underline: bool) -> Self {
        self.下划线 = underline;
        self
    }
    
    pub fn with_strikethrough(mut self, strikethrough: bool) -> Self {
        self.删除线 = strikethrough;
        self
    }
    
    pub fn with_italic(mut self, italic: bool) -> Self {
        self.斜体 = italic;
        self
    }
    
    pub fn set_text(&mut self, 文本: &str) {
        self.文本 = 文本.to_string();
    }
    
    pub fn get_text(&self) -> &str {
        &self.文本
    }
    
    pub fn set_alignment(&mut self, alignment: LabelAlignment) {
        self.对齐方式 = alignment;
    }
    
    pub fn get_alignment(&self) -> LabelAlignment {
        self.对齐方式
    }
    
    pub fn set_font_size(&mut self, size: f32) {
        self.字体大小 = size;
    }
    
    pub fn get_font_size(&self) -> f32 {
        self.字体大小
    }
    
    pub fn set_text_color(&mut self, color: &str) {
        self.字体颜色 = color.to_string();
    }
    
    pub fn get_text_color(&self) -> &str {
        &self.字体颜色
    }
    
    pub fn set_background_color(&mut self, color: Option<&str>) {
        self.背景颜色 = color.map(|c| c.to_string());
    }
    
    pub fn get_background_color(&self) -> Option<&String> {
        self.背景颜色.as_ref()
    }
    
    pub fn set_underline(&mut self, underline: bool) {
        self.下划线 = underline;
    }
    
    pub fn is_underline(&self) -> bool {
        self.下划线
    }
    
    pub fn set_strikethrough(&mut self, strikethrough: bool) {
        self.删除线 = strikethrough;
    }
    
    pub fn is_strikethrough(&self) -> bool {
        self.删除线
    }
    
    pub fn set_italic(&mut self, italic: bool) {
        self.斜体 = italic;
    }
    
    pub fn is_italic(&self) -> bool {
        self.斜体
    }
    
    pub fn calculate_text_width(&self) -> f32 {
        let char_width = self.字体大小 * 0.6;
        let text_len = self.文本.chars().count() as f32;
        char_width * text_len
    }
    
    pub fn calculate_text_height(&self) -> f32 {
        let line_height = self.字体大小 * self.行间距;
        let lines = if self.换行方式 == TextWrap::不换行 {
            1
        } else {
            let max_width = self.宽度;
            let char_width = self.字体大小 * 0.6;
            let chars_per_line = (max_width / char_width).floor() as usize;
            let total_chars = self.文本.chars().count();
            ((total_chars + chars_per_line - 1) / chars_per_line).max(1)
        };
        line_height * lines as f32
    }
    
    pub fn render(&self) {
        if !self.可见 {
            return;
        }
        
        let bg_info = if let Some(ref bg) = self.背景颜色 {
            format!(", 背景色={}", bg)
        } else {
            String::new()
        };
        
        let border_info = if let Some(ref border) = self.边框颜色 {
            format!(", 边框={}px, 边框色={}", self.边框宽度, border)
        } else {
            String::new()
        };
        
        let style_info = format!(
            "{}{}{}{}{}",
            if self.斜体 { ", 斜体" } else { "" },
            if self.下划线 { ", 下划线" } else { "" },
            if self.删除线 { ", 删除线" } else { "" },
            if self.字体粗细 != 400 { format!(", 粗细={}", self.字体粗细) } else { String::new() },
            if self.字间距 != 0.0 { format!(", 字间距={}", self.字间距) } else { String::new() }
        );
        
        println!("绘制标签: 位置=({:.1}, {:.1}), 文本='{}', 字体={}px, 颜色={}{}{}{}", 
                 self.x, self.y, self.文本, self.字体大小, self.字体颜色, bg_info, border_info, style_info);
    }
}

impl Control for Label {
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

impl EventHandler for Label {
    fn handle_event(&mut self, _event: &Event) {
        // 标签控件通常不处理事件
    }
}

impl Default for Label {
    fn default() -> Self {
        Self::new("标签")
    }
}