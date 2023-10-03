mod code_utils;

use std::{path::PathBuf, fs::{File, OpenOptions}, io::{Read, Write}, fmt::Display};

use clipboard::{ClipboardProvider, ClipboardContext};
use crossterm::event::{Event, KeyEventKind, KeyCode, KeyModifiers, ModifierKeyCode};
use itertools::Itertools;


use self::code_utils::{Code, CodeHistory, Line,};
#[derive(Clone)]
struct LineSelection {
    line: Line,
    start: u16,
    end: u16,
}

impl Display for LineSelection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = f.write_str(self.line.get_line().chars().enumerate().filter(|tuple| (tuple.0 as u16) >= self.start && (tuple.0 as u16) < self.end).map(|tuple| tuple.1).join("").as_str());
        Ok(())
    }
}

impl LineSelection {
    
    pub fn new(line: Line) -> LineSelection {
        LineSelection { line: line, start: 0, end: 0 }
    }

    pub fn is_selection_started(&self) -> bool {
        self.start != self.end  
    }

    pub fn start_selection(&mut self, position: u16) {
        self.start = position;
        self.end = position;
    }

    pub fn select_left(&mut self) {
        self.end-=1;
    }

    pub fn select_right(&mut self) {
        self.end+=1;
    }

    pub fn get_selection(&mut self) -> String {
        if self.start > self.end {
            let tmp = self.start.clone();
            self.start = self.end.clone();
            self.end = tmp;
        }
        self.line.get_line().clone()
            .chars()
            .into_iter()
            .enumerate()
            .filter(|(i,_el)| (*i as u16)>=self.start && (*i as u16)<self.end)
            .map(|(_i,el)| el).join("").to_string()
    }
}

#[derive(Clone)]
struct CodeSelection {
    selection: Vec<LineSelection>,
    start: u16,
    end: u16
}

impl Display for CodeSelection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.clone().get_selection().into_iter().map(|line| line.to_string()).join("\n").as_str());
        Ok(())
    }
}

impl CodeSelection {

    pub fn new(selection: Vec<LineSelection>) -> CodeSelection{
        let selection_length = selection.clone().len();
        CodeSelection { selection: selection.clone(), start: (selection_length - 1) as u16, end: (selection_length - 1) as u16 }
    }

    pub fn select_left(&mut self) {
        if self.selection.get(self.end as usize).is_some() {
            let line = &mut self.selection.get(self.end as usize).unwrap().to_owned();
            line.select_left();
        }
    }

    pub fn select_right(&mut self) {
        if self.selection.get(self.end as usize).is_some() {
            let line = &mut self.selection.get(self.end as usize).unwrap().to_owned();
            line.select_right();
        }
    }

    pub fn select_up(&mut self) {
        if self.end > 0 {
            self.end -= 1;
        }
    }

    pub fn select_down(&mut self) {
        if self.end > 0 {
            self.end += 1;
        }
    }

    pub fn get_selection(&mut self) -> Vec<LineSelection>{
        if self.start > self.end {
            let tmp = self.start;
            self.start = self.end;
            self.end = tmp;
        }
        self.selection.clone().into_iter().enumerate().filter(|line_selection| (line_selection.0 as u16) >= self.start && (line_selection.0 as u16) < self.end).map(|tuple| tuple.1).collect_vec()
    }
}

pub struct CodeState {
    current: Code,
    history: CodeHistory,
    code_selection: Option<CodeSelection>
}

impl CodeState {
    
    pub fn new(file: &Option<PathBuf>) -> CodeState {
        let mut content: Vec<Line> = Vec::default();
        let mut string_content = String::default();

        if let Some(path) = file {
            if let Ok(mut file) = File::open(path) {
                if file.read_to_string(&mut string_content).is_ok() {
                    string_content.to_string().split("\n").enumerate().map(|line| Line::new(line.0 as u16, line.1.to_string())).for_each(|line| content.push(line));
                }
            }    
        }
        let code = Code::new(content);
        let code_clone = code.clone();
        CodeState { current: code, history: CodeHistory::new(code_clone), code_selection: None }
    }

