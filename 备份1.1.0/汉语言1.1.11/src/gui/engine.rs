use eframe::egui;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct Widget {
    pub id: String,
    pub widget_type: WidgetType,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub visible: bool,
    pub text: String,
    pub value: String,
    pub on_click: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum WidgetType {
    按钮,
    标签,
    输入框,
    复选框,
    列表框,
}

#[derive(Debug, Clone)]
pub struct WindowState {
    pub title: String,
    pub width: f32,
    pub height: f32,
    pub widgets: Vec<Widget>,
    pub should_close: bool,
    pub messages: Vec<String>,
}

impl WindowState {
    pub fn new(title: &str, width: f32, height: f32) -> Self {
        WindowState {
            title: title.to_string(),
            width,
            height,
            widgets: Vec::new(),
            should_close: false,
            messages: Vec::new(),
        }
    }

    pub fn add_widget(&mut self, widget: Widget) {
        self.widgets.push(widget);
    }

    pub fn add_message(&mut self, msg: String) {
        self.messages.push(msg);
    }
}

pub struct GuiEngine {
    windows: Arc<Mutex<HashMap<String, WindowState>>>,
    current_window: Arc<Mutex<Option<String>>>,
}

impl GuiEngine {
    pub fn new() -> Self {
        GuiEngine {
            windows: Arc::new(Mutex::new(HashMap::new())),
            current_window: Arc::new(Mutex::new(None)),
        }
    }

    pub fn create_window(&self, id: &str, title: &str, width: f32, height: f32) {
        let mut windows = self.windows.lock().unwrap();
        windows.insert(id.to_string(), WindowState::new(title, width, height));
    }

    pub fn set_current_window(&self, id: &str) {
        let mut current = self.current_window.lock().unwrap();
        *current = Some(id.to_string());
    }

    pub fn add_button(&self, id: &str, text: &str, x: f32, y: f32, width: f32, height: f32) {
        if let Some(window_id) = self.current_window.lock().unwrap().as_ref() {
            if let Some(window) = self.windows.lock().unwrap().get_mut(window_id) {
                window.add_widget(Widget {
                    id: id.to_string(),
                    widget_type: WidgetType::按钮,
                    x,
                    y,
                    width,
                    height,
                    visible: true,
                    text: text.to_string(),
                    value: String::new(),
                    on_click: None,
                });
            }
        }
    }

    pub fn add_label(&self, id: &str, text: &str, x: f32, y: f32) {
        if let Some(window_id) = self.current_window.lock().unwrap().as_ref() {
            if let Some(window) = self.windows.lock().unwrap().get_mut(window_id) {
                window.add_widget(Widget {
                    id: id.to_string(),
                    widget_type: WidgetType::标签,
                    x,
                    y,
                    width: 0.0,
                    height: 0.0,
                    visible: true,
                    text: text.to_string(),
                    value: String::new(),
                    on_click: None,
                });
            }
        }
    }

    pub fn add_input(&self, id: &str, placeholder: &str, x: f32, y: f32, width: f32, height: f32) {
        if let Some(window_id) = self.current_window.lock().unwrap().as_ref() {
            if let Some(window) = self.windows.lock().unwrap().get_mut(window_id) {
                window.add_widget(Widget {
                    id: id.to_string(),
                    widget_type: WidgetType::输入框,
                    x,
                    y,
                    width,
                    height,
                    visible: true,
                    text: placeholder.to_string(),
                    value: String::new(),
                    on_click: None,
                });
            }
        }
    }

    pub fn show_window(&self, id: &str) {
        println!("[DEBUG] show_window 被调用，窗口ID: {}", id);
        
        let (window_title, window_width, window_height, widgets) = {
            let windows = self.windows.lock().unwrap();
            if let Some(window_state) = windows.get(id) {
                println!("[DEBUG] 找到窗口状态: {}x{}", window_state.width, window_state.height);
                (
                    window_state.title.clone(),
                    window_state.width,
                    window_state.height,
                    window_state.widgets.clone(),
                )
            } else {
                println!("[DEBUG] 未找到窗口状态!");
                return;
            }
        };
        
        println!("[DEBUG] 窗口标题: {}", window_title);
        println!("[DEBUG] 控件数量: {}", widgets.len());
        
        let engine = self.clone();
        let window_id = id.to_string();
        
        {
            let mut windows = self.windows.lock().unwrap();
            if let Some(window) = windows.get_mut(&window_id) {
                window.widgets = widgets;
            }
        }
        
        println!("[DEBUG] 正在创建viewport...");
        
        let viewport = egui::ViewportBuilder::default()
            .with_inner_size(egui::vec2(window_width, window_height))
            .with_title(&window_title);
            
        let native_options = eframe::NativeOptions {
            viewport,
            default_theme: eframe::Theme::Light,
            ..Default::default()
        };
        
        println!("[DEBUG] 正在调用 eframe::run_native...");
        
        let result = eframe::run_native(
            &window_title,
            native_options,
            Box::new(|cc| {
                println!("[DEBUG] GuiApp::new 被调用");
                Box::new(GuiApp::new(cc, engine, window_id))
            }),
        );
        
        match result {
            Ok(_) => println!("[DEBUG] 窗口已关闭"),
            Err(e) => println!("[DEBUG] 窗口错误: {:?}", e),
        }
    }

    pub fn add_message(&self, id: &str, msg: String) {
        if let Some(window) = self.windows.lock().unwrap().get_mut(id) {
            window.add_message(msg);
        }
    }
}

impl Clone for GuiEngine {
    fn clone(&self) -> Self {
        GuiEngine {
            windows: Arc::clone(&self.windows),
            current_window: Arc::clone(&self.current_window),
        }
    }
}

struct GuiApp {
    engine: GuiEngine,
    window_id: String,
    input_values: HashMap<String, String>,
}

impl GuiApp {
    fn new(cc: &eframe::CreationContext<'_>, engine: GuiEngine, window_id: String) -> Self {
        // 配置中文字体
        let mut fonts = egui::FontDefinitions::default();
        
        // 尝试加载系统字体
        // Windows系统字体路径
        let font_paths = [
            "C:/Windows/Fonts/msyh.ttc",      // 微软雅黑
            "C:/Windows/Fonts/simhei.ttf",    // 黑体
            "C:/Windows/Fonts/simsun.ttc",    // 宋体
            "C:/Windows/Fonts/msgothic.ttc",  // 日文哥特体（备用）
        ];
        
        let mut font_loaded = false;
        for font_path in &font_paths {
            if let Ok(font_data) = std::fs::read(font_path) {
                println!("[DEBUG] 加载字体: {}", font_path);
                fonts.font_data.insert(
                    "chinese_font".to_owned(),
                    egui::FontData::from_owned(font_data),
                );
                
                // 将中文字体添加到所有字体族
                fonts.families.get_mut(&egui::FontFamily::Proportional).unwrap()
                    .insert(0, "chinese_font".to_owned());
                fonts.families.get_mut(&egui::FontFamily::Monospace).unwrap()
                    .push("chinese_font".to_owned());
                
                font_loaded = true;
                break;
            }
        }
        
        if !font_loaded {
            println!("[DEBUG] 警告: 未能加载任何中文字体");
        }
        
        cc.egui_ctx.set_fonts(fonts);
        
        GuiApp {
            engine,
            window_id,
            input_values: HashMap::new(),
        }
    }
}

impl eframe::App for GuiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let (title, widgets) = {
            let windows = self.engine.windows.lock().unwrap();
            if let Some(window) = windows.get(&self.window_id) {
                (window.title.clone(), window.widgets.clone())
            } else {
                return;
            }
        };
        
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(&title);
            ui.separator();
            
            for widget in &widgets {
                if !widget.visible {
                    continue;
                }
                
                ui.add_space(10.0);
                
                match widget.widget_type {
                    WidgetType::按钮 => {
                        if ui.button(&widget.text).clicked() {
                            self.engine.add_message(&self.window_id, format!("按钮 '{}' 被点击了!", widget.text));
                        }
                    }
                    WidgetType::标签 => {
                        ui.label(&widget.text);
                    }
                    WidgetType::输入框 => {
                        let value = self.input_values.entry(widget.id.clone()).or_insert_with(String::new);
                        ui.text_edit_singleline(value);
                    }
                    _ => {}
                }
            }
            
            ui.separator();
            ui.heading("消息日志:");
            
            let messages = {
                let windows = self.engine.windows.lock().unwrap();
                if let Some(window) = windows.get(&self.window_id) {
                    window.messages.clone()
                } else {
                    Vec::new()
                }
            };
            
            for msg in &messages {
                ui.label(msg);
            }
        });
    }
}

static mut GUI_ENGINE: Option<GuiEngine> = None;

pub fn init_gui_engine() {
    unsafe {
        if GUI_ENGINE.is_none() {
            GUI_ENGINE = Some(GuiEngine::new());
        }
    }
}

pub fn get_gui_engine() -> &'static GuiEngine {
    unsafe {
        if GUI_ENGINE.is_none() {
            init_gui_engine();
        }
        GUI_ENGINE.as_ref().unwrap()
    }
}
