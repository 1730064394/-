use crate::bytecode::{Bytecode, OpCode};
use crate::error::{RuntimeError, RuntimeResult};
use crate::runtime::value::{Environment, Value};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub struct BytecodeVM {
    pub stack: Vec<Value>,
    pub locals: Vec<Value>,
    pub globals: Rc<RefCell<Environment>>,
    pub pc: usize,
    pub functions: HashMap<String, (Vec<String>, Bytecode)>,
    call_stack: Vec<CallFrame>,
}

struct CallFrame {
    return_pc: usize,
    locals: Vec<Value>,
    _function_locals_count: usize,
}

impl BytecodeVM {
    pub fn new() -> Self {
        let mut env = Environment::new();
        Self::register_builtins(&mut env);

        BytecodeVM {
            stack: Vec::new(),
            locals: Vec::new(),
            globals: Rc::new(RefCell::new(env)),
            pc: 0,
            functions: HashMap::new(),
            call_stack: Vec::new(),
        }
    }

    fn register_builtins(env: &mut Environment) {
        env.define(
            "长度".to_string(),
            Value::内置函数 {
                名称: "长度".to_string(),
                函数: builtin_length,
            },
        );
    }

    pub fn run(&mut self, bytecode: &Bytecode) -> RuntimeResult<Value> {
        self.pc = 0;
        // 预分配局部变量空间，避免动态扩展
        self.locals.resize(bytecode.local_names.len(), Value::空值);
        // 预分配栈空间，减少动态内存分配
        self.stack.reserve(64);

        // 优化的指令执行循环
        let instructions = &bytecode.instructions;
        while self.pc < instructions.len() {
            let op = &instructions[self.pc];
            self.execute_op(op, bytecode)?;
            self.pc += 1;
        }

        if let Some(result) = self.stack.pop() {
            Ok(result)
        } else {
            Ok(Value::空值)
        }
    }

