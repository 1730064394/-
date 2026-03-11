use crate::bytecode::{Bytecode, OpCode};
use std::collections::HashSet;

pub struct BytecodeVerifier {
    errors: Vec<String>,
    warnings: Vec<String>,
}

impl BytecodeVerifier {
    pub fn new() -> Self {
        BytecodeVerifier {
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }

    pub fn verify(&mut self, bytecode: &Bytecode) -> Result<(), Vec<String>> {
        self.errors.clear();
        self.warnings.clear();

        self.verify_jump_targets(bytecode);
        self.verify_stack_balance(bytecode);
        self.verify_local_access(bytecode);
        self.verify_instruction_sanity(bytecode);

        if !self.errors.is_empty() {
            return Err(self.errors.clone());
        }

        Ok(())
    }

    fn verify_jump_targets(&mut self, bytecode: &Bytecode) {
        let mut jump_targets = HashSet::new();
        let instruction_count = bytecode.instructions.len();

        for (i, op) in bytecode.instructions.iter().enumerate() {
            match op {
                OpCode::Jump(target)
                | OpCode::JumpIfFalse(target)
                | OpCode::JumpIfTrue(target) => {
                    if *target >= instruction_count {
                        self.errors.push(format!(
                            "指令 {}: 跳转目标 {} 超出范围 (最大: {})",
                            i,
                            target,
                            instruction_count - 1
                        ));
                    }
                    jump_targets.insert(*target);
                }
                _ => {}
            }
        }

        for target in jump_targets {
            if target >= instruction_count {
                self.errors.push(format!(
                    "跳转目标 {} 超出范围 (最大: {})",
                    target,
                    instruction_count - 1
                ));
            }
        }
    }

    fn verify_stack_balance(&mut self, bytecode: &Bytecode) {
        let mut stack_size = 0;
        let mut max_stack_size = 0;

        for (i, op) in bytecode.instructions.iter().enumerate() {
            match op {
                OpCode::PushInt(_)
                | OpCode::PushFloat(_)
                | OpCode::PushString(_)
                | OpCode::PushBool(_)
                | OpCode::PushNull => {
                    stack_size += 1;
                }
                OpCode::PushList(n) => {
                    stack_size -= n;
                    stack_size += 1;
                }
                OpCode::PushDict(n) => {
                    stack_size -= 2 * n;
                    stack_size += 1;
                }
                OpCode::PushSet(n) => {
                    stack_size -= n;
                    stack_size += 1;
                }
                OpCode::LoadLocal(_) | OpCode::LoadGlobal(_) => {
                    stack_size += 1;
                }
                OpCode::StoreLocal(_) | OpCode::StoreGlobal(_) => {
                    stack_size -= 1;
                }
                OpCode::LoadProperty(_) => {
                    stack_size -= 1;
                    stack_size += 1;
                }
                OpCode::StoreProperty(_) => {
                    stack_size -= 2;
                    stack_size += 1;
                }
                OpCode::LoadIndex => {
                    stack_size -= 2;
                    stack_size += 1;
                }
                OpCode::StoreIndex => {
                    stack_size -= 3;
                    stack_size += 1;
                }
                OpCode::LoadSlice => {
                    stack_size -= 4;
                    stack_size += 1;
                }
                OpCode::CallFunction(arg_count) => {
                    stack_size -= arg_count + 1;
                    stack_size += 1;
                }
                OpCode::CallMethod(_, arg_count) => {
                    stack_size -= arg_count + 1;
                    stack_size += 1;
                }
                OpCode::NewObject(_) => {
                    stack_size += 1;
                }
                OpCode::Add
                | OpCode::Subtract
                | OpCode::Multiply
                | OpCode::Divide
                | OpCode::Modulo
                | OpCode::Equal
                | OpCode::NotEqual
                | OpCode::Greater
                | OpCode::Less
                | OpCode::GreaterEqual
                | OpCode::LessEqual
                | OpCode::And
                | OpCode::Or => {
                    stack_size -= 2;
                    stack_size += 1;
                }
                OpCode::Not | OpCode::Negate | OpCode::Positive => {
                    stack_size -= 1;
                    stack_size += 1;
                }
                OpCode::Jump(_) => {}
                OpCode::JumpIfFalse(_) | OpCode::JumpIfTrue(_) => {
                    stack_size -= 1;
                }
                OpCode::Return => {
                    if stack_size > 0 {
                        stack_size -= 1;
                    }
                }
                OpCode::Print(n) => {
                    stack_size -= n;
                    stack_size += 1;
                }
                OpCode::Nop => {}
                OpCode::Swap => {}
                OpCode::ListAppend => {
                    stack_size -= 2;
                    stack_size += 1;
                }
                OpCode::StringConcat => {
                    stack_size -= 2;
                    stack_size += 1;
                }
                OpCode::SetAdd => {
                    stack_size -= 2;
                    stack_size += 1;
                }
            }

            // stack_size 是无符号整数，不需要检查是否小于 0
            // 但我们仍然需要检查栈操作是否导致了逻辑上的下溢
            if stack_size > 10000 { // 简单的栈大小限制检查
                self.warnings.push(format!(
                    "指令 {}: 栈大小 {} 可能过大",
                    i, stack_size
                ));
            }

            max_stack_size = max_stack_size.max(stack_size);
        }
    }

    fn verify_local_access(&mut self, bytecode: &Bytecode) {
        let local_count = bytecode.local_names.len();

        for (i, op) in bytecode.instructions.iter().enumerate() {
            match op {
                OpCode::LoadLocal(idx) | OpCode::StoreLocal(idx) => {
                    if *idx >= local_count {
                        self.errors.push(format!(
                            "指令 {}: 局部变量索引 {} 超出范围 (最大: {})",
                            i,
                            idx,
                            local_count - 1
                        ));
                    }
                }
                _ => {}
            }
        }
    }

    fn verify_instruction_sanity(&mut self, bytecode: &Bytecode) {
        for (i, op) in bytecode.instructions.iter().enumerate() {
            match op {
                OpCode::PushList(n) | OpCode::PushDict(n) | OpCode::PushSet(n) => {
                    if *n > 10000 {
                        self.warnings.push(format!(
                            "指令 {}: 列表/字典/集合元素数量 {} 可能过大",
                            i, n
                        ));
                    }
                }
                OpCode::Print(n) => {
                    if *n > 100 {
                        self.warnings.push(format!(
                            "指令 {}: 打印参数数量 {} 可能过大",
                            i, n
                        ));
                    }
                }
                _ => {}
            }
        }
    }

    pub fn get_warnings(&self) -> &[String] {
        &self.warnings
    }

    pub fn get_errors(&self) -> &[String] {
        &self.errors
    }
}

impl Default for BytecodeVerifier {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bytecode::Bytecode;

    #[test]
    fn test_valid_bytecode() {
        let mut bytecode = Bytecode::new();
        bytecode.add_instruction(OpCode::PushInt(42), None);
        bytecode.add_instruction(OpCode::PushInt(10), None);
        bytecode.add_instruction(OpCode::Add, None);
        bytecode.add_instruction(OpCode::Print(1), None);

        let mut verifier = BytecodeVerifier::new();
        let result = verifier.verify(&bytecode);
        assert!(result.is_ok());
    }

    #[test]
    fn test_invalid_jump_target() {
        let mut bytecode = Bytecode::new();
        bytecode.add_instruction(OpCode::PushBool(true), None);
        bytecode.add_instruction(OpCode::JumpIfFalse(100), None);

        let mut verifier = BytecodeVerifier::new();
        let result = verifier.verify(&bytecode);
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_local_index() {
        let mut bytecode = Bytecode::new();
        bytecode.add_local("x".to_string());
        bytecode.add_instruction(OpCode::LoadLocal(5), None);

        let mut verifier = BytecodeVerifier::new();
        let result = verifier.verify(&bytecode);
        assert!(result.is_err());
    }
}
