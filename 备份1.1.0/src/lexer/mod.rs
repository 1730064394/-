mod token;

pub use token::*;

use crate::error::{LexerError, LexerResult};
use unicode_segmentation::UnicodeSegmentation;

pub struct Lexer {
    #[allow(dead_code)]
    source: String,
    chars: Vec<String>,
    pos: usize,
    line: usize,
    column: usize,
    indent_stack: Vec<usize>,
    #[allow(dead_code)]
    pending_tokens: Vec<Token>,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        let chars: Vec<String> = source
            .graphemes(true)
            .map(|s| s.to_string())
            .collect();
        
        Lexer {
            source: source.to_string(),
            chars,
            pos: 0,
            line: 1,
            column: 1,
            indent_stack: vec![0],
            pending_tokens: Vec::new(),
        }
    }
    
    pub fn tokenize(&mut self) -> LexerResult<Vec<Token>> {
        let mut tokens = Vec::new();
        
        while self.pos < self.chars.len() {
            self.handle_indentation(&mut tokens)?;
            
            if self.pos >= self.chars.len() {
                break;
            }
            
            let ch = self.current_char().to_string();
            
            if ch == "　" || ch == " " || ch == "\t" {
                self.advance();
                continue;
            }
            
            if ch == "\n" || ch == "\r" {
                tokens.push(Token::new(
                    TokenType::换行,
                    TokenValue::Newline,
                    self.line,
                    self.column,
                ));
                self.advance();
                if ch == "\r" && self.peek_char() == "\n" {
                    self.advance();
                }
                continue;
            }
            
            if self.is_comment_start() {
                self.skip_comment(&mut tokens)?;
                continue;
            }
            
            if let Some(token) = self.try_parse_keyword()? {
                tokens.push(token);
                continue;
            }
            
            if let Some(token) = self.try_parse_operator()? {
                tokens.push(token);
                continue;
            }
            
            if let Some(token) = self.try_parse_delimiter()? {
                tokens.push(token);
                continue;
            }
            
            if let Some(token) = self.try_parse_string()? {
                tokens.push(token);
                continue;
            }
            
            if let Some(token) = self.try_parse_number()? {
                tokens.push(token);
                continue;
            }
            
            if let Some(token) = self.try_parse_identifier()? {
                tokens.push(token);
                continue;
            }
            
            return Err(LexerError::UnknownCharacter(
                self.chars[self.pos].chars().next().unwrap_or('？'),
                self.line,
                self.column,
            ));
        }
        
        while self.indent_stack.len() > 1 {
            self.indent_stack.pop();
            tokens.push(Token::new(
                TokenType::缩进,
                TokenValue::Indent(0),
                self.line,
                self.column,
            ));
        }
        
        tokens.push(Token::eof(self.line, self.column));
        Ok(tokens)
    }
    
    fn current_char(&self) -> &str {
        if self.pos < self.chars.len() {
            &self.chars[self.pos]
        } else {
            ""
        }
    }
    
    fn peek_char(&self) -> &str {
        if self.pos + 1 < self.chars.len() {
            &self.chars[self.pos + 1]
        } else {
            ""
        }
    }
    
    fn advance(&mut self) {
        if self.pos < self.chars.len() {
            if self.chars[self.pos] == "\n" {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
            self.pos += 1;
        }
    }
    
    fn advance_by(&mut self, n: usize) {
        for _ in 0..n {
            self.advance();
        }
    }
    
    fn handle_indentation(&mut self, tokens: &mut Vec<Token>) -> LexerResult<()> {
        if self.column != 1 {
            return Ok(());
        }
        
        let mut indent = 0;
        
        while self.pos < self.chars.len() {
            let ch = self.current_char();
            if ch == "　" {
                indent += 2;
                self.advance();
            } else if ch == " " {
                indent += 1;
                self.advance();
            } else if ch == "\t" {
                indent += 4;
                self.advance();
            } else {
                break;
            }
        }
        
        if self.pos >= self.chars.len() || self.current_char() == "\n" || self.current_char() == "\r" {
            return Ok(());
        }
        
        let current_indent = *self.indent_stack.last().unwrap_or(&0);
        
        if indent > current_indent {
            self.indent_stack.push(indent);
            tokens.push(Token::new(
                TokenType::缩进,
                TokenValue::Indent(indent),
                self.line,
                self.column,
            ));
        } else if indent < current_indent {
            while let Some(&top) = self.indent_stack.last() {
                if top > indent {
                    self.indent_stack.pop();
                    tokens.push(Token::new(
                        TokenType::缩进,
                        TokenValue::Indent(0),
                        self.line,
                        self.column,
                    ));
                } else {
                    break;
                }
            }
        }
        
        Ok(())
    }
    
    fn is_comment_start(&self) -> bool {
        // 支持 "注释：" 格式
        let is_zhu = self.current_char() == "注";
        let is_shi = self.peek_ahead(1) == "释";
        let is_colon = self.peek_ahead(2) == "：" || self.peek_ahead(2) == ":";
        let is_zhushi = is_zhu && is_shi && is_colon;
        
        // 支持 "//" 格式
        let is_slash1 = self.current_char() == "/";
        let is_slash2 = self.peek_ahead(1) == "/";
        let is_double_slash = is_slash1 && is_slash2;
        
        is_zhushi || is_double_slash
    }
    
    fn peek_ahead(&self, n: usize) -> &str {
        if self.pos + n < self.chars.len() {
            &self.chars[self.pos + n]
        } else {
            ""
        }
    }
    
    fn skip_comment(&mut self, tokens: &mut Vec<Token>) -> LexerResult<()> {
        let start_col = self.column;
        let mut comment = String::new();
        
        // 检查是哪种注释格式
        let is_double_slash = self.current_char() == "/" && self.peek_ahead(1) == "/";
        
        if is_double_slash {
            // 跳过 "//"
            self.advance_by(2);
            comment.push_str("//");
        } else {
            // 跳过 "注释："
            self.advance_by(3);
            comment.push_str("注释：");
        }
        
        while self.pos < self.chars.len() {
            let ch = self.current_char();
            if ch == "\n" || ch == "\r" {
                break;
            }
            comment.push_str(ch);
            self.advance();
        }
        
        tokens.push(Token::new(
            TokenType::注释,
            TokenValue::Comment(comment),
            self.line,
            start_col,
        ));
        
        Ok(())
    }
    
    fn try_parse_keyword(&mut self) -> LexerResult<Option<Token>> {
        let keywords = [
            ("定义", Keyword::定义),
            ("变量", Keyword::变量),
            ("函数", Keyword::函数),
            ("返回", Keyword::返回),
            ("如果", Keyword::如果),
            ("否则如果", Keyword::否则如果),
            ("否则", Keyword::否则),
            ("则", Keyword::则),
            ("循环", Keyword::循环),
            ("当", Keyword::当),
            ("对于", Keyword::对于),
            ("在", Keyword::在),
            ("结束", Keyword::结束),
            ("为", Keyword::为),
            ("是", Keyword::是),
            ("真", Keyword::真),
            ("假", Keyword::假),
            ("空", Keyword::空),
            ("且", Keyword::且),
            ("或", Keyword::或),
            ("非", Keyword::非),
            ("导入", Keyword::导入),
            ("从", Keyword::从),
            ("尝试", Keyword::尝试),
            ("捕获", Keyword::捕获),
            ("抛出", Keyword::抛出),
            ("类", Keyword::类),
            ("继承", Keyword::继承),
            ("新建", Keyword::新建),
            ("自", Keyword::自),
            ("打印", Keyword::打印),
            ("输入", Keyword::输入),
            ("大于", Keyword::大于),
            ("小于", Keyword::小于),
            ("等于", Keyword::等于),
            ("不等于", Keyword::不等于),
            ("大于等于", Keyword::大于等于),
            ("小于等于", Keyword::小于等于),
            ("公有", Keyword::公有),
            ("私有", Keyword::私有),
            ("保护", Keyword::保护),
            ("属性", Keyword::属性),
            ("方法", Keyword::方法),
            ("使用", Keyword::使用),
            ("作为", Keyword::作为),
            ("生成", Keyword::生成),
            ("枚举", Keyword::枚举),
        ];
        
        for (keyword, kw_type) in keywords.iter() {
            if self.match_string(keyword) {
                let token = Token::new(
                    TokenType::关键字,
                    TokenValue::Keyword(kw_type.clone()),
                    self.line,
                    self.column,
                );
                self.advance_by(keyword.graphemes(true).count());
                return Ok(Some(token));
            }
        }
        
        Ok(None)
    }
    
    fn match_string(&self, s: &str) -> bool {
        let s_chars: Vec<&str> = s.graphemes(true).collect();
        for (i, c) in s_chars.iter().enumerate() {
            if self.pos + i >= self.chars.len() || &self.chars[self.pos + i] != *c {
                return false;
            }
        }
        true
    }
    
    fn try_parse_operator(&mut self) -> LexerResult<Option<Token>> {
        let operators = [
            ("＋＝", Operator::加等于),
            ("－＝", Operator::减等于),
            ("×＝", Operator::乘等于),
            ("÷＝", Operator::除等于),
            ("＝＝", Operator::等于),
            ("！＝", Operator::不等于),
            ("＞＝", Operator::大于等于),
            ("＜＝", Operator::小于等于),
            ("＋", Operator::加),
            ("－", Operator::减),
            ("×", Operator::乘),
            ("÷", Operator::除),
            ("％", Operator::取余),
            ("＝", Operator::赋值),
            ("＞", Operator::大于),
            ("＜", Operator::小于),
            ("+", Operator::加),
            ("-", Operator::减),
            ("*", Operator::乘),
            ("/", Operator::除),
            ("%", Operator::取余),
            ("==", Operator::等于),
            ("!=", Operator::不等于),
            (">=", Operator::大于等于),
            ("<=", Operator::小于等于),
            (">", Operator::大于),
            ("<", Operator::小于),
            ("=", Operator::赋值),
            ("+=", Operator::加等于),
            ("-=", Operator::减等于),
            ("*=", Operator::乘等于),
            ("/=", Operator::除等于),
        ];
        
        for (op_str, op_type) in operators.iter() {
            if self.match_string(op_str) {
                let token = Token::new(
                    TokenType::运算符,
                    TokenValue::Operator(op_type.clone()),
                    self.line,
                    self.column,
                );
                self.advance_by(op_str.graphemes(true).count());
                return Ok(Some(token));
            }
        }
        
        Ok(None)
    }
    
    fn try_parse_delimiter(&mut self) -> LexerResult<Option<Token>> {
        let delimiters = [
            ("（", Delimiter::左括号),
            ("）", Delimiter::右括号),
            ("［", Delimiter::左方括号),
            ("］", Delimiter::右方括号),
            ("【", Delimiter::左方括号),
            ("】", Delimiter::右方括号),
            ("｛", Delimiter::左花括号),
            ("｝", Delimiter::右花括号),
            ("，", Delimiter::逗号),
            ("：", Delimiter::冒号),
            ("；", Delimiter::分号),
            ("．", Delimiter::点),
            ("·", Delimiter::点),
            ("(", Delimiter::左括号),
            (")", Delimiter::右括号),
            ("[", Delimiter::左方括号),
            ("]", Delimiter::右方括号),
            ("{", Delimiter::左花括号),
            ("}", Delimiter::右花括号),
            (",", Delimiter::逗号),
            (":", Delimiter::冒号),
            (";", Delimiter::分号),
            (".", Delimiter::点),
        ];
        
        for (del_str, del_type) in delimiters.iter() {
            if self.match_string(del_str) {
                let token = Token::new(
                    TokenType::分隔符,
                    TokenValue::Delimiter(del_type.clone()),
                    self.line,
                    self.column,
                );
                self.advance_by(del_str.graphemes(true).count());
                return Ok(Some(token));
            }
        }
        
        Ok(None)
    }
    
    fn try_parse_string(&mut self) -> LexerResult<Option<Token>> {
        let quote_chars = ["「", "」", "\"", "'"];
        
        let is_f_string = self.current_char() == "f" || self.current_char() == "格式";
        let offset = if is_f_string { 1 } else { 0 };
        
        let quote_char = if is_f_string {
            if self.pos + 1 < self.chars.len() {
                self.chars[self.pos + 1].as_str()
            } else {
                return Ok(None);
            }
        } else {
            self.current_char()
        };
        
        let start_quote = if quote_chars.contains(&quote_char) {
            quote_char.to_string()
        } else {
            return Ok(None);
        };
        
        let start_line = self.line;
        let start_col = self.column;
        
        if is_f_string {
            self.advance();
        }
        self.advance();
        
        let mut string_value = String::new();
        let end_quote = if start_quote == "「" { "」" } else { &start_quote };
        
        while self.pos < self.chars.len() {
            let ch = self.current_char();
            
            if ch == end_quote {
                self.advance();
                if is_f_string {
                    return Ok(Some(Token::new(
                        TokenType::格式化字符串,
                        TokenValue::String(string_value),
                        start_line,
                        start_col,
                    )));
                }
                return Ok(Some(Token::new(
                    TokenType::字符串,
                    TokenValue::String(string_value),
                    start_line,
                    start_col,
                )));
            }
            
            if ch == "\\" {
                self.advance();
                if self.pos < self.chars.len() {
                    let escaped = self.current_char();
                    match escaped {
                        "n" => string_value.push('\n'),
                        "t" => string_value.push('\t'),
                        "r" => string_value.push('\r'),
                        "\\" => string_value.push('\\'),
                        "\"" => string_value.push('"'),
                        "'" => string_value.push('\''),
                        _ => {
                            string_value.push('\\');
                            string_value.push_str(escaped);
                        }
                    }
                    self.advance();
                }
            } else {
                string_value.push_str(ch);
                self.advance();
            }
        }
        
        Err(LexerError::UnclosedString(start_line, start_col))
    }
    
    fn try_parse_number(&mut self) -> LexerResult<Option<Token>> {
        let ch = self.current_char();
        let is_digit = ch.chars().all(|c| c.is_ascii_digit());
        let is_chinese_num = ["零", "一", "二", "三", "四", "五", "六", "七", "八", "九", "十", "百", "千", "万", "亿"].contains(&ch);
        
        if !is_digit && !is_chinese_num && ch != "." && ch != "．" && ch != "点" {
            return Ok(None);
        }
        
        let start_col = self.column;
        let mut num_str = String::new();
        let mut has_dot = false;
        
        if is_chinese_num {
            while self.pos < self.chars.len() {
                let ch = self.current_char();
                if ["零", "一", "二", "三", "四", "五", "六", "七", "八", "九", "十", "百", "千", "万", "亿"].contains(&ch) {
                    num_str.push_str(ch);
                    self.advance();
                } else if ch == "点" && !has_dot {
                    num_str.push('.');
                    has_dot = true;
                    self.advance();
                } else {
                    break;
                }
            }
            
            let value = self.chinese_number_to_arabic(&num_str);
            if has_dot {
                return Ok(Some(Token::new(
                    TokenType::浮点数,
                    TokenValue::Float(value),
                    self.line,
                    start_col,
                )));
            } else {
                return Ok(Some(Token::new(
                    TokenType::整数,
                    TokenValue::Integer(value as i64),
                    self.line,
                    start_col,
                )));
            }
        }
        
        while self.pos < self.chars.len() {
            let ch = self.current_char();
            if ch.chars().all(|c| c.is_ascii_digit()) {
                num_str.push_str(ch);
                self.advance();
            } else if (ch == "." || ch == "．") && !has_dot {
                num_str.push('.');
                has_dot = true;
                self.advance();
            } else {
                break;
            }
        }
        
        if num_str.is_empty() || num_str == "." {
            return Ok(None);
        }
        
        if has_dot {
            match num_str.parse::<f64>() {
                Ok(value) => Ok(Some(Token::new(
                    TokenType::浮点数,
                    TokenValue::Float(value),
                    self.line,
                    start_col,
                ))),
                Err(_) => Err(LexerError::InvalidNumber(num_str, self.line, start_col)),
            }
        } else {
            match num_str.parse::<i64>() {
                Ok(value) => Ok(Some(Token::new(
                    TokenType::整数,
                    TokenValue::Integer(value),
                    self.line,
                    start_col,
                ))),
                Err(_) => Err(LexerError::InvalidNumber(num_str, self.line, start_col)),
            }
        }
    }
    
    fn chinese_number_to_arabic(&self, chinese: &str) -> f64 {
        let digit_map = [
            ("零", 0), ("一", 1), ("二", 2), ("三", 3), ("四", 4),
            ("五", 5), ("六", 6), ("七", 7), ("八", 8), ("九", 9),
        ];
        
        let unit_map = [
            ("十", 10), ("百", 100), ("千", 1000), ("万", 10000), ("亿", 100000000),
        ];
        
        let mut result: f64 = 0.0;
        let mut temp: f64 = 0.0;
        
        for g in chinese.graphemes(true) {
            if let Some((_, val)) = digit_map.iter().find(|(c, _)| *c == g) {
                temp = *val as f64;
            } else if let Some((_, val)) = unit_map.iter().find(|(c, _)| *c == g) {
                if temp == 0.0 {
                    temp = 1.0;
                }
                result += temp * (*val as f64);
                temp = 0.0;
            }
        }
        
        result + temp
    }
    
    fn try_parse_identifier(&mut self) -> LexerResult<Option<Token>> {
        let ch = self.current_char();
        
        if ch.is_empty() || ch == " " || ch == "\t" || ch == "\n" || ch == "\r" {
            return Ok(None);
        }
        
        let is_delimiter = ["（", "）", "［", "］", "【", "】", "｛", "｝", "，", "：", "；", "．", "·",
            "(", ")", "[", "]", "{", "}", ",", ":", ";"].contains(&ch);
        let is_operator = ["＋", "－", "×", "÷", "％", "＝", "＞", "＜", "！",
            "+", "-", "*", "/", "%", "=", ">", "<", "!"].contains(&ch);
        
        if is_delimiter || is_operator {
            return Ok(None);
        }
        
        let start_col = self.column;
        let mut identifier = String::new();
        
        while self.pos < self.chars.len() {
            let ch = self.current_char();
            
            if ch == " " || ch == "\t" || ch == "\n" || ch == "\r" {
                break;
            }
            
            let is_delimiter = ["（", "）", "［", "］", "【", "】", "｛", "｝", "，", "：", "；", "．", "·",
                "(", ")", "[", "]", "{", "}", ",", ":", ";", ""].contains(&ch);
            let is_operator = ["＋", "－", "×", "÷", "％", "＝", "＞", "＜", "！",
                "+", "-", "*", "/", "%", "=", ">", "<", "!"].contains(&ch);
            
            if is_delimiter || is_operator {
                break;
            }
            
            identifier.push_str(ch);
            self.advance();
        }
        
        if identifier.is_empty() {
            return Ok(None);
        }
        
        Ok(Some(Token::new(
            TokenType::标识符,
            TokenValue::Identifier(identifier),
            self.line,
            start_col,
        )))
    }
}

pub fn tokenize(source: &str) -> LexerResult<Vec<Token>> {
    let mut lexer = Lexer::new(source);
    lexer.tokenize()
}
