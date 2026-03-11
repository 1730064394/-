use std::collections::{HashMap, HashSet};
use std::fmt;

#[derive(Debug, Clone)]
pub enum Value {
    整数(i64),
    浮点数(f64),
    字符串(String),
    布尔值(bool),
    空值,
    列表(Vec<Value>),
    字典(HashMap<String, Value>),
    集合(Vec<Value>),
    函数 {
        名称: String,
        参数: Vec<String>,
        默认值: Vec<Option<crate::parser::ast::Expression>>,
        可变参数名: Option<String>,
        闭包: Environment,
    },
    内置函数 {
        名称: String,
        函数: fn(Vec<Value>) -> RuntimeResult<Value>,
    },
    对象 {
        类名: String,
        属性: HashMap<String, Value>,
        属性权限: HashMap<String, crate::parser::ast::AccessModifier>,
    },
    类 {
        名称: String,
        父类: Option<String>,
        方法: HashMap<String, Value>,
        方法权限: HashMap<String, crate::parser::ast::AccessModifier>,
        属性默认值: HashMap<String, Value>,
        属性权限: HashMap<String, crate::parser::ast::AccessModifier>,
    },
    生成器 {
        值: Box<Value>,
        完成: bool,
    },
}

impl Value {
    pub fn type_name(&self) -> &str {
        match self {
            Value::整数(_) => "整数",
            Value::浮点数(_) => "浮点数",
            Value::字符串(_) => "字符串",
            Value::布尔值(_) => "布尔值",
            Value::空值 => "空值",
            Value::列表(_) => "列表",
            Value::字典(_) => "字典",
            Value::集合(_) => "集合",
            Value::函数 { .. } => "函数",
            Value::内置函数 { .. } => "内置函数",
            Value::对象 { .. } => "对象",
            Value::类 { .. } => "类",
            Value::生成器 { .. } => "生成器",
        }
    }
    
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::布尔值(b) => *b,
            Value::整数(n) => *n != 0,
            Value::浮点数(n) => *n != 0.0,
            Value::字符串(s) => !s.is_empty(),
            Value::空值 => false,
            Value::列表(v) => !v.is_empty(),
            Value::字典(m) => !m.is_empty(),
            Value::集合(v) => !v.is_empty(),
            _ => true,
        }
    }
    
    pub fn to_string_value(&self) -> String {
        match self {
            Value::整数(n) => n.to_string(),
            Value::浮点数(n) => {
                if n.fract() == 0.0 {
                    format!("{:.1}", n)
                } else {
                    n.to_string()
                }
            }
            Value::字符串(s) => s.to_string(),
            Value::布尔值(b) => if *b { "真" } else { "假" }.to_string(),
            Value::空值 => "空".to_string(),
            Value::列表(v) => {
                let elements: Vec<String> = v.iter().map(|e| e.to_string_value()).collect();
                format!("[{}]", elements.join("，"))
            }
            Value::字典(m) => {
                let pairs: Vec<String> = m
                    .iter()
                    .map(|(k, v)| format!("{}: {}", k, v.to_string_value()))
                    .collect();
                format!("{{{}}}", pairs.join("，"))
            }
            Value::集合(v) => {
                let elements: Vec<String> = v.iter().map(|e| e.to_string_value()).collect();
                format!("{{{}}}", elements.join("，"))
            }
            Value::函数 { 名称, .. } => format!("<函数 {}>", 名称),
            Value::内置函数 { 名称, .. } => format!("<内置函数 {}>", 名称),
            Value::对象 { 类名, 属性, .. } => {
                let props: Vec<String> = 属性
                    .iter()
                    .map(|(k, v)| format!("{}: {}", k, v.to_string_value()))
                    .collect();
                format!("<{} 对象 {{{}}}>" , 类名, props.join("，"))
            }
            Value::类 { 名称, .. } => format!("<类 {}>", 名称),
            Value::生成器 { 完成, .. } => {
                if *完成 {
                    "<生成器 (已完成)>".to_string()
                } else {
                    "<生成器>".to_string()
                }
            }
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string_value())
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::整数(a), Value::整数(b)) => a == b,
            (Value::浮点数(a), Value::浮点数(b)) => a == b,
            (Value::整数(a), Value::浮点数(b)) => (*a as f64) == *b,
            (Value::浮点数(a), Value::整数(b)) => *a == (*b as f64),
            (Value::字符串(a), Value::字符串(b)) => a == b,
            (Value::布尔值(a), Value::布尔值(b)) => a == b,
            (Value::空值, Value::空值) => true,
            (Value::列表(a), Value::列表(b)) => a == b,
            (Value::字典(a), Value::字典(b)) => a == b,
            (Value::集合(a), Value::集合(b)) => a == b,
            _ => false,
        }
    }
}

use crate::error::RuntimeResult;

#[derive(Debug, Clone)]
pub struct Environment {
    variables: HashMap<String, Value>,
    parent: Option<Box<Environment>>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            variables: HashMap::new(),
            parent: None,
        }
    }
    
    pub fn with_parent(parent: Environment) -> Self {
        Environment {
            variables: HashMap::new(),
            parent: Some(Box::new(parent)),
        }
    }
    
    pub fn define(&mut self, name: String, value: Value) {
        self.variables.insert(name, value);
    }
    
    pub fn get(&self, name: &str) -> Option<&Value> {
        if let Some(value) = self.variables.get(name) {
            Some(value)
        } else if let Some(parent) = &self.parent {
            parent.get(name)
        } else {
            None
        }
    }
    
    pub fn set(&mut self, name: &str, value: Value) -> bool {
        if self.variables.contains_key(name) {
            self.variables.insert(name.to_string(), value);
            true
        } else if let Some(parent) = &mut self.parent {
            parent.set(name, value)
        } else {
            false
        }
    }
    
    pub fn get_mut(&mut self, name: &str) -> Option<&mut Value> {
        if self.variables.contains_key(name) {
            self.variables.get_mut(name)
        } else if let Some(parent) = &mut self.parent {
            parent.get_mut(name)
        } else {
            None
        }
    }
    
    pub fn remove(&mut self, name: &str) {
        self.variables.remove(name);
    }
    
    pub fn get_all_variables(&self) -> &HashMap<String, Value> {
        &self.variables
    }
    
    pub fn has(&self, name: &str) -> bool {
        self.variables.contains_key(name)
            || self.parent.as_ref().map_or(false, |p| p.has(name))
    }
}

impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}
