pub mod bytecode;
pub mod debugger;
pub mod error;
pub mod gui;
pub mod ide;
pub mod interpreter;
pub mod lexer;
pub mod parser;
pub mod repl;
pub mod runtime;
pub mod stdlib;

pub use bytecode::{Bytecode, BytecodeCompiler, OpCode, BytecodeVM, BytecodeVerifier};
pub use debugger::{Debugger, DebugStatus, Breakpoint, DebugFrame};
pub use debugger::vm::DebugBytecodeVM;
pub use interpreter::Interpreter;

#[cfg(test)]
mod tests {
    use crate::lexer;
    use crate::parser;
    use crate::interpreter::Interpreter;
    use crate::bytecode::{Bytecode, BytecodeCompiler, BytecodeVM, BytecodeVerifier, OpCode};

    #[test]
    fn test_lexer_chinese() {
        let source = "打印（「你好」）";
        let tokens = lexer::tokenize(source).unwrap();
        assert!(tokens.len() > 0, "应该有tokens");
    }

    #[test]
    fn test_parser_print() {
        let source = "打印（「你好」）";
        let tokens = lexer::tokenize(source).unwrap();
        let program = parser::parse(tokens).unwrap();
        assert_eq!(program.statements.len(), 1);
    }

    #[test]
    fn test_interpreter_print() {
        let source = "打印（「你好」）";
        let tokens = lexer::tokenize(source).unwrap();
        let program = parser::parse(tokens).unwrap();
        let mut interpreter = Interpreter::new();
        let result = interpreter.run(&program);
        assert!(result.is_ok());
    }

    #[test]
    fn test_variable() {
        let source = r#"定义 甲 为 整数
甲 ＝ 十"#;
        let tokens = lexer::tokenize(source).unwrap();
        let program = parser::parse(tokens).unwrap();
        let mut interpreter = Interpreter::new();
        let result = interpreter.run(&program);
        assert!(result.is_ok());
    }

    #[test]
    fn test_function() {
        let source = r#"定义 加一 为 函数（数值）
    返回 数值 ＋ 一
结束
定义 结果 ＝ 加一（五）"#;
        let tokens = lexer::tokenize(source).unwrap();
        let program = parser::parse(tokens).unwrap();
        let mut interpreter = Interpreter::new();
        let result = interpreter.run(&program);
        assert!(result.is_ok());
    }

    #[test]
    fn test_if_statement() {
        let source = r#"如果 真 则
    打印（「是」）
否则
    打印（「否」）
结束"#;
        let tokens = lexer::tokenize(source).unwrap();
        let program = parser::parse(tokens).unwrap();
        let mut interpreter = Interpreter::new();
        let result = interpreter.run(&program);
        assert!(result.is_ok());
    }

    #[test]
    fn test_while_loop() {
        let source = r#"定义 甲 为 整数
甲 ＝ 零
当 甲 小于 三 则
    甲 ＝ 甲 ＋ 一
结束"#;
        let tokens = lexer::tokenize(source).unwrap();
        let program = parser::parse(tokens).unwrap();
        let mut interpreter = Interpreter::new();
        let result = interpreter.run(&program);
        assert!(result.is_ok());
    }

    #[test]
    fn test_for_loop() {
        let source = r#"对于 甲 在 范围（零，三）则
    打印（甲）
结束"#;
        let tokens = lexer::tokenize(source).unwrap();
        let program = parser::parse(tokens).unwrap();
        let mut interpreter = Interpreter::new();
        let result = interpreter.run(&program);
        if let Err(ref e) = result {
            eprintln!("Error: {:?}", e);
        }
        assert!(result.is_ok());
    }

    #[test]
    fn test_list() {
        let source = r#"定义 列表 ＝ 【一，二，三】
打印（列表）"#;
        let tokens = lexer::tokenize(source).unwrap();
        let program = parser::parse(tokens).unwrap();
        let mut interpreter = Interpreter::new();
        let result = interpreter.run(&program);
        assert!(result.is_ok());
    }

    #[test]
    fn test_dict() {
        let source = r#"定义 字典 ＝ ｛「甲」：一，「乙」：二｝
打印（字典）"#;
        let tokens = lexer::tokenize(source).unwrap();
        let program = parser::parse(tokens).unwrap();
        let mut interpreter = Interpreter::new();
        let result = interpreter.run(&program);
        assert!(result.is_ok());
    }

