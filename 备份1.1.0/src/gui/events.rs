use std::sync::{Arc, Mutex};

pub type EventCallback = Box<dyn Fn(&Event) + Send + Sync>;

#[derive(Debug, Clone, PartialEq)]
pub enum EventType {
    点击,
    双击,
    右键点击,
    鼠标移动,
    鼠标按下,
    鼠标松开,
    键盘按下,
    键盘松开,
    焦点获得,
    焦点失去,
    窗口关闭,
    窗口大小改变,
    窗口移动,
    文本改变,
    值改变,
}

#[derive(Debug, Clone)]
pub struct Event {
    pub 事件类型: EventType,
    pub 鼠标位置: Option<(f32, f32)>,
    pub 键码: Option<u32>,
    pub 文本内容: Option<String>,
    pub 时间戳: std::time::Instant,
}

impl Event {
    pub fn new(事件类型: EventType) -> Self {
        Event {
            事件类型,
            鼠标位置: None,
            键码: None,
            文本内容: None,
            时间戳: std::time::Instant::now(),
        }
    }
    
    pub fn with_mouse_pos(mut self, x: f32, y: f32) -> Self {
        self.鼠标位置 = Some((x, y));
        self
    }
    
    pub fn with_keycode(mut self, code: u32) -> Self {
        self.键码 = Some(code);
        self
    }
    
    pub fn with_text(mut self, text: String) -> Self {
        self.文本内容 = Some(text);
        self
    }
}

pub trait EventHandler {
    fn handle_event(&mut self, event: &Event);
}

pub struct EventDispatcher {
    handlers: Vec<Arc<Mutex<dyn EventHandler + Send>>>,
}

impl EventDispatcher {
    pub fn new() -> Self {
        EventDispatcher {
            handlers: Vec::new(),
        }
    }
    
    pub fn register_handler(&mut self, handler: Arc<Mutex<dyn EventHandler + Send>>) {
        self.handlers.push(handler);
    }
    
    pub fn dispatch(&self, event: &Event) {
        for handler in &self.handlers {
            if let Ok(mut h) = handler.lock() {
                h.handle_event(event);
            }
        }
    }
}

impl Default for EventDispatcher {
    fn default() -> Self {
        Self::new()
    }
}