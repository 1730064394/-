use std::fmt;

#[derive(Debug, Clone)]
pub struct FunctionParameter {
    pub 名称: String,
    pub 类型: Option<String>,
    pub 默认值: Option<Expression>,
    pub 可变参数: bool,
}

#[derive(Debug, Clone)]
pub enum Statement {
    变量定义 {
        名称: String,
        类型: Option<String>,
        初始值: Option<Expression>,
    },
    解构赋值 {
        变量列表: Vec<String>,
        值: Expression,
    },
    函数定义 {
        名称: String,
        参数: Vec<FunctionParameter>,
        可变参数名: Option<String>,
        返回类型: Option<String>,
        函数体: Vec<Statement>,
    },
    表达式语句 {
        表达式: Expression,
    },
    赋值语句 {
        目标: Expression,
        值: Expression,
    },
    如果语句 {
        条件: Expression,
        如果体: Vec<Statement>,
        否则如果分支: Vec<(Expression, Vec<Statement>)>,
        否则体: Option<Vec<Statement>>,
    },
    循环语句 {
        条件: Expression,
        循环体: Vec<Statement>,
    },
    对于循环 {
        变量: String,
        可迭代对象: Expression,
        循环体: Vec<Statement>,
    },
    返回语句 {
        值: Option<Expression>,
    },
    打印语句 {
        参数: Vec<Expression>,
    },
    导入语句 {
        模块名: String,
        别名: Option<String>,
    },
    类定义 {
        名称: String,
        父类: Option<String>,
        成员: Vec<ClassMember>,
    },
    尝试语句 {
        尝试体: Vec<Statement>,
        捕获分支: Vec<(String, Vec<Statement>)>,
    },
    抛出语句 {
        错误: Expression,
    },
    With语句 {
        表达式: Expression,
        变量名: Option<String>,
        语句体: Vec<Statement>,
    },
    Yield语句 {
        值: Option<Expression>,
    },
    枚举定义 {
        名称: String,
        成员: Vec<(String, Option<Expression>)>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AccessModifier {
    公有,
    私有,
    保护,
}

#[derive(Debug, Clone)]
pub enum ClassMember {
    属性 {
        名称: String,
        类型: Option<String>,
        默认值: Option<Expression>,
        访问权限: AccessModifier,
    },
    方法 {
        名称: String,
        参数: Vec<FunctionParameter>,
        返回类型: Option<String>,
        函数体: Vec<Statement>,
        访问权限: AccessModifier,
    },
}

#[derive(Debug, Clone)]
pub enum Expression {
    整数 {
        值: i64,
    },
    浮点数 {
        值: f64,
    },
    字符串 {
        值: String,
    },
    格式化字符串 {
        部分: Vec<FormatPart>,
    },
    布尔值 {
        值: bool,
    },
    空值,
    标识符 {
        名称: String,
    },
    二元运算 {
        左: Box<Expression>,
        运算符: BinaryOperator,
        右: Box<Expression>,
    },
    一元运算 {
        运算符: UnaryOperator,
        操作数: Box<Expression>,
    },
    函数调用 {
        函数名: String,
        参数: Vec<Expression>,
    },
    匿名函数 {
        参数: Vec<FunctionParameter>,
        可变参数名: Option<String>,
        函数体: Vec<Statement>,
    },
    方法调用 {
        对象: Box<Expression>,
        方法名: String,
        参数: Vec<Expression>,
    },
    属性访问 {
        对象: Box<Expression>,
        属性名: String,
    },
    索引访问 {
        对象: Box<Expression>,
        索引: Box<Expression>,
    },
    切片访问 {
        对象: Box<Expression>,
        开始: Option<Box<Expression>>,
        结束: Option<Box<Expression>>,
        步长: Option<Box<Expression>>,
    },
    列表 {
        元素: Vec<Expression>,
    },
    列表推导式 {
        表达式: Box<Expression>,
        变量: String,
        可迭代对象: Box<Expression>,
        条件: Option<Box<Expression>>,
    },
    字典 {
        键值对: Vec<(Expression, Expression)>,
    },
    字典推导式 {
        键表达式: Box<Expression>,
        值表达式: Box<Expression>,
        变量: String,
        可迭代对象: Box<Expression>,
        条件: Option<Box<Expression>>,
    },
    集合 {
        元素: Vec<Expression>,
    },
    集合推导式 {
        表达式: Box<Expression>,
        变量: String,
        可迭代对象: Box<Expression>,
        条件: Option<Box<Expression>>,
    },
    三元表达式 {
        条件: Box<Expression>,
        真值: Box<Expression>,
        假值: Box<Expression>,
    },
    新建对象 {
        类名: String,
        参数: Vec<Expression>,
    },
}

#[derive(Debug, Clone, Copy)]
pub enum BinaryOperator {
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
    且,
    或,
}

#[derive(Debug, Clone)]
pub enum FormatPart {
    文本(String),
    表达式(Expression),
}

impl fmt::Display for BinaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            BinaryOperator::加 => "＋",
            BinaryOperator::减 => "－",
            BinaryOperator::乘 => "×",
            BinaryOperator::除 => "÷",
            BinaryOperator::取余 => "％",
            BinaryOperator::等于 => "＝＝",
            BinaryOperator::不等于 => "！＝",
            BinaryOperator::大于 => "＞",
            BinaryOperator::小于 => "＜",
            BinaryOperator::大于等于 => "＞＝",
            BinaryOperator::小于等于 => "＜＝",
            BinaryOperator::且 => "且",
            BinaryOperator::或 => "或",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum UnaryOperator {
    非,
    负,
    正,
}

impl fmt::Display for UnaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            UnaryOperator::非 => "非",
            UnaryOperator::负 => "－",
            UnaryOperator::正 => "＋",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone)]
pub struct Program {
    pub statements: Vec<Statement>,
}

impl Program {
    pub fn new() -> Self {
        Program {
            statements: Vec::new(),
        }
    }
    
    pub fn from(statements: Vec<Statement>) -> Self {
        Program { statements }
    }
}
