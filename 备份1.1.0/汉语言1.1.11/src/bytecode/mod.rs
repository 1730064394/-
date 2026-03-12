mod compiler;
mod vm;
mod verifier;

pub use compiler::BytecodeCompiler;
pub use vm::BytecodeVM;
pub use verifier::BytecodeVerifier;

use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum OpCode {
    Nop,
    PushInt(i64),
    PushFloat(f64),
    PushString(String),
    PushBool(bool),
    PushNull,
    PushList(usize),
    PushDict(usize),
    PushSet(usize),
    LoadLocal(usize),
    StoreLocal(usize),
    LoadGlobal(String),
    StoreGlobal(String),
    LoadProperty(String),
    StoreProperty(String),
    LoadIndex,
    StoreIndex,
    LoadSlice,
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Equal,
    NotEqual,
    Greater,
    Less,
    GreaterEqual,
    LessEqual,
    And,
    Or,
    Not,
    Negate,
    Positive,
    CallFunction(usize),
    CallMethod(String, usize),
    Jump(usize),
    JumpIfFalse(usize),
    JumpIfTrue(usize),
    NewObject(String),
    Return,
    Print(usize),
    Swap,
    ListAppend,
    StringConcat,
    SetAdd,
}

impl fmt::Display for OpCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OpCode::Nop => write!(f, "NOP"),
            OpCode::PushInt(n) => write!(f, "PUSH_INT {}", n),
            OpCode::PushFloat(n) => write!(f, "PUSH_FLOAT {}", n),
            OpCode::PushString(s) => write!(f, "PUSH_STRING \"{}\"", s),
            OpCode::PushBool(b) => write!(f, "PUSH_BOOL {}", b),
            OpCode::PushNull => write!(f, "PUSH_NULL"),
            OpCode::PushList(n) => write!(f, "PUSH_LIST {}", n),
            OpCode::PushDict(n) => write!(f, "PUSH_DICT {}", n),
            OpCode::LoadLocal(i) => write!(f, "LOAD_LOCAL {}", i),
            OpCode::StoreLocal(i) => write!(f, "STORE_LOCAL {}", i),
            OpCode::LoadGlobal(s) => write!(f, "LOAD_GLOBAL {}", s),
            OpCode::StoreGlobal(s) => write!(f, "STORE_GLOBAL {}", s),
            OpCode::LoadProperty(s) => write!(f, "LOAD_PROPERTY {}", s),
            OpCode::StoreProperty(s) => write!(f, "STORE_PROPERTY {}", s),
            OpCode::LoadIndex => write!(f, "LOAD_INDEX"),
            OpCode::StoreIndex => write!(f, "STORE_INDEX"),
            OpCode::LoadSlice => write!(f, "LOAD_SLICE"),
            OpCode::Add => write!(f, "ADD"),
            OpCode::Subtract => write!(f, "SUB"),
            OpCode::Multiply => write!(f, "MUL"),
            OpCode::Divide => write!(f, "DIV"),
            OpCode::Modulo => write!(f, "MOD"),
            OpCode::Equal => write!(f, "EQ"),
            OpCode::NotEqual => write!(f, "NEQ"),
            OpCode::Greater => write!(f, "GT"),
            OpCode::Less => write!(f, "LT"),
            OpCode::GreaterEqual => write!(f, "GTE"),
            OpCode::LessEqual => write!(f, "LTE"),
            OpCode::And => write!(f, "AND"),
            OpCode::Or => write!(f, "OR"),
            OpCode::Not => write!(f, "NOT"),
            OpCode::Negate => write!(f, "NEG"),
            OpCode::Positive => write!(f, "POS"),
            OpCode::CallFunction(n) => write!(f, "CALL_FUNC {}", n),
            OpCode::CallMethod(s, n) => write!(f, "CALL_METHOD {} {}", s, n),
            OpCode::Jump(i) => write!(f, "JUMP {}", i),
            OpCode::JumpIfFalse(i) => write!(f, "JUMP_IF_FALSE {}", i),
            OpCode::JumpIfTrue(i) => write!(f, "JUMP_IF_TRUE {}", i),
            OpCode::NewObject(s) => write!(f, "NEW_OBJECT {}", s),
            OpCode::Return => write!(f, "RETURN"),
            OpCode::Print(n) => write!(f, "PRINT {}", n),
            OpCode::Swap => write!(f, "SWAP"),
            OpCode::ListAppend => write!(f, "LIST_APPEND"),
            OpCode::StringConcat => write!(f, "STRING_CONCAT"),
            OpCode::PushSet(n) => write!(f, "PUSH_SET {}", n),
            OpCode::SetAdd => write!(f, "SET_ADD"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Bytecode {
    pub instructions: Vec<OpCode>,
    pub constants: Vec<crate::runtime::value::Value>,
    pub local_names: Vec<String>,
    pub line_numbers: Vec<Option<usize>>,
    pub file_name: Option<String>,
}

impl Bytecode {
    pub fn new() -> Self {
        Bytecode {
            instructions: Vec::new(),
            constants: Vec::new(),
            local_names: Vec::new(),
            line_numbers: Vec::new(),
            file_name: None,
        }
    }
    
    pub fn add_instruction(&mut self, op: OpCode, line: Option<usize>) -> usize {
        let index = self.instructions.len();
        self.instructions.push(op);
        self.line_numbers.push(line);
        index
    }
    
    pub fn add_constant(&mut self, value: crate::runtime::value::Value) -> usize {
        self.constants.push(value);
        self.constants.len() - 1
    }
    
    pub fn add_local(&mut self, name: String) -> usize {
        self.local_names.push(name);
        self.local_names.len() - 1
    }
    
    pub fn get_local_index(&self, name: &str) -> Option<usize> {
        self.local_names.iter().position(|n| n == name)
    }
    
    pub fn patch_jump(&mut self, index: usize, target: usize) {
        match &mut self.instructions[index] {
            OpCode::Jump(ref mut t) |
            OpCode::JumpIfFalse(ref mut t) |
            OpCode::JumpIfTrue(ref mut t) => {
                *t = target;
            }
            _ => panic!("Cannot patch non-jump instruction"),
        }
    }
    
    pub fn len(&self) -> usize {
        self.instructions.len()
    }
    
    pub fn is_empty(&self) -> bool {
        self.instructions.is_empty()
    }
    
    pub fn disassemble(&self) -> String {
        let mut result = String::new();
        for (i, op) in self.instructions.iter().enumerate() {
            result.push_str(&format!("{:04} | {}\n", i, op));
        }
        result
    }
}

impl Default for Bytecode {
    fn default() -> Self {
        Self::new()
    }
}
