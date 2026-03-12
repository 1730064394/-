use std::cell::RefCell;
use std::io::{self, Write};
use std::rc::Rc;

use crate::error::{LexerError, ParserError, RuntimeError};
use crate::interpreter::Interpreter;
use crate::lexer::tokenize;
use crate::parser::parse;

const VERSION: &str = "0.1.0";
const PROMPT: &str = "汉语编程>>> ";
const CONTINUE_PROMPT: &str = "..........>>> ";

pub struct Repl {
    interpreter: Rc<RefCell<Interpreter>>,
    buffer: String,
    in_block: bool,
    indent_level: usize,
}

impl Repl {
    pub fn new() -> Self {
        Repl {
            interpreter: Rc::new(RefCell::new(Interpreter::new())),
            buffer: String::new(),
            in_block: false,
            indent_level: 0,
        }
    }
    
    pub fn run(&mut self) {
        self.print_welcome();
        
        loop {
            let prompt = if self.in_block {
                CONTINUE_PROMPT
            } else {
                PROMPT
            };
            
            print!("{}", prompt);
            io::stdout().flush().unwrap();
            
            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(0) => {
                    println!("\n再见！");
                    break;
                }
                Ok(_) => {
                    let line = input.trim_end();
                    
                    if line.is_empty() {
                        if self.in_block {
                            self.indent_level = self.indent_level.saturating_sub(1);
                            if self.indent_level == 0 {
                                self.execute_buffer();
                            }
                        }
                        continue;
                    }
                    
                    if self.handle_special_command(line) {
                        continue;
                    }
                    
                    self.process_line(line);
                }
                Err(e) => {
                    eprintln!("读取输入错误: {}", e);
                    break;
                }
            }
        }
    }
    
    fn print_welcome(&self) {
        println!();
        println!("╔══════════════════════════════════════════════════════════╗");
        println!("║                                                          ║");
        println!("║              欢迎使用 中文编程 语言 v{}              ║", VERSION);
        println!("║                                                          ║");
        println!("║  输入「帮助」获取帮助信息，输入「退出」退出程序          ║");
        println!("║                                                          ║");
        println!("╚══════════════════════════════════════════════════════════╝");
        println!();
    }
    
    fn handle_special_command(&mut self, line: &str) -> bool {
        match line {
            "帮助" | "help" | "？"=> {
                self.print_help();
                true
            }
            "退出" | "exit" | "quit" | "再见" => {
                println!("再见！");
                std::process::exit(0);
            }
            "清屏" | "clear" | "cls" => {
                self.clear_screen();
                true
            }
            "版本" | "version" => {
                println!("中文编程 v{}", VERSION);
                true
            }
            "重置" | "reset" => {
                self.interpreter = Rc::new(RefCell::new(Interpreter::new()));
                self.buffer.clear();
                self.in_block = false;
                self.indent_level = 0;
                println!("环境已重置");
                true
            }
            _ => false,
        }
    }
    
    fn print_help(&self) {
        println!();
        println!("════════════════════════════════════════════════════════════");
        println!("                        帮助信息");
        println!("════════════════════════════════════════════════════════════");
        println!();
        println!("【特殊命令】");
        println!("  帮助 / help     - 显示此帮助信息");
        println!("  退出 / exit     - 退出程序");
        println!("  清屏 / clear    - 清除屏幕");
        println!("  版本 / version  - 显示版本信息");
        println!("  重置 / reset    - 重置解释器环境");
        println!();
        println!("【基本语法】");
        println!("  定义 变量名 为 类型");
        println!("  变量名 ＝ 值");
        println!("  定义函数 函数名（参数）为");
        println!("      函数体");
        println!("  结束");
        println!();
        println!("【控制结构】");
        println!("  如果 条件 则");
        println!("      代码块");
        println!("  否则");
        println!("      代码块");
        println!("  结束");
        println!();
        println!("  循环 条件");
        println!("      代码块");
        println!("  结束");
        println!();
        println!("  对于 变量 在 可迭代对象");
        println!("      代码块");
        println!("  结束");
        println!();
        println!("【内置函数】");
        println!("  打印（值）          - 打印输出");
        println!("  长度（对象）        - 获取长度");
        println!("  类型（值）          - 获取类型");
        println!("  转字符串（值）      - 转换为字符串");
        println!("  转整数（值）        - 转换为整数");
        println!("  转浮点数（值）      - 转换为浮点数");
        println!("  范围（开始，结束）  - 生成范围列表");
        println!("  追加（列表，元素）  - 追加元素到列表");
        println!("  排序（列表）        - 排序列表");
        println!("  反转（列表或字符串）- 反转");
        println!();
        println!("【示例】");
        println!("  打印（「你好，世界！」）");
        println!("  定义 甲 为 整数");
        println!("  甲 ＝ 十");
        println!("  打印（甲）");
        println!();
        println!("════════════════════════════════════════════════════════════");
        println!();
    }
    
    fn clear_screen(&self) {
        print!("\x1B[2J\x1B[1;1H");
        io::stdout().flush().unwrap();
    }
    
    fn process_line(&mut self, line: &str) {
        let trimmed = line.trim_start();
        let leading_spaces = line.len() - trimmed.len();
        let current_indent = leading_spaces / 2;
        
        if self.in_block {
            if current_indent > 0 {
                if !self.buffer.is_empty() {
                    self.buffer.push('\n');
                }
                self.buffer.push_str(line);
                
                if trimmed.starts_with("如果") || trimmed.starts_with("循环") 
                    || trimmed.starts_with("对于") || trimmed.starts_with("定义函数")
                    || trimmed.starts_with("尝试") || trimmed.starts_with("类") {
                    self.indent_level += 1;
                }
                
                if trimmed.starts_with("结束") {
                    self.indent_level = self.indent_level.saturating_sub(1);
                    if self.indent_level == 0 {
                        self.execute_buffer();
                    }
                }
            } else if trimmed.starts_with("结束") {
                self.buffer.push_str("\n结束");
                self.execute_buffer();
            } else if trimmed.starts_with("否则") || trimmed.starts_with("否则如果") {
                if !self.buffer.is_empty() {
                    self.buffer.push('\n');
                }
                self.buffer.push_str(trimmed);
            } else {
                if !self.buffer.is_empty() {
                    self.buffer.push('\n');
                }
                self.buffer.push_str(trimmed);
            }
        } else {
            if trimmed.starts_with("如果") || trimmed.starts_with("循环") 
                || trimmed.starts_with("对于") || trimmed.starts_with("定义函数")
                || trimmed.starts_with("尝试") || trimmed.starts_with("类") {
                self.buffer = trimmed.to_string();
                self.in_block = true;
                self.indent_level = 1;
            } else {
                self.buffer = trimmed.to_string();
                self.execute_buffer();
            }
        }
    }
    
    fn execute_buffer(&mut self) {
        if self.buffer.is_empty() {
            self.in_block = false;
            return;
        }
        
        match self.execute_code(&self.buffer.clone()) {
            Ok(Some(value)) => {
                if !matches!(value, crate::runtime::Value::空值) {
                    println!("{}", value);
                }
            }
            Ok(None) => {}
            Err(e) => {
                self.print_error(&e);
            }
        }
        
        self.buffer.clear();
        self.in_block = false;
        self.indent_level = 0;
    }
    
    fn execute_code(&mut self, code: &str) -> Result<Option<crate::runtime::Value>, ExecutionError> {
        let tokens = tokenize(code).map_err(ExecutionError::Lexer)?;
        let program = parse(tokens).map_err(ExecutionError::Parser)?;
        let result = self.interpreter.borrow_mut().run(&program).map_err(ExecutionError::Runtime)?;
        
        Ok(Some(result))
    }
    
    fn print_error(&self, error: &ExecutionError) {
        match error {
            ExecutionError::Lexer(e) => {
                eprintln!("词法错误: {}", e);
            }
            ExecutionError::Parser(e) => {
                eprintln!("语法错误: {}", e);
            }
            ExecutionError::Runtime(e) => {
                eprintln!("运行时错误: {}", e);
            }
        }
    }
}

#[derive(Debug)]
enum ExecutionError {
    Lexer(LexerError),
    Parser(ParserError),
    Runtime(RuntimeError),
}

impl Default for Repl {
    fn default() -> Self {
        Self::new()
    }
}

pub fn run_repl() {
    let mut repl = Repl::new();
    repl.run();
}
