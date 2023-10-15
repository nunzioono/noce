pub mod code_history;
pub mod code_selection;
pub mod code;

use std::{fs::{File, OpenOptions}, io::Write, error::Error, path::PathBuf};
use self::{code::{Code, Line}, code_history::CodeHistory, code_selection::CodeSelection};
use clipboard::{ClipboardProvider, ClipboardContext};
use crossterm::event::{KeyEventKind, Event, KeyCode, KeyModifiers, ModifierKeyCode};

use super::{Component, ComponentType, AppContext};

#[derive(Debug, PartialEq, Eq)]
pub struct CodeComponent {
    current: Code,
    history: CodeHistory,
    selection: Option<CodeSelection>,
}

impl Component for CodeComponent {

    fn get_type(&self) -> ComponentType {
        ComponentType::Code
    }

    fn handle_event(&mut self, context: &mut AppContext, event: Event) {
        if let Event::Key(key) = event {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char(char) => {

                        let mut char_normalized = char.clone().to_string();
                        char_normalized = char_normalized.to_lowercase().to_string();
                        if char_normalized == "x" && key.modifiers.contains(KeyModifiers::CONTROL) {
                            let mut cut = String::default();
                            if let Some(selection) = self.selection.as_mut() {
                                if selection.is_selecting() {
                                    let code = selection.get_selection();
                                    cut = code.to_string();
                                }
                            }
                            let clipboard: Result<ClipboardContext, Box<dyn Error>> = ClipboardProvider::new();
                            if let Ok(mut context) =  clipboard {
                                let _ = context.set_contents(cut);
                            } 
                            self.selection = None;
                        } else if char_normalized == "c" && key.modifiers.contains(KeyModifiers::CONTROL) {
                            let mut copy = String::default();
                            if let Some(selection) = self.selection.as_mut() {
                                if selection.is_selecting() {
                                    let code = selection.get_selection();
                                    copy = code.to_string()
                                }
                            }
                            let clipboard: Result<ClipboardContext, Box<dyn Error>> = ClipboardProvider::new();
                            if let Ok(mut context) =  clipboard {
                                let _ = context.set_contents(copy);
                            } 
                        } else if char_normalized == "v" && key.modifiers.contains(KeyModifiers::CONTROL) {
                            let clipboard: Result<ClipboardContext, Box<dyn Error>> = ClipboardProvider::new();
                            if let Ok(mut context) =  clipboard {
                                if let Ok(contents) = context.get_contents() {
                                    contents.split("\n").for_each(|line| {
                                        let number = self.current.get_content().into_iter().map(|line| line.get_number()).max().take().unwrap() + 1;
                                        let line = Line::new(number, line.to_string());
                                        let _ = self.current.add_line(line);
                                    });                                
                                }
                            }
                        } else if char_normalized == "s" && key.modifiers.contains(KeyModifiers::CONTROL) {
                                self.history.use_last();
                                let code = self.history.get_current_code();
                                let utf8_code = code.to_string().chars().map(|char| char as u8).fold(vec![], |mut vec, char| {
                                    vec.push(char);
                                    vec
                                });
                                if let Some(path) = context.active_file() {
                                    if path.is_file() {
                                        let f = OpenOptions::new().append(true).open(path);
                                        if let Ok(mut file) = f {
                                            let _ = file.write_all(&utf8_code);
                                        }    
                                    }
                                } else if let Some(path) = context.active_file() {
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
                                let last_number = self.current.get_content().into_iter().map(|x| x.get_number()).fold(0, |line1, line2| {
                                    if line1 > line2 { line1 } else { line2 }
                                });
                                if let Some(last_line) = self.current.get_line(last_number) {
                                    let mutable_clone = &mut last_line.clone();
                                    mutable_clone.set_string(last_line.get_string() + &String::from("\n"));
                                    self.current.change_line(last_number, mutable_clone.get_string());
                                    self.current.add_line(Line::new(last_number+1, String::default()));    
                                }
                            } else {
                                let last_line = self.current.get_line(self.current.get_content().into_iter().map(|x| x.get_number()).fold(0, |line1, line2| {
                                    if line1 > line2 { line1 } else { line2 }
                                }));
                                if let Some(last_line) = last_line {
                                    let mutable_clone = &mut last_line.clone();
                                    mutable_clone.set_string(last_line.get_string() + &char.to_string());
                                    self.current.change_line(last_line.get_number(),mutable_clone.get_string());    
                                }
                            }
                        }
                    },
                    KeyCode::Delete => {
                        let last_number = self.current.get_content().into_iter().map(|x| x.get_number()).fold(0, |line1, line2| {
                            if line1 > line2 { line1 } else { line2 }
                        });
                        let last_line = self.current.get_line(last_number).unwrap();
                        self.current.change_line(last_line.get_number(), last_line.get_string()[..last_line.get_string().len()-1].to_string());
                    },
                    KeyCode::Enter => {
                        let last_number = self.current.get_content().into_iter().map(|x| x.get_number()).fold(0, |line1, line2| {
                            if line1 > line2 { line1 } else { line2 }
                        });
                        let last_line = self.current.get_line(last_number);
                        if let Some(last_line) = last_line {
                            let mutable_clone = &mut last_line.clone();
                            mutable_clone.set_string(last_line.get_string() + &String::from("\n"));
                            self.current.change_line(last_number, mutable_clone.get_string());
                            self.current.add_line(Line::new(last_number+1, String::default()));
    
                        }
                    },
                    KeyCode::Up => {
                        self.current.set_x(self.current.get_x() - 1);
                    },
                    KeyCode::Down => {
                        self.current.set_x(self.current.get_x() + 1);
                    },
                    KeyCode::Left => {
                        self.current.set_y(self.current.get_y() - 1);
                    },
                    KeyCode::Right => {
                        self.current.set_y(self.current.get_y() + 1);
                    },
                    KeyCode::Modifier(ModifierKeyCode::LeftShift) => {
                        if let Some(selection) = &mut self.selection {
                            if let Some(current_line) = selection.get_selection().get_line(self.current.get_x()) {
                                if self.current.get_y() > 0 {
                                    let new_value = current_line.get_string().chars().enumerate()
                                    .filter(|tuple| tuple.0 < self.current.get_y() - 1)
                                    .map(|tuple| tuple.1)
                                    .fold(String::default(), |mut char1, char2| {
                                        char1.push(char2);
                                        char1
                                    });
                                    selection.get_selection().change_line_at_cursor(new_value);    
                                    self.current.set_y(self.current.get_y() -1);    
                                }
                            }
                        }
                    },
                    KeyCode::Modifier(ModifierKeyCode::RightShift) => {
                        if let Some(selection) = &mut self.selection {
                            if let Some(current_line) = selection.get_selection().get_line(self.current.get_x()) {
                                if self.current.get_y() < current_line.get_string().len()-1 {
                                    let new_value = current_line.get_string().chars().enumerate()
                                    .filter(|tuple| tuple.0 < self.current.get_y() + 1)
                                    .map(|tuple| tuple.1).fold(String::default(), |mut char1, char2| {
                                        char1.push(char2);
                                        char1
                                    });
                                    selection.get_selection().change_line_at_cursor(new_value);
                                    self.current.set_y(self.current.get_y() +1);    
                                }
    
                            }
                        }
                    },
                    KeyCode::Esc => {
                        context.set_focus(None);
                        context.set_hover(self.get_type());             
                    },
                    _ => {}
                }
            } if key.kind == KeyEventKind::Repeat {
                match key.code {
                    KeyCode::Char(char) => {
                        if char == '\n' {
                            let last_number = self.current.get_content().into_iter().map(|x| x.get_number()).fold(0, |line1, line2| {
                                if line1 > line2 { line1 } else { line2 }
                            });
                            if let Some(last_line) = self.current.get_line(last_number) {
                                let new_value_last_line = &mut last_line.clone();
                                new_value_last_line.set_string(last_line.get_string() + &String::from("\n"));
                                self.current.change_line(last_number, new_value_last_line.get_string());
                                self.current.add_line(Line::new(last_number+1, String::default()));    
                            }
                        } else {
                            let current_code_copy = self.current.clone();
                            let last_line = current_code_copy.get_line(self.current.get_content().into_iter().map(|x| x.get_number()).fold(0, |line1, line2| {
                                if line1 > line2 { line1 } else { line2 }
                            })).unwrap();
                            let new_value_last_line = &mut last_line.clone();
                            new_value_last_line.set_string(last_line.get_string() + &char.to_string());
                            self.current.change_line(last_line.get_number(), last_line.get_string());
                        }
                    },
                    KeyCode::Delete => {
                        let last_number = self.current.get_content().into_iter().map(|x| x.get_number()).fold(0, |line1, line2| {
                            if line1 > line2 { line1 } else { line2 }
                        });
                        let last_line = self.current.get_line(last_number).unwrap();
                        self.current.change_line(last_line.get_number(), last_line.get_string()[..last_line.get_string().len()-1].to_string());
                    },
                    KeyCode::Enter => {
                        let last_number = self.current.get_content().into_iter().map(|x| x.get_number()).fold(0, |line1, line2| {
                            if line1 > line2 { line1 } else { line2 }
                        });
                        let last_line = self.current.get_line(last_number);
                        if let Some(last_line) = last_line {
                            let mutable_clone = &mut last_line.clone();
                            mutable_clone.set_string(last_line.get_string() + &String::from("\n"));
                            self.current.change_line(last_number, mutable_clone.get_string());
                            self.current.add_line(Line::new(last_number+1, String::default()));
    
                        }
                    },
                    KeyCode::Up => {
                        self.current.set_x(self.current.get_x() - 1);
                    },
                    KeyCode::Down => {
                        self.current.set_x(self.current.get_x() + 1);
                    },
                    KeyCode::Left => {
                        self.current.set_y(self.current.get_y() - 1);
                    },
                    KeyCode::Right => {
                        self.current.set_y(self.current.get_y() + 1);
                    },
                    _ => {}
                }
            }
        }
    }
}

impl CodeComponent {

    pub fn new(code: Code) -> Self {
        CodeComponent {
            current: code.clone(),
            history: CodeHistory::new(code),
            selection: None,
        }
    }

    pub fn set_current(&mut self, active_file: Option<PathBuf>) {
        self.current = Code::new(active_file);
    }

    pub fn get_current(&self) -> &Code {
        &self.current
    }

    pub fn get_history(&self) -> &CodeHistory {
        &self.history
    }

    pub fn get_selection(&self) -> &Option<CodeSelection> {
        &self.selection
    }
}