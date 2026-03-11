use crate::bytecode::Bytecode;
use std::collections::HashMap;

pub mod vm;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Breakpoint {
    pub file: String,
    pub line: usize,
    pub enabled: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DebugStatus {
    Running,
    Paused,
    Stopped,
    BreakpointHit,
    StepOver,
    StepInto,
    StepOut,
}

#[derive(Debug, Clone)]
pub struct DebugFrame {
    pub function_name: String,
    pub file: String,
    pub line: usize,
    pub variables: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct Debugger {
    pub breakpoints: Vec<Breakpoint>,
    pub status: DebugStatus,
    pub current_frame: Option<DebugFrame>,
    pub call_stack: Vec<DebugFrame>,
    pub watched_variables: Vec<String>,
    pub program: Option<Bytecode>,
    pub source_files: HashMap<String, String>,
}

impl Debugger {
    pub fn new() -> Self {
        Debugger {
            breakpoints: Vec::new(),
            status: DebugStatus::Stopped,
            current_frame: None,
            call_stack: Vec::new(),
            watched_variables: Vec::new(),
            program: None,
            source_files: HashMap::new(),
        }
    }

    pub fn add_breakpoint(&mut self, file: String, line: usize) {
        let breakpoint = Breakpoint {
            file,
            line,
            enabled: true,
        };
        self.breakpoints.push(breakpoint);
    }

    pub fn remove_breakpoint(&mut self, file: &str, line: usize) {
        self.breakpoints.retain(|bp| !(bp.file == file && bp.line == line));
    }

    pub fn toggle_breakpoint(&mut self, file: &str, line: usize) {
        for bp in &mut self.breakpoints {
            if bp.file == file && bp.line == line {
                bp.enabled = !bp.enabled;
                break;
            }
        }
    }

    pub fn clear_breakpoints(&mut self) {
        self.breakpoints.clear();
    }

    pub fn is_breakpoint(&self, file: &str, line: usize) -> bool {
        self.breakpoints.iter().any(|bp| 
            bp.file == file && bp.line == line && bp.enabled
        )
    }

    pub fn add_watched_variable(&mut self, variable: String) {
        if !self.watched_variables.contains(&variable) {
            self.watched_variables.push(variable);
        }
    }

    pub fn remove_watched_variable(&mut self, variable: &str) {
        self.watched_variables.retain(|v| v != variable);
    }

    pub fn load_program(&mut self, program: Bytecode) {
        self.program = Some(program);
        self.status = DebugStatus::Stopped;
        self.current_frame = None;
        self.call_stack.clear();
    }

    pub fn load_source_file(&mut self, file: String, content: String) {
        self.source_files.insert(file, content);
    }

    pub fn step_over(&mut self) {
        self.status = DebugStatus::StepOver;
    }

    pub fn step_into(&mut self) {
        self.status = DebugStatus::StepInto;
    }

    pub fn step_out(&mut self) {
        self.status = DebugStatus::StepOut;
    }

    pub fn continue_execution(&mut self) {
        self.status = DebugStatus::Running;
    }

    pub fn stop(&mut self) {
        self.status = DebugStatus::Stopped;
        self.current_frame = None;
        self.call_stack.clear();
    }

    pub fn update_frame(&mut self, frame: DebugFrame) {
        self.current_frame = Some(frame);
    }

    pub fn update_call_stack(&mut self, stack: Vec<DebugFrame>) {
        self.call_stack = stack;
    }
}
