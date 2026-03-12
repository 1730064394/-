#[allow(dead_code)]
pub struct Editor {
    content: String,
    cursor_position: usize,
    selection_start: Option<usize>,
}

#[allow(dead_code)]
impl Editor {
    pub fn new() -> Self {
        Editor {
            content: String::new(),
            cursor_position: 0,
            selection_start: None,
        }
    }
    
    pub fn set_content(&mut self, content: &str) {
        self.content = content.to_string();
        self.cursor_position = self.content.len();
    }
    
    pub fn get_content(&self) -> &str {
        &self.content
    }
    
    pub fn insert_char(&mut self, c: char) {
        self.content.insert(self.cursor_position, c);
        self.cursor_position += c.len_utf8();
    }
    
    pub fn insert_str(&mut self, s: &str) {
        self.content.insert_str(self.cursor_position, s);
        self.cursor_position += s.len();
    }
    
    pub fn delete_char(&mut self) {
        if self.cursor_position < self.content.len() {
            self.content.remove(self.cursor_position);
        }
    }
    
    pub fn backspace(&mut self) {
        if self.cursor_position > 0 {
            self.cursor_position -= 1;
            self.content.remove(self.cursor_position);
        }
    }
    
    pub fn move_cursor_left(&mut self) {
        if self.cursor_position > 0 {
            self.cursor_position -= 1;
        }
    }
    
    pub fn move_cursor_right(&mut self) {
        if self.cursor_position < self.content.len() {
            self.cursor_position += 1;
        }
    }
    
    pub fn move_cursor_to_start(&mut self) {
        self.cursor_position = 0;
    }
    
    pub fn move_cursor_to_end(&mut self) {
        self.cursor_position = self.content.len();
    }
    
    pub fn clear(&mut self) {
        self.content.clear();
        self.cursor_position = 0;
    }
}

impl Default for Editor {
    fn default() -> Self {
        Self::new()
    }
}
