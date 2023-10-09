use super::code::{Line, Code};

#[derive(Debug, PartialEq, Eq)]
pub struct Change {
    number: u16,
    from: String,
    to: String,
}

trait ChangeBuilder {
    fn create_change_with_strings(number: u16, from: String, to: String) -> Change;
    fn create_change_with_line(line: Line, to: String) -> Change;
}

impl ChangeBuilder for Change {
    fn create_change_with_strings(number: u16, from: String, to: String) -> Change {
        Change { number, from, to }
    }

    fn create_change_with_line(line: Line, to: String) -> Change {
        Change {
            number: line.get_number(),
            from: line.get_string(),
            to,
        }
    }
}

impl Change {

    pub fn get_number(&self) -> u16 {
        self.number
    }

    pub fn get_from(&self) -> &str {
        &self.from
    }

    pub fn get_to(&self) -> &str {
        &self.to
    }

    pub fn get_old_line(&self) -> Line {
        Line::new(
            self.get_number(),
            self.get_from().to_string()
        )
    }

    pub fn get_new_line(&self) -> Line {
        Line::new(
            self.get_number(),
            self.get_to().to_string()
        )

    }

}

#[derive(Debug, PartialEq, Eq)]
pub struct CodeHistory {
    initial: Code,
    changes: Vec<Change>,
    version: u16,
}

impl CodeHistory {
    pub fn new(code: Code) -> CodeHistory {
        CodeHistory {
            initial: code.clone(),
            changes: vec![],
            version: 0,
        }
    }

    pub fn new_with_changes(code: Code, changes: Vec<Change>) -> CodeHistory {
        CodeHistory {
            initial: code.clone(),
            changes,
            version: 0,
        }
    }

    pub fn add_change(&mut self, change: Change) {
        self.changes.push(change);
    }

    pub fn add_change_at_position(&mut self, position: u16, change: Change) {
        if (position as usize) <= self.changes.len() {
            self.changes.insert(position as usize, change);
        }
    }


    pub fn get_current_code(&self) -> Code {
        let mut code = self.initial.clone();
        for i in 0..self.version as usize {
            if i < self.changes.len() {
                let line_number = self.changes[i].get_number();
                code.change_line(line_number, self.changes[i].get_to().to_string());
            }
        }
        code
    }

    pub fn remove_change(&mut self) {
        if self.version > 0 {
            self.version -= 1;
        }
    }

    pub fn use_previous(&mut self) {
        if self.version > 0 {
            self.version -= 1;
        }
    }

    pub fn use_next(&mut self) {
        if self.version < self.changes.len() as u16 {
            self.version += 1;
        }
    }

    pub fn use_last(&mut self) {
        self.version = self.changes.len() as u16;
    }

    pub fn reset(&mut self) {
        self.version = 0;
    }
}
