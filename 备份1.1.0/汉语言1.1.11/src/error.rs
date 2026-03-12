use thiserror::Error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct SourceLocation {
    pub line: usize,
    pub column: usize,
    pub file: Option<String>,
}

impl fmt::Display for SourceLocation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.file {
            Some(file) => write!(f, "{}:{}:{}", file, self.line, self.column),
            None => write!(f, "行 {}:{}", self.line, self.column),
        }
    }
}

#[derive(Debug, Clone)]
pub struct StackFrame {
    pub function_name: String,
    pub location: SourceLocation,
}

#[derive(Debug, Clone, Default)]
pub struct CallStack {
    pub frames: Vec<StackFrame>,
}

impl CallStack {
    pub fn new() -> Self {
        CallStack { frames: Vec::new() }
    }
    
    pub fn push(&mut self, frame: StackFrame) {
        self.frames.push(frame);
    }
    
    pub fn pop(&mut self) -> Option<StackFrame> {
        self.frames.pop()
    }
    
    pub fn format_trace(&self) -> String {
        if self.frames.is_empty() {
            return "  (无调用栈信息)".to_string();
        }
        
        let mut result = String::new();
        for (i, frame) in self.frames.iter().enumerate().rev() {
            result.push_str(&format!(
                "  #{} 在 {} ({})\n",
                i, frame.function_name, frame.location
            ));
        }
        result
    }
}

#[derive(Error, Debug)]
pub enum LexerError {
    #[error("未知字符: '{0}' 在第 {1} 行，第 {2} 列")]
    UnknownCharacter(char, usize, usize),
    
    #[error("未闭合的字符串，开始于第 {0} 行，第 {1} 列")]
    UnclosedString(usize, usize),
    
    #[error("无效的数字格式: '{0}' 在第 {1} 行，第 {2} 列")]
    InvalidNumber(String, usize, usize),
    
    #[error("无效的标识符: '{0}' 在第 {1} 行，第 {2} 列")]
    InvalidIdentifier(String, usize, usize),
}

#[derive(Error, Debug)]
pub enum ParserError {
    #[error("意外的标记: 期望 '{expected}'，但得到 '{actual}' 在第 {line} 行，第 {column} 列")]
    UnexpectedToken {
        expected: String,
        actual: String,
        line: usize,
        column: usize,
    },
    
    #[error("缺少标记: '{0}'")]
    MissingToken(String),
    
    #[error("语法错误: {0} 在第 {1} 行，第 {2} 列")]
    SyntaxError(String, usize, usize),
    
    #[error("缩进错误: 期望 {expected} 个缩进，但得到 {actual} 个")]
    IndentationError { expected: usize, actual: usize },
}

#[derive(Error, Debug)]
pub enum RuntimeError {
    #[error("未定义的变量: '{0}'")]
    UndefinedVariable(String),
    
    #[error("未定义的函数: '{0}'")]
    UndefinedFunction(String),
    
    #[error("类型错误: {0}")]
    TypeError(String),
    
    #[error("参数数量错误: 函数 '{function}' 期望 {expected} 个参数，但得到 {actual} 个")]
    ArgumentCountError {
        function: String,
        expected: usize,
        actual: usize,
    },
    
    #[error("除零错误")]
    DivisionByZero,
    
    #[error("索引越界: 索引 {index} 超出范围 {length}")]
    IndexOutOfBounds { index: usize, length: usize },
    
    #[error("运行时错误: {0}")]
    General(String),
    
    #[error("属性错误: 对象 '{0}' 没有属性 '{1}'")]
    AttributeError(String, String),
    
    #[error("键错误: 字典没有键 '{0}'")]
    KeyError(String),
    
    #[error("Promise错误: {0}")]
    PromiseError(String),
    
    #[error("异步错误: {0}")]
    AsyncError(String),
    
    #[error("类型注解错误: {0}")]
    TypeAnnotationError(String),
    
    #[error("导入错误: 无法导入模块 '{0}'")]
    ImportError(String),
    
    #[error("文件错误: {0}")]
    FileError(String),
    
    #[error("网络错误: {0}")]
    NetworkError(String),
}

pub type LexerResult<T> = Result<T, LexerError>;
pub type ParserResult<T> = Result<T, ParserError>;
pub type RuntimeResult<T> = Result<T, RuntimeError>;

pub fn format_error_with_source(error: &str, source: &str, line: usize, column: usize) -> String {
    let lines: Vec<&str> = source.lines().collect();
    let mut result = format!("错误: {}\n\n", error);
    
    let start_line = if line > 2 { line - 2 } else { 1 };
    let end_line = (line + 2).min(lines.len());
    
    for i in start_line..=end_line {
        let line_num = i;
        let line_content = lines.get(i - 1).unwrap_or(&"");
        
        if line_num == line {
            result.push_str(&format!(">>> {:4} | {}\n", line_num, line_content));
            
            let pointer = " ".repeat(column + 8);
            result.push_str(&format!("{}^\n", pointer));
            result.push_str(&format!("{}|\n", pointer));
        } else {
            result.push_str(&format!("    {:4} | {}\n", line_num, line_content));
        }
    }
    
    result
}
