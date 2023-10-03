mod code_utils;

use std::{path::PathBuf, fs::{File, OpenOptions}, io::{Read, Write}};

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

impl LineSelection {
    
    pub fn new(line: Line) -> LineSelection {
        LineSelection { line: line, start: 0, end: 0 }
    }

    pub fn get_selection(&self) -> String {
        self.line.get_line().chars().enumerate().filter(|tuple| (tuple.0 as u16) >= self.start && (tuple.0 as u16) < self.end).map(|tuple| tuple.1.to_string()).join("").to_string()
    }
}

#[derive(Clone)]
struct CodeSelection {
    selection: Vec<LineSelection>
}

impl CodeSelection {

    pub fn new(selection: Vec<LineSelection>) -> CodeSelection{
        CodeSelection { selection }
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

        if let Ok(mut file) = File::open(file.clone().unwrap()) {
            if file.read_to_string(&mut string_content).is_ok() {
                string_content.to_string().split("\n").enumerate().map(|line| Line::new(line.0 as u16, line.1.to_string())).for_each(|line| content.push(line));
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
                        let char_normalized = char.clone();
                        char_normalized.to_lowercase();
                        if char_normalized == 'x' && key.modifiers.contains(KeyModifiers::CONTROL) {
                            let cut = self.code_selection.clone().unwrap().selection.into_iter().map(|selection| selection.get_selection()).join("\n").to_string();
                            let mut ctx: ClipboardContext= ClipboardProvider::new().unwrap();
                            let _ = ctx.set_contents(cut);
                            self.code_selection = None;
                        } else if char_normalized == 'c' && key.modifiers.contains(KeyModifiers::CONTROL) {
                            let copy = self.code_selection.clone().unwrap().selection.into_iter().map(|selection| selection.get_selection()).join("\n").to_string();
                            let mut ctx: ClipboardContext= ClipboardProvider::new().unwrap();
                            let _ = ctx.set_contents(copy);
                        } else if char_normalized == 'v' && key.modifiers.contains(KeyModifiers::CONTROL) {
                            let mut ctx: ClipboardContext= ClipboardProvider::new().unwrap();
                            if let Ok(contents) = ctx.get_contents() {
                                contents.split("\n").for_each(|line| self.current.add_line(line.to_string()));                                
                            }
                        } else if char_normalized == 's' && key.modifiers.contains(KeyModifiers::CONTROL) {
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
                        } else if char_normalized == 'z' && key.modifiers.contains(KeyModifiers::CONTROL) {
                            self.history.use_previous();
                            let code = self.history.get_current_code();
                            self.current = code.clone();                            
                        } else if char_normalized == 'y' && key.modifiers.contains(KeyModifiers::CONTROL) {
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
                    KeyCode::Delete => {},
                    KeyCode::Enter => {},
                    KeyCode::Up => {},
                    KeyCode::Down => {},
                    KeyCode::Left => {},
                    KeyCode::Right => {},
                    KeyCode::Modifier(ModifierKeyCode::LeftShift) => {

                    },
                    KeyCode::Modifier(ModifierKeyCode::RightShift) => {

                    },
                    KeyCode::Esc => {}
                    _ => {}
                }
            } if key.kind == KeyEventKind::Repeat {
                match key.code {
                    KeyCode::Char(char) => {
                        
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