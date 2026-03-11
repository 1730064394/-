use crate::gui::events::{Event, EventType, EventDispatcher, EventHandler};
use crate::gui::Control;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WindowState {
    正常,
    最小化,
    最大化,
    全屏,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WindowStyle {
    固定大小,
    可调整大小,
    无边框,
    工具窗口,
}

pub struct Window {
    id: String,
    标题: String,
    宽度: f32,
    高度: f32,
    x: f32,
    y: f32,
    状态: WindowState,
    样式: WindowStyle,
    可见: bool,
    事件分发器: EventDispatcher,
    子控件: Vec<Arc<Mutex<dyn Control + Send>>>,
    #[allow(dead_code)]
    父窗口: Option<String>,
}

impl Window {
    pub fn new(标题: &str) -> Self {
        Window {
            id: format!("window_{}", std::time::Instant::now().elapsed().as_nanos()),
            标题: 标题.to_string(),
            宽度: 800.0,
            高度: 600.0,
            x: 100.0,
            y: 100.0,
            状态: WindowState::正常,
            样式: WindowStyle::可调整大小,
            可见: false,
            事件分发器: EventDispatcher::new(),
            子控件: Vec::new(),
            父窗口: None,
        }
    }
    
    pub fn with_size(mut self, width: f32, height: f32) -> Self {
        self.宽度 = width;
        self.高度 = height;
        self
    }
    
    pub fn with_position(mut self, x: f32, y: f32) -> Self {
        self.x = x;
        self.y = y;
        self
    }
    
    pub fn with_style(mut self, style: WindowStyle) -> Self {
        self.样式 = style;
        self
    }
    
    pub fn get_id(&self) -> &str {
        &self.id
    }
    
    pub fn get_title(&self) -> &str {
        &self.标题
    }
    
    pub fn set_title(&mut self, 标题: &str) {
        self.标题 = 标题.to_string();
    }
    
    pub fn get_size(&self) -> (f32, f32) {
        (self.宽度, self.高度)
    }
    
    pub fn set_size(&mut self, width: f32, height: f32) {
        self.宽度 = width;
        self.高度 = height;
        let event = Event::new(EventType::窗口大小改变);
        self.事件分发器.dispatch(&event);
    }
    
    pub fn get_position(&self) -> (f32, f32) {
        (self.x, self.y)
    }
    
    pub fn set_position(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
        let event = Event::new(EventType::窗口移动);
        self.事件分发器.dispatch(&event);
    }
    
    pub fn get_state(&self) -> WindowState {
        self.状态
    }
    
    pub fn minimize(&mut self) {
        self.状态 = WindowState::最小化;
        self.可见 = false;
    }
    
    pub fn maximize(&mut self) {
        self.状态 = WindowState::最大化;
        self.可见 = true;
    }
    
    pub fn restore(&mut self) {
        self.状态 = WindowState::正常;
        self.可见 = true;
    }
    
    pub fn close(&mut self) {
        let event = Event::new(EventType::窗口关闭);
        self.事件分发器.dispatch(&event);
        self.可见 = false;
    }
    
    pub fn show(&mut self) {
        self.可见 = true;
    }
    
    pub fn hide(&mut self) {
        self.可见 = false;
    }
    
    pub fn is_visible(&self) -> bool {
        self.可见
    }
    
    pub fn add_control(&mut self, control: Arc<Mutex<dyn Control + Send>>) {
        self.子控件.push(control);
    }
    
    pub fn remove_control(&mut self, control_id: &str) {
        self.子控件.retain(|c| {
            if let Ok(ctrl) = c.lock() {
                ctrl.get_id() != control_id
            } else {
                true
            }
        });
    }
    
    pub fn register_event_handler(&mut self, handler: Arc<Mutex<dyn EventHandler + Send>>) {
        self.事件分发器.register_handler(handler);
    }
    
    pub fn draw(&self) {
        if !self.可见 {
            return;
        }
        
        for control in &self.子控件 {
            if let Ok(ctrl) = control.lock() {
                ctrl.draw();
            }
        }
    }
    
    pub fn dispatch_event(&mut self, event: &Event) {
        self.事件分发器.dispatch(event);
        
        for control in &self.子控件 {
            if let Ok(mut ctrl) = control.lock() {

                ctrl.handle_event(event);
            }
        }
    }
}

impl EventHandler for Window {
    fn handle_event(&mut self, event: &Event) {
        match event.事件类型 {
            EventType::窗口关闭 => {
                self.close();
            }
            EventType::窗口大小改变 => {
                // 窗口大小改变时的处理逻辑
            }
            EventType::窗口移动 => {
                // 窗口移动时的处理逻辑
            }
            _ => {}
        }
    }
}

impl Default for Window {
    fn default() -> Self {
        Self::new("新窗口")
    }
}