    #[test]
    fn test_index_assignment() {
        let source = r#"定义 列表 ＝ 【一，二，三】
列表【零】＝ 十
打印（列表）"#;
        let tokens = lexer::tokenize(source).unwrap();
        let program = parser::parse(tokens).unwrap();
        let mut interpreter = Interpreter::new();
        let result = interpreter.run(&program);
        if let Err(e) = &result {
            println!("Error: {:?}", e);
        }
        assert!(result.is_ok());
    }

    #[test]
    fn test_property_assignment() {
        let source = r#"定义 甲 ＝ ｛「值」：一｝
甲．值 ＝ 十
打印（甲）"#;
        let tokens = lexer::tokenize(source).unwrap();
        let program = parser::parse(tokens).unwrap();
        let mut interpreter = Interpreter::new();
        let result = interpreter.run(&program);
        assert!(result.is_ok());
    }

    #[test]
    fn test_dict_index_assignment() {
        let source = r#"定义 字典 ＝ ｛「甲」：一｝
字典【「甲」】＝ 十
打印（字典）"#;
        let tokens = lexer::tokenize(source).unwrap();
        let program = parser::parse(tokens).unwrap();
        let mut interpreter = Interpreter::new();
        let result = interpreter.run(&program);
        if let Err(e) = &result {
            println!("Error: {:?}", e);
        }
        assert!(result.is_ok());
    }

    #[test]
    fn test_builtin_functions() {
        let source = r#"定义 存在 ＝ 文件存在（「不存在的文件.txt」）
打印（存在）"#;
        let tokens = lexer::tokenize(source).unwrap();
        let program = parser::parse(tokens).unwrap();
        let mut interpreter = Interpreter::new();
        let result = interpreter.run(&program);
        if let Err(ref e) = result {
            eprintln!("Error: {:?}", e);
        }
        assert!(result.is_ok());
    }

    #[test]
    fn test_bytecode_push_int() {
        let mut bytecode = Bytecode::new();
        let _ = bytecode.add_instruction(OpCode::PushInt(42), None);
        assert_eq!(bytecode.instructions.len(), 1);
        assert_eq!(bytecode.instructions[0], OpCode::PushInt(42));
    }

    #[test]
    fn test_bytecode_push_string() {
        let mut bytecode = Bytecode::new();
        let _ = bytecode.add_instruction(OpCode::PushString("你好".to_string()), None);
        assert_eq!(bytecode.instructions.len(), 1);
        assert_eq!(bytecode.instructions[0], OpCode::PushString("你好".to_string()));
    }

    #[test]
    fn test_bytecode_add_local() {
        let mut bytecode = Bytecode::new();
        let idx1 = bytecode.add_local("甲".to_string());
        let idx2 = bytecode.add_local("乙".to_string());
        assert_eq!(idx1, 0);
        assert_eq!(idx2, 1);
        assert_eq!(bytecode.local_names.len(), 2);
    }

    #[test]
    fn test_bytecode_get_local_index() {
        let mut bytecode = Bytecode::new();
        let _ = bytecode.add_local("甲".to_string());
        let _ = bytecode.add_local("乙".to_string());
        
        assert_eq!(bytecode.get_local_index("甲"), Some(0));
        assert_eq!(bytecode.get_local_index("乙"), Some(1));
        assert_eq!(bytecode.get_local_index("丙"), None);
    }

    #[test]
    fn test_bytecode_patch_jump() {
        let mut bytecode = Bytecode::new();
        let jump_idx = bytecode.add_instruction(OpCode::Jump(0), None);
        let _ = bytecode.add_instruction(OpCode::PushInt(1), None);
        let target = bytecode.len();
        
        bytecode.patch_jump(jump_idx, target);
        
        if let OpCode::Jump(t) = &bytecode.instructions[jump_idx] {
            assert_eq!(*t, target);
        } else {
            panic!("Expected Jump instruction");
        }
    }

    #[test]
    fn test_bytecode_disassemble() {
        let mut bytecode = Bytecode::new();
        let _ = bytecode.add_instruction(OpCode::PushInt(1), None);
        let _ = bytecode.add_instruction(OpCode::PushInt(2), None);
        let _ = bytecode.add_instruction(OpCode::Add, None);
        
        let disasm = bytecode.disassemble();
        assert!(disasm.contains("PUSH_INT"));
        assert!(disasm.contains("ADD"));
    }

