pub mod code_history;
pub mod code_selection;
pub mod code;
pub mod code_utils;

use std::{fs::File, io::Read, path::PathBuf};
use self::{code::{Code, Line}, code_history::CodeHistory, code_utils::{handle_up, handle_down, handle_left, handle_right, handle_cut, handle_copy, handle_paste, handle_save, handle_undo, handle_redo, handle_char, handle_delete, handle_enter}};
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
            self.get_mut_current().remove_cursor();
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char(char) => {
                        let mut char_normalized = char.clone().to_string();
                        char_normalized = char_normalized.to_lowercase().to_string();
                        if char_normalized == "x" && key.modifiers.contains(KeyModifiers::CONTROL) {
                            handle_cut(self);
                        } else if char_normalized == "c" && key.modifiers.contains(KeyModifiers::CONTROL) {
                            handle_copy(self);                            
                        } else if char_normalized == "v" && key.modifiers.contains(KeyModifiers::CONTROL) {
                            handle_paste(self);
                        } else if char_normalized == "s" && key.modifiers.contains(KeyModifiers::CONTROL) {
                            handle_save(self, context);
                        } else if char_normalized == "z" && key.modifiers.contains(KeyModifiers::CONTROL) {
                            handle_undo(self);                            
                        } else if char_normalized == "y" && key.modifiers.contains(KeyModifiers::CONTROL) {
                            handle_redo(self);
                        } else {
                            handle_char(self, char.to_string());
                        }
                    },
                    KeyCode::Backspace => {
                        handle_delete(self);
                    },
                    KeyCode::Enter => {
                        handle_enter(self);
                    },
                    KeyCode::Up => {
                        handle_up(self, event.clone());
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
                    KeyCode::Esc => {
                        context.set_focus(None);
                        context.set_hover(self.get_type());             
                    },
                    _ => {}
                }
            } if key.kind == KeyEventKind::Repeat {
                match key.code {
                    KeyCode::Char(char) => {
                        handle_char(self, char.to_string())
                    },
                    KeyCode::Backspace => {
                        handle_delete(self);
                    },
                    KeyCode::Enter => {
                        handle_enter(self);
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
        self.get_mut_current().set_cursor();

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