    pub fn process(&mut self, event: Event, ref_file: Option<PathBuf>) {
        if let Event::Key(key) = event {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char(char) => {
                        let mut char_normalized = char.clone().to_string();
                        char_normalized = char_normalized.to_lowercase().to_string();
                        if char_normalized == "x" && key.modifiers.contains(KeyModifiers::CONTROL) {
                            let cut = self.code_selection.clone().unwrap().selection.into_iter().map(|selection| selection.clone().get_selection()).join("\n").to_string();
                            let mut ctx: ClipboardContext= ClipboardProvider::new().unwrap();
                            let _ = ctx.set_contents(cut);
                            self.code_selection = None;
                        } else if char_normalized == "c" && key.modifiers.contains(KeyModifiers::CONTROL) {
                            let copy = self.code_selection.clone().unwrap().selection.into_iter().map(|selection| selection.clone().get_selection()).join("\n").to_string();
                            let mut ctx: ClipboardContext= ClipboardProvider::new().unwrap();
                            let _ = ctx.set_contents(copy);
                        } else if char_normalized == "v" && key.modifiers.contains(KeyModifiers::CONTROL) {
                            let mut ctx: ClipboardContext= ClipboardProvider::new().unwrap();
                            if let Ok(contents) = ctx.get_contents() {
                                contents.split("\n").for_each(|line| self.current.add_line(line.to_string()));                                
                            }
                        } else if char_normalized == "s" && key.modifiers.contains(KeyModifiers::CONTROL) {
                            self.history.use_last();
                            let code = self.history.get_current_code();
                            let utf8_code = code.to_string().chars().map(|char| char as u8).collect_vec();
                            if let Some(path) = ref_file {
                                if path.is_file() {
                                    let f = OpenOptions::new().append(true).open(path);
                                    if let Ok(mut file) = f {
                                        let _ = file.write_all(&utf8_code);
                                    }    
                                }
                            } else if let Some(path) = ref_file {
                                let f = File::create(path);
                                if let Ok(mut file) = f {
                                    let _ = file.write_all(&utf8_code);
                                }
                            } 
                        } else if char_normalized == "z" && key.modifiers.contains(KeyModifiers::CONTROL) {
                            self.history.use_previous();
                            let code = self.history.get_current_code();
                            self.current = code.clone();                            
                        } else if char_normalized == "y" && key.modifiers.contains(KeyModifiers::CONTROL) {
                            self.history.use_next();
                            let code = self.history.get_current_code();
                            self.current = code.clone();
                        } else {
                            if char == '\n' {
                                self.current.add_line(char.to_string());
                            } else {
                                self.current.append(char.to_string().as_str());
                            }
                        }
                    },
                    KeyCode::Delete => {
                        self.current.remove_char();
                    },
                    KeyCode::Enter => {
                        self.current.add_line(String::default());
                    },
                    KeyCode::Up => {
                        self.current.up();
                    },
                    KeyCode::Down => {
                        self.current.down();
                    },
                    KeyCode::Left => {
                        self.current.left();
                    },
                    KeyCode::Right => {
                        self.current.right();
                    },
                    KeyCode::Modifier(ModifierKeyCode::LeftShift) => {
                        if let Some(selection) = &mut self.code_selection {
                            selection.select_left();
                        }
                    },
                    KeyCode::Modifier(ModifierKeyCode::RightShift) => {
                        if let Some(selection) = &mut self.code_selection {
                            selection.select_right();
                        }
                    },
                    KeyCode::Esc => {
                        todo!("ref cell to hover and focus dell'app")
                    }
                    _ => {}
                }
            } if key.kind == KeyEventKind::Repeat {
                match key.code {
                    KeyCode::Char(char) => {
                        if char == '\n' {
                            self.current.add_line(char.to_string());
                        } else {
                            self.current.append(char.to_string().as_str());
                        }
                    },
                    KeyCode::Delete => {},
                    KeyCode::Enter => {},
                    KeyCode::Up => {},
                    KeyCode::Down => {},
                    KeyCode::Left => {},
                    KeyCode::Right => {},
                    _ => {}
                }
            }
        }
    }

}