    pub fn execute_op(&mut self, op: &OpCode, bytecode: &Bytecode) -> RuntimeResult<()> {
        match op {
            OpCode::Nop => {}

            OpCode::PushInt(n) => {
                self.stack.push(Value::整数(*n));
            }

            OpCode::PushFloat(n) => {
                self.stack.push(Value::浮点数(*n));
            }

            OpCode::PushString(s) => {
                self.stack.push(Value::字符串(s.clone()));
            }

            OpCode::PushBool(b) => {
                self.stack.push(Value::布尔值(*b));
            }

            OpCode::PushNull => {
                self.stack.push(Value::空值);
            }

            OpCode::PushList(n) => {
                let mut elements = Vec::new();
                for _ in 0..*n {
                    elements.push(
                        self.stack
                            .pop()
                            .ok_or_else(|| RuntimeError::General("栈下溢: PushList".to_string()))?,
                    );
                }
                elements.reverse();
                self.stack.push(Value::列表(elements));
            }

            OpCode::PushDict(n) => {
                let mut map = HashMap::new();
                for _ in 0..*n {
                    let value = self
                        .stack
                        .pop()
                        .ok_or_else(|| RuntimeError::General("栈下溢: PushDict".to_string()))?;
                    let key = self
                        .stack
                        .pop()
                        .ok_or_else(|| RuntimeError::General("栈下溢: PushDict key".to_string()))?;
                    map.insert(key.to_string_value(), value);
                }
                self.stack.push(Value::字典(map));
            }

            OpCode::PushSet(n) => {
                let mut seen = std::collections::HashSet::new();
                let mut result = Vec::new();
                for _ in 0..*n {
                    let value = self
                        .stack
                        .pop()
                        .ok_or_else(|| RuntimeError::General("栈下溢: PushSet".to_string()))?;
                    let key = value.to_string_value();
                    if !seen.contains(&key) {
                        seen.insert(key);
                        result.push(value);
                    }
                }
                self.stack.push(Value::集合(result));
            }

            OpCode::LoadLocal(idx) => {
                let value = self.locals.get(*idx).cloned().ok_or_else(|| {
                    RuntimeError::General(format!("无效的局部变量索引: {}", idx))
                })?;
                self.stack.push(value);
            }

            OpCode::StoreLocal(idx) => {
                let value = self
                    .stack
                    .pop()
                    .ok_or_else(|| RuntimeError::General("栈下溢: StoreLocal".to_string()))?;
                if *idx < self.locals.len() {
                    self.locals[*idx] = value;
                } else {
                    return Err(RuntimeError::General(format!(
                        "无效的局部变量索引: {}",
                        idx
                    )));
                }
            }

            OpCode::LoadGlobal(name) => {
                let value = self
                    .globals
                    .borrow()
                    .get(name)
                    .cloned()
                    .ok_or_else(|| RuntimeError::UndefinedVariable(name.clone()))?;
                self.stack.push(value);
            }

            OpCode::StoreGlobal(name) => {
                let value = self
                    .stack
                    .pop()
                    .ok_or_else(|| RuntimeError::General("栈下溢: StoreGlobal".to_string()))?;
                self.globals.borrow_mut().define(name.clone(), value);
            }

            OpCode::LoadProperty(name) => {
                let obj = self
                    .stack
                    .pop()
                    .ok_or_else(|| RuntimeError::General("栈下溢: LoadProperty".to_string()))?;
                let value = self.get_property(&obj, name)?;
                self.stack.push(value);
            }

            OpCode::StoreProperty(name) => {
                let value = self
                    .stack
                    .pop()
                    .ok_or_else(|| RuntimeError::General("栈下溢: StoreProperty value".to_string()))?;
                let obj = self
                    .stack
                    .pop()
                    .ok_or_else(|| RuntimeError::General("栈下溢: StoreProperty obj".to_string()))?;
                let result = self.set_property(obj, name, value)?;
                self.stack.push(result);
            }

            OpCode::LoadIndex => {
                let index = self
                    .stack
                    .pop()
                    .ok_or_else(|| RuntimeError::General("栈下溢: LoadIndex index".to_string()))?;
                let obj = self
                    .stack
                    .pop()
                    .ok_or_else(|| RuntimeError::General("栈下溢: LoadIndex obj".to_string()))?;
                let value = self.get_index(&obj, &index)?;
                self.stack.push(value);
            }

            OpCode::StoreIndex => {
                let value = self
                    .stack
                    .pop()
                    .ok_or_else(|| RuntimeError::General("栈下溢: StoreIndex value".to_string()))?;
                let index = self
                    .stack
                    .pop()
                    .ok_or_else(|| RuntimeError::General("栈下溢: StoreIndex index".to_string()))?;
                let obj = self
                    .stack
                    .pop()
                    .ok_or_else(|| RuntimeError::General("栈下溢: StoreIndex obj".to_string()))?;
                let result = self.set_index(obj, &index, value)?;
                self.stack.push(result);
            }

            OpCode::LoadSlice => {
                let step = self
                    .stack
                    .pop()
                    .ok_or_else(|| RuntimeError::General("栈下溢: LoadSlice step".to_string()))?;
                let end = self
                    .stack
                    .pop()
                    .ok_or_else(|| RuntimeError::General("栈下溢: LoadSlice end".to_string()))?;
                let start = self
                    .stack
                    .pop()
                    .ok_or_else(|| RuntimeError::General("栈下溢: LoadSlice start".to_string()))?;
                let obj = self
                    .stack
                    .pop()
                    .ok_or_else(|| RuntimeError::General("栈下溢: LoadSlice obj".to_string()))?;
                
                let start_val = if start == Value::空值 { None } else { Some(&start) };
                let end_val = if end == Value::空值 { None } else { Some(&end) };
                let step_val = if step == Value::空值 { None } else { Some(&step) };
                
                let value = self.get_slice(&obj, start_val, end_val, step_val)?;
                self.stack.push(value);
            }

            OpCode::Add => {
                // 优化：直接从栈顶获取元素，减少边界检查
                if self.stack.len() < 2 {
                    return Err(RuntimeError::General("栈下溢: Add".to_string()));
                }
                let right = self.stack.pop().unwrap();
                let left = self.stack.pop().unwrap();
                let result = self.add(&left, &right)?;
                self.stack.push(result);
            }

            OpCode::Subtract => {
                // 优化：直接从栈顶获取元素，减少边界检查
                if self.stack.len() < 2 {
                    return Err(RuntimeError::General("栈下溢: Subtract".to_string()));
                }
                let right = self.stack.pop().unwrap();
                let left = self.stack.pop().unwrap();
                let result = self.subtract(&left, &right)?;
                self.stack.push(result);
            }

            OpCode::Multiply => {
                // 优化：直接从栈顶获取元素，减少边界检查
                if self.stack.len() < 2 {
                    return Err(RuntimeError::General("栈下溢: Multiply".to_string()));
                }
                let right = self.stack.pop().unwrap();
                let left = self.stack.pop().unwrap();
                let result = self.multiply(&left, &right)?;
                self.stack.push(result);
            }

            OpCode::Divide => {
                // 优化：直接从栈顶获取元素，减少边界检查
                if self.stack.len() < 2 {
                    return Err(RuntimeError::General("栈下溢: Divide".to_string()));
                }
                let right = self.stack.pop().unwrap();
                let left = self.stack.pop().unwrap();
                let result = self.divide(&left, &right)?;
                self.stack.push(result);
            }

            OpCode::Modulo => {
                let right = self
                    .stack
                    .pop()
                    .ok_or_else(|| RuntimeError::General("栈下溢: Modulo right".to_string()))?;
                let left = self
                    .stack
                    .pop()
                    .ok_or_else(|| RuntimeError::General("栈下溢: Modulo left".to_string()))?;
                let result = self.modulo(&left, &right)?;
                self.stack.push(result);
            }

            OpCode::Equal => {
                // 优化：直接从栈顶获取元素，减少边界检查
                if self.stack.len() < 2 {
                    return Err(RuntimeError::General("栈下溢: Equal".to_string()));
                }
                let right = self.stack.pop().unwrap();
                let left = self.stack.pop().unwrap();
                self.stack.push(Value::布尔值(left == right));
            }

            OpCode::NotEqual => {
                // 优化：直接从栈顶获取元素，减少边界检查
                if self.stack.len() < 2 {
                    return Err(RuntimeError::General("栈下溢: NotEqual".to_string()));
                }
                let right = self.stack.pop().unwrap();
                let left = self.stack.pop().unwrap();
                self.stack.push(Value::布尔值(left != right));
            }

            OpCode::Greater => {
                // 优化：直接从栈顶获取元素，减少边界检查
                if self.stack.len() < 2 {
                    return Err(RuntimeError::General("栈下溢: Greater".to_string()));
                }
                let right = self.stack.pop().unwrap();
                let left = self.stack.pop().unwrap();
                let result = self.compare(&left, &right, |a, b| a > b)?;
                self.stack.push(result);
            }

            OpCode::Less => {
                // 优化：直接从栈顶获取元素，减少边界检查
                if self.stack.len() < 2 {
                    return Err(RuntimeError::General("栈下溢: Less".to_string()));
                }
                let right = self.stack.pop().unwrap();
                let left = self.stack.pop().unwrap();
                let result = self.compare(&left, &right, |a, b| a < b)?;
                self.stack.push(result);
            }

            OpCode::GreaterEqual => {
                // 优化：直接从栈顶获取元素，减少边界检查
                if self.stack.len() < 2 {
                    return Err(RuntimeError::General("栈下溢: GreaterEqual".to_string()));
                }
                let right = self.stack.pop().unwrap();
                let left = self.stack.pop().unwrap();
                let result = self.compare(&left, &right, |a, b| a >= b)?;
                self.stack.push(result);
            }

            OpCode::LessEqual => {
                // 优化：直接从栈顶获取元素，减少边界检查
                if self.stack.len() < 2 {
                    return Err(RuntimeError::General("栈下溢: LessEqual".to_string()));
                }
                let right = self.stack.pop().unwrap();
                let left = self.stack.pop().unwrap();
                let result = self.compare(&left, &right, |a, b| a <= b)?;
                self.stack.push(result);
            }

            OpCode::And => {
                // 优化：直接从栈顶获取元素，减少边界检查
                if self.stack.len() < 2 {
                    return Err(RuntimeError::General("栈下溢: And".to_string()));
                }
                let right = self.stack.pop().unwrap();
                let left = self.stack.pop().unwrap();
                self.stack.push(Value::布尔值(left.is_truthy() && right.is_truthy()));
            }

            OpCode::Or => {
                // 优化：直接从栈顶获取元素，减少边界检查
                if self.stack.len() < 2 {
                    return Err(RuntimeError::General("栈下溢: Or".to_string()));
                }
                let right = self.stack.pop().unwrap();
                let left = self.stack.pop().unwrap();
                self.stack.push(Value::布尔值(left.is_truthy() || right.is_truthy()));
            }

            OpCode::Not => {
                // 优化：直接从栈顶获取元素，减少边界检查
                if self.stack.is_empty() {
                    return Err(RuntimeError::General("栈下溢: Not".to_string()));
                }
                let value = self.stack.pop().unwrap();
                self.stack.push(Value::布尔值(!value.is_truthy()));
            }

            OpCode::Negate => {
                let value = self
                    .stack
                    .pop()
                    .ok_or_else(|| RuntimeError::General("栈下溢: Negate".to_string()))?;
                let result = match value {
                    Value::整数(n) => Value::整数(-n),
                    Value::浮点数(n) => Value::浮点数(-n),
                    _ => {
                        return Err(RuntimeError::TypeError(format!(
                            "类型 {} 无法取负",
                            value.type_name()
                        )));
                    }
                };
                self.stack.push(result);
            }

            OpCode::Positive => {
                let value = self
                    .stack
                    .pop()
                    .ok_or_else(|| RuntimeError::General("栈下溢: Positive".to_string()))?;
                match value {
                    Value::整数(_) | Value::浮点数(_) => self.stack.push(value),
                    _ => {
                        return Err(RuntimeError::TypeError(format!(
                            "类型 {} 无法取正",
                            value.type_name()
                        )));
                    }
                }
            }

            OpCode::Jump(target) => {
                self.pc = *target - 1;
            }

            OpCode::JumpIfFalse(target) => {
                let value = self
                    .stack
                    .pop()
                    .ok_or_else(|| RuntimeError::General("栈下溢: JumpIfFalse".to_string()))?;
                if !value.is_truthy() {
                    self.pc = *target - 1;
                }
            }

            OpCode::JumpIfTrue(target) => {
                let value = self
                    .stack
                    .pop()
                    .ok_or_else(|| RuntimeError::General("栈下溢: JumpIfTrue".to_string()))?;
                if value.is_truthy() {
                    self.pc = *target - 1;
                }
            }

            OpCode::CallFunction(arg_count) => {
                let func = self
                    .stack
                    .pop()
                    .ok_or_else(|| RuntimeError::General("栈下溢: CallFunction func".to_string()))?;
                
                let mut args = Vec::new();
                for _ in 0..*arg_count {
                    args.push(
                        self.stack
                            .pop()
                            .ok_or_else(|| RuntimeError::General("栈下溢: CallFunction args".to_string()))?,
                    );
                }
                args.reverse();
                
                let result = self.call_function(&func, &args)?;
                self.stack.push(result);
            }

            OpCode::CallMethod(method_name, arg_count) => {
                let mut args = Vec::new();
                for _ in 0..*arg_count {
                    args.push(
                        self.stack
                            .pop()
                            .ok_or_else(|| RuntimeError::General("栈下溢: CallMethod args".to_string()))?,
                    );
                }
                args.reverse();
                
                let obj = self
                    .stack
                    .pop()
                    .ok_or_else(|| RuntimeError::General("栈下溢: CallMethod obj".to_string()))?;
                
                let result = self.call_method(&obj, method_name, &args)?;
                self.stack.push(result);
            }

            OpCode::NewObject(class_name) => {
                let class = self
                    .globals
                    .borrow()
                    .get(class_name)
                    .cloned()
                    .ok_or_else(|| RuntimeError::UndefinedVariable(class_name.clone()))?;
                
                let obj = self.create_object(&class)?;
                self.stack.push(obj);
            }

            OpCode::Return => {
                if let Some(frame) = self.call_stack.pop() {
                    self.pc = frame.return_pc;
                    self.locals = frame.locals;
                } else {
                    self.pc = bytecode.instructions.len();
                }
            }

            OpCode::Print(n) => {
                let mut output = Vec::new();
                for _ in 0..*n {
                    let value = self
                        .stack
                        .pop()
                        .ok_or_else(|| RuntimeError::General("栈下溢: Print".to_string()))?;
                    output.push(value.to_string_value());
                }
                output.reverse();
                println!("{}", output.join(" "));
                self.stack.push(Value::空值);
            }
            
            OpCode::Swap => {
                let a = self.stack.pop().ok_or_else(|| RuntimeError::General("栈下溢: Swap".to_string()))?;
                let b = self.stack.pop().ok_or_else(|| RuntimeError::General("栈下溢: Swap".to_string()))?;
                self.stack.push(a);
                self.stack.push(b);
            }
            
            OpCode::ListAppend => {
                let value = self.stack.pop().ok_or_else(|| RuntimeError::General("栈下溢: ListAppend".to_string()))?;
                let list = self.stack.pop().ok_or_else(|| RuntimeError::General("栈下溢: ListAppend".to_string()))?;
                
                match list {
                    Value::列表(mut v) => {
                        v.push(value);
                        self.stack.push(Value::列表(v));
                    }
                    _ => return Err(RuntimeError::TypeError("ListAppend需要列表类型".to_string())),
                }
            }
            
            OpCode::StringConcat => {
                let right = self.stack.pop().ok_or_else(|| RuntimeError::General("栈下溢: StringConcat".to_string()))?;
                let left = self.stack.pop().ok_or_else(|| RuntimeError::General("栈下溢: StringConcat".to_string()))?;
                
                let left_str = match left {
                    Value::字符串(s) => s,
                    Value::整数(n) => n.to_string(),
                    Value::浮点数(n) => n.to_string(),
                    Value::布尔值(b) => b.to_string(),
                    Value::空值 => "空值".to_string(),
                    _ => return Err(RuntimeError::TypeError("StringConcat需要字符串类型".to_string())),
                };
                
                let right_str = match right {
                    Value::字符串(s) => s,
                    Value::整数(n) => n.to_string(),
                    Value::浮点数(n) => n.to_string(),
                    Value::布尔值(b) => b.to_string(),
                    Value::空值 => "空值".to_string(),
                    _ => return Err(RuntimeError::TypeError("StringConcat需要字符串类型".to_string())),
                };
                
                self.stack.push(Value::字符串(format!("{}{}", left_str, right_str)));
            }
            
            OpCode::SetAdd => {
                let value = self.stack.pop().ok_or_else(|| RuntimeError::General("栈下溢: SetAdd".to_string()))?;
                let set_val = self.stack.pop().ok_or_else(|| RuntimeError::General("栈下溢: SetAdd set".to_string()))?;
                
                match set_val {
                    Value::集合(mut elements) => {
                        let key = value.to_string_value();
                        let mut seen: std::collections::HashSet<String> = elements.iter().map(|v| v.to_string_value()).collect();
                        if !seen.contains(&key) {
                            seen.insert(key);
                            elements.push(value);
                        }
                        self.stack.push(Value::集合(elements));
                    }
                    _ => return Err(RuntimeError::TypeError("SetAdd需要集合类型".to_string())),
                }
            }
        }
        Ok(())
    }

