use crate::bytecode::Bytecode;
use crate::bytecode::OpCode;
use crate::parser::ast::*;
use crate::parser::ast::FormatPart;

pub struct BytecodeCompiler {
    bytecode: Bytecode,
    loop_breaks: Vec<usize>,
    loop_continues: Vec<usize>,
}

impl BytecodeCompiler {
    pub fn new() -> Self {
        BytecodeCompiler {
            bytecode: Bytecode::new(),
            loop_breaks: Vec::new(),
            loop_continues: Vec::new(),
        }
    }
    
    pub fn compile_program(&mut self, program: &Program) -> Result<Bytecode, String> {
        for stmt in &program.statements {
            self.compile_statement(stmt)?;
        }
        Ok(self.bytecode.clone())
    }
    
    fn compile_statement(&mut self, stmt: &Statement) -> Result<(), String> {
        match stmt {
            Statement::变量定义 { 名称, 初始值, .. } => {
                let local_idx = self.bytecode.add_local(名称.clone());
                if let Some(expr) = 初始值 {
                    self.compile_expression(expr)?;
                    let _ = self.bytecode.add_instruction(OpCode::StoreLocal(local_idx), None);
                } else {
                    let _ = self.bytecode.add_instruction(OpCode::PushNull, None);
                    let _ = self.bytecode.add_instruction(OpCode::StoreLocal(local_idx), None);
                }
            }
            
            Statement::解构赋值 { 变量列表, 值 } => {
                self.compile_expression(值)?;
                let temp_local = self.bytecode.add_local("__destructure_temp".to_string());
                let _ = self.bytecode.add_instruction(OpCode::StoreLocal(temp_local), None);
                
                for (i, var_name) in 变量列表.iter().enumerate() {
                    let _ = self.bytecode.add_instruction(OpCode::LoadLocal(temp_local), None);
                    let _ = self.bytecode.add_instruction(OpCode::PushInt(i as i64), None);
                    let _ = self.bytecode.add_instruction(OpCode::LoadIndex, None);
                    
                    let var_local = self.bytecode.add_local(var_name.clone());
                    let _ = self.bytecode.add_instruction(OpCode::StoreLocal(var_local), None);
                }
            }
            
            Statement::函数定义 { 名称, 参数, 可变参数名, 函数体, .. } => {
                self.compile_function(名称, 参数, 可变参数名, 函数体)?;
            }
            
            Statement::表达式语句 { 表达式 } => {
                self.compile_expression(表达式)?;
            }
            
            Statement::赋值语句 { 目标, 值 } => {
                self.compile_expression(值)?;
                self.compile_assignment_target(目标)?;
            }
            
            Statement::如果语句 { 条件, 如果体, 否则如果分支, 否则体 } => {
                let mut jump_offsets = Vec::new();
                
                self.compile_expression(条件)?;
                let else_jump = self.bytecode.add_instruction(OpCode::JumpIfFalse(0), None);
                self.compile_block(如果体)?;
                let end_jump = self.bytecode.add_instruction(OpCode::Jump(0), None);
                jump_offsets.push(end_jump);
                self.bytecode.patch_jump(else_jump, self.bytecode.len());
                
                for (elif_cond, elif_body) in 否则如果分支 {
                    self.compile_expression(elif_cond)?;
                    let elif_jump = self.bytecode.add_instruction(OpCode::JumpIfFalse(0), None);
                    self.compile_block(elif_body)?;
                    let end_jump = self.bytecode.add_instruction(OpCode::Jump(0), None);
                    jump_offsets.push(end_jump);
                    self.bytecode.patch_jump(elif_jump, self.bytecode.len());
                }
                
                if let Some(else_body) = 否则体 {
                    self.compile_block(else_body)?;
                }
                
                let end_offset = self.bytecode.len();
                for offset in jump_offsets {
                    self.bytecode.patch_jump(offset, end_offset);
                }
            }
            
            Statement::循环语句 { 条件, 循环体 } => {
                let loop_start = self.bytecode.len();
                self.loop_breaks.push(0);
                self.loop_continues.push(loop_start);
                
                self.compile_expression(条件)?;
                let exit_jump = self.bytecode.add_instruction(OpCode::JumpIfFalse(0), None);
                self.compile_block(循环体)?;
                let _ = self.bytecode.add_instruction(OpCode::Jump(loop_start), None);
                
                let loop_end = self.bytecode.len();
                self.bytecode.patch_jump(exit_jump, loop_end);
                if let Some(break_jump) = self.loop_breaks.pop() {
                    if break_jump != 0 {
                        self.bytecode.patch_jump(break_jump, loop_end);
                    }
                }
                self.loop_continues.pop();
            }
            
            Statement::对于循环 { 变量, 可迭代对象, 循环体 } => {
                self.compile_for_loop(变量, 可迭代对象, 循环体)?;
            }
            
            Statement::返回语句 { 值 } => {
                if let Some(expr) = 值 {
                    self.compile_expression(expr)?;
                } else {
                    let _ = self.bytecode.add_instruction(OpCode::PushNull, None);
                }
                let _ = self.bytecode.add_instruction(OpCode::Return, None);
            }
            
            Statement::打印语句 { 参数 } => {
                for arg in 参数 {
                    self.compile_expression(arg)?;
                }
                let _ = self.bytecode.add_instruction(OpCode::Print(参数.len()), None);
            }
            
            Statement::导入语句 { 模块名, 别名 } => {
                // 编译导入语句
                // 这里简化处理，实际需要更复杂的逻辑来处理模块加载和导入
                let import_name = 别名.clone().unwrap_or_else(|| 模块名.clone());
                
                // 为导入的模块创建一个全局变量
                let _ = self.bytecode.add_instruction(OpCode::PushNull, None);
                let _ = self.bytecode.add_instruction(OpCode::StoreGlobal(import_name), None);
            }
            
            Statement::类定义 { 名称, 父类: _, 成员: _ } => {
                let _class_local = self.bytecode.add_local(名称.clone());
                
                // 编译类定义
                // 这里简化处理，实际需要更复杂的逻辑来处理继承、方法重载等
                let _ = self.bytecode.add_instruction(OpCode::PushNull, None);
                let _ = self.bytecode.add_instruction(OpCode::StoreGlobal(名称.clone()), None);
            }
            
            Statement::尝试语句 { 尝试体, 捕获分支 } => {
                // 编译尝试语句
                // 这里简化处理，实际需要更复杂的逻辑来处理异常捕获
                self.compile_block(尝试体)?;
                
                // 编译捕获分支
                for (_异常类型, 捕获体) in 捕获分支 {
                    self.compile_block(捕获体)?;
                }
            }
            
            Statement::抛出语句 { 错误 } => {
                self.compile_expression(&错误)?;
            }
            
            Statement::With语句 { 表达式, 变量名, 语句体 } => {
                self.compile_expression(表达式)?;
                let temp_local = self.bytecode.add_local("__with_temp".to_string());
                let _ = self.bytecode.add_instruction(OpCode::StoreLocal(temp_local), None);
                
                if let Some(var) = 变量名 {
                    let var_local = self.bytecode.add_local(var.clone());
                    let _ = self.bytecode.add_instruction(OpCode::LoadLocal(temp_local), None);
                    let _ = self.bytecode.add_instruction(OpCode::StoreLocal(var_local), None);
                }
                
                self.compile_block(语句体)?;
            }
            
            Statement::Yield语句 { 值 } => {
                if let Some(v) = 值 {
                    self.compile_expression(v)?;
                } else {
                    let _ = self.bytecode.add_instruction(OpCode::PushNull, None);
                }
            }
            
            Statement::枚举定义 { 名称, 成员 } => {
                let _ = self.bytecode.add_instruction(OpCode::PushNull, None);
                let enum_local = self.bytecode.add_local(名称.clone());
                let _ = self.bytecode.add_instruction(OpCode::StoreLocal(enum_local), None);
                for (_member_name, _member_value) in 成员 {
                    // 简化处理
                }
            }
        }
        Ok(())
    }
    