    #[test]
    fn test_bytecode_vm_push_pop() {
        let mut vm = BytecodeVM::new();
        let mut bytecode = Bytecode::new();
        
        let _ = bytecode.add_instruction(OpCode::PushInt(42), None);
        let _ = bytecode.add_instruction(OpCode::PushInt(10), None);
        let _ = bytecode.add_instruction(OpCode::Add, None);
        
        let result = vm.run(&bytecode);
        assert!(result.is_ok());
        if let Ok(crate::runtime::value::Value::整数(n)) = result {
            assert_eq!(n, 52);
        } else {
            panic!("Expected integer result");
        }
    }

    #[test]
    fn test_bytecode_vm_arithmetic() {
        let mut vm = BytecodeVM::new();
        let mut bytecode = Bytecode::new();
        
        let _ = bytecode.add_instruction(OpCode::PushInt(10), None);
        let _ = bytecode.add_instruction(OpCode::PushInt(3), None);
        let _ = bytecode.add_instruction(OpCode::Subtract, None);
        
        let result = vm.run(&bytecode);
        assert!(result.is_ok());
        if let Ok(crate::runtime::value::Value::整数(n)) = result {
            assert_eq!(n, 7);
        }
    }

    #[test]
    fn test_bytecode_vm_comparison() {
        let mut vm = BytecodeVM::new();
        let mut bytecode = Bytecode::new();
        
        let _ = bytecode.add_instruction(OpCode::PushInt(10), None);
        let _ = bytecode.add_instruction(OpCode::PushInt(5), None);
        let _ = bytecode.add_instruction(OpCode::Greater, None);
        
        let result = vm.run(&bytecode);
        assert!(result.is_ok());
        if let Ok(crate::runtime::value::Value::布尔值(b)) = result {
            assert!(b);
        }
    }

    #[test]
    fn test_bytecode_vm_list() {
        let mut vm = BytecodeVM::new();
        let mut bytecode = Bytecode::new();
        
        let _ = bytecode.add_instruction(OpCode::PushInt(1), None);
        let _ = bytecode.add_instruction(OpCode::PushInt(2), None);
        let _ = bytecode.add_instruction(OpCode::PushInt(3), None);
        let _ = bytecode.add_instruction(OpCode::PushList(3), None);
        
        let result = vm.run(&bytecode);
        assert!(result.is_ok());
        if let Ok(crate::runtime::value::Value::列表(v)) = result {
            assert_eq!(v.len(), 3);
        }
    }

    #[test]
    fn test_bytecode_compiler_simple() {
        let source = "打印（「你好」）";
        let tokens = lexer::tokenize(source).unwrap();
        let program = parser::parse(tokens).unwrap();
        
        let mut compiler = BytecodeCompiler::new();
        let bytecode = compiler.compile_program(&program);
        
        assert!(bytecode.is_ok());
        let bc = bytecode.unwrap();
        assert!(!bc.instructions.is_empty());
    }

    #[test]
    fn test_bytecode_compiler_variable() {
        let source = r#"定义 甲 为 整数
甲 ＝ 十"#;
        let tokens = lexer::tokenize(source).unwrap();
        let program = parser::parse(tokens).unwrap();
        
        let mut compiler = BytecodeCompiler::new();
        let bytecode = compiler.compile_program(&program);
        
        assert!(bytecode.is_ok());
        let bc = bytecode.unwrap();
        assert!(bc.local_names.contains(&"甲".to_string()));
    }

    #[test]
    fn test_bytecode_compiler_if() {
        let source = r#"如果 真 则
    打印（「是」）
结束"#;
        let tokens = lexer::tokenize(source).unwrap();
        let program = parser::parse(tokens).unwrap();
        
        let mut compiler = BytecodeCompiler::new();
        let bytecode = compiler.compile_program(&program);
        
        assert!(bytecode.is_ok());
        let bc = bytecode.unwrap();
        assert!(bc.instructions.iter().any(|op| matches!(op, OpCode::JumpIfFalse(_))));
    }