    fn get_property(&self, obj: &Value, name: &str) -> RuntimeResult<Value> {
        match obj {
            Value::对象 { 属性, .. } => 属性
                .get(name)
                .cloned()
                .ok_or_else(|| RuntimeError::General(format!("对象没有属性 {}", name))),
            Value::列表(v) => match name {
                "长度" => Ok(Value::整数(v.len() as i64)),
                _ => Err(RuntimeError::TypeError(format!(
                    "列表没有属性 {}",
                    name
                ))),
            },
            Value::字符串(s) => match name {
                "长度" => Ok(Value::整数(s.len() as i64)),
                _ => Err(RuntimeError::TypeError(format!(
                    "字符串没有属性 {}",
                    name
                ))),
            },
            Value::字典(m) => match name {
                "长度" => Ok(Value::整数(m.len() as i64)),
                _ => m.get(name)
                    .cloned()
                    .ok_or_else(|| RuntimeError::General(format!("字典没有属性 {}", name))),
            },
            _ => Err(RuntimeError::TypeError(format!(
                "类型 {} 不支持属性访问",
                obj.type_name()
            ))),
        }
    }

    fn set_property(&self, obj: Value, name: &str, value: Value) -> RuntimeResult<Value> {
        match obj {
            Value::对象 { 属性, .. } => {
                let mut new_attrs = 属性;
                new_attrs.insert(name.to_string(), value);
                Ok(Value::空值)
            }
            Value::字典(mut m) => {
                m.insert(name.to_string(), value);
                Ok(Value::空值)
            }
            _ => Err(RuntimeError::TypeError(format!(
                "类型 {} 不支持属性赋值",
                obj.type_name()
            ))),
        }
    }

