use chinese_programming::bytecode::{BytecodeCompiler, BytecodeVM, BytecodeVerifier};
use chinese_programming::ide;
use chinese_programming::interpreter::Interpreter;
use chinese_programming::lexer;
use chinese_programming::parser;
use chinese_programming::repl;
use std::env;
use std::fs;
use std::path::Path;

#[cfg(windows)]
fn setup_console() {
    use windows::Win32::System::Console::{SetConsoleOutputCP, SetConsoleCP};
    unsafe {
        let _ = SetConsoleOutputCP(65001);
        let _ = SetConsoleCP(65001);
    }
}

#[cfg(not(windows))]
fn setup_console() {}

fn main() {
    setup_console();
    
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        print_banner();
        println!("用法:");
        println!("  hanyu              - 启动交互式REPL");
        println!("  hanyu ide          - 启动图形化IDE");
        println!("  hanyu <文件.hy>    - 运行指定的中文编程文件");
        println!("  hanyu --bytecode <文件.hy> - 使用字节码执行文件");
        println!("  hanyu --help       - 显示帮助信息");
        println!();
        println!("正在启动交互式REPL...");
        println!();
        repl::run_repl();
        return;
    }
    
    match args[1].as_str() {
        "ide" | "IDE" => {
            start_ide();
        }
        "--help" | "-h" | "help" | "帮助" => {
            print_help();
        }
        "--version" | "-v" | "version" | "版本" => {
            println!("中文编程 v{}", env!("CARGO_PKG_VERSION"));
        }
        "--bytecode" | "-b" => {
            if args.len() < 3 {
                eprintln!("错误: 请指定要运行的文件");
                std::process::exit(1);
            }
            let file = &args[2];
            if file.ends_with(".hy") || file.ends_with(".汉") {
                run_file_with_bytecode(file);
            } else {
                eprintln!("错误: 不支持的文件格式。请使用 .hy 或 .汉 扩展名。");
                std::process::exit(1);
            }
        }
        file => {
            if file.ends_with(".hy") || file.ends_with(".汉") {
                run_file(file);
            } else {
                eprintln!("错误: 不支持的文件格式。请使用 .hy 或 .汉 扩展名。");
                std::process::exit(1);
            }
        }
    }
}

