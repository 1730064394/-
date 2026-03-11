use eframe::egui;
use std::sync::{Arc, Mutex};

use crate::interpreter::Interpreter;
use crate::lexer::tokenize;
use crate::parser::parse;

pub struct ChineseProgrammingApp {
    code_editor: String,
    output: String,
    console_input: String,
    interpreter: Arc<Mutex<Interpreter>>,
    current_file: Option<String>,
    theme: Theme,
    show_console: bool,
    font_size: f32,
    show_about: bool,
    show_help: bool,
    unsaved_changes: bool,
}

#[derive(Clone, Copy, PartialEq)]
enum Theme {
    Light,
    Dark,
}

impl ChineseProgrammingApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        ChineseProgrammingApp {
            code_editor: String::new(),
            output: String::new(),
            console_input: String::new(),
            interpreter: Arc::new(Mutex::new(Interpreter::new())),
            current_file: None,
            theme: Theme::Dark,
            show_console: true,
            font_size: 16.0,
            show_about: false,
            show_help: false,
            unsaved_changes: false,
        }
    }
    
    fn run_code(&mut self) {
        self.output.clear();
        
        match tokenize(&self.code_editor) {
            Ok(tokens) => {
                match parse(tokens) {
                    Ok(program) => {
                        let result = self.interpreter.lock().unwrap().run(&program);
                        match result {
                            Ok(value) => {
                                if !matches!(value, crate::runtime::Value::空值) {
                                    self.output = format!("{}\n", value);
                                }
                                self.output.push_str("程序执行完成。\n");
                            }
                            Err(e) => {
                                self.output = format!("运行时错误: {}\n", e);
                            }
                        }
                    }
                    Err(e) => {
                        self.output = format!("语法错误: {}\n", e);
                    }
                }
            }
            Err(e) => {
                self.output = format!("词法错误: {}\n", e);
            }
        }
    }
    
    fn clear_output(&mut self) {
        self.output.clear();
    }
    
    fn new_file(&mut self) {
        self.code_editor.clear();
        self.output.clear();
        self.current_file = None;
        self.unsaved_changes = false;
    }
    
    fn save_file(&self) {
        if let Some(path) = &self.current_file {
            if let Err(e) = std::fs::write(path, &self.code_editor) {
                eprintln!("保存文件失败: {}", e);
            }
        }
    }
    
    fn save_file_as(&mut self) {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("中文编程文件", &["hy", "汉"])
            .save_file()
        {
            let path_str = path.to_string_lossy().to_string();
            if let Err(e) = std::fs::write(&path, &self.code_editor) {
                self.output = format!("保存文件失败: {}\n", e);
            } else {
                self.current_file = Some(path_str);
                self.unsaved_changes = false;
                self.output = "文件保存成功。\n".to_string();
            }
        }
    }
    
    fn open_file_dialog(&mut self) {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("中文编程文件", &["hy", "汉"])
            .pick_file()
        {
            let path_str = path.to_string_lossy().to_string();
            match std::fs::read_to_string(&path) {
                Ok(content) => {
                    self.code_editor = content;
                    self.current_file = Some(path_str);
                    self.unsaved_changes = false;
                    self.output.clear();
                }
                Err(e) => {
                    self.output = format!("打开文件失败: {}\n", e);
                }
            }
        }
    }
    
    fn execute_console_command(&mut self) {
        if self.console_input.trim().is_empty() {
            return;
        }
        
        let input = self.console_input.clone();
        self.output.push_str(&format!(">>> {}\n", input));
        self.console_input.clear();
        
        match tokenize(&input) {
            Ok(tokens) => {
                match parse(tokens) {
                    Ok(program) => {
                        let result = self.interpreter.lock().unwrap().run(&program);
                        match result {
                            Ok(value) => {
                                if !matches!(value, crate::runtime::Value::空值) {
                                    self.output.push_str(&format!("{}\n", value));
                                }
                            }
                            Err(e) => {
                                self.output.push_str(&format!("错误: {}\n", e));
                            }
                        }
                    }
                    Err(e) => {
                        self.output.push_str(&format!("语法错误: {}\n", e));
                    }
                }
            }
            Err(e) => {
                self.output.push_str(&format!("词法错误: {}\n", e));
            }
        }
    }
}