    fn get_index(&self, obj: &Value, index: &Value) -> RuntimeResult<Value> {
        match (obj, index) {
            (Value::列表(v), Value::整数(i)) => {
                let idx = *i as usize;
                if idx < v.len() {
                    Ok(v[idx].clone())
                } else {
                    Err(RuntimeError::IndexOutOfBounds {
                        index: idx,
                        length: v.len(),
                    })
                }
            }
            (Value::字符串(s), Value::整数(i)) => {
                let idx = *i as usize;
                if idx < s.len() {
                    Ok(Value::字符串(s.chars().nth(idx).unwrap().to_string()))
                } else {
                    Err(RuntimeError::IndexOutOfBounds {
                        index: idx,
                        length: s.len(),
                    })
                }
            }
            (Value::字典(m), Value::字符串(k)) => m
                .get(k)
                .cloned()
                .ok_or_else(|| RuntimeError::General(format!("字典没有键 {}", k))),
            _ => Err(RuntimeError::TypeError(format!(
                "类型 {} 不支持索引访问",
                obj.type_name()
            ))),
        }
    }
    
    fn get_slice(&self, obj: &Value, start: Option<&Value>, end: Option<&Value>, step: Option<&Value>) -> RuntimeResult<Value> {
        let step_val = match step {
            Some(Value::整数(s)) => *s,
            Some(_) => return Err(RuntimeError::TypeError("步长必须是整数".to_string())),
            None => 1,
        };
        
        if step_val == 0 {
            return Err(RuntimeError::General("切片步长不能为零".to_string()));
        }
        
        match obj {
            Value::列表(v) => {
                let len = v.len() as i64;
                let start_idx = match start {
                    Some(Value::整数(s)) => {
                        let s = *s;
                        if s < 0 { (len + s).max(0) as usize } else { s.min(len) as usize }
                    }
                    Some(_) => return Err(RuntimeError::TypeError("切片开始必须是整数".to_string())),
                    None => if step_val > 0 { 0 } else { len as usize },
                };
                
                let end_idx = match end {
                    Some(Value::整数(e)) => {
                        let e = *e;
                        if e < 0 { (len + e).max(0) as usize } else { e.min(len) as usize }
                    }
                    Some(_) => return Err(RuntimeError::TypeError("切片结束必须是整数".to_string())),
                    None => if step_val > 0 { len as usize } else { 0 },
                };
                
                let result: Vec<Value> = if step_val > 0 {
                    (start_idx..end_idx).step_by(step_val as usize)
                        .filter_map(|i| if i < v.len() { Some(v[i].clone()) } else { None })
                        .collect()
                } else {
                    let step_abs = (-step_val) as usize;
                    (start_idx..end_idx).rev()
                        .enumerate()
                        .filter_map(|(i, idx)| if i % step_abs == 0 && idx < v.len() { Some(v[idx].clone()) } else { None })
                        .collect()
                };
                
                Ok(Value::列表(result))
            }
            Value::字符串(s) => {
                let chars: Vec<char> = s.chars().collect();
                let len = chars.len() as i64;
                let start_idx = match start {
                    Some(Value::整数(s)) => {
                        let s = *s;
                        if s < 0 { (len + s).max(0) as usize } else { s.min(len) as usize }
                    }
                    Some(_) => return Err(RuntimeError::TypeError("切片开始必须是整数".to_string())),
                    None => if step_val > 0 { 0 } else { len as usize },
                };
                
                let end_idx = match end {
                    Some(Value::整数(e)) => {
                        let e = *e;
                        if e < 0 { (len + e).max(0) as usize } else { e.min(len) as usize }
                    }
                    Some(_) => return Err(RuntimeError::TypeError("切片结束必须是整数".to_string())),
                    None => if step_val > 0 { len as usize } else { 0 },
                };
                
                let result: String = if step_val > 0 {
                    (start_idx..end_idx).step_by(step_val as usize)
                        .filter_map(|i| chars.get(i).copied())
                        .collect()
                } else {
                    let step_abs = (-step_val) as usize;
                    (start_idx..end_idx).rev()
                        .enumerate()
                        .filter_map(|(i, idx)| if i % step_abs == 0 { chars.get(idx).copied() } else { None })
                        .collect()
                };
                
                Ok(Value::字符串(result))
            }
            _ => Err(RuntimeError::TypeError(format!(
                "类型 {} 不支持切片操作",
                obj.type_name()
            ))),
        }
    }

