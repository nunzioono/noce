use super::code::{Line, Code};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LineSelection {
    line: Line,
    start: u16,
    end: u16,
}

impl LineSelection {
    pub fn new(line: Line) -> LineSelection {
        LineSelection {
            line,
            start: 0,
            end: 0,
        }
    }

    pub fn get_line(&self) -> &Line {
        &self.line
    }

    pub fn get_selection(&self) -> String {
        let start = self.start as usize;
        let end = self.end as usize;
        self.line.get_string()[start..end].to_string()
    }

    pub fn get_start(&self) -> u16 {
        self.start
    }

    pub fn get_end(&self) -> u16 {
        self.end
    }

    pub fn select_left(&mut self) {
        if self.start > 0 {
            self.start -= 1;
        }
    }

    pub fn select_right(&mut self) {
        if self.end < self.line.get_string().len() as u16 {
            self.end += 1;
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct CodeSelection {
    code: Code,
    lines: Vec<LineSelection>,
    start: u16,
    end: u16
}

impl CodeSelection {

    pub fn new(code: Code) -> CodeSelection {
        CodeSelection {
            code: code,
            lines: Vec::new(),
            start: 0,
            end: 0,
        }
    }
    pub fn is_selecting(&self) -> bool {
        !self.lines.is_empty()
    }

    pub fn add_selection(&mut self, selection: LineSelection) {
        if !self.lines.is_empty() {
            if let Some(last_line) = self.lines.last() {
                if last_line.line.get_number() + 1 == selection.get_line().get_number() {
                    self.lines.push(selection);
                }
            }
        }
    }

    pub fn remove_selection(&mut self) {
        self.lines.clear();
    }

    pub fn get_selection(&mut self) -> Code {
        let code = &mut self.code;

        self.lines.clone().into_iter()
        .map(|line_selection|
            Line::new(line_selection.get_line().get_number(), line_selection.get_line().get_string()
            .chars().into_iter()
            .enumerate().filter(|tuple| tuple.0 >= line_selection.get_start().into() && tuple.0 < line_selection.get_end().into())
            .map(|tuple| tuple.1).fold(String::new(),|mut char1,char2| {
                char1.push(char2);
                char1
            })))
        .for_each(|line| {let _ = code.add_line(line);});

        code.clone()
    }
}

