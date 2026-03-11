use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    关键字,
    标识符,
    整数,
    浮点数,
    字符串,
    格式化字符串,
    运算符,
    分隔符,
    注释,
    换行,
    缩进,
    文件结束,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Keyword {
    定义,
    变量,
    函数,
    返回,
    如果,
    否则,
    否则如果,
    则,
    循环,
    当,
    对于,
    在,
    结束,
    为,
    是,
    真,
    假,
    空,
    且,
    或,
    非,
    导入,
    从,
    尝试,
    捕获,
    抛出,
    类,
    继承,
    新建,
    自,
    打印,
    输入,
    大于,
    小于,
    等于,
    不等于,
    大于等于,
    小于等于,
    公有,
    私有,
    保护,
    属性,
    方法,
    使用,
    作为,
    生成,
    枚举,
}

impl fmt::Display for Keyword {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Keyword::定义 => "定义",
            Keyword::变量 => "变量",
            Keyword::函数 => "函数",
            Keyword::返回 => "返回",
            Keyword::如果 => "如果",
            Keyword::否则 => "否则",
            Keyword::否则如果 => "否则如果",
            Keyword::则 => "则",
            Keyword::循环 => "循环",
            Keyword::当 => "当",
            Keyword::对于 => "对于",
            Keyword::在 => "在",
            Keyword::结束 => "结束",
            Keyword::为 => "为",
            Keyword::是 => "是",
            Keyword::真 => "真",
            Keyword::假 => "假",
            Keyword::空 => "空",
            Keyword::且 => "且",
            Keyword::或 => "或",
            Keyword::非 => "非",
            Keyword::导入 => "导入",
            Keyword::从 => "从",
            Keyword::尝试 => "尝试",
            Keyword::捕获 => "捕获",
            Keyword::抛出 => "抛出",
            Keyword::类 => "类",
            Keyword::继承 => "继承",
            Keyword::新建 => "新建",
            Keyword::自 => "自",
            Keyword::打印 => "打印",
            Keyword::输入 => "输入",
            Keyword::大于 => "大于",
            Keyword::小于 => "小于",
            Keyword::等于 => "等于",
            Keyword::不等于 => "不等于",
            Keyword::大于等于 => "大于等于",
            Keyword::小于等于 => "小于等于",
            Keyword::公有 => "公有",
            Keyword::私有 => "私有",
            Keyword::保护 => "保护",
            Keyword::属性 => "属性",
            Keyword::方法 => "方法",
            Keyword::使用 => "使用",
            Keyword::作为 => "作为",
            Keyword::生成 => "生成",
            Keyword::枚举 => "枚举",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    加,
    减,
    乘,
    除,
    取余,
    等于,
    不等于,
    大于,
    小于,
    大于等于,
    小于等于,
    赋值,
    加等于,
    减等于,
    乘等于,
    除等于,
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Operator::加 => "＋",
            Operator::减 => "－",
            Operator::乘 => "×",
            Operator::除 => "÷",
            Operator::取余 => "％",
            Operator::等于 => "＝＝",
            Operator::不等于 => "！＝",
            Operator::大于 => "＞",
            Operator::小于 => "＜",
            Operator::大于等于 => "＞＝",
            Operator::小于等于 => "＜＝",
            Operator::赋值 => "＝",
            Operator::加等于 => "＋＝",
            Operator::减等于 => "－＝",
            Operator::乘等于 => "×＝",
            Operator::除等于 => "÷＝",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Delimiter {
    左括号,
    右括号,
    左方括号,
    右方括号,
    左花括号,
    右花括号,
    逗号,
    冒号,
    分号,
    点,
}

impl fmt::Display for Delimiter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Delimiter::左括号 => "（",
            Delimiter::右括号 => "）",
            Delimiter::左方括号 => "［",
            Delimiter::右方括号 => "］",
            Delimiter::左花括号 => "｛",
            Delimiter::右花括号 => "｝",
            Delimiter::逗号 => "，",
            Delimiter::冒号 => "：",
            Delimiter::分号 => "；",
            Delimiter::点 => "．",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub value: TokenValue,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone)]
pub enum TokenValue {
    Keyword(Keyword),
    Identifier(String),
    Integer(i64),
    Float(f64),
    String(String),
    Operator(Operator),
    Delimiter(Delimiter),
    Comment(String),
    Newline,
    Indent(usize),
    Eof,
}

impl Token {
    pub fn new(token_type: TokenType, value: TokenValue, line: usize, column: usize) -> Self {
        Token {
            token_type,
            value,
            line,
            column,
        }
    }
    
    pub fn eof(line: usize, column: usize) -> Self {
        Token {
            token_type: TokenType::文件结束,
            value: TokenValue::Eof,
            line,
            column,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.value {
            TokenValue::Keyword(kw) => write!(f, "关键字({})", kw),
            TokenValue::Identifier(id) => write!(f, "标识符({})", id),
            TokenValue::Integer(n) => write!(f, "整数({})", n),
            TokenValue::Float(n) => write!(f, "浮点数({})", n),
            TokenValue::String(s) => write!(f, "字符串(\"{}\")", s),
            TokenValue::Operator(op) => write!(f, "运算符({})", op),
            TokenValue::Delimiter(del) => write!(f, "分隔符({})", del),
            TokenValue::Comment(c) => write!(f, "注释({})", c),
            TokenValue::Newline => write!(f, "换行"),
            TokenValue::Indent(n) => write!(f, "缩进({})", n),
            TokenValue::Eof => write!(f, "文件结束"),
        }
    }
}