    #[test]
    fn test_bytecode_compiler_loop() {
        let source = r#"定义 甲 为 整数
甲 ＝ 零
当 甲 小于 五 则
    甲 ＝ 甲 ＋ 一
结束"#;
        let tokens = lexer::tokenize(source).unwrap();
        let program = parser::parse(tokens).unwrap();
        
        let mut compiler = BytecodeCompiler::new();
        let bytecode = compiler.compile_program(&program);
        
        assert!(bytecode.is_ok());
        let bc = bytecode.unwrap();
        assert!(bc.instructions.iter().any(|op| matches!(op, OpCode::Jump(_))));
    }

    #[test]
    fn test_bytecode_verifier_valid() {
        let mut bytecode = Bytecode::new();
        let _ = bytecode.add_instruction(OpCode::PushInt(1), None);
        let _ = bytecode.add_instruction(OpCode::PushInt(2), None);
        let _ = bytecode.add_instruction(OpCode::Add, None);
        
        let mut verifier = BytecodeVerifier::new();
        let result = verifier.verify(&bytecode);
        assert!(result.is_ok());
    }

    #[test]
    fn test_bytecode_verifier_invalid_jump() {
        let mut bytecode = Bytecode::new();
        let _ = bytecode.add_instruction(OpCode::Jump(100), None);
        
        let mut verifier = BytecodeVerifier::new();
        let result = verifier.verify(&bytecode);
        assert!(result.is_err());
    }

    #[test]
    fn test_stdlib_stack() {
        let source = r#"定义 栈 ＝ 栈新建（）
定义 栈2 ＝ 栈推入（栈，一）
定义 栈3 ＝ 栈推入（栈2，二）
定义 甲 ＝ 栈弹出（栈3）
打印（甲）"#;
        let tokens = lexer::tokenize(source).unwrap();
        let program = parser::parse(tokens).unwrap();
        let mut interpreter = Interpreter::new();
        let result = interpreter.run(&program);
        assert!(result.is_ok());
    }

    #[test]
    fn test_stdlib_queue() {
        let source = r#"定义 队列 ＝ 队列新建（）
定义 队列2 ＝ 队列入队（队列，一）
定义 队列3 ＝ 队列入队（队列2，二）
定义 甲 ＝ 队列出队（队列3）
打印（甲）"#;
        let tokens = lexer::tokenize(source).unwrap();
        let program = parser::parse(tokens).unwrap();
        let mut interpreter = Interpreter::new();
        let result = interpreter.run(&program);
        assert!(result.is_ok());
    }

    #[test]
    fn test_stdlib_hashmap() {
        let source = r#"定义 表 ＝ 哈希表新建（）
定义 表2 ＝ 哈希表设置（表，「甲」，一）
定义 表3 ＝ 哈希表设置（表2，「乙」，二）
定义 甲 ＝ 哈希表获取（表3，「甲」）
打印（甲）"#;
        let tokens = lexer::tokenize(source).unwrap();
        let program = parser::parse(tokens).unwrap();
        let mut interpreter = Interpreter::new();
        let result = interpreter.run(&program);
        assert!(result.is_ok());
    }

    #[test]
    fn test_stdlib_file_operations() {
        let source = r#"定义 存在 ＝ 文件存在（「test.txt」）
打印（存在）"#;
        let tokens = lexer::tokenize(source).unwrap();
        let program = parser::parse(tokens).unwrap();
        let mut interpreter = Interpreter::new();
        let result = interpreter.run(&program);
        assert!(result.is_ok());
    }

    #[test]
    fn test_bytecode_integration() {
        let source = r#"定义 甲 为 整数
甲 ＝ 十
打印（甲）"#;
        let tokens = lexer::tokenize(source).unwrap();
        let program = parser::parse(tokens).unwrap();
        
        let mut compiler = BytecodeCompiler::new();
        let bytecode = compiler.compile_program(&program).unwrap();
        
        let mut verifier = BytecodeVerifier::new();
        let verify_result = verifier.verify(&bytecode);
        assert!(verify_result.is_ok());
        
        let mut vm = BytecodeVM::new();
        let run_result = vm.run(&bytecode);
        assert!(run_result.is_ok());
    }

