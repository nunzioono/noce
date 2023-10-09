use std::{error::Error, path::PathBuf, process::{Command, Stdio}, rc::Rc, borrow::BorrowMut};

use clipboard::{ClipboardContext, ClipboardProvider};
use crossterm::event::{Event, KeyEventKind, KeyCode, KeyModifiers, ModifierKeyCode};
use regex::Regex;


use self::{terminal_command::TerminalCommand, terminal_history::{ExecutedTerminalHistory, ExecutedTerminalCommand}, terminal_selection::TerminalSelection};

use super::{Component, ComponentType, AppContext};

pub mod terminal_command;
pub mod terminal_history;
pub mod terminal_selection;

// Terminal State
pub struct TerminalComponent {
    current_command: Rc<TerminalCommand>,
    commands_history: Rc<ExecutedTerminalHistory>,
    selection: Rc<TerminalSelection>
}

impl TerminalComponent {
    pub fn new() -> Self {
        TerminalComponent {
            current_command: Rc::new(TerminalCommand::default()),
            commands_history: Rc::new(ExecutedTerminalHistory::default()),
            selection: Rc::new(TerminalSelection::new()),
        }
    }

    pub fn get_history(&self) -> &ExecutedTerminalHistory {
        &self.commands_history
    }

    pub fn get_current_command(&self) -> &TerminalCommand {
        &self.current_command
    }

    pub fn get_selection(&self) -> &TerminalSelection {
        &self.selection
    }
}

impl Component for TerminalComponent {

    fn get_type(&self) -> ComponentType {
        ComponentType::Terminal
    }