impl eframe::App for ChineseProgrammingApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.menu_button("文件", |ui| {
                    if ui.button("新建").clicked() {
                        self.new_file();
                        ui.close_menu();
                    }
                    if ui.button("打开...").clicked() {
                        self.open_file_dialog();
                        ui.close_menu();
                    }
                    if ui.button("保存").clicked() {
                        if self.current_file.is_some() {
                            self.save_file();
                            self.unsaved_changes = false;
                        } else {
                            self.save_file_as();
                        }
                        ui.close_menu();
                    }
                    if ui.button("另存为...").clicked() {
                        self.save_file_as();
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.button("退出").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                
                ui.menu_button("编辑", |ui| {
                    if ui.button("撤销").clicked() {
                        ui.close_menu();
                    }
                    if ui.button("重做").clicked() {
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.button("剪切").clicked() {
                        ui.close_menu();
                    }
                    if ui.button("复制").clicked() {
                        ui.close_menu();
                    }
                    if ui.button("粘贴").clicked() {
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.button("全选").clicked() {
                        ui.close_menu();
                    }
                });
                
                ui.menu_button("运行", |ui| {
                    if ui.button("运行程序").clicked() {
                        self.run_code();
                        ui.close_menu();
                    }
                    if ui.button("清空输出").clicked() {
                        self.clear_output();
                        ui.close_menu();
                    }
                });
                
                ui.menu_button("视图", |ui| {
                    if ui.button(if self.show_console { "隐藏控制台" } else { "显示控制台" }).clicked() {
                        self.show_console = !self.show_console;
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.button("浅色主题").clicked() {
                        self.theme = Theme::Light;
                        ui.close_menu();
                    }
                    if ui.button("深色主题").clicked() {
                        self.theme = Theme::Dark;
                        ui.close_menu();
                    }
                    ui.separator();
                    ui.label("字体大小:");
                    ui.add(egui::Slider::new(&mut self.font_size, 12.0..=24.0));
                });
                
                ui.menu_button("帮助", |ui| {
                    if ui.button("使用说明").clicked() {
                        self.show_help = true;
                        ui.close_menu();
                    }
                    if ui.button("关于").clicked() {
                        self.show_about = true;
                        ui.close_menu();
                    }
                });
            });
        });
        
        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("▶ 运行").clicked() {
                    self.run_code();
                }
                if ui.button("⏹ 停止").clicked() {
                }
                if ui.button("🗑 清空").clicked() {
                    self.clear_output();
                }
                ui.separator();
                if ui.button("📄 新建").clicked() {
                    self.new_file();
                }
                if ui.button("💾 保存").clicked() {
                    self.save_file();
                }
                ui.separator();
                ui.label(format!("字体: {:.0}px", self.font_size));
            });
        });
        
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.set_min_width(ui.available_width() * 0.6);
                    ui.label(egui::RichText::new("代码编辑器").size(14.0).strong());
                    
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        let theme = egui::Visuals::dark();
                        let mut frame = egui::Frame::default();
                        frame.fill = theme.extreme_bg_color;
                        frame.inner_margin = egui::Margin::same(8.0);
                        frame.rounding = egui::Rounding::same(4.0);
                        
                        frame.show(ui, |ui| {
                            let _text_style = egui::TextStyle::Monospace;
                            let mut layouter = |ui: &egui::Ui, string: &str, wrap_width: f32| {
                                let mut layout_job = egui::text::LayoutJob::default();
                                layout_job.wrap.max_width = wrap_width;
                                
                                for line in string.lines() {
                                    let mut is_comment = false;
                                    let mut is_keyword = false;
                                    
                                    let keywords = ["定义", "变量", "函数", "返回", "如果", "否则", 
                                        "循环", "当", "对于", "在", "结束", "为", "是", "真", "假", 
                                        "空", "且", "或", "非", "打印", "导入", "类", "新建"];
                                    
                                    for kw in keywords {
                                        if line.contains(kw) {
                                            is_keyword = true;
                                            break;
                                        }
                                    }
                                    
                                    if line.starts_with("注释：") {
                                        is_comment = true;
                                    }
                                    
                                    let color = if is_comment {
                                        egui::Color32::from_rgb(106, 153, 85)
                                    } else if is_keyword {
                                        egui::Color32::from_rgb(86, 156, 214)
                                    } else {
                                        egui::Color32::from_rgb(212, 212, 212)
                                    };
                                    
                                    layout_job.append(
                                        line,
                                        0.0,
                                        egui::TextFormat {
                                            font_id: egui::FontId::monospace(self.font_size),
                                            color,
                                            ..Default::default()
                                        },
                                    );
                                    layout_job.append(
                                        "\n",
                                        0.0,
                                        egui::TextFormat::simple(
                                            egui::FontId::monospace(self.font_size),
                                            egui::Color32::WHITE,
                                        ),
                                    );
                                }
                                
                                ui.fonts(|f| f.layout_job(layout_job))
                            };
                            
                            ui.add(
                                egui::TextEdit::multiline(&mut self.code_editor)
                                    .desired_width(f32::INFINITY)
                                    .desired_rows(20)
                                    .font(egui::FontId::monospace(self.font_size))
                                    .layouter(&mut layouter),
                            );
                        });
                    });
                });
                
                ui.separator();
                
                ui.vertical(|ui| {
                    ui.set_min_width(ui.available_width() * 0.35);
                    ui.label(egui::RichText::new("输出控制台").size(14.0).strong());
                    
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        let theme = egui::Visuals::dark();
                        let mut frame = egui::Frame::default();
                        frame.fill = theme.extreme_bg_color;
                        frame.inner_margin = egui::Margin::same(8.0);
                        frame.rounding = egui::Rounding::same(4.0);
                        
                        frame.show(ui, |ui| {
                            ui.add(
                                egui::TextEdit::multiline(&mut self.output)
                                    .desired_width(f32::INFINITY)
                                    .desired_rows(15)
                                    .font(egui::FontId::monospace(self.font_size))
                                    .interactive(false),
                            );
                        });
                    });
                    
                    ui.separator();
                    
                    ui.label(egui::RichText::new("交互式输入").size(12.0));
                    
                    let response = ui.add(
                        egui::TextEdit::singleline(&mut self.console_input)
                            .desired_width(f32::INFINITY)
                            .font(egui::FontId::monospace(self.font_size))
                            .hint_text("输入代码并按回车执行..."),
                    );
                    
                    if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                        self.execute_console_command();
                        response.request_focus();
                    }
                });
            });
        });
        
        egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("就绪");
                ui.separator();
                ui.label(format!("文件: {}", self.current_file.as_deref().unwrap_or("未保存")));
                ui.separator();
                ui.label(format!("代码行数: {}", self.code_editor.lines().count()));
                ui.separator();
                ui.label(if self.unsaved_changes { "未保存" } else { "已保存" });
                ui.separator();
                ui.label("中文编程 v0.1.0");
            });
        });
        
        if self.show_about {
            egui::Window::new("关于")
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.heading("中文编程语言");
                        ui.label("版本 0.1.0");
                        ui.separator();
                        ui.label("用中文编写代码，让编程更自然");
                        ui.separator();
                        ui.label("© 2024 中文编程团队");
                        ui.separator();
                        if ui.button("确定").clicked() {
                            self.show_about = false;
                        }
                    });
                });
        }
        
        if self.show_help {
            egui::Window::new("使用说明")
                .collapsible(false)
                .resizable(true)
                .default_size([500.0, 400.0])
                .show(ctx, |ui| {
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        ui.heading("基本语法");
                        ui.label("• 定义变量: 定义 变量名 为 类型");
                        ui.label("• 赋值: 变量名 ＝ 值");
                        ui.label("• 打印: 打印（内容）");
                        ui.label("• 条件: 如果 条件 则 ... 否则 ... 结束");
                        ui.label("• 循环: 当 条件 则 ... 结束");
                        ui.label("• 函数: 定义 函数名 为 函数（参数）... 结束");
                        ui.separator();
                        
                        ui.heading("数据类型");
                        ui.label("• 整数: 一、二、三...");
                        ui.label("• 浮点数: 3.14");
                        ui.label("• 字符串: 「内容」");
                        ui.label("• 列表: ［一，二，三］");
                        ui.label("• 字典: ｛「键」: 值｝");
                        ui.separator();
                        
                        ui.heading("快捷键");
                        ui.label("• Ctrl+N: 新建文件");
                        ui.label("• Ctrl+O: 打开文件");
                        ui.label("• Ctrl+S: 保存文件");
                        ui.label("• F5: 运行程序");
                        ui.separator();
                        
                        if ui.button("关闭").clicked() {
                            self.show_help = false;
                        }
                    });
                });
        }
    }
}
