pub mod code_history;
pub mod code_selection;
pub mod code;
pub mod code_utils;

use std::{fs::{File, OpenOptions}, io::{Write, Read}, error::Error, path::PathBuf, vec};
use self::{code::{Code, Line}, code_history::CodeHistory, code_utils::{handle_up, handle_down, handle_left, handle_right}};
use clipboard::{ClipboardProvider, ClipboardContext};
use crossterm::event::{KeyEventKind, Event, KeyCode, KeyModifiers};

use super::{Component, ComponentType, AppContext};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct CodeComponent {
    current: Code,
    history: CodeHistory,
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
                        let current_code = self.get_current().get_content();

                        let mut char_normalized = char.clone().to_string();
                        char_normalized = char_normalized.to_lowercase().to_string();
                        if char_normalized == "x" && key.modifiers.contains(KeyModifiers::CONTROL) {
                            let cut;
                            if let Some(selection) = self.get_current().get_selection() {
                                if selection.get_start() != selection.get_end() {
                                    let mut code: Vec<String> = vec![];
                                    for i in selection.get_start().get_x()..selection.get_end().get_x() {
                                        if let Some(line) = current_code.get(i) {
                                            let selected_line: String;
                                            if line.get_number() == selection.get_start().get_x() {
                                                selected_line = line.get_string()[selection.get_start().get_y()..].to_string();
                                                if let Some(current_string) = code.get(line.get_number()) {
                                                    let new_string: String = current_string.replace(&selected_line, "").clone();
                                                    code.push(new_string.clone());    
                                                }
                                            } else if line.get_number() == selection.get_end().get_x() {
                                                selected_line = line.get_string()[..selection.get_end().get_y()].to_string();
                                                if let Some(current_string) = code.get(line.get_number()) {
                                                    let new_string: String = current_string.replace(&selected_line, "").clone();
                                                    code.push(new_string.clone());    
                                                }
                                            } else {
                                                code.push(line.get_string());
                                            }
                                        }
                                    }
                                    cut = code.join("\n").to_string();
                                    let clipboard: Result<ClipboardContext, Box<dyn Error>> = ClipboardProvider::new();
                                    if let Ok(mut context) =  clipboard {
                                        let _ = context.set_contents(cut);
                                    } 

                                    self.get_mut_current().flush_selection();
                                }
                            }
                        } else if char_normalized == "c" && key.modifiers.contains(KeyModifiers::CONTROL) {
                            let copy;
                            if let Some(selection) = self.get_current().get_selection() {
                                if selection.get_start() != selection.get_end() {
                                    let mut code: Vec<String> = vec![];
                                    for i in selection.get_start().get_x()..selection.get_end().get_x() {
                                        if let Some(line) = self.get_current().get_line(i) {
                                            let selected_line: String;
                                            if line.get_number() == selection.get_start().get_x() {
                                                selected_line = line.get_string()[selection.get_start().get_y()..].to_string();
                                                if let Some(current_string) = code.get(line.get_number()) {
                                                    let new_string: String = current_string.replace(&selected_line, "").clone();
                                                    code.push(new_string.clone());    
                                                }
                                            } else if line.get_number() == selection.get_end().get_x() {
                                                selected_line = line.get_string()[..selection.get_end().get_y()].to_string();
                                                if let Some(current_string) = code.get(line.get_number()) {
                                                    let new_string: String = current_string.replace(&selected_line, "").clone();
                                                    code.push(new_string.clone());    
                                                }
                                            } else {
                                                code.push(line.get_string());
                                            }
                                        }
                                    }
                                    copy = code.join("\n").to_string();
                                    let clipboard: Result<ClipboardContext, Box<dyn Error>> = ClipboardProvider::new();
                                    if let Ok(mut context) =  clipboard {
                                        let _ = context.set_contents(copy);
                                    } 
                                }
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
                            self.current.remove_cursor();
                            if let Some(current_line) = self.current.get_line(self.current.get_cursor().get_x()) {
                                self.current.change_line_at_cursor(current_line.get_string()[..self.current.get_cursor().get_y()].to_string() + &char.to_string() + &current_line.get_string()[self.current.get_cursor().get_y()..].to_string());    
                            }
                            let y = self.current.get_cursor().get_y();
                            self.current.get_mut_cursor().set_y(y+1);
                            self.current.set_cursor();
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
                        {
                            let mut_code = self.get_mut_current();
                            mut_code.remove_cursor();    
                        }
                        let code = self.get_current().clone();
                        let mut_code = self.get_mut_current();
                        if let Some(current_line) = code.get_content().get(code.get_cursor().get_x()) {
                            let line_number = current_line.get_number().clone();
                            let new_current_string = current_line.get_string()[..code.get_cursor().get_y()].to_string().clone();
                            let new_generated_string = current_line.get_string()[code.get_cursor().get_y()..].to_string().clone();
                            mut_code.flush();
                            for number in 0 .. line_number {
                                if let Some(line) = code.get_line(number) {
                                    mut_code.add_line(line.clone());                                    
                                }
                            }
                            mut_code.add_line(Line::new(current_line.get_number(), new_current_string));
                            mut_code.get_mut_cursor().set_x(code.get_cursor().get_x());
                            mut_code.get_mut_cursor().set_y(code.get_cursor().get_y());
                            mut_code.add_line(Line::new(current_line.get_number() + 1, new_generated_string));
                            for number in current_line.get_number() + 1.. code.get_content().len() {
                                if let Some(line) = code.get_line(number) {
                                    let mut new_line = line.clone();
                                    new_line.set_number(number + 1);
                                    mut_code.add_line(new_line.clone());                                    
                                }
                            }
                            mut_code.set_cursor();
                        }
                    },
                    KeyCode::Up => {
                        self.get_mut_current().remove_cursor();
                        handle_up(self, event.clone());
                        self.get_mut_current().set_cursor();
                    },
                    KeyCode::Down => {
                        self.get_mut_current().remove_cursor();
                        handle_down(self, event.clone());
                        self.get_mut_current().set_cursor();
                    },
                    KeyCode::Left => {
                        self.get_mut_current().remove_cursor();
                        handle_left(self, event.clone());
                        self.get_mut_current().set_cursor();
                    },
                    KeyCode::Right => {
                        self.get_mut_current().remove_cursor();
                        handle_right(self, event.clone());
                        self.get_mut_current().set_cursor();
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
                        self.current.remove_cursor();
                        if let Some(current_line) = self.current.get_line(self.current.get_cursor().get_x()) {
                            self.current.change_line_at_cursor(current_line.get_string()[..self.current.get_cursor().get_y()].to_string() + &char.to_string() + &current_line.get_string()[self.current.get_cursor().get_y()..].to_string());    
                        }
                        let y = self.current.get_cursor().get_y();
                        self.current.get_mut_cursor().set_y(y+1);
                        self.current.set_cursor();
                    },
                    KeyCode::Delete => {
                        let last_number = self.current.get_content().into_iter().map(|x| x.get_number()).fold(0, |line1, line2| {
                            if line1 > line2 { line1 } else { line2 }
                        });
                        let last_line = self.current.get_line(last_number).unwrap();
                        self.current.change_line(last_line.get_number(), last_line.get_string()[..last_line.get_string().len()-1].to_string());
                    },
                    KeyCode::Enter => {
                        {
                            let mut_code = self.get_mut_current();
                            mut_code.remove_cursor();    
                        }
                        let code = self.get_current().clone();
                        let mut_code = self.get_mut_current();
                        if let Some(current_line) = code.get_content().get(code.get_cursor().get_x()) {
                            let line_number = current_line.get_number().clone();
                            let new_current_string = current_line.get_string()[..code.get_cursor().get_y()].to_string().clone();
                            let new_generated_string = current_line.get_string()[code.get_cursor().get_y()..].to_string().clone();
                            mut_code.flush();
                            for number in 0 .. line_number {
                                if let Some(line) = code.get_line(number) {
                                    mut_code.add_line(line.clone());                                    
                                }
                            }
                            mut_code.add_line(Line::new(current_line.get_number(), new_current_string));
                            mut_code.get_mut_cursor().set_x(code.get_cursor().get_x());
                            mut_code.get_mut_cursor().set_y(code.get_cursor().get_y());
                            mut_code.add_line(Line::new(current_line.get_number() + 1, new_generated_string));
                            for number in current_line.get_number() + 1.. code.get_content().len() {
                                if let Some(line) = code.get_line(number) {
                                    let mut new_line = line.clone();
                                    new_line.set_number(number + 1);
                                    mut_code.add_line(new_line.clone());                                    
                                }
                            }
                            mut_code.set_cursor();
                        }
                    },
                    KeyCode::Up => {
                        handle_up(self, event);

                    },
                    KeyCode::Down => {
                        handle_down(self, event.clone());

                    },
                    KeyCode::Left => {
                        handle_left(self, event.clone());

                    },
                    KeyCode::Right => {
                        handle_right(self, event.clone());
                    },
                    _ => {}
                }
            }
        }
    }
}

impl CodeComponent {

    pub fn new() -> Self {
        let code = Code::new();
        CodeComponent {
            current: code.clone(),
            history: CodeHistory::new(code.clone()),
        }
    }

    pub fn set_current(&mut self, active_file: Option<PathBuf>) {
        if let Some(path) = active_file {
            let file = File::open(path);
            if let Ok(mut file) = file {
                let mut contents = String::new();
                let _ = file.read_to_string(&mut contents);
                contents
                .split("\n")
                .enumerate()
                .for_each(|tuple| {
                    let line = Line::new(tuple.0, tuple.1.to_string());
                    self.current.add_line(line);
                })
            }
            self.current.set_cursor();
        }
    }

    pub fn get_current(&self) -> &Code {
        &self.current
    }

    pub fn get_mut_current(&mut self) -> &mut Code {
        &mut self.current
    }

    pub fn get_history(&self) -> &CodeHistory {
        &self.history
    }


}