    fn set_index(&self, obj: Value, index: &Value, value: Value) -> RuntimeResult<Value> {
        match (obj, index) {
            (Value::列表(mut v), Value::整数(i)) => {
                let idx = *i as usize;
                if idx < v.len() {
                    v[idx] = value;
                    Ok(Value::空值)
                } else {
                    Err(RuntimeError::IndexOutOfBounds {
                        index: idx,
                        length: v.len(),
                    })
                }
            }
            (Value::字典(mut m), Value::字符串(k)) => {
                m.insert(k.clone(), value);
                Ok(Value::空值)
            }
            (obj, _) => Err(RuntimeError::TypeError(format!(
                "类型 {} 不支持索引赋值",
                obj.type_name()
            ))),
        }
    }

    fn call_function(&mut self, func: &Value, args: &[Value]) -> RuntimeResult<Value> {
        match func {
            Value::内置函数 { 函数, .. } => 函数(args.to_vec()),
            Value::函数 { 名称, 参数, 可变参数名, 闭包: _, .. } => {
                let min_args = if 可变参数名.is_some() { 0 } else { 参数.len() };
                
                if 可变参数名.is_none() && args.len() != 参数.len() {
                    return Err(RuntimeError::ArgumentCountError {
                        function: 名称.clone(),
                        expected: 参数.len(),
                        actual: args.len(),
                    });
                }
                
                if 可变参数名.is_some() && args.len() < 参数.len() {
                    return Err(RuntimeError::ArgumentCountError {
                        function: 名称.clone(),
                        expected: 参数.len(),
                        actual: args.len(),
                    });
                }
                
                if let Some((_, func_bytecode)) = self.functions.get(名称).cloned() {
                    let frame = CallFrame {
                        return_pc: self.pc,
                        locals: self.locals.clone(),
                        _function_locals_count: func_bytecode.local_names.len(),
                    };
                    self.call_stack.push(frame);
                    
                    self.locals = vec![Value::空值; func_bytecode.local_names.len()];
                    for (i, arg) in args.iter().enumerate() {
                        if i < self.locals.len() {
                            self.locals[i] = arg.clone();
                        }
                    }
                    
                    if let Some(vararg) = 可变参数名 {
                        if let Some(idx) = func_bytecode.local_names.iter().position(|n| n == vararg) {
                            let vararg_values: Vec<Value> = if args.len() > 参数.len() {
                                args[参数.len()..].to_vec()
                            } else {
                                vec![]
                            };
                            if idx < self.locals.len() {
                                self.locals[idx] = Value::列表(vararg_values);
                            }
                        }
                    }
                    
                    self.pc = 0;
                    let result = self.run(&func_bytecode)?;
                    
                    if let Some(frame) = self.call_stack.pop() {
                        self.pc = frame.return_pc;
                        self.locals = frame.locals;
                    }
                    
                    Ok(result)
                } else {
                    Ok(Value::空值)
                }
            }
            _ => Err(RuntimeError::TypeError(format!(
                "类型 {} 不可调用",
                func.type_name()
            ))),
        }
    }