    #[test]
    fn test_comment_slash() {
        // 测试 // 格式的注释
        let source = "// 这是注释\n定义 主函数 为 函数() {\n    打印(\"你好\")\n}";
        println!("源代码: {:?}", source);
        let tokens = lexer::tokenize(source);
        match &tokens {
            Ok(t) => {
                println!("Tokens:");
                for token in t {
                    println!("  {:?}", token);
                }
            }
            Err(e) => println!("词法错误: {:?}", e),
        }
        assert!(tokens.is_ok(), "词法分析应该成功: {:?}", tokens.err());
        
        let program = parser::parse(tokens.unwrap());
        assert!(program.is_ok(), "语法分析应该成功: {:?}", program.err());
    }

    #[test]
    fn test_string_literal_required() {
        // 测试字符串值必须用引号包围
        let source = r#"定义 名字 ＝ "张三"
定义 年龄 ＝ 18
打印("我是：", 名字, "我的年龄是：", 年龄)"#;
        
        let tokens = lexer::tokenize(source);
        assert!(tokens.is_ok(), "词法分析应该成功: {:?}", tokens.err());
        
        let program = parser::parse(tokens.unwrap());
        assert!(program.is_ok(), "语法分析应该成功: {:?}", program.err());
        
        // 运行测试
        let mut interpreter = Interpreter::new();
        let result = interpreter.run(&program.unwrap());
        assert!(result.is_ok(), "执行应该成功: {:?}", result.err());
    }

    #[test]
    fn test_bare_identifier_error() {
        // 测试裸标识符应该报错
        let source = r#"定义 名字 ＝ 张三"#;
        
        let tokens = lexer::tokenize(source);
        assert!(tokens.is_ok(), "词法分析应该成功: {:?}", tokens.err());
        
        let program = parser::parse(tokens.unwrap());
        assert!(program.is_err(), "语法分析应该失败，因为字符串没有用引号包围");
        
        let err_msg = format!("{:?}", program.err());
        assert!(err_msg.contains("引号"), "错误信息应该提示使用引号: {}", err_msg);
    }

    #[test]
    fn test_gui_example() {
        // 测试GUI示例程序
        let source = r#"// 汉语编程 GUI 示例程序
定义 主函数 为 函数() {
    打印("========================================")
    打印("   欢迎使用汉语编程 GUI 示例！")
    打印("========================================")
    
    定义 窗口标题 ＝ "我的第一个汉语编程 GUI 窗口"
    定义 窗口宽度 ＝ 800
    定义 窗口高度 ＝ 600
    
    打印("窗口标题: ", 窗口标题)
    打印("窗口尺寸: ", 窗口宽度, " x ", 窗口高度)
    
    定义 按钮1文本 ＝ "点击我"
    定义 按钮2文本 ＝ "取消"
    定义 按钮3文本 ＝ "确定"
    
    打印("按钮1: ", 按钮1文本)
    打印("按钮2: ", 按钮2文本)
    打印("按钮3: ", 按钮3文本)
    
    定义 欢迎标签 ＝ "欢迎使用本系统"
    打印(欢迎标签)
    
    定义 用户名 ＝ "张三"
    定义 年龄 ＝ 25
    
    打印("用户名: ", 用户名)
    打印("年龄: ", 年龄)
    
    定义 菜单列表 ＝ ["文件", "编辑", "视图", "帮助"]
    打印("菜单: ", 菜单列表)
    
    打印("GUI 界面创建成功！")
}

主函数()"#;
        
        let tokens = lexer::tokenize(source);
        assert!(tokens.is_ok(), "词法分析应该成功: {:?}", tokens.err());
        
        let program = parser::parse(tokens.unwrap());
        assert!(program.is_ok(), "语法分析应该成功: {:?}", program.err());
        
        // 运行程序
        let mut interpreter = Interpreter::new();
        let result = interpreter.run(&program.unwrap());
        assert!(result.is_ok(), "程序执行应该成功: {:?}", result.err());
    }

    #[test]
    fn test_gui_demo() {
        // 测试完整的GUI演示程序
        let source = include_str!("gui_demo.hy");
        
        let tokens = lexer::tokenize(source);
        assert!(tokens.is_ok(), "词法分析应该成功: {:?}", tokens.err());
        
        let program = parser::parse(tokens.unwrap());
        assert!(program.is_ok(), "语法分析应该成功: {:?}", program.err());
        
        // 运行程序
        let mut interpreter = Interpreter::new();
        let result = interpreter.run(&program.unwrap());
        assert!(result.is_ok(), "程序执行应该成功: {:?}", result.err());
    }
}
