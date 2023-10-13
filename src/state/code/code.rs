use std::{fmt::{self}, ops::Add};



#[derive(Clone, Default, Debug, PartialEq, Eq)]
pub struct Line {
    number: u16,
    line: String,
}

impl Line {
    pub fn new(number: u16, line: String) -> Line {
        Line { number, line }
    }

    pub fn set_number(&mut self, number: u16) {
        self.number = number;
    }

    pub fn get_number(&self) -> u16 {
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
    x: u16,
    y: u16
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
        Code { content: Vec::new(), x: 0, y: 0 }
    }

    pub fn get_x(&self) -> u16 {
        self.x
    } 

    pub fn get_y(&self) -> u16 {
        self.y
    } 

    pub fn set_x(&mut self, x: u16) -> &mut Self {
        self.x = x;
        self
    } 

    pub fn set_y(&mut self, y: u16) -> &mut Self {
        self.y = y;
        self
    } 
    
    pub fn remove_line(&mut self, number: u16) -> &mut Code {
        self.content.retain(|line| line.number != number);
        self
    }

    pub fn remove_line_at_cursor(&mut self) -> &mut Code {
        self.content.retain(|line| line.number != self.x);
        self
    }

    pub fn change_line(&mut self, number: u16, new_value: String) -> &mut Code {
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

    pub fn get_line(&self, number: u16) -> Option<&Line> {
        self.content.iter().find(|line| line.number == number)
    }

    pub fn get_content(&self) -> Vec<Line> {
        self.content.clone()
    }

}