fn print_banner() {
    println!();
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║                                                              ║");
    println!("║              中文编程语言 v{}                              ║", env!("CARGO_PKG_VERSION"));
    println!("║                                                              ║");
    println!("║          用中文编写代码，让编程更自然                        ║");
    println!("║                                                              ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!();
}

fn print_help() {
    print_banner();
    println!("中文编程语言 - 使用说明");
    println!();
    println!("命令:");
    println!("  hanyu              启动交互式REPL环境");
    println!("  hanyu ide          启动图形化IDE");
    println!("  hanyu <文件>       运行指定的中文编程文件");
    println!("  hanyu --help       显示此帮助信息");
    println!("  hanyu --version    显示版本信息");
    println!();
    println!("示例代码:");
    println!("  打印（「你好，世界！」）");
    println!("  定义 甲 为 整数");
    println!("  甲 ＝ 十");
    println!("  如果 甲 大于 五 则");
    println!("      打印（「甲大于五」）");
    println!("  结束");
    println!();
    println!("更多信息请访问: https://github.com/chinese-programming");
}

fn start_ide() {
    match ide::run_ide() {
        Ok(_) => {}
        Err(e) => {
            eprintln!("启动IDE失败: {}", e);
            std::process::exit(1);
        }
    }
}

fn run_file(filename: &str) {
    let path = Path::new(filename);
    
    if !path.exists() {
        eprintln!("错误: 文件 '{}' 不存在", filename);
        std::process::exit(1);
    }
    
    let source = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("错误: 无法读取文件 '{}': {}", filename, e);
            std::process::exit(1);
        }
    };
    
    println!("正在运行: {}", filename);
    println!("────────────────────────────────────────");
    println!();
    
    match lexer::tokenize(&source) {
        Ok(tokens) => {
            match parser::parse(tokens) {
                Ok(program) => {
                    let mut interpreter = Interpreter::new();
                    if let Err(e) = interpreter.run(&program) {
                        eprintln!();
                        eprintln!("╔══════════════════════════════════════════════════════════════╗");
                        eprintln!("║                      运行时错误                              ║");
                        eprintln!("╚══════════════════════════════════════════════════════════════╝");
                        eprintln!();
                        eprintln!("错误类型: {}", e);
                        eprintln!("文件: {}", filename);
                        eprintln!();
                        std::process::exit(1);
                    }
                    println!();
                    println!("────────────────────────────────────────");
                    println!("程序执行完成。");
                }
                Err(e) => {
                    eprintln!();
                    eprintln!("╔══════════════════════════════════════════════════════════════╗");
                    eprintln!("║                       语法错误                               ║");
                    eprintln!("╚══════════════════════════════════════════════════════════════╝");
                    eprintln!();
                    
                    let error_msg = format!("{}", e);
                    let (line, column) = extract_error_location(&e);
                    
                    if line > 0 {
                        eprintln!("{}", chinese_programming::error::format_error_with_source(
                            &error_msg, &source, line, column
                        ));
                    } else {
                        eprintln!("错误: {}", error_msg);
                    }
                    
                    std::process::exit(1);
                }
            }
        }
        Err(e) => {
            eprintln!();
            eprintln!("╔══════════════════════════════════════════════════════════════╗");
            eprintln!("║                       词法错误                               ║");
            eprintln!("╚══════════════════════════════════════════════════════════════╝");
            eprintln!();
            eprintln!("错误: {}", e);
            std::process::exit(1);
        }
    }
}

fn extract_error_location(error: &chinese_programming::error::ParserError) -> (usize, usize) {
    use chinese_programming::error::ParserError;
    match error {
        ParserError::UnexpectedToken { line, column, .. } => (*line, *column),
        ParserError::SyntaxError(_, line, column) => (*line, *column),
        _ => (0, 0),
    }
}

fn run_file_with_bytecode(filename: &str) {
    let path = Path::new(filename);
    
    if !path.exists() {
        eprintln!("错误: 文件 '{}' 不存在", filename);
        std::process::exit(1);
    }
    
    let source = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("错误: 无法读取文件 '{}': {}", filename, e);
            std::process::exit(1);
        }
    };
    
    println!("正在使用字节码运行: {}", filename);
    println!("────────────────────────────────────────");
    println!();
    
    match lexer::tokenize(&source) {
        Ok(tokens) => {
            match parser::parse(tokens) {
                Ok(program) => {
                    println!("编译字节码编译中...");
                    
                    let mut compiler = BytecodeCompiler::new();
                    match compiler.compile_program(&program) {
                        Ok(bytecode) => {
                            println!("字节码验证中...");
                            let mut verifier = BytecodeVerifier::new();
                            if let Err(errors) = verifier.verify(&bytecode) {
                                for error in errors {
                                    eprintln!("字节码验证错误: {}", error);
                                }
                                std::process::exit(1);
                            }
                            
                            println!("字节码执行中...");
                            println!();
                            
                            let mut vm = BytecodeVM::new();
                            if let Err(e) = vm.run(&bytecode) {
                                eprintln!();
                                eprintln!("字节码执行错误: {}", e);
                                std::process::exit(1);
                            }
                            
                            println!();
                            println!("────────────────────────────────────────");
                            println!("字节码程序执行完成。");
                        }
                        Err(e) => {
                            eprintln!("字节码编译错误: {}", e);
                            std::process::exit(1);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("语法错误: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Err(e) => {
            eprintln!("词法错误: {}", e);
            std::process::exit(1);
        }
    }
}
