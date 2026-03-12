mod app;
mod editor;
mod highlighter;

pub use app::ChineseProgrammingApp;

use eframe::egui;

pub fn run_ide() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_min_inner_size([800.0, 600.0])
            .with_title("中文编程 IDE"),
        ..Default::default()
    };
    
    eframe::run_native(
        "中文编程 IDE",
        options,
        Box::new(|cc| Box::new(ChineseProgrammingApp::new(cc))),
    )
}