    fn compile_function(&mut self, name: &str, params: &[FunctionParameter], vararg_name: &Option<String>, body: &[Statement]) -> Result<(), String> {
        let _func_start = self.bytecode.len();
        
        for param in params {
            let local_idx = self.bytecode.add_local(param.名称.clone());
            if let Some(ref 默认值) = param.默认值 {
                self.compile_expression(默认值)?;
                let _ = self.bytecode.add_instruction(OpCode::StoreLocal(local_idx), None);
            }
        }
        
        if let Some(vararg) = vararg_name {
            let _ = self.bytecode.add_local(vararg.clone());
        }
        
        self.compile_block(body)?;
        
        let _ = self.bytecode.add_instruction(OpCode::PushNull, None);
        let _ = self.bytecode.add_instruction(OpCode::Return, None);
        
        let _func_end = self.bytecode.len();
        
        let local_idx = self.bytecode.add_local(name.to_string());
        let _ = self.bytecode.add_instruction(OpCode::PushNull, None);
        let _ = self.bytecode.add_instruction(OpCode::StoreLocal(local_idx), None);
        
        Ok(())
    }
    
    fn compile_for_loop(&mut self, var_name: &str, iterable: &Expression, body: &[Statement]) -> Result<(), String> {
        let iter_local = self.bytecode.add_local(format!("__iter_{}", var_name));
        let idx_local = self.bytecode.add_local(format!("__idx_{}", var_name));
        let var_local = self.bytecode.add_local(var_name.to_string());
        
        self.compile_expression(iterable)?;
        let _ = self.bytecode.add_instruction(OpCode::StoreLocal(iter_local), None);
        
        let _ = self.bytecode.add_instruction(OpCode::PushInt(0), None);
        let _ = self.bytecode.add_instruction(OpCode::StoreLocal(idx_local), None);
        
        let loop_start = self.bytecode.len();
        self.loop_breaks.push(0);
        self.loop_continues.push(loop_start);
        
        let _ = self.bytecode.add_instruction(OpCode::LoadLocal(iter_local), None);
        let _ = self.bytecode.add_instruction(OpCode::LoadProperty("长度".to_string()), None);
        
        let _ = self.bytecode.add_instruction(OpCode::LoadLocal(idx_local), None);
        
        let _ = self.bytecode.add_instruction(OpCode::Less, None);
        
        let exit_jump = self.bytecode.add_instruction(OpCode::JumpIfFalse(0), None);
        
        let _ = self.bytecode.add_instruction(OpCode::LoadLocal(iter_local), None);
        let _ = self.bytecode.add_instruction(OpCode::LoadLocal(idx_local), None);
        let _ = self.bytecode.add_instruction(OpCode::LoadIndex, None);
        let _ = self.bytecode.add_instruction(OpCode::StoreLocal(var_local), None);
        
        self.compile_block(body)?;
        
        let _ = self.bytecode.add_instruction(OpCode::LoadLocal(idx_local), None);
        let _ = self.bytecode.add_instruction(OpCode::PushInt(1), None);
        let _ = self.bytecode.add_instruction(OpCode::Add, None);
        let _ = self.bytecode.add_instruction(OpCode::StoreLocal(idx_local), None);
        
        let _ = self.bytecode.add_instruction(OpCode::Jump(loop_start), None);
        
        let loop_end = self.bytecode.len();
        self.bytecode.patch_jump(exit_jump, loop_end);
        
        if let Some(break_jump) = self.loop_breaks.pop() {
            if break_jump != 0 {
                self.bytecode.patch_jump(break_jump, loop_end);
            }
        }
        self.loop_continues.pop();
        
        Ok(())
    }
    
