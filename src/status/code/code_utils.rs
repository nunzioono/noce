use std::{fmt::Display, ops::Add, collections::HashMap};

use itertools::Itertools;

#[derive(Default)]
pub struct Line {
    number: u16,
    line: String,
}

impl Add<String> for Line {
    type Output = Line;

    fn add(mut self, rhs: String) -> Self::Output {
        self.line += &rhs.to_string();
        return self
    }
}

impl Clone for Line {
    fn clone(&self) -> Self {
        Line { number: self.number, line: self.line.clone() }    
    }
}

impl Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = f.write_str(self.line.as_str());
        Ok(())
    }
}

impl Line {
    pub fn new(number: u16, line: String) -> Line {
        Line {
            number: number,
            line: line
        }
    }    

    pub fn set_number(&mut self, number: u16) {
        self.number = number;
    }

    pub fn set_line(&mut self, line: String) {
        self.line = line;
    }

    pub fn get_number(&self) -> u16 {
        self.number
    }

    pub fn get_line(&self) -> String {
        self.line.clone()
    }

}

#[derive(Default,Clone)]
pub struct Code {
    content: HashMap<u16,String>,
    current_line: u16,
    current_char: u16,
}

impl Display for Code {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.content.clone().into_iter().map(|line| {
            line.1.to_string()
        }).join("\n"))
    }
}

impl Add<Line> for Code {
    type Output = Code;

    fn add(mut self, rhs: Line) -> Self::Output {
        self.content.insert(rhs.get_number(),rhs.get_line());
        self    
    }
}

impl Code {
    pub fn new(lines: Vec<Line>) -> Code {
        Code {
            content: lines.into_iter().map(|line| (line.get_number(),line.get_line())).collect(),
            current_line: 0,
            current_char: 0
        }
    }

    pub fn up(&mut self) {
        self.current_line-=1;
    }

    pub fn down(&mut self) {
        self.current_line+=1;
    }

    pub fn left(&mut self) {
        if self.current_char - 1 > 0 {
            self.current_char -= 1;
        }
    }

    pub fn right(&mut self) {
        if let Some(line) = self.content.get(&self.current_line) {
            if self.current_char + 1 <line.len() as u16 {
                self.current_char += 1;
            }
    
        }
    }

    pub fn remove_char (&mut self) {
        self.content.entry(self.current_line)
        .and_modify(|value| {
            value.pop();
        });
    }

    pub fn add_line(&mut self, line: String) {
        let number = self.content.keys().max();
        if let Some(number) = number {
            self.content.insert(*number, line);            
        }
    }

    pub fn append(&mut self, new_value: &str) {
        let last_line_key = self.content.keys().max();
        if let Some(number) = last_line_key {
            self.content.entry(*number)
            .and_modify(|mut existing_value| *existing_value+=new_value)
            .or_insert(new_value.to_string());
        }
    }

    pub fn insert_change(&mut self, change: Change) {
        let line = self.content.get(&change.get_old_line().get_number()).unwrap();
        line.replace(&change.get_old_line().get_line(), &change.get_new_line().get_line());
        self.content.insert(change.get_old_line().get_number(), line.clone().to_string());
    }
}

#[derive(Clone)]
pub struct Change {
    number: u16,
    from: String,
    to: String
}

impl Change {
    pub fn new(number: u16, from: String, to: String) -> Change {
        Change { number: number, from: from, to: to }
    }

    pub fn build_from(from: Line, to: String) -> Change {
        Change { number: from.number, from: from.get_line(), to: to }
    }

    pub fn get_old_line(&self) -> Line {
        Line { number: self.number, line: self.from.clone() }
    }

    pub fn get_new_line(&self) -> Line {
        Line { number: self.number, line: self.to.clone() }
    }
}


#[derive(Default)]
pub struct CodeHistory {
    initial: Code,
    changes: HashMap<u16,Change>,
    version: u16
}

impl CodeHistory {
    pub fn new(code: Code) -> CodeHistory {
        CodeHistory { initial: code, changes: HashMap::default(), version: 0 }
    }

    pub fn new_with_changes(code: Code, changes: Vec<Change>) -> CodeHistory {
        CodeHistory { initial: code, changes: changes.into_iter().map(|change| (change.get_old_line().get_number(),change)).collect(), version: 0 }
    }

    pub fn add_change(&mut self, change: Change) {
        self.changes.insert(change.get_old_line().get_number(),change);
    }

    pub fn get_current_code(&self) -> Code {
        let mut initial = self.initial.clone();
        self.changes.clone().into_iter().for_each(|change| initial.insert_change(change.1));
        initial
    }

    pub fn use_previous(&mut self) {
        if self.version as usize - 1 >= 0 {
            self.version-=1;
        }
    }

    pub fn use_next(&mut self) {
        if self.version as usize + 1 < self.changes.len() {
            self.version+=1;
        }
    }

    pub fn use_last(&mut self) {
        self.version = (self.changes.len() -1) as u16;
    }
}