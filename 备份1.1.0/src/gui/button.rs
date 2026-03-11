use crate::gui::events::{Event, EventType, EventHandler};
use crate::gui::Control;


#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ButtonState {
    正常,
    悬停,
    按下,
    禁用,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ButtonStyle {
    默认,
    主要,
    次要,
    成功,
    警告,
    危险,
    链接,
}

pub struct Button {
    id: String,
    文本: String,
    x: f32,
    y: f32,
    宽度: f32,
    高度: f32,
    状态: ButtonState,
    样式: ButtonStyle,
    可见: bool,
    点击回调: Option<Box<dyn Fn() + Send + Sync>>,
    悬停回调: Option<Box<dyn Fn() + Send + Sync>>,
    字体大小: f32,
    字体颜色: String,
    背景颜色: String,
    边框颜色: String,
    圆角半径: f32,
}

impl Button {
    pub fn new(文本: &str) -> Self {
        Button {
            id: format!("button_{}", std::time::Instant::now().elapsed().as_nanos()),
            文本: 文本.to_string(),
            x: 0.0,
            y: 0.0,
            宽度: 120.0,
            高度: 40.0,
            状态: ButtonState::正常,
            样式: ButtonStyle::默认,
            可见: true,
            点击回调: None,
            悬停回调: None,
            字体大小: 16.0,
            字体颜色: "#000000".to_string(),
            背景颜色: "#f0f0f0".to_string(),
            边框颜色: "#cccccc".to_string(),
            圆角半径: 4.0,
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
    
    pub fn with_style(mut self, style: ButtonStyle) -> Self {
        self.样式 = style;
        self.背景颜色 = self.get_style_color(style);
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
    
    pub fn with_corner_radius(mut self, radius: f32) -> Self {
        self.圆角半径 = radius;
        self
    }
    
    pub fn set_text(&mut self, 文本: &str) {
        self.文本 = 文本.to_string();
    }
    
    pub fn get_text(&self) -> &str {
        &self.文本
    }
    
    pub fn set_state(&mut self, state: ButtonState) {
        self.状态 = state;
    }
    
    pub fn get_state(&self) -> ButtonState {
        self.状态
    }
    
    pub fn set_style(&mut self, style: ButtonStyle) {
        self.样式 = style;
        self.背景颜色 = self.get_style_color(style);
    }
    
    pub fn set_enabled(&mut self, enabled: bool) {
        self.状态 = if enabled {
            ButtonState::正常
        } else {
            ButtonState::禁用
        };
    }
    
    pub fn is_enabled(&self) -> bool {
        self.状态 != ButtonState::禁用
    }
    
    pub fn set_click_callback(&mut self, callback: Box<dyn Fn() + Send + Sync>) {
        self.点击回调 = Some(callback);
    }
    
    pub fn set_hover_callback(&mut self, callback: Box<dyn Fn() + Send + Sync>) {
        self.悬停回调 = Some(callback);
    }
    
    pub fn click(&mut self) {
        if !self.is_enabled() {
            return;
        }
        
        self.状态 = ButtonState::按下;
        
        if let Some(ref callback) = self.点击回调 {
            callback();
        }
        
        self.状态 = ButtonState::正常;
    }
    
    pub fn hover(&mut self) {
        if !self.is_enabled() {
            return;
        }
        
        self.状态 = ButtonState::悬停;
        
        if let Some(ref callback) = self.悬停回调 {
            callback();
        }
    }
    
    pub fn leave(&mut self) {
        if !self.is_enabled() {
            return;
        }
        
        self.状态 = ButtonState::正常;
    }
    
    fn get_style_color(&self, style: ButtonStyle) -> String {
        match style {
            ButtonStyle::默认 => "#f0f0f0".to_string(),
            ButtonStyle::主要 => "#007bff".to_string(),
            ButtonStyle::次要 => "#6c757d".to_string(),
            ButtonStyle::成功 => "#28a745".to_string(),
            ButtonStyle::警告 => "#ffc107".to_string(),
            ButtonStyle::危险 => "#dc3545".to_string(),
            ButtonStyle::链接 => "#ffffff".to_string(),
        }
    }
    
    fn darken_color(&self, color: &str, factor: f32) -> String {
        let factor = factor.clamp(0.0, 1.0);
        
        if color.starts_with('#') && color.len() == 7 {
            let r = u8::from_str_radix(&color[1..3], 16).unwrap_or(0);
            let g = u8::from_str_radix(&color[3..5], 16).unwrap_or(0);
            let b = u8::from_str_radix(&color[5..7], 16).unwrap_or(0);
            
            let new_r = (r as f32 * (1.0 - factor)).round() as u8;
            let new_g = (g as f32 * (1.0 - factor)).round() as u8;
            let new_b = (b as f32 * (1.0 - factor)).round() as u8;
            
            format!("#{:02x}{:02x}{:02x}", new_r, new_g, new_b)
        } else {
            color.to_string()
        }
    }
    
    pub fn render(&self) {
        if !self.可见 {
            return;
        }
        
        let bg_color = match self.状态 {
            ButtonState::按下 => self.darken_color(&self.背景颜色, 0.2),
            ButtonState::悬停 => self.darken_color(&self.背景颜色, 0.1),
            ButtonState::禁用 => "#e9ecef".to_string(),
            ButtonState::正常 => self.背景颜色.clone(),
        };
        
        println!("绘制按钮: 位置=({:.1}, {:.1}), 大小=({:.1}×{:.1}), 文本='{}', 颜色={}", 
                 self.x, self.y, self.宽度, self.高度, self.文本, bg_color);
    }
}

impl Control for Button {
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

impl EventHandler for Button {
    fn handle_event(&mut self, event: &Event) {
        match event.事件类型 {
            EventType::点击 => {
                self.click();
            }
            EventType::鼠标移动 => {
                if let Some((mx, my)) = event.鼠标位置 {
                    if mx >= self.x && mx <= self.x + self.宽度 &&
                       my >= self.y && my <= self.y + self.高度 {
                        self.hover();
                    } else {
                        self.leave();
                    }
                }
            }
            _ => {}
        }
    }
}

impl Default for Button {
    fn default() -> Self {
        Self::new("按钮")
    }
}