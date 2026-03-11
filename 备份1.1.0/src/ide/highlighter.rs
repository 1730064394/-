use std::collections::HashSet;

#[allow(dead_code)]
pub struct Highlighter {
    keywords: HashSet<String>,
    operators: HashSet<String>,
}

#[allow(dead_code)]
impl Highlighter {
    pub fn new() -> Self {
        let keywords = [
            "定义", "变量", "函数", "返回", "如果", "否则如果", "否则",
            "循环", "当", "对于", "在", "结束", "为", "是", "真", "假",
            "空", "且", "或", "非", "打印", "输入", "导入", "从",
            "尝试", "捕获", "抛出", "类", "继承", "新建", "自",
        ].iter().map(|s| s.to_string()).collect();
        
        let operators = [
            "＋", "－", "×", "÷", "％", "＝", "＞", "＜", "！",
            "＋＝", "－＝", "×＝", "÷＝", "＝＝", "！＝", "＞＝", "＜＝",
            "+", "-", "*", "/", "%", "=", ">", "<", "!",
            "+=", "-=", "*=", "/=", "==", "!=", ">=", "<=",
        ].iter().map(|s| s.to_string()).collect();
        
        Highlighter { keywords, operators }
    }
    
    pub fn is_keyword(&self, word: &str) -> bool {
        self.keywords.contains(word)
    }
    
    pub fn is_operator(&self, op: &str) -> bool {
        self.operators.contains(op)
    }
    
    pub fn highlight_line(&self, line: &str) -> Vec<HighlightedSegment> {
        let mut segments = Vec::new();
        let mut current_pos = 0;
        let chars: Vec<char> = line.chars().collect();
        
        while current_pos < chars.len() {
            let remaining: String = chars[current_pos..].iter().collect();
            
            if remaining.starts_with("注释：") {
                segments.push(HighlightedSegment {
                    text: line[current_pos..].to_string(),
                    style: HighlightStyle::Comment,
                });
                break;
            }
            
            if remaining.starts_with("「") {
                if let Some(end_pos) = remaining.find("」") {
                    segments.push(HighlightedSegment {
                        text: remaining[..=end_pos].to_string(),
                        style: HighlightStyle::String,
                    });
                    current_pos += end_pos + 1;
                    continue;
                }
            }
            
            let mut found_keyword = false;
            for kw in &self.keywords {
                if remaining.starts_with(kw) {
                    let next_pos = kw.chars().count();
                    if current_pos + next_pos >= chars.len() 
                        || !chars[current_pos + next_pos].is_alphanumeric() {
                        segments.push(HighlightedSegment {
                            text: kw.clone(),
                            style: HighlightStyle::Keyword,
                        });
                        current_pos += next_pos;
                        found_keyword = true;
                        break;
                    }
                }
            }
            
            if found_keyword {
                continue;
            }
            
            let mut found_operator = false;
            for op in &self.operators {
                if remaining.starts_with(op) {
                    segments.push(HighlightedSegment {
                        text: op.clone(),
                        style: HighlightStyle::Operator,
                    });
                    current_pos += op.chars().count();
                    found_operator = true;
                    break;
                }
            }
            
            if found_operator {
                continue;
            }
            
            let ch = chars[current_pos];
            if ch.is_ascii_digit() {
                let mut num_end = current_pos;
                while num_end < chars.len() 
                    && (chars[num_end].is_ascii_digit() || chars[num_end] == '.') {
                    num_end += 1;
                }
                segments.push(HighlightedSegment {
                    text: chars[current_pos..num_end].iter().collect(),
                    style: HighlightStyle::Number,
                });
                current_pos = num_end;
                continue;
            }
            
            segments.push(HighlightedSegment {
                text: ch.to_string(),
                style: HighlightStyle::Normal,
            });
            current_pos += 1;
        }
        
        segments
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(dead_code)]
pub enum HighlightStyle {
    Normal,
    Keyword,
    String,
    Number,
    Comment,
    Operator,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct HighlightedSegment {
    pub text: String,
    pub style: HighlightStyle,
}

impl Default for Highlighter {
    fn default() -> Self {
        Self::new()
    }
}