    fn call_method(&self, obj: &Value, method_name: &str, _args: &[Value]) -> RuntimeResult<Value> {
        match (obj, method_name) {
            (Value::字符串(s), "长度") => Ok(Value::整数(s.len() as i64)),
            (Value::字符串(s), "大写") => Ok(Value::字符串(s.to_uppercase())),
            (Value::字符串(s), "小写") => Ok(Value::字符串(s.to_lowercase())),
            (Value::字符串(s), "去除空白") => Ok(Value::字符串(s.trim().to_string())),
            (Value::列表(v), "长度") => Ok(Value::整数(v.len() as i64)),
            (Value::列表(v), "首元素") if !v.is_empty() => Ok(v[0].clone()),
            (Value::列表(v), "末元素") if !v.is_empty() => Ok(v[v.len() - 1].clone()),
            (Value::字典(m), "长度") => Ok(Value::整数(m.len() as i64)),
            (Value::字典(m), "键列表") => {
                let keys: Vec<Value> = m.keys().map(|k| Value::字符串(k.clone())).collect();
                Ok(Value::列表(keys))
            }
            (Value::字典(m), "值列表") => Ok(Value::列表(m.values().cloned().collect())),
            _ => Err(RuntimeError::TypeError(format!(
                "类型 {} 没有方法 {}",
                obj.type_name(),
                method_name
            ))),
        }
    }

