
// Terminal Command
#[derive(Clone)]
pub struct TerminalCommand {
    command_buffer: String,
    position: usize,
}

impl Default for TerminalCommand {
    fn default() -> Self {
        TerminalCommand {
            command_buffer: String::new(),
            position: 0,
        }
    }
}

impl TerminalCommand {
    pub fn flush(&mut self) {
        self.command_buffer.clear();
        self.position = 0;
    }

    pub fn remove(&mut self) {
        if self.position > 0 {
            self.position -= 1;
            self.command_buffer.pop();
        }
    }

    pub fn move_cursor_forward(&mut self) {
        if self.position < self.command_buffer.len() {
            self.position += 1;
        }
    }

    pub fn move_cursor_backward(&mut self) {
        if self.position > 0 {
            self.position -= 1;
        }
    }

    pub fn add(&mut self, char: char) {
        self.command_buffer.push(char);
    }

    pub fn get_position(&self) -> usize {
        self.position
    }

    pub fn set_position(&mut self, position: usize) {
        self.position = position;
    }

    pub fn get_buffer(&self) -> &String {
        &self.command_buffer
    }

    pub fn set_buffer(&mut self, buffer: String) {
        self.command_buffer = buffer;
    }
}