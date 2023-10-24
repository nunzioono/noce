use std::{fmt::{self}, ops::Add};

use super::{code_utils::Point, code_selection::CodeSelection};



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
    cursor: Point,
    selection: Option<CodeSelection>,
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
        Code { content: vec![], cursor_displayed: false, cursor: Point::default(), selection: None }
    }

    pub fn get_cursor(&self) -> &Point {
        &self.cursor
    }

    pub fn get_mut_cursor(&mut self) -> &mut Point {
        &mut self.cursor
    }

    pub fn flush(&mut self) {
        self.content.clear()
    }

    pub fn is_cursor_displayed(&self) -> bool {
        self.cursor_displayed
    }
    
    pub fn remove_line(&mut self, number: usize) {
        self.content.retain(|line| line.number != number);
    }

    pub fn remove_line_at_cursor(&mut self) {
        self.content.retain(|line| line.number != self.cursor.get_x());
    }

    pub fn replace_line(&mut self, number: usize, from: &String, to: &String) {
        if let Some(line) = self.content.get_mut(number) {
            line.set_string(line.get_string().replace(from, to));
        }
    }

    pub fn change_line(&mut self, number: usize, new_value: String) {
        for line in &mut self.content {
            if line.number == number {
                line.line = new_value;
                break;
            }
        }
    }

    pub fn change_line_at_cursor(&mut self, new_value: String) {
        for line in &mut self.content {
            if line.number == self.cursor.get_x() {
                line.line = new_value;
                break;
            }
        }
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
            if let Some(line) = self.content.get(self.cursor.get_x()) {
                let mut line_with_cursor = line.get_string().clone();
                line_with_cursor.insert(self.cursor.get_y(), '|');
                self.change_line_at_cursor(line_with_cursor);
                self.cursor_displayed = true;
            }
        }        
    }

    pub fn remove_cursor(&mut self) {
        if self.cursor_displayed {
            if let Some(line) = self.content.get(self.cursor.get_x()) {
                let mut line_without_cursor = line.get_string().clone();
                if line_without_cursor.len() > 0 {
                    line_without_cursor.remove(self.cursor.get_y());
                    self.change_line_at_cursor(line_without_cursor);
                    self.cursor_displayed = false;
                }
            }
        }
    }

    

    pub fn flush_selection(&mut self) {
        self.selection = None;
    }

    pub fn set_selection_start(&mut self, start: Point) {
        if let Some(selection) = &mut self.selection {
            selection.set_start(start);
        }
    }

    pub fn set_selection_end(&mut self, end: Point) {
        if let Some(selection) = &mut self.selection {
            selection.set_end(end);
        }
    }

    pub fn create_selection(&mut self, start: Point, end: Point) {
        self.selection = Some(CodeSelection::new(start, end));
    }

    pub fn get_selection(&self) -> &Option<CodeSelection> {
        &self.selection
    }

}