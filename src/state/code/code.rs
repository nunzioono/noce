use std::{fmt::{self}, ops::Add};



#[derive(Clone, Default, Debug, PartialEq, Eq)]
pub struct Line {
    number: usize,
    line: String,
}

impl Line {
    pub fn new(number: usize, line: String) -> Line {
        Line { number, line }
    }

    pub fn set_number(&mut self, number: usize) {
        self.number = number;
    }

    pub fn get_number(&self) -> usize {
        self.number
    }

    pub fn set_string(&mut self, line: String) {
        self.line = line;
    }

    pub fn get_string(&self) -> String {
        self.line.clone()
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Code {
    content: Vec<Line>,
    cursor_displayed: bool,
    x: usize,
    y: usize
}

impl fmt::Display for Code {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in &self.content {
            writeln!(f, "{}: {}", line.number, line.line)?;
        }
        Ok(())
    }
}

impl Add<Line> for Code {
    type Output = Code;

    fn add(mut self, line: Line) -> Code {
        self.content.push(line);
        self
    }
}

impl Code {
    pub fn new() -> Code {
        Code { content: vec![], cursor_displayed: false, x: 0, y: 0 }
    }

    pub fn flush(&mut self) {
        self.content.clear()
    }

    pub fn is_cursor_displayed(&self) -> bool {
        self.cursor_displayed
    }

    pub fn get_x(&self) -> usize {
        self.x
    } 

    pub fn get_y(&self) -> usize {
        self.y
    } 

    pub fn set_x(&mut self, x: usize) -> &mut Self {
        self.x = x;
        self
    } 

    pub fn set_y(&mut self, y: usize) -> &mut Self {
        self.y = y;
        self
    } 
    
    pub fn remove_line(&mut self, number: usize) -> &mut Code {
        self.content.retain(|line| line.number != number);
        self
    }

    pub fn remove_line_at_cursor(&mut self) -> &mut Code {
        self.content.retain(|line| line.number != self.x);
        self
    }

    pub fn change_line(&mut self, number: usize, new_value: String) -> &mut Code {
        for line in &mut self.content {
            if line.number == number {
                line.line = new_value;
                break;
            }
        }
        self
    }

    pub fn change_line_at_cursor(&mut self, new_value: String) -> &mut Code {
        for line in &mut self.content {
            if line.number == self.x {
                line.line = new_value;
                break;
            }
        }
        self
    }

    pub fn add_line(&mut self, line: Line) -> &mut Code {
        self.content.push(line);
        self
    }

    pub fn get_line(&self, number: usize) -> Option<&Line> {
        self.content.iter().find(|line| line.number == number)
    }

    pub fn set_line_number(&mut self, number: usize) {
        let line = self.content.remove(number);
        self.content.insert(number + 1, line.clone())
    }

    pub fn get_content(&self) -> &Vec<Line> {
        &self.content
    }

    pub fn set_cursor(&mut self) {
        if !self.cursor_displayed {
            if let Some(line) = self.content.get(self.get_x()) {
                let mut line_with_cursor = line.get_string().clone();
                line_with_cursor.insert(self.get_y(), '|');
                self.change_line_at_cursor(line_with_cursor);
                self.cursor_displayed = true;
            }
        }        
    }

    pub fn remove_cursor(&mut self) {
        if self.cursor_displayed {
            if let Some(line) = self.content.get(self.get_x()) {
                let mut line_without_cursor = line.get_string().clone();
                if line_without_cursor.len() > 0 {
                    line_without_cursor.remove(self.get_y());
                    self.change_line_at_cursor(line_without_cursor);
                    self.cursor_displayed = false;
                }
            }
        }
    }

}