    fn compile_block(&mut self, stmts: &[Statement]) -> Result<(), String> {
        for stmt in stmts {
            self.compile_statement(stmt)?;
        }
        Ok(())
    }
    
    fn compile_expression(&mut self, expr: &Expression) -> Result<(), String> {
        match expr {
            Expression::整数 { 值 } => {
                let _ = self.bytecode.add_instruction(OpCode::PushInt(*值), None);
            }
            
            Expression::浮点数 { 值 } => {
                let _ = self.bytecode.add_instruction(OpCode::PushFloat(*值), None);
            }
            
            Expression::字符串 { 值 } => {
                let _ = self.bytecode.add_instruction(OpCode::PushString(值.clone()), None);
            }
            
            Expression::格式化字符串 { 部分 } => {
                let mut concat_parts = Vec::new();
                let mut current_text = String::new();
                
                for part in 部分 {
                    match part {
                        FormatPart::文本(text) => {
                            current_text.push_str(text);
                        }
                        FormatPart::表达式(expr) => {
                            if !current_text.is_empty() {
                                concat_parts.push((None, current_text.clone()));
                                current_text.clear();
                            }
                            concat_parts.push((Some(expr.clone()), String::new()));
                        }
                    }
                }
                
                if !current_text.is_empty() {
                    concat_parts.push((None, current_text));
                }
                
                if concat_parts.is_empty() {
                    let _ = self.bytecode.add_instruction(OpCode::PushString(String::new()), None);
                } else if concat_parts.len() == 1 && concat_parts[0].0.is_none() {
                    let _ = self.bytecode.add_instruction(OpCode::PushString(concat_parts[0].1.clone()), None);
                } else {
                    let result_local = self.bytecode.add_local("__fstring_result".to_string());
                    let _ = self.bytecode.add_instruction(OpCode::PushString(String::new()), None);
                    let _ = self.bytecode.add_instruction(OpCode::StoreLocal(result_local), None);
                    
                    for (expr_opt, text) in concat_parts {
                        if let Some(expr) = expr_opt {
                            let _ = self.bytecode.add_instruction(OpCode::LoadLocal(result_local), None);
                            self.compile_expression(&expr)?;
                            let _ = self.bytecode.add_instruction(OpCode::StringConcat, None);
                            let _ = self.bytecode.add_instruction(OpCode::StoreLocal(result_local), None);
                        }
                        if !text.is_empty() {
                            let _ = self.bytecode.add_instruction(OpCode::LoadLocal(result_local), None);
                            let _ = self.bytecode.add_instruction(OpCode::PushString(text), None);
                            let _ = self.bytecode.add_instruction(OpCode::StringConcat, None);
                            let _ = self.bytecode.add_instruction(OpCode::StoreLocal(result_local), None);
                        }
                    }
                    
                    let _ = self.bytecode.add_instruction(OpCode::LoadLocal(result_local), None);
                }
            }
            
            Expression::布尔值 { 值 } => {
                let _ = self.bytecode.add_instruction(OpCode::PushBool(*值), None);
            }
            
            Expression::空值 => {
                let _ = self.bytecode.add_instruction(OpCode::PushNull, None);
            }
            
            Expression::标识符 { 名称 } => {
                if let Some(idx) = self.bytecode.get_local_index(名称) {
                    let _ = self.bytecode.add_instruction(OpCode::LoadLocal(idx), None);
                } else {
                    let _ = self.bytecode.add_instruction(OpCode::LoadGlobal(名称.clone()), None);
                }
            }
            
            Expression::二元运算 { 左, 运算符, 右 } => {
                self.compile_expression(左)?;
                self.compile_expression(右)?;
                match 运算符 {
                    BinaryOperator::加 => { let _ = self.bytecode.add_instruction(OpCode::Add, None); },
                    BinaryOperator::减 => { let _ = self.bytecode.add_instruction(OpCode::Subtract, None); },
                    BinaryOperator::乘 => { let _ = self.bytecode.add_instruction(OpCode::Multiply, None); },
                    BinaryOperator::除 => { let _ = self.bytecode.add_instruction(OpCode::Divide, None); },
                    BinaryOperator::取余 => { let _ = self.bytecode.add_instruction(OpCode::Modulo, None); },
                    BinaryOperator::等于 => { let _ = self.bytecode.add_instruction(OpCode::Equal, None); },
                    BinaryOperator::不等于 => { let _ = self.bytecode.add_instruction(OpCode::NotEqual, None); },
                    BinaryOperator::大于 => { let _ = self.bytecode.add_instruction(OpCode::Greater, None); },
                    BinaryOperator::小于 => { let _ = self.bytecode.add_instruction(OpCode::Less, None); },
                    BinaryOperator::大于等于 => { let _ = self.bytecode.add_instruction(OpCode::GreaterEqual, None); },
                    BinaryOperator::小于等于 => { let _ = self.bytecode.add_instruction(OpCode::LessEqual, None); },
                    BinaryOperator::且 => { let _ = self.bytecode.add_instruction(OpCode::And, None); },
                    BinaryOperator::或 => { let _ = self.bytecode.add_instruction(OpCode::Or, None); },
                }
            }
            
            Expression::一元运算 { 运算符, 操作数 } => {
                self.compile_expression(操作数)?;
                match 运算符 {
                    UnaryOperator::非 => { let _ = self.bytecode.add_instruction(OpCode::Not, None); },
                    UnaryOperator::负 => { let _ = self.bytecode.add_instruction(OpCode::Negate, None); },
                    UnaryOperator::正 => { let _ = self.bytecode.add_instruction(OpCode::Positive, None); },
                }
            }
            
            Expression::函数调用 { 函数名, 参数 } => {
                for arg in 参数 {
                    self.compile_expression(arg)?;
                }
                let _ = self.bytecode.add_instruction(OpCode::LoadGlobal(函数名.clone()), None);
                let _ = self.bytecode.add_instruction(OpCode::CallFunction(参数.len()), None);
            }
            
            Expression::匿名函数 { 参数, 可变参数名, 函数体 } => {
                let func_name = format!("__匿名函数_{}", self.bytecode.len());
                self.compile_function(&func_name, 参数, 可变参数名, 函数体)?;
                let _ = self.bytecode.add_instruction(OpCode::LoadGlobal(func_name), None);
            }
            
            Expression::方法调用 { 对象, 方法名, 参数 } => {
                self.compile_expression(对象)?;
                for arg in 参数 {
                    self.compile_expression(arg)?;
                }
                let _ = self.bytecode.add_instruction(OpCode::CallMethod(方法名.clone(), 参数.len()), None);
            }
            
            Expression::属性访问 { 对象, 属性名 } => {
                self.compile_expression(对象)?;
                let _ = self.bytecode.add_instruction(OpCode::LoadProperty(属性名.clone()), None);
            }
            
            Expression::索引访问 { 对象, 索引 } => {
                self.compile_expression(对象)?;
                self.compile_expression(索引)?;
                let _ = self.bytecode.add_instruction(OpCode::LoadIndex, None);
            }
            
            Expression::切片访问 { 对象, 开始, 结束, 步长 } => {
                self.compile_expression(对象)?;
                
                match 开始 {
                    Some(s) => self.compile_expression(s)?,
                    None => { let _ = self.bytecode.add_instruction(OpCode::PushNull, None); },
                }
                
                match 结束 {
                    Some(e) => self.compile_expression(e)?,
                    None => { let _ = self.bytecode.add_instruction(OpCode::PushNull, None); },
                }
                
                match 步长 {
                    Some(st) => self.compile_expression(st)?,
                    None => { let _ = self.bytecode.add_instruction(OpCode::PushNull, None); },
                }
                
                let _ = self.bytecode.add_instruction(OpCode::LoadSlice, None);
            }
            
            Expression::列表 { 元素 } => {
                for elem in 元素 {
                    self.compile_expression(elem)?;
                }
                let _ = self.bytecode.add_instruction(OpCode::PushList(元素.len()), None);
            }
            
            Expression::列表推导式 { 表达式, 变量, 可迭代对象, 条件 } => {
                let result_local = self.bytecode.add_local("__listcomp_result".to_string());
                let iter_local = self.bytecode.add_local(format!("__listcomp_iter_{}", 变量));
                let idx_local = self.bytecode.add_local(format!("__listcomp_idx_{}", 变量));
                let var_local = self.bytecode.add_local(变量.clone());
                
                let _ = self.bytecode.add_instruction(OpCode::PushList(0), None);
                let _ = self.bytecode.add_instruction(OpCode::StoreLocal(result_local), None);
                
                self.compile_expression(可迭代对象)?;
                let _ = self.bytecode.add_instruction(OpCode::StoreLocal(iter_local), None);
                
                let _ = self.bytecode.add_instruction(OpCode::PushInt(0), None);
                let _ = self.bytecode.add_instruction(OpCode::StoreLocal(idx_local), None);
                
                let loop_start = self.bytecode.len();
                
                let _ = self.bytecode.add_instruction(OpCode::LoadLocal(iter_local), None);
                let _ = self.bytecode.add_instruction(OpCode::LoadProperty("长度".to_string()), None);
                let _ = self.bytecode.add_instruction(OpCode::LoadLocal(idx_local), None);
                let _ = self.bytecode.add_instruction(OpCode::Less, None);
                
                let exit_jump = self.bytecode.add_instruction(OpCode::JumpIfFalse(0), None);
                
                let _ = self.bytecode.add_instruction(OpCode::LoadLocal(iter_local), None);
                let _ = self.bytecode.add_instruction(OpCode::LoadLocal(idx_local), None);
                let _ = self.bytecode.add_instruction(OpCode::LoadIndex, None);
                let _ = self.bytecode.add_instruction(OpCode::StoreLocal(var_local), None);
                
                if let Some(ref cond) = 条件 {
                    self.compile_expression(cond)?;
                    let skip_jump = self.bytecode.add_instruction(OpCode::JumpIfFalse(0), None);
                    
                    self.compile_expression(表达式)?;
                    let _ = self.bytecode.add_instruction(OpCode::LoadLocal(result_local), None);
                    let _ = self.bytecode.add_instruction(OpCode::Swap, None);
                    let _ = self.bytecode.add_instruction(OpCode::ListAppend, None);
                    
                    let skip_end = self.bytecode.add_instruction(OpCode::Jump(0), None);
                    self.bytecode.patch_jump(skip_jump, self.bytecode.len());
                    
                    let _ = self.bytecode.add_instruction(OpCode::PushInt(1), None);
                    let _ = self.bytecode.add_instruction(OpCode::LoadLocal(idx_local), None);
                    let _ = self.bytecode.add_instruction(OpCode::Add, None);
                    let _ = self.bytecode.add_instruction(OpCode::StoreLocal(idx_local), None);
                    let _ = self.bytecode.add_instruction(OpCode::Jump(loop_start), None);
                    
                    self.bytecode.patch_jump(skip_end, self.bytecode.len());
                } else {
                    self.compile_expression(表达式)?;
                    let _ = self.bytecode.add_instruction(OpCode::LoadLocal(result_local), None);
                    let _ = self.bytecode.add_instruction(OpCode::Swap, None);
                    let _ = self.bytecode.add_instruction(OpCode::ListAppend, None);
                }
                
                let _ = self.bytecode.add_instruction(OpCode::PushInt(1), None);
                let _ = self.bytecode.add_instruction(OpCode::LoadLocal(idx_local), None);
                let _ = self.bytecode.add_instruction(OpCode::Add, None);
                let _ = self.bytecode.add_instruction(OpCode::StoreLocal(idx_local), None);
                let _ = self.bytecode.add_instruction(OpCode::Jump(loop_start), None);
                
                self.bytecode.patch_jump(exit_jump, self.bytecode.len());
                
                let _ = self.bytecode.add_instruction(OpCode::LoadLocal(result_local), None);
            }
            
            Expression::字典 { 键值对 } => {
                for (key, value) in 键值对 {
                    self.compile_expression(key)?;
                    self.compile_expression(value)?;
                }
                let _ = self.bytecode.add_instruction(OpCode::PushDict(键值对.len()), None);
            }
            
            Expression::字典推导式 { 键表达式, 值表达式, 变量, 可迭代对象, 条件 } => {
                self.compile_expression(可迭代对象)?;
                let result_local = self.bytecode.add_local("__dict_comp_result".to_string());
                let _ = self.bytecode.add_instruction(OpCode::PushDict(0), None);
                let _ = self.bytecode.add_instruction(OpCode::StoreLocal(result_local), None);
                
                let iter_local = self.bytecode.add_local("__iter".to_string());
                let _ = self.bytecode.add_instruction(OpCode::StoreLocal(iter_local), None);
                
                let loop_start = self.bytecode.len();
                let _ = self.bytecode.add_instruction(OpCode::LoadLocal(iter_local), None);
                let _ = self.bytecode.add_instruction(OpCode::CallMethod("迭代下一个".to_string(), 0), None);
                
                let item_local = self.bytecode.add_local(变量.clone());
                let _ = self.bytecode.add_instruction(OpCode::StoreLocal(item_local), None);
                
                if let Some(cond) = 条件 {
                    self.compile_expression(cond)?;
                    let skip_jump = self.bytecode.add_instruction(OpCode::JumpIfFalse(0), None);
                    
                    self.compile_expression(键表达式)?;
                    self.compile_expression(值表达式)?;
                    let _ = self.bytecode.add_instruction(OpCode::LoadLocal(result_local), None);
                    let _ = self.bytecode.add_instruction(OpCode::Swap, None);
                    let _ = self.bytecode.add_instruction(OpCode::Swap, None);
                    let _ = self.bytecode.add_instruction(OpCode::StoreIndex, None);
                    
                    let loop_end = self.bytecode.len();
                    self.bytecode.patch_jump(skip_jump, loop_end);
                } else {
                    self.compile_expression(键表达式)?;
                    self.compile_expression(值表达式)?;
                    let _ = self.bytecode.add_instruction(OpCode::LoadLocal(result_local), None);
                    let _ = self.bytecode.add_instruction(OpCode::Swap, None);
                    let _ = self.bytecode.add_instruction(OpCode::Swap, None);
                    let _ = self.bytecode.add_instruction(OpCode::StoreIndex, None);
                }
                
                let _ = self.bytecode.add_instruction(OpCode::Jump(loop_start), None);
                let _ = self.bytecode.add_instruction(OpCode::LoadLocal(result_local), None);
            }
            
            Expression::集合 { 元素 } => {
                for elem in 元素 {
                    self.compile_expression(elem)?;
                }
                let _ = self.bytecode.add_instruction(OpCode::PushSet(元素.len()), None);
            }
            
            Expression::集合推导式 { 表达式, 变量, 可迭代对象, 条件 } => {
                self.compile_expression(可迭代对象)?;
                let result_local = self.bytecode.add_local("__set_comp_result".to_string());
                let _ = self.bytecode.add_instruction(OpCode::PushSet(0), None);
                let _ = self.bytecode.add_instruction(OpCode::StoreLocal(result_local), None);
                
                let iter_local = self.bytecode.add_local("__iter".to_string());
                let _ = self.bytecode.add_instruction(OpCode::StoreLocal(iter_local), None);
                
                let loop_start = self.bytecode.len();
                let _ = self.bytecode.add_instruction(OpCode::LoadLocal(iter_local), None);
                let _ = self.bytecode.add_instruction(OpCode::CallMethod("迭代下一个".to_string(), 0), None);
                
                let item_local = self.bytecode.add_local(变量.clone());
                let _ = self.bytecode.add_instruction(OpCode::StoreLocal(item_local), None);
                
                if let Some(cond) = 条件 {
                    self.compile_expression(cond)?;
                    let skip_jump = self.bytecode.add_instruction(OpCode::JumpIfFalse(0), None);
                    
                    self.compile_expression(表达式)?;
                    let _ = self.bytecode.add_instruction(OpCode::LoadLocal(result_local), None);
                    let _ = self.bytecode.add_instruction(OpCode::SetAdd, None);
                    
                    let loop_end = self.bytecode.len();
                    self.bytecode.patch_jump(skip_jump, loop_end);
                } else {
                    self.compile_expression(表达式)?;
                    let _ = self.bytecode.add_instruction(OpCode::LoadLocal(result_local), None);
                    let _ = self.bytecode.add_instruction(OpCode::SetAdd, None);
                }
                
                let _ = self.bytecode.add_instruction(OpCode::Jump(loop_start), None);
                let _ = self.bytecode.add_instruction(OpCode::LoadLocal(result_local), None);
            }
            
            Expression::三元表达式 { 条件, 真值, 假值 } => {
                self.compile_expression(条件)?;
                let else_jump = self.bytecode.add_instruction(OpCode::JumpIfFalse(0), None);
                
                self.compile_expression(真值)?;
                let end_jump = self.bytecode.add_instruction(OpCode::Jump(0), None);
                
                let else_pos = self.bytecode.len();
                self.bytecode.patch_jump(else_jump, else_pos);
                
                self.compile_expression(假值)?;
                
                let end_pos = self.bytecode.len();
                self.bytecode.patch_jump(end_jump, end_pos);
            }
            
            Expression::新建对象 { 类名, 参数 } => {
                for arg in 参数 {
                    self.compile_expression(arg)?;
                }
                let _ = self.bytecode.add_instruction(OpCode::NewObject(类名.clone()), None);
            }
        }
        Ok(())
    }
    
    fn compile_assignment_target(&mut self, target: &Expression) -> Result<(), String> {
        match target {
            Expression::标识符 { 名称 } => {
                if let Some(idx) = self.bytecode.get_local_index(名称) {
                    let _ = self.bytecode.add_instruction(OpCode::StoreLocal(idx), None);
                } else {
                    let _ = self.bytecode.add_instruction(OpCode::StoreGlobal(名称.clone()), None);
                }
            }
            Expression::属性访问 { 对象, 属性名 } => {
                self.compile_expression(对象)?;
                let _ = self.bytecode.add_instruction(OpCode::StoreProperty(属性名.clone()), None);
            }
            Expression::索引访问 { 对象, 索引 } => {
                self.compile_expression(对象)?;
                self.compile_expression(索引)?;
                let _ = self.bytecode.add_instruction(OpCode::StoreIndex, None);
            }
            _ => return Err("无效的赋值目标".to_string()),
        }
        Ok(())
    }
}

impl Default for BytecodeCompiler {
    fn default() -> Self {
        Self::new()
    }
}
