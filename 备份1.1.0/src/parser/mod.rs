pub mod ast;

pub use ast::*;
use ast::FormatPart;

use crate::error::{ParserError, ParserResult};
use crate::lexer::{Delimiter, Keyword, Operator, Token, TokenType, TokenValue};

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0 }
    }
    
    pub fn parse(&mut self) -> ParserResult<Program> {
        let mut statements = Vec::new();
        
        while !self.is_at_end() {
            self.skip_newlines();
            if self.is_at_end() {
                break;
            }
            
            if let Some(stmt) = self.parse_statement()? {
                statements.push(stmt);
            }
            self.skip_newlines();
        }
        
        Ok(Program::from(statements))
    }
    
    fn current_token(&self) -> &Token {
        if self.pos < self.tokens.len() {
            &self.tokens[self.pos]
        } else {
            &self.tokens[self.tokens.len() - 1]
        }
    }
    
    #[allow(dead_code)]
    fn peek_token(&self, offset: usize) -> &Token {
        let idx = self.pos + offset;
        if idx < self.tokens.len() {
            &self.tokens[idx]
        } else {
            &self.tokens[self.tokens.len() - 1]
        }
    }
    
    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.pos += 1;
        }
        self.current_token()
    }
    
    fn is_at_end(&self) -> bool {
        matches!(self.current_token().value, TokenValue::Eof)
    }
    
    fn skip_newlines(&mut self) {
        loop {
            if matches!(self.current_token().value, TokenValue::Newline) {
                self.advance();
            } else if matches!(self.current_token().token_type, TokenType::注释) {
                self.advance();
            } else {
                break;
            }
        }
    }
    
    #[allow(dead_code)]
    fn skip_comments(&mut self) {
        while matches!(self.current_token().token_type, TokenType::注释) {
            self.advance();
        }
    }
    
    #[allow(dead_code)]
    fn skip_newlines_and_comments(&mut self) {
        self.skip_newlines();
    }
    
    fn skip_indents(&mut self) {
        while matches!(self.current_token().value, TokenValue::Indent(_)) {
            self.advance();
        }
    }
    
    #[allow(dead_code)]
    fn get_indent_level(&self) -> Option<usize> {
        match &self.current_token().value {
            TokenValue::Indent(level) => Some(*level),
            _ => None,
        }
    }
    
    fn check_keyword(&self, keyword: Keyword) -> bool {
        matches!(&self.current_token().value, TokenValue::Keyword(k) if *k == keyword)
    }
    
    #[allow(dead_code)]
    fn check_token_type(&self, token_type: TokenType) -> bool {
        self.current_token().token_type == token_type
    }
    
    fn check_delimiter(&self, delimiter: Delimiter) -> bool {
        matches!(&self.current_token().value, TokenValue::Delimiter(d) if *d == delimiter)
    }
    
    fn check_operator(&self, operator: Operator) -> bool {
        matches!(&self.current_token().value, TokenValue::Operator(o) if *o == operator)
    }
    
    fn consume_keyword(&mut self, keyword: Keyword) -> ParserResult<()> {
        if self.check_keyword(keyword.clone()) {
            self.advance();
            Ok(())
        } else {
            Err(ParserError::UnexpectedToken {
                expected: keyword.to_string(),
                actual: self.current_token().to_string(),
                line: self.current_token().line,
                column: self.current_token().column,
            })
        }
    }
    
    fn consume_delimiter(&mut self, delimiter: Delimiter) -> ParserResult<()> {
        if self.check_delimiter(delimiter.clone()) {
            self.advance();
            Ok(())
        } else {
            Err(ParserError::UnexpectedToken {
                expected: delimiter.to_string(),
                actual: self.current_token().to_string(),
                line: self.current_token().line,
                column: self.current_token().column,
            })
        }
    }
    
    fn consume_identifier(&mut self) -> ParserResult<String> {
        match &self.current_token().value {
            TokenValue::Identifier(name) => {
                let name = name.clone();
                self.advance();
                Ok(name)
            }
            _ => Err(ParserError::UnexpectedToken {
                expected: "标识符".to_string(),
                actual: self.current_token().to_string(),
                line: self.current_token().line,
                column: self.current_token().column,
            }),
        }
    }
    
    fn parse_statement(&mut self) -> ParserResult<Option<Statement>> {
        self.skip_indents();
        
        // 跳过注释
        while matches!(self.current_token().token_type, TokenType::注释) {
            self.advance();
            self.skip_indents();
        }
        
        if self.check_keyword(Keyword::定义) {
            self.parse_definition()
        } else if self.check_keyword(Keyword::变量) {
            self.parse_variable_definition()
        } else if self.check_keyword(Keyword::如果) {
            self.parse_if_statement()
        } else if self.check_keyword(Keyword::循环) {
            self.parse_loop_statement()
        } else if self.check_keyword(Keyword::当) {
            self.parse_while_statement()
        } else if self.check_keyword(Keyword::对于) {
            self.parse_for_statement()
        } else if self.check_keyword(Keyword::返回) {
            self.parse_return_statement()
        } else if self.check_keyword(Keyword::打印) {
            self.parse_print_statement()
        } else if self.check_keyword(Keyword::导入) {
            self.parse_import_statement()
        } else if self.check_keyword(Keyword::类) {
            self.parse_class_definition()
        } else if self.check_keyword(Keyword::尝试) {
            self.parse_try_statement()
        } else if self.check_keyword(Keyword::抛出) {
            self.parse_throw_statement()
        } else if self.check_keyword(Keyword::使用) {
            self.parse_with_statement()
        } else if self.check_keyword(Keyword::生成) {
            self.parse_yield_statement()
        } else if self.check_keyword(Keyword::枚举) {
            self.parse_enum_definition()
        } else if self.check_keyword(Keyword::结束) {
            Ok(None)
        } else {
            self.parse_expression_statement()
        }
    }
    
    fn parse_definition(&mut self) -> ParserResult<Option<Statement>> {
        self.consume_keyword(Keyword::定义)?;
        
        if self.check_delimiter(Delimiter::左括号) {
            self.advance();
            let mut var_list = Vec::new();
            
            if !self.check_delimiter(Delimiter::右括号) {
                var_list.push(self.consume_identifier()?);
                while self.check_delimiter(Delimiter::逗号) {
                    self.advance();
                    var_list.push(self.consume_identifier()?);
                }
            }
            
            self.consume_delimiter(Delimiter::右括号)?;
            if !self.check_operator(Operator::赋值) {
                return Err(ParserError::UnexpectedToken {
                    expected: "=".to_string(),
                    actual: self.current_token().to_string(),
                    line: self.current_token().line,
                    column: self.current_token().column,
                });
            }
            self.advance();
            let value = self.parse_expression()?;
            
            return Ok(Some(Statement::解构赋值 {
                变量列表: var_list,
                值: value,
            }));
        }
        
        let name = self.consume_identifier()?;
        
        if self.check_keyword(Keyword::函数) {
            return self.parse_function_definition(name);
        }
        
        let mut var_type = None;
        if self.check_keyword(Keyword::为) || self.check_keyword(Keyword::是) {
            self.advance();
            if self.check_keyword(Keyword::函数) {
                return self.parse_function_definition(name);
            }
            var_type = Some(self.consume_identifier()?);
        }
        
        let mut initial_value = None;
        if self.check_operator(Operator::赋值) {
            self.advance();
            if let TokenValue::Identifier(ident_name) = &self.current_token().value {
                let next_token = self.peek_token(1);
                let is_valid_expr = matches!(&next_token.value, 
                    TokenValue::Delimiter(Delimiter::左括号) | 
                    TokenValue::Delimiter(Delimiter::左方括号) |
                    TokenValue::Operator(_));
                if !is_valid_expr {
                    let ident_name = ident_name.clone();
                    let line = self.current_token().line;
                    let column = self.current_token().column;
                    return Err(ParserError::SyntaxError(
                        format!("字符串值必须用引号包围，请使用 \"{}\" 代替 {}", ident_name, ident_name),
                        line,
                        column,
                    ));
                }
            }
            initial_value = Some(self.parse_expression()?);
        }
        
        Ok(Some(Statement::变量定义 {
            名称: name,
            类型: var_type,
            初始值: initial_value,
        }))
    }
    
    fn parse_variable_definition(&mut self) -> ParserResult<Option<Statement>> {
        self.consume_keyword(Keyword::变量)?;
        let name = self.consume_identifier()?;
        
        let mut var_type = None;
        if self.check_keyword(Keyword::为) || self.check_keyword(Keyword::是) {
            self.advance();
            var_type = Some(self.consume_identifier()?);
        }
        
        let mut initial_value = None;
        if self.check_operator(Operator::赋值) {
            self.advance();
            // 检查是否是裸标识符（没有用引号包围的字符串，且不是函数调用、索引访问或二元运算）
            if let TokenValue::Identifier(ident_name) = &self.current_token().value {
                // 检查下一个token是否是左括号、左方括号或运算符
                let next_token = self.peek_token(1);
                let is_valid_expr = matches!(&next_token.value, 
                    TokenValue::Delimiter(Delimiter::左括号) | 
                    TokenValue::Delimiter(Delimiter::左方括号) |
                    TokenValue::Operator(_));
                if !is_valid_expr {
                    let ident_name = ident_name.clone();
                    let line = self.current_token().line;
                    let column = self.current_token().column;
                    return Err(ParserError::SyntaxError(
                        format!("字符串值必须用引号包围，请使用 \"{}\" 代替 {}", ident_name, ident_name),
                        line,
                        column,
                    ));
                }
            }
            initial_value = Some(self.parse_expression()?);
        }
        
        Ok(Some(Statement::变量定义 {
            名称: name,
            类型: var_type,
            初始值: initial_value,
        }))
    }
    
    fn parse_function_definition(&mut self, name: String) -> ParserResult<Option<Statement>> {
        self.consume_keyword(Keyword::函数)?;
        
        self.consume_delimiter(Delimiter::左括号)?;
        let params = self.parse_parameters()?;
        self.consume_delimiter(Delimiter::右括号)?;
        
        let mut return_type = None;
        if self.check_keyword(Keyword::为) || self.check_keyword(Keyword::是) {
            self.advance();
            return_type = Some(self.consume_identifier()?);
        }
        
        self.skip_newlines();
        
        // 检查是否有花括号包围的函数体
        let body = if self.check_delimiter(Delimiter::左花括号) {
            self.advance(); // 消耗 {
            let stmts = self.parse_block()?;
            self.consume_delimiter(Delimiter::右花括号)?;
            stmts
        } else {
            self.parse_block()?
        };
        
        Ok(Some(Statement::函数定义 {
            名称: name,
            参数: params.0,
            可变参数名: params.1,
            返回类型: return_type,
            函数体: body,
        }))
    }
    
    fn parse_parameters(&mut self) -> ParserResult<(Vec<FunctionParameter>, Option<String>)> {
        let mut params = Vec::new();
        let mut vararg_name = None;
        
        if self.check_delimiter(Delimiter::右括号) {
            return Ok((params, vararg_name));
        }
        
        loop {
            if self.check_operator(Operator::乘) {
                self.advance();
                let name = self.consume_identifier()?;
                vararg_name = Some(name);
                break;
            }
            
            let name = self.consume_identifier()?;
            let mut param_type = None;
            let mut default_value = None;
            
            if self.check_keyword(Keyword::为) || self.check_keyword(Keyword::是) {
                self.advance();
                param_type = Some(self.consume_identifier()?);
            }
            
            if self.check_operator(Operator::赋值) {
                self.advance();
                default_value = Some(self.parse_expression()?);
            }
            
            params.push(FunctionParameter {
                名称: name,
                类型: param_type,
                默认值: default_value,
                可变参数: false,
            });
            
            if !self.check_delimiter(Delimiter::逗号) {
                break;
            }
            self.advance();
        }
        
        Ok((params, vararg_name))
    }
    
    fn parse_block(&mut self) -> ParserResult<Vec<Statement>> {
        let mut statements = Vec::new();
        
        loop {
            self.skip_newlines();
            self.skip_indents();
            
            if self.is_at_end() 
               || self.check_keyword(Keyword::结束) 
               || self.check_keyword(Keyword::否则) 
               || self.check_keyword(Keyword::否则如果)
               || self.check_delimiter(Delimiter::右花括号) {
                break;
            }
            
            if let Some(stmt) = self.parse_statement()? {
                statements.push(stmt);
            }
        }
        
        if self.check_keyword(Keyword::结束) {
            self.advance();
            if self.check_keyword(Keyword::函数) || self.check_keyword(Keyword::类) 
                || self.check_keyword(Keyword::如果) || self.check_keyword(Keyword::循环) {
                self.advance();
            }
        }
        
        Ok(statements)
    }
    
    fn parse_if_statement(&mut self) -> ParserResult<Option<Statement>> {
        self.consume_keyword(Keyword::如果)?;
        let condition = self.parse_expression()?;
        
        if self.check_keyword(Keyword::则) {
            self.advance();
        }
        
        self.skip_newlines();
        let if_body = self.parse_block()?;
        
        let mut else_if_branches = Vec::new();
        let mut else_body = None;
        
        loop {
            self.skip_newlines();
            
            if self.check_keyword(Keyword::否则如果) {
                self.advance();
                let else_if_condition = self.parse_expression()?;
                if self.check_keyword(Keyword::则) {
                    self.advance();
                }
                self.skip_newlines();
                let else_if_body = self.parse_block()?;
                else_if_branches.push((else_if_condition, else_if_body));
            } else if self.check_keyword(Keyword::否则) {
                self.advance();
                self.skip_newlines();
                else_body = Some(self.parse_block()?);
                break;
            } else {
                break;
            }
        }
        
        Ok(Some(Statement::如果语句 {
            条件: condition,
            如果体: if_body,
            否则如果分支: else_if_branches,
            否则体: else_body,
        }))
    }
    
    fn parse_loop_statement(&mut self) -> ParserResult<Option<Statement>> {
        self.consume_keyword(Keyword::循环)?;
        
        let condition = if !self.check_keyword(Keyword::结束) {
            Some(self.parse_expression()?)
        } else {
            None
        };
        
        self.skip_newlines();
        let body = self.parse_block()?;
        
        Ok(Some(Statement::循环语句 {
            条件: condition.unwrap_or(Expression::布尔值 { 值: true }),
            循环体: body,
        }))
    }
    
    fn parse_while_statement(&mut self) -> ParserResult<Option<Statement>> {
        self.consume_keyword(Keyword::当)?;
        let 条件 = self.parse_expression()?;
        
        if self.check_keyword(Keyword::则) {
            self.advance();
        }
        
        self.skip_newlines();
        let 循环体 = self.parse_block()?;
        
        Ok(Some(Statement::循环语句 {
            条件,
            循环体,
        }))
    }
    
    fn parse_for_statement(&mut self) -> ParserResult<Option<Statement>> {
        self.consume_keyword(Keyword::对于)?;
        let var_name = self.consume_identifier()?;
        
        self.consume_keyword(Keyword::在)?;
        let iterable = self.parse_expression()?;
        
        // 处理可选的「则」关键字
        if self.check_keyword(Keyword::则) {
            self.advance();
        }
        
        self.skip_newlines();
        let body = self.parse_block()?;
        
        Ok(Some(Statement::对于循环 {
            变量: var_name,
            可迭代对象: iterable,
            循环体: body,
        }))
    }
    
    fn parse_return_statement(&mut self) -> ParserResult<Option<Statement>> {
        self.consume_keyword(Keyword::返回)?;
        
        let value = if !matches!(self.current_token().value, TokenValue::Newline | TokenValue::Eof) {
            Some(self.parse_expression()?)
        } else {
            None
        };
        
        Ok(Some(Statement::返回语句 { 值: value }))
    }
    
    fn parse_print_statement(&mut self) -> ParserResult<Option<Statement>> {
        self.consume_keyword(Keyword::打印)?;
        self.consume_delimiter(Delimiter::左括号)?;
        
        let mut args = Vec::new();
        if !self.check_delimiter(Delimiter::右括号) {
            loop {
                args.push(self.parse_expression()?);
                if !self.check_delimiter(Delimiter::逗号) {
                    break;
                }
                self.advance();
            }
        }
        
        self.consume_delimiter(Delimiter::右括号)?;
        
        Ok(Some(Statement::打印语句 { 参数: args }))
    }
    
    fn parse_import_statement(&mut self) -> ParserResult<Option<Statement>> {
        self.consume_keyword(Keyword::导入)?;
        let module_name = self.consume_identifier()?;
        
        let mut alias = None;
        if self.check_keyword(Keyword::为) || self.check_keyword(Keyword::是) {
            self.advance();
            alias = Some(self.consume_identifier()?);
        }
        
        Ok(Some(Statement::导入语句 {
            模块名: module_name,
            别名: alias,
        }))
    }
    
    fn parse_class_definition(&mut self) -> ParserResult<Option<Statement>> {
        self.consume_keyword(Keyword::类)?;
        let name = self.consume_identifier()?;
        
        let mut parent = None;
        if self.check_keyword(Keyword::继承) {
            self.advance();
            parent = Some(self.consume_identifier()?);
        }
        
        self.skip_newlines();
        
        let mut members = Vec::new();
        while !self.is_at_end() && !self.check_keyword(Keyword::结束) {
            self.skip_newlines();
            if self.check_keyword(Keyword::结束) {
                break;
            }
            
            let access_modifier = if self.check_keyword(Keyword::公有) {
                self.advance();
                AccessModifier::公有
            } else if self.check_keyword(Keyword::私有) {
                self.advance();
                AccessModifier::私有
            } else if self.check_keyword(Keyword::保护) {
                self.advance();
                AccessModifier::保护
            } else {
                AccessModifier::公有
            };
            
            if self.check_keyword(Keyword::定义) || self.check_keyword(Keyword::变量) || self.check_keyword(Keyword::属性) || self.check_keyword(Keyword::方法) {
                let is_method = self.check_keyword(Keyword::方法);
                if self.check_keyword(Keyword::定义) || self.check_keyword(Keyword::变量) || self.check_keyword(Keyword::属性) {
                    self.advance();
                } else if self.check_keyword(Keyword::方法) {
                    self.advance();
                }
                
                let member_name = self.consume_identifier()?;
                
                if self.check_keyword(Keyword::函数) || is_method {
                    let func_stmt = self.parse_function_definition(member_name.clone())?;
                    if let Some(Statement::函数定义 { 参数, 返回类型, 函数体, .. }) = func_stmt {
                        members.push(ClassMember::方法 {
                            名称: member_name,
                            参数,
                            返回类型,
                            函数体,
                            访问权限: access_modifier,
                        });
                    }
                } else {
                    let mut member_type = None;
                    if self.check_keyword(Keyword::为) || self.check_keyword(Keyword::是) {
                        self.advance();
                        member_type = Some(self.consume_identifier()?);
                    }
                    
                    let mut default_value = None;
                    if self.check_operator(Operator::赋值) {
                        self.advance();
                        default_value = Some(self.parse_expression()?);
                    }
                    
                    members.push(ClassMember::属性 {
                        名称: member_name,
                        类型: member_type,
                        默认值: default_value,
                        访问权限: access_modifier,
                    });
                }
            } else {
                self.advance();
            }
        }
        
        if self.check_keyword(Keyword::结束) {
            self.advance();
            if self.check_keyword(Keyword::类) {
                self.advance();
            }
        }
        
        Ok(Some(Statement::类定义 {
            名称: name,
            父类: parent,
            成员: members,
        }))
    }
    
    fn parse_try_statement(&mut self) -> ParserResult<Option<Statement>> {
        self.consume_keyword(Keyword::尝试)?;
        self.skip_newlines();
        let try_body = self.parse_block()?;
        
        let mut catch_branches = Vec::new();
        while self.check_keyword(Keyword::捕获) {
            self.advance();
            let error_var = self.consume_identifier()?;
            self.skip_newlines();
            let catch_body = self.parse_block()?;
            catch_branches.push((error_var, catch_body));
        }
        
        Ok(Some(Statement::尝试语句 {
            尝试体: try_body,
            捕获分支: catch_branches,
        }))
    }
    
    fn parse_throw_statement(&mut self) -> ParserResult<Option<Statement>> {
        self.consume_keyword(Keyword::抛出)?;
        let error = self.parse_expression()?;
        
        Ok(Some(Statement::抛出语句 { 错误: error }))
    }
    
    fn parse_with_statement(&mut self) -> ParserResult<Option<Statement>> {
        self.consume_keyword(Keyword::使用)?;
        let expr = self.parse_expression()?;
        
        let var_name = if self.check_keyword(Keyword::作为) {
            self.advance();
            Some(self.consume_identifier()?)
        } else {
            None
        };
        
        self.skip_newlines();
        self.consume_delimiter(Delimiter::左花括号)?;
        self.skip_newlines();
        
        let mut body = Vec::new();
        while !self.check_delimiter(Delimiter::右花括号) {
            if let Some(stmt) = self.parse_statement()? {
                body.push(stmt);
            }
            self.skip_newlines();
        }
        self.consume_delimiter(Delimiter::右花括号)?;
        
        Ok(Some(Statement::With语句 {
            表达式: expr,
            变量名: var_name,
            语句体: body,
        }))
    }
    
    fn parse_yield_statement(&mut self) -> ParserResult<Option<Statement>> {
        self.consume_keyword(Keyword::生成)?;
        
        let value = if self.check_delimiter(Delimiter::右花括号) {
            None
        } else {
            Some(self.parse_expression()?)
        };
        
        Ok(Some(Statement::Yield语句 { 值: value }))
    }
    
    fn parse_enum_definition(&mut self) -> ParserResult<Option<Statement>> {
        self.consume_keyword(Keyword::枚举)?;
        let name = self.consume_identifier()?;
        
        self.skip_newlines();
        self.consume_delimiter(Delimiter::左花括号)?;
        self.skip_newlines();
        
        let mut members = Vec::new();
        let mut index = 0i64;
        
        while !self.check_delimiter(Delimiter::右花括号) {
            let member_name = self.consume_identifier()?;
            
            let value = if self.check_operator(Operator::赋值) {
                self.advance();
                Some(self.parse_expression()?)
            } else {
                None
            };
            
            if value.is_none() {
                members.push((member_name, Some(Expression::整数 { 值: index })));
                index += 1;
            } else {
                members.push((member_name, value));
                index += 1;
            }
            
            self.skip_newlines();
            
            if self.check_delimiter(Delimiter::逗号) {
                self.advance();
                self.skip_newlines();
            }
        }
        self.consume_delimiter(Delimiter::右花括号)?;
        
        Ok(Some(Statement::枚举定义 {
            名称: name,
            成员: members,
        }))
    }
    
    fn parse_expression_statement(&mut self) -> ParserResult<Option<Statement>> {
        let expr = self.parse_expression()?;
        
        if self.check_operator(Operator::赋值) 
            || self.check_operator(Operator::加等于)
            || self.check_operator(Operator::减等于)
            || self.check_operator(Operator::乘等于)
            || self.check_operator(Operator::除等于) {
            
            let op = match &self.current_token().value {
                TokenValue::Operator(Operator::赋值) => None,
                TokenValue::Operator(Operator::加等于) => Some(BinaryOperator::加),
                TokenValue::Operator(Operator::减等于) => Some(BinaryOperator::减),
                TokenValue::Operator(Operator::乘等于) => Some(BinaryOperator::乘),
                TokenValue::Operator(Operator::除等于) => Some(BinaryOperator::除),
                _ => None,
            };
            
            self.advance();
            let value = self.parse_expression()?;
            
            let final_value = if let Some(bin_op) = op {
                Expression::二元运算 {
                    左: Box::new(expr.clone()),
                    运算符: bin_op,
                    右: Box::new(value),
                }
            } else {
                value
            };
            
            Ok(Some(Statement::赋值语句 {
                目标: expr,
                值: final_value,
            }))
        } else {
            Ok(Some(Statement::表达式语句 { 表达式: expr }))
        }
    }
    
    fn parse_expression(&mut self) -> ParserResult<Expression> {
        let condition = self.parse_or()?;
        
        if self.check_keyword(Keyword::则) {
            self.advance();
            let true_value = self.parse_or()?;
            
            self.consume_keyword(Keyword::否则)?;
            let false_value = self.parse_expression()?;
            
            return Ok(Expression::三元表达式 {
                条件: Box::new(condition),
                真值: Box::new(true_value),
                假值: Box::new(false_value),
            });
        }
        
        Ok(condition)
    }
    
    fn parse_or(&mut self) -> ParserResult<Expression> {
        let mut left = self.parse_and()?;
        
        while self.check_keyword(Keyword::或) {
            self.advance();
            let right = self.parse_and()?;
            left = Expression::二元运算 {
                左: Box::new(left),
                运算符: BinaryOperator::或,
                右: Box::new(right),
            };
        }
        
        Ok(left)
    }
    
    fn parse_and(&mut self) -> ParserResult<Expression> {
        let mut left = self.parse_equality()?;
        
        while self.check_keyword(Keyword::且) {
            self.advance();
            let right = self.parse_equality()?;
            left = Expression::二元运算 {
                左: Box::new(left),
                运算符: BinaryOperator::且,
                右: Box::new(right),
            };
        }
        
        Ok(left)
    }
    
    fn parse_equality(&mut self) -> ParserResult<Expression> {
        let mut left = self.parse_comparison()?;
        
        loop {
            let op = if self.check_operator(Operator::等于) {
                BinaryOperator::等于
            } else if self.check_operator(Operator::不等于) {
                BinaryOperator::不等于
            } else {
                break;
            };
            
            self.advance();
            let right = self.parse_comparison()?;
            left = Expression::二元运算 {
                左: Box::new(left),
                运算符: op,
                右: Box::new(right),
            };
        }
        
        Ok(left)
    }
    
    fn parse_comparison(&mut self) -> ParserResult<Expression> {
        let mut left = self.parse_term()?;
        
        loop {
            let op = if self.check_operator(Operator::大于) || self.check_keyword(Keyword::大于) {
                BinaryOperator::大于
            } else if self.check_operator(Operator::小于) || self.check_keyword(Keyword::小于) {
                BinaryOperator::小于
            } else if self.check_operator(Operator::大于等于) || self.check_keyword(Keyword::大于等于) {
                BinaryOperator::大于等于
            } else if self.check_operator(Operator::小于等于) || self.check_keyword(Keyword::小于等于) {
                BinaryOperator::小于等于
            } else if self.check_operator(Operator::等于) || self.check_keyword(Keyword::等于) {
                BinaryOperator::等于
            } else if self.check_operator(Operator::不等于) || self.check_keyword(Keyword::不等于) {
                BinaryOperator::不等于
            } else {
                break;
            };
            
            self.advance();
            let right = self.parse_term()?;
            left = Expression::二元运算 {
                左: Box::new(left),
                运算符: op,
                右: Box::new(right),
            };
        }
        
        Ok(left)
    }
    
    fn parse_term(&mut self) -> ParserResult<Expression> {
        let mut left = self.parse_factor()?;
        
        loop {
            let op = if self.check_operator(Operator::加) {
                BinaryOperator::加
            } else if self.check_operator(Operator::减) {
                BinaryOperator::减
            } else {
                break;
            };
            
            self.advance();
            let right = self.parse_factor()?;
            left = Expression::二元运算 {
                左: Box::new(left),
                运算符: op,
                右: Box::new(right),
            };
        }
        
        Ok(left)
    }
    
    fn parse_factor(&mut self) -> ParserResult<Expression> {
        let mut left = self.parse_unary()?;
        
        loop {
            let op = if self.check_operator(Operator::乘) {
                BinaryOperator::乘
            } else if self.check_operator(Operator::除) {
                BinaryOperator::除
            } else if self.check_operator(Operator::取余) {
                BinaryOperator::取余
            } else {
                break;
            };
            
            self.advance();
            let right = self.parse_unary()?;
            left = Expression::二元运算 {
                左: Box::new(left),
                运算符: op,
                右: Box::new(right),
            };
        }
        
        Ok(left)
    }
    
    fn parse_unary(&mut self) -> ParserResult<Expression> {
        if self.check_keyword(Keyword::非) {
            self.advance();
            let operand = self.parse_unary()?;
            return Ok(Expression::一元运算 {
                运算符: UnaryOperator::非,
                操作数: Box::new(operand),
            });
        }
        
        if self.check_operator(Operator::减) {
            self.advance();
            let operand = self.parse_unary()?;
            return Ok(Expression::一元运算 {
                运算符: UnaryOperator::负,
                操作数: Box::new(operand),
            });
        }
        
        self.parse_postfix()
    }
    
    fn parse_postfix(&mut self) -> ParserResult<Expression> {
        let mut expr = self.parse_primary()?;
        
        loop {
            if self.check_delimiter(Delimiter::左括号) {
                self.advance();
                let args = self.parse_arguments()?;
                self.consume_delimiter(Delimiter::右括号)?;
                
                expr = match expr {
                    Expression::标识符 { 名称 } => Expression::函数调用 {
                        函数名: 名称,
                        参数: args,
                    },
                    Expression::属性访问 { 对象, 属性名 } => Expression::方法调用 {
                        对象,
                        方法名: 属性名,
                        参数: args,
                    },
                    _ => {
                        return Err(ParserError::SyntaxError(
                            "缺少函数参数名".to_string(),
                            self.current_token().line,
                            self.current_token().column,
                        ));
                    }
                };
            } else if self.check_delimiter(Delimiter::点) {
                self.advance();
                let attr_name = self.consume_identifier()?;
                expr = Expression::属性访问 {
                    对象: Box::new(expr),
                    属性名: attr_name,
                };
            } else if self.check_delimiter(Delimiter::左方括号) {
                self.advance();
                
                let mut 开始 = None;
                let mut 结束 = None;
                let mut 步长 = None;
                let mut 为切片 = false;
                
                if self.check_delimiter(Delimiter::冒号) {
                    为切片 = true;
                    self.advance();
                } else {
                    let first = self.parse_slice_element()?;
                    
                    if self.check_delimiter(Delimiter::冒号) {
                        为切片 = true;
                        开始 = Some(Box::new(first));
                        self.advance();
                    } else {
                        self.consume_delimiter(Delimiter::右方括号)?;
                        expr = Expression::索引访问 {
                            对象: Box::new(expr),
                            索引: Box::new(first),
                        };
                        continue;
                    }
                }
                
                if 为切片 {
                    if !self.check_delimiter(Delimiter::冒号) && !self.check_delimiter(Delimiter::右方括号) {
                        结束 = Some(Box::new(self.parse_slice_element()?));
                    }
                    
                    if self.check_delimiter(Delimiter::冒号) {
                        self.advance();
                        if !self.check_delimiter(Delimiter::右方括号) {
                            步长 = Some(Box::new(self.parse_slice_element()?));
                        }
                    }
                    
                    self.consume_delimiter(Delimiter::右方括号)?;
                    expr = Expression::切片访问 {
                        对象: Box::new(expr),
                        开始,
                        结束,
                        步长,
                    };
                }
            } else {
                break;
            }
        }
        
        Ok(expr)
    }
    
    fn parse_slice_element(&mut self) -> ParserResult<Expression> {
        if self.check_operator(Operator::减) {
            self.advance();
            let operand = self.parse_primary()?;
            return Ok(Expression::一元运算 {
                运算符: UnaryOperator::负,
                操作数: Box::new(operand),
            });
        }
        self.parse_primary()
    }
    
    fn parse_primary(&mut self) -> ParserResult<Expression> {
        if self.current_token().token_type == TokenType::格式化字符串 {
            if let TokenValue::String(s) = &self.current_token().value {
                let s = s.clone();
                self.advance();
                return self.parse_f_string(&s);
            }
        }
        
        match &self.current_token().value.clone() {
            TokenValue::Integer(n) => {
                let n = *n;
                self.advance();
                Ok(Expression::整数 { 值: n })
            }
            TokenValue::Float(n) => {
                let n = *n;
                self.advance();
                Ok(Expression::浮点数 { 值: n })
            }
            TokenValue::String(s) => {
                let s = s.clone();
                self.advance();
                Ok(Expression::字符串 { 值: s })
            }
            TokenValue::Keyword(Keyword::真) => {
                self.advance();
                Ok(Expression::布尔值 { 值: true })
            }
            TokenValue::Keyword(Keyword::假) => {
                self.advance();
                Ok(Expression::布尔值 { 值: false })
            }
            TokenValue::Keyword(Keyword::空) => {
                self.advance();
                Ok(Expression::空值)
            }
            TokenValue::Keyword(Keyword::新建) => {
                self.advance();
                let class_name = self.consume_identifier()?;
                self.consume_delimiter(Delimiter::左括号)?;
                let args = self.parse_arguments()?;
                self.consume_delimiter(Delimiter::右括号)?;
                Ok(Expression::新建对象 {
                    类名: class_name,
                    参数: args,
                })
            }
            TokenValue::Keyword(Keyword::函数) => {
                self.advance();
                
                let (params, vararg_name) = if self.check_delimiter(Delimiter::左括号) {
                    self.consume_delimiter(Delimiter::左括号)?;
                    let result = self.parse_parameters()?;
                    self.consume_delimiter(Delimiter::右括号)?;
                    result
                } else {
                    (vec![], None)
                };
                
                let body = if self.check_delimiter(Delimiter::左花括号) {
                    self.advance();
                    let stmts = self.parse_block()?;
                    self.consume_delimiter(Delimiter::右花括号)?;
                    stmts
                } else {
                    self.parse_block()?
                };
                
                Ok(Expression::匿名函数 {
                    参数: params,
                    可变参数名: vararg_name,
                    函数体: body,
                })
            }
            TokenValue::Identifier(name) => {
                let name = name.clone();
                self.advance();
                Ok(Expression::标识符 { 名称: name })
            }
            TokenValue::Delimiter(Delimiter::左括号) => {
                self.advance();
                let expr = self.parse_expression()?;
                self.consume_delimiter(Delimiter::右括号)?;
                Ok(expr)
            }
            TokenValue::Delimiter(Delimiter::左方括号) => {
                self.advance();
                
                if self.check_delimiter(Delimiter::右方括号) {
                    self.consume_delimiter(Delimiter::右方括号)?;
                    return Ok(Expression::列表 { 元素: vec![] });
                }
                
                let first_expr = self.parse_expression()?;
                
                if self.check_keyword(Keyword::对于) {
                    self.advance();
                    
                    let var_name = self.consume_identifier()?;
                    
                    self.consume_keyword(Keyword::在)?;
                    let iterable = self.parse_expression()?;
                    
                    let condition = if self.check_keyword(Keyword::如果) {
                        self.advance();
                        Some(Box::new(self.parse_expression()?))
                    } else {
                        None
                    };
                    
                    self.consume_delimiter(Delimiter::右方括号)?;
                    
                    Ok(Expression::列表推导式 {
                        表达式: Box::new(first_expr),
                        变量: var_name,
                        可迭代对象: Box::new(iterable),
                        条件: condition,
                    })
                } else {
                    let mut elements = vec![first_expr];
                    
                    while self.check_delimiter(Delimiter::逗号) {
                        self.advance();
                        elements.push(self.parse_expression()?);
                    }
                    
                    self.consume_delimiter(Delimiter::右方括号)?;
                    Ok(Expression::列表 { 元素: elements })
                }
            }
            TokenValue::Delimiter(Delimiter::左花括号) => {
                self.advance();
                
                if self.check_delimiter(Delimiter::右花括号) {
                    self.consume_delimiter(Delimiter::右花括号)?;
                    return Ok(Expression::字典 { 键值对: vec![] });
                }
                
                let first_key = self.parse_expression()?;
                
                if self.check_delimiter(Delimiter::冒号) {
                    self.consume_delimiter(Delimiter::冒号)?;
                    let first_value = self.parse_expression()?;
                    
                    if self.check_keyword(Keyword::对于) {
                        self.advance();
                        let var_name = self.consume_identifier()?;
                        self.consume_keyword(Keyword::在)?;
                        let iterable = self.parse_expression()?;
                        
                        let condition = if self.check_keyword(Keyword::如果) {
                            self.advance();
                            Some(Box::new(self.parse_expression()?))
                        } else {
                            None
                        };
                        
                        self.consume_delimiter(Delimiter::右花括号)?;
                        
                        return Ok(Expression::字典推导式 {
                            键表达式: Box::new(first_key),
                            值表达式: Box::new(first_value),
                            变量: var_name,
                            可迭代对象: Box::new(iterable),
                            条件: condition,
                        });
                    }
                    
                    let mut pairs = vec![(first_key, first_value)];
                    while self.check_delimiter(Delimiter::逗号) {
                        self.advance();
                        let key = self.parse_expression()?;
                        self.consume_delimiter(Delimiter::冒号)?;
                        let value = self.parse_expression()?;
                        pairs.push((key, value));
                    }
                    
                    self.consume_delimiter(Delimiter::右花括号)?;
                    Ok(Expression::字典 { 键值对: pairs })
                } else if self.check_keyword(Keyword::对于) {
                    self.advance();
                    let var_name = self.consume_identifier()?;
                    self.consume_keyword(Keyword::在)?;
                    let iterable = self.parse_expression()?;
                    
                    let condition = if self.check_keyword(Keyword::如果) {
                        self.advance();
                        Some(Box::new(self.parse_expression()?))
                    } else {
                        None
                    };
                    
                    self.consume_delimiter(Delimiter::右花括号)?;
                    
                    Ok(Expression::集合推导式 {
                        表达式: Box::new(first_key),
                        变量: var_name,
                        可迭代对象: Box::new(iterable),
                        条件: condition,
                    })
                } else {
                    let mut elements = vec![first_key];
                    while self.check_delimiter(Delimiter::逗号) {
                        self.advance();
                        elements.push(self.parse_expression()?);
                    }
                    
                    self.consume_delimiter(Delimiter::右花括号)?;
                    Ok(Expression::集合 { 元素: elements })
                }
            }
            _ => Err(ParserError::UnexpectedToken {
                expected: "类型名称或等号".to_string(),
                actual: self.current_token().to_string(),
                line: self.current_token().line,
                column: self.current_token().column,
            }),
        }
    }
    
    fn parse_f_string(&mut self, content: &str) -> ParserResult<Expression> {
        let mut parts = Vec::new();
        let mut current_text = String::new();
        let mut chars = content.chars().peekable();
        
        while let Some(ch) = chars.next() {
            if ch == '{' {
                if !current_text.is_empty() {
                    parts.push(FormatPart::文本(current_text.clone()));
                    current_text.clear();
                }
                
                let mut expr_content = String::new();
                let mut brace_count = 1;
                
                while brace_count > 0 {
                    if let Some(c) = chars.next() {
                        if c == '{' {
                            brace_count += 1;
                            expr_content.push(c);
                        } else if c == '}' {
                            brace_count -= 1;
                            if brace_count > 0 {
                                expr_content.push(c);
                            }
                        } else {
                            expr_content.push(c);
                        }
                    } else {
                        break;
                    }
                }
                
                if !expr_content.is_empty() {
                    let expr_tokens = crate::lexer::tokenize(&expr_content)
                        .map_err(|e| ParserError::SyntaxError(
                            format!("格式化字符串表达式解析失败: {}", e),
                            self.current_token().line,
                            self.current_token().column,
                        ))?;
                    
                    let mut expr_parser = Parser::new(expr_tokens);
                    let expr = expr_parser.parse_expression()?;
                    parts.push(FormatPart::表达式(expr));
                }
            } else if ch == '}' {
                current_text.push(ch);
            } else {
                current_text.push(ch);
            }
        }
        
        if !current_text.is_empty() {
            parts.push(FormatPart::文本(current_text));
        }
        
        Ok(Expression::格式化字符串 { 部分: parts })
    }
    
    fn parse_arguments(&mut self) -> ParserResult<Vec<Expression>> {
        let mut args = Vec::new();
        
        if self.check_delimiter(Delimiter::右括号) {
            return Ok(args);
        }
        
        loop {
            args.push(self.parse_expression()?);
            if !self.check_delimiter(Delimiter::逗号) {
                break;
            }
            self.advance();
        }
        
        Ok(args)
    }
    
    fn parse_list_elements(&mut self) -> ParserResult<Vec<Expression>> {
        let mut elements = Vec::new();
        
        if self.check_delimiter(Delimiter::右方括号) {
            return Ok(elements);
        }
        
        loop {
            elements.push(self.parse_expression()?);
            if !self.check_delimiter(Delimiter::逗号) {
                break;
            }
            self.advance();
        }
        
        Ok(elements)
    }
    
    fn parse_dict_elements(&mut self) -> ParserResult<Vec<(Expression, Expression)>> {
        let mut pairs = Vec::new();
        
        if self.check_delimiter(Delimiter::右花括号) {
            return Ok(pairs);
        }
        
        loop {
            let key = self.parse_expression()?;
            self.consume_delimiter(Delimiter::冒号)?;
            let value = self.parse_expression()?;
            pairs.push((key, value));
            
            if !self.check_delimiter(Delimiter::逗号) {
                break;
            }
            self.advance();
        }
        
        Ok(pairs)
    }
}

pub fn parse(tokens: Vec<Token>) -> ParserResult<Program> {
    let mut parser = Parser::new(tokens);
    parser.parse()
}