    fn handle_event(&mut self, context: &mut AppContext, event: Event) {
        if let Event::Key(key) = event {
            let command = &*Rc::clone(&self.current_command);
            let history = &*Rc::clone(&self.commands_history);
            let selection = &*Rc::clone(&self.selection);
            let mutable_command = Rc::get_mut(&mut self.current_command);
            let mutable_history = Rc::get_mut(&mut self.commands_history);
            let mutable_selection = Rc::get_mut(&mut self.selection);

            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char(char) => {
                        let mut char_normalized = char.clone().to_string();
                        char_normalized = char_normalized.to_lowercase().to_string();
                        if char_normalized == "x" && key.modifiers.contains(KeyModifiers::CONTROL) {
                            let cut = selection.get_selection();
                            let clipboard: Result<ClipboardContext, Box<dyn Error>> = ClipboardProvider::new();
                            if let Ok(mut context) =  clipboard {
                                let _ = context.set_contents(cut);
                            } 
                        } else if char_normalized == "c" && key.modifiers.contains(KeyModifiers::CONTROL) {
                            let copy = selection.get_selection();
                            let clipboard: Result<ClipboardContext, Box<dyn Error>> = ClipboardProvider::new();
                            if let Ok(mut context) =  clipboard {
                                let _ = context.set_contents(copy);
                            } 
                        } else if char_normalized == "v" && key.modifiers.contains(KeyModifiers::CONTROL) {
                            let clipboard: Result<ClipboardContext, Box<dyn Error>> = ClipboardProvider::new();
                            if let Ok(mut context) =  clipboard {
                                if let Ok(contents) = context.get_contents() {
                                    if let Some(mutable_command) = mutable_command {
                                        mutable_command.set_buffer(command.get_buffer().to_owned() + &contents);
                                    }
                                }
                            }                  
                        } else {
                            if let Some(mutable_command) = mutable_command {
                                if let Some(mutable_selection) = mutable_selection {    
                                    mutable_selection.clear_selection();
                                    mutable_command.add(char);
                                }
                            }
                        }
                    },
                    KeyCode::Up => {
                        if let Some(mutable_command) = mutable_command {
                            if let Some(mutable_history) = mutable_history {
                                if let Some(mutable_selection) = mutable_selection {
                                    if let Some(previous_command) = mutable_history.up() {
                                        mutable_selection.clear_selection();
                                        mutable_command.set_buffer(command.get_buffer().to_owned() + previous_command.get_command());
                                    }             
                                }
                            }
                        }
                    },
                    KeyCode::Down => {
                        if let Some(mutable_command) = mutable_command {
                            if let Some(mutable_history) = mutable_history {
                                if let Some(mutable_selection) = mutable_selection {
                                    if let Some(previous_command) = mutable_history.up() {
                                        mutable_selection.clear_selection();
                                        mutable_command.set_buffer(command.get_buffer().to_owned() + previous_command.get_command());
                                    }             
                                }
                            }
                        }
                    },
                    KeyCode::Left => {
                        if let Some(mutable_command) = mutable_command {
                            if let Some(mutable_selection) = mutable_selection {    
                                mutable_selection.clear_selection();
                                mutable_command.move_cursor_backward();
                            }
                        }
                    },
                    KeyCode::Right => {
                        if let Some(mutable_command) = mutable_command {
                            if let Some(mutable_selection) = mutable_selection {    
                                mutable_selection.clear_selection();
                                mutable_command.move_cursor_forward();
                            }
                        }
                    },
                    KeyCode::Enter => {
                        if let Some(mutable_selection) = mutable_selection {
                            if let Some(mutable_history) = mutable_history {
                                if let Some(mutable_command) = mutable_command {
                                    mutable_selection.clear_selection();
                                    let command_string: String = command.get_buffer().clone();
                                    let re = Regex::new(r#""[^"]+"|\S+"#);
                                    if let Ok(re) = re {

                                        let command_args: Vec<&str> = re.find_iter(command_string.as_str())
                                        .map(|m| m.as_str())
                                        .collect();
                                        let mut command_output: String = String::from("");

                                        if command_string.len() > 0 {

                                            if *command_args.get(0).unwrap() == "cd" && command_args.len() == 2 {
                                                let folder_to_access = *command_args.get(1).unwrap();
                                                let mut path: PathBuf = PathBuf::new();

                                                path.push(context.active_folder());
                                                path.push(folder_to_access);
                                    
                                                if path.is_dir() {
                                                    context.set_active_folder(path);
                                                }
                                        
                                            } else if *command_args.get(0).unwrap() == "cls" || *command_args.get(0).unwrap() == "clear" {    
                                                                                
                                                mutable_history.flush();
                                                mutable_command.flush();    

                                            } else if command_args.len() > 0 { 
                                    
                                                let output = if cfg!(target_os = "windows") {
                                                    Command::new("powershell")
                                                            .args(&["-c", command_string.as_str().clone()])
                                                            .current_dir(context.active_folder().display().to_string())
                                                            .stdout(Stdio::piped())
                                                            .stderr(Stdio::piped())
                                                            .output()
                                                            .expect("failed to execute process")
                                                } else {
                                                    Command::new("sh")
                                                            .arg("-c")
                                                            .arg(command_string.clone())
                                                            .current_dir(context.active_folder().display().to_string())
                                                            .stdout(Stdio::piped())
                                                            .stderr(Stdio::piped())
                                                            .output()
                                                            .expect("failed to execute process")
                                                };
                                    
                                                let mut is_error = false;
                                                if let Ok(error_string) = String::from_utf8(output.stderr) {
                                                    if error_string.len() > 0 {
                                                        is_error = true;
                                                        command_output = error_string;
                                                    }
                                                }
                                                if let Ok (output_string) = String::from_utf8(output.stdout) {
                                                    if output_string.len() > 0 && !is_error {
                                                        command_output = output_string;
                                                    }
                                                }
                                                
                                            }

                                        }

                                        mutable_command.flush();
                                        mutable_history.add(ExecutedTerminalCommand::new(command_string, context.active_folder().clone(), command_output.clone()));
                                    }

                                }
                            }
                        }
                        
                    },
                    KeyCode::Delete => {
                        if let Some(mutable_selection) = mutable_selection {
                            if let Some(mutable_command) = mutable_command {
                                if !selection.is_empty() {
                                    mutable_selection.clear_selection();
                                }    
                                mutable_command.remove();    
                            }
                        }
                    },
                    KeyCode::Modifier(ModifierKeyCode::LeftShift) => {
                        if let Some(mutable_command) = mutable_command {
                            if let Some(mutable_selection) = mutable_selection {
                                let current_command = command;
                                let pos = command.get_position();

                                if selection.is_empty() {
                                    mutable_selection.start_selection(pos - 1, pos);
                                } else {
                                    let start_selection =  selection.get_start();
                                    let end_selection = selection.get_end();
                                    mutable_selection.set_command(current_command.clone());
                                    mutable_selection.start_selection( start_selection- 1, end_selection);    
                                }
                                mutable_command.set_position(pos - 1);
                            }
                        }
                    },
                    KeyCode::Modifier(ModifierKeyCode::RightShift) => {
                        if let Some(mutable_command) = mutable_command {
                            if let Some(mutable_selection) = mutable_selection {
                                let pos = command.get_position();

                                if selection.is_empty() {
                                    mutable_selection.start_selection(pos + 1, pos);
                                } else {
                                    let start_selection =  selection.get_start();
                                    let end_selection = selection.get_end();
                                    mutable_selection.set_command(command.clone());
                                    mutable_selection.start_selection( start_selection+ 1, end_selection);    
                                }
                                mutable_command.set_position(pos + 1);                    
                            }
                        }
                    },
                    KeyCode::Esc => {
                        context.set_focus(None);
                        context.set_hover(self.get_type());                                          
                    },
                    _ => {}
                }
            } else if key.kind == KeyEventKind::Repeat {
                match key.code {
                    KeyCode::Char(char) => {
                        if let Some(mutable_command) = mutable_command {
                            mutable_command.add(char);
                        }   
                    },
                    KeyCode::Up => {
                        if let Some(mutable_command) = mutable_command {
                            if let Some(mutable_history) = mutable_history {
                                if let Some(previous_command) = mutable_history.up() {
                                    mutable_command.set_buffer(command.get_buffer().to_owned() + previous_command.get_command());
                                }         
                            }
                        }
                    },
                    KeyCode::Down => {
                        if let Some(mutable_command) = mutable_command {
                            if let Some(mutable_history) = mutable_history {
                                if let Some(newer_command) = mutable_history.down() {
                                    mutable_command.set_buffer(command.get_buffer().to_owned() + newer_command.get_command());
                                } 
                            }
                        }
                    },
                    KeyCode::Left => {
                        if let Some(mutable_command) = mutable_command {
                            let position = command.get_position();
                            if position > 0 {
                                mutable_command.set_position(position - 1);
                            }    
                        }
                    },
                    KeyCode::Right => {
                        if let Some(mutable_command) = mutable_command {
                            let position = command.get_position();
                            if position > 0 {
                                mutable_command.set_position(position + 1);
                            }    
                        }
                    },
                    KeyCode::Delete => {
                        if let Some(mutable_command) = mutable_command {
                            if let Some(mutable_selection) = mutable_selection {
                                if !selection.is_empty() {
                                    mutable_selection.clear_selection();
                                }
                                mutable_command.remove();
                            }
                        }
                    },
                    KeyCode::Modifier(ModifierKeyCode::LeftShift) => {
                        if let Some(mutable_selection) = mutable_selection {
                            if let Some(mutable_command) = mutable_command {
                            let pos = command.get_position();

                            if selection.is_empty() {
                                mutable_selection.start_selection(pos - 1, pos);
                            } else {
                                let start_selection =  selection.get_start();
                                let end_selection = selection.get_end();
                                mutable_selection.set_command(command.clone());
                                mutable_selection.start_selection( start_selection- 1, end_selection);    
                            }
                            mutable_command.set_position(pos - 1);
                            }
                        }
                    },
                    KeyCode::Modifier(ModifierKeyCode::RightShift) => {
                        if let Some(mutable_selection) = mutable_selection {
                            if let Some(mutable_command) = mutable_command {
                                let pos = command.get_position();

                                if selection.is_empty() {
                                    mutable_selection.start_selection(pos + 1, pos);
                                } else {
                                    let start_selection =  selection.get_start();
                                    let end_selection = selection.get_end();
                                    mutable_selection.set_command(command.clone());
                                    mutable_selection.start_selection( start_selection+ 1, end_selection);    
                                }
                                mutable_command.set_position(pos + 1);
                            }
                        }
                    },
                    _ => {}
                }
            } 
        }
    }
}