    fn create_object(&self, class: &Value) -> RuntimeResult<Value> {
        match class {
            Value::类 { 名称, 方法, 属性默认值, 属性权限, 元类, .. } => {
                // 检查是否有元类
                if let Some(meta) = 元类 {
                    // 元类逻辑：直接创建一个默认对象，然后调用元类的初始化方法
                    // 首先创建一个默认对象
                    let mut attributes = HashMap::new();
                    for (method_name, method_value) in 方法 {
                        attributes.insert(method_name.clone(), method_value.clone());
                    }
                    for (prop_name, prop_value) in 属性默认值 {
                        attributes.insert(prop_name.clone(), prop_value.clone());
                    }
                    let obj = Value::对象 {
                        类名: 名称.clone(),
                        属性: attributes,
                        属性权限: 属性权限.clone(),
                    };
                    
                    // 调用元类的 初始化 方法
                    if let Value::类 { 方法: meta_methods, .. } = &**meta {
                        if let Some(init_method) = meta_methods.get("初始化") {
                            // 这里简化处理，实际需要更复杂的逻辑来调用元类方法
                            // 暂时跳过元类方法调用，避免类型错误
                        }
                    }
                    
                    return Ok(obj);
                }
                
                // 默认对象创建逻辑
                let mut attributes = HashMap::new();
                
                for (method_name, method_value) in 方法 {
                    attributes.insert(method_name.clone(), method_value.clone());
                }
                
                for (prop_name, prop_value) in 属性默认值 {
                    attributes.insert(prop_name.clone(), prop_value.clone());
                }
                
                Ok(Value::对象 {
                    类名: 名称.clone(),
                    属性: attributes,
                    属性权限: 属性权限.clone(),
                })
            }
            _ => Err(RuntimeError::TypeError(format!(
                "{} 不是类",
                class.type_name()
            ))),
        }
    }

    fn add(&self, left: &Value, right: &Value) -> RuntimeResult<Value> {
        match (left, right) {
            (Value::整数(a), Value::整数(b)) => Ok(Value::整数(a + b)),
            (Value::浮点数(a), Value::浮点数(b)) => Ok(Value::浮点数(a + b)),
            (Value::整数(a), Value::浮点数(b)) => Ok(Value::浮点数(*a as f64 + b)),
            (Value::浮点数(a), Value::整数(b)) => Ok(Value::浮点数(a + *b as f64)),
            (Value::字符串(a), Value::字符串(b)) => Ok(Value::字符串(format!("{}{}", a, b))),
            (Value::列表(a), Value::列表(b)) => Ok(Value::列表([a.clone(), b.clone()].concat())),
            _ => Err(RuntimeError::TypeError(format!(
                "无法将类型 {} 和 {} 相加",
                left.type_name(),
                right.type_name()
            ))),
        }
    }

    fn subtract(&self, left: &Value, right: &Value) -> RuntimeResult<Value> {
        match (left, right) {
            (Value::整数(a), Value::整数(b)) => Ok(Value::整数(a - b)),
            (Value::浮点数(a), Value::浮点数(b)) => Ok(Value::浮点数(a - b)),
            (Value::整数(a), Value::浮点数(b)) => Ok(Value::浮点数(*a as f64 - b)),
            (Value::浮点数(a), Value::整数(b)) => Ok(Value::浮点数(a - *b as f64)),
            _ => Err(RuntimeError::TypeError(format!(
                "无法将类型 {} 和 {} 相减",
                left.type_name(),
                right.type_name()
            ))),
        }
    }

