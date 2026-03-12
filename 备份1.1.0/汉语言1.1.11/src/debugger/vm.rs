use crate::bytecode::{Bytecode, BytecodeVM};
use crate::debugger::{DebugFrame, DebugStatus, Debugger};
use crate::error::{RuntimeError, RuntimeResult};
use crate::runtime::value::Value;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub struct DebugBytecodeVM {
    vm: BytecodeVM,
    debugger: Rc<RefCell<Debugger>>,
}

impl DebugBytecodeVM {
    pub fn new(debugger: Rc<RefCell<Debugger>>) -> Self {
        DebugBytecodeVM {
            vm: BytecodeVM::new(),
            debugger,
        }
    }

    pub fn run(&mut self, bytecode: &Bytecode) -> RuntimeResult<Value> {
        {
            let mut debugger = self.debugger.borrow_mut();
            debugger.status = DebugStatus::Running;
        }

        let result = self.run_debug(bytecode);

        {
            let mut debugger = self.debugger.borrow_mut();
            debugger.status = DebugStatus::Stopped;
        }

        result
    }

    fn run_debug(&mut self, bytecode: &Bytecode) -> RuntimeResult<Value> {
        self.vm.pc = 0;
        self.vm.locals = vec![Value::空值; bytecode.local_names.len()];

        while self.vm.pc < bytecode.instructions.len() {
            let op = &bytecode.instructions[self.vm.pc];

            // 检查断点
            let should_break = {
                let debugger = self.debugger.borrow();
                if let Some(file) = &bytecode.file_name {
                    if let Some(line) = bytecode.line_numbers.get(self.vm.pc) {
                        if let Some(line_num) = line {
                            if debugger.is_breakpoint(file, *line_num) {
                                true
                            } else {
                                false
                            }
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                } else {
                    false
                }
            };

            if should_break {
                {
                    let mut debugger = self.debugger.borrow_mut();
                    debugger.status = DebugStatus::BreakpointHit;
                }
                self.update_debug_frame(bytecode);
                return Err(RuntimeError::General("调试断点命中".to_string()));
            }

            // 检查调试状态
            let should_step = {
                let debugger = self.debugger.borrow();
                matches!(debugger.status, DebugStatus::StepOver | DebugStatus::StepInto | DebugStatus::StepOut)
            };

            if should_step {
                {
                    let mut debugger = self.debugger.borrow_mut();
                    debugger.status = DebugStatus::Paused;
                }
                self.update_debug_frame(bytecode);
                return Err(RuntimeError::General("调试步骤执行".to_string()));
            }

            self.vm.execute_op(op, bytecode)?;
            self.vm.pc += 1;
        }

        if let Some(result) = self.vm.stack.pop() {
            Ok(result)
        } else {
            Ok(Value::空值)
        }
    }

    fn update_debug_frame(&mut self, bytecode: &Bytecode) {
        let mut debugger = self.debugger.borrow_mut();

        // 构建当前帧信息
        let mut variables = HashMap::new();
        for (i, name) in bytecode.local_names.iter().enumerate() {
            if let Some(value) = self.vm.locals.get(i) {
                variables.insert(name.clone(), format!("{:?}", value));
            }
        }

        let frame = DebugFrame {
            function_name: "main".to_string(),
            file: bytecode.file_name.clone().unwrap_or("<unknown>".to_string()),
            line: bytecode.line_numbers.get(self.vm.pc).and_then(|l| *l).unwrap_or(0),
            variables,
        };

        debugger.update_frame(frame);
    }

    pub fn step_over(&mut self) {
        let mut debugger = self.debugger.borrow_mut();
        debugger.step_over();
    }

    pub fn step_into(&mut self) {
        let mut debugger = self.debugger.borrow_mut();
        debugger.step_into();
    }

    pub fn step_out(&mut self) {
        let mut debugger = self.debugger.borrow_mut();
        debugger.step_out();
    }

    pub fn continue_execution(&mut self) {
        let mut debugger = self.debugger.borrow_mut();
        debugger.continue_execution();
    }

    pub fn stop(&mut self) {
        let mut debugger = self.debugger.borrow_mut();
        debugger.stop();
    }
}