    fn multiply(&self, left: &Value, right: &Value) -> RuntimeResult<Value> {
        match (left, right) {
            (Value::整数(a), Value::整数(b)) => Ok(Value::整数(a * b)),
            (Value::浮点数(a), Value::浮点数(b)) => Ok(Value::浮点数(a * b)),
            (Value::整数(a), Value::浮点数(b)) => Ok(Value::浮点数(*a as f64 * b)),
            (Value::浮点数(a), Value::整数(b)) => Ok(Value::浮点数(a * *b as f64)),
            (Value::字符串(s), Value::整数(n)) | (Value::整数(n), Value::字符串(s)) => {
                Ok(Value::字符串(s.repeat(*n as usize)))
            }
            _ => Err(RuntimeError::TypeError(format!(
                "无法将类型 {} 和 {} 相乘",
                left.type_name(),
                right.type_name()
            ))),
        }
    }

    fn divide(&self, left: &Value, right: &Value) -> RuntimeResult<Value> {
        match (left, right) {
            (Value::整数(a), Value::整数(b)) => {
                if *b == 0 {
                    return Err(RuntimeError::DivisionByZero);
                }
                Ok(Value::整数(a / b))
            }
            (Value::浮点数(a), Value::浮点数(b)) => {
                if *b == 0.0 {
                    return Err(RuntimeError::DivisionByZero);
                }
                Ok(Value::浮点数(a / b))
            }
            (Value::整数(a), Value::浮点数(b)) => {
                if *b == 0.0 {
                    return Err(RuntimeError::DivisionByZero);
                }
                Ok(Value::浮点数(*a as f64 / b))
            }
            (Value::浮点数(a), Value::整数(b)) => {
                if *b == 0 {
                    return Err(RuntimeError::DivisionByZero);
                }
                Ok(Value::浮点数(a / *b as f64))
            }
            _ => Err(RuntimeError::TypeError(format!(
                "无法将类型 {} 和 {} 相除",
                left.type_name(),
                right.type_name()
            ))),
        }
    }

    fn modulo(&self, left: &Value, right: &Value) -> RuntimeResult<Value> {
        match (left, right) {
            (Value::整数(a), Value::整数(b)) => {
                if *b == 0 {
                    return Err(RuntimeError::DivisionByZero);
                }
                Ok(Value::整数(a % b))
            }
            (Value::浮点数(a), Value::浮点数(b)) => {
                if *b == 0.0 {
                    return Err(RuntimeError::DivisionByZero);
                }
                Ok(Value::浮点数(a % b))
            }
            (Value::整数(a), Value::浮点数(b)) => {
                if *b == 0.0 {
                    return Err(RuntimeError::DivisionByZero);
                }
                Ok(Value::浮点数(*a as f64 % b))
            }
            (Value::浮点数(a), Value::整数(b)) => {
                if *b == 0 {
                    return Err(RuntimeError::DivisionByZero);
                }
                Ok(Value::浮点数(a % *b as f64))
            }
            _ => Err(RuntimeError::TypeError(format!(
                "无法对类型 {} 和 {} 取余",
                left.type_name(),
                right.type_name()
            ))),
        }
    }

    fn compare<F>(&self, left: &Value, right: &Value, cmp: F) -> RuntimeResult<Value>
    where
        F: Fn(f64, f64) -> bool,
    {
        let left_num = match left {
            Value::整数(n) => *n as f64,
            Value::浮点数(n) => *n,
            _ => {
                return Err(RuntimeError::TypeError(format!(
                    "类型 {} 无法比较",
                    left.type_name()
                )));
            }
        };

        let right_num = match right {
            Value::整数(n) => *n as f64,
            Value::浮点数(n) => *n,
            _ => {
                return Err(RuntimeError::TypeError(format!(
                    "类型 {} 无法比较",
                    right.type_name()
                )));
            }
        };

        Ok(Value::布尔值(cmp(left_num, right_num)))
    }
}

impl Default for BytecodeVM {
    fn default() -> Self {
        Self::new()
    }
}

fn builtin_length(args: Vec<Value>) -> RuntimeResult<Value> {
    if args.len() != 1 {
        return Err(RuntimeError::ArgumentCountError {
            function: "长度".to_string(),
            expected: 1,
            actual: args.len(),
        });
    }
    match &args[0] {
        Value::字符串(s) => Ok(Value::整数(s.len() as i64)),
        Value::列表(v) => Ok(Value::整数(v.len() as i64)),
        Value::字典(m) => Ok(Value::整数(m.len() as i64)),
        _ => Err(RuntimeError::TypeError(format!(
            "类型 {} 没有长度",
            args[0].type_name()
        ))),
    }
}
