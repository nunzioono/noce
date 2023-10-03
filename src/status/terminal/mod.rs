extern crate regex;
extern crate clipboard;

pub mod terminal_utils;

use std::{path::PathBuf, env, fs::metadata, process::{Command, Stdio}, ops::AddAssign};

use clipboard::{ClipboardProvider, ClipboardContext};
use crossterm::event::{Event, KeyCode, ModifierKeyCode, KeyModifiers, KeyEventKind};
use itertools::Itertools;
use regex::Regex;

use self::terminal_utils::{ExecutedCommandHistory, TerminalCommand, ExecutedTerminalCommand};

struct TerminalSelection {
    command: String,
    start: u16,
    end: u16
}

impl AddAssign<String> for TerminalSelection {
    fn add_assign(&mut self, rhs: String) {
        self.command = self.command.to_string() + &rhs
    }
}

impl TerminalSelection {

    fn new(cmd: &TerminalCommand) -> TerminalSelection {
        TerminalSelection {
            command: cmd.to_string(),
            start: cmd.get_position(),
            end: cmd.get_position()
        }
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
        self.command.clone()
            .chars()
            .into_iter()
            .enumerate()
            .filter(|(i,_el)| (*i as u16)>=self.start && (*i as u16)<self.end)
            .map(|(_i,el)| el).join("").to_string()
    }

    pub fn flush(&mut self) {
        self.command = "".to_string();
        self.start = 0;
        self.end = 0;
    }
}

pub struct TerminalState {
    current_command: TerminalCommand,
    commands_history: ExecutedCommandHistory,
    command_selection: TerminalSelection
}

impl TerminalState {

    pub fn new() -> Self {
        TerminalState { current_command: TerminalCommand::default(), commands_history: ExecutedCommandHistory::default(), command_selection: TerminalSelection::new(&TerminalCommand::default()) }
    }

    pub fn run_command(&mut self, ref_folder: &mut PathBuf) {

        let command_string: String = self.current_command.to_string();
        let re = Regex::new(r#""[^"]+"|\S+"#).unwrap();
        let command_args: Vec<&str> = re.find_iter(command_string.as_str())
            .map(|m| m.as_str())
            .collect();
        let mut command_output: String = String::from("");
        let mut change_folder = false;
        let mut path: Option<String>= None;

        if command_string.len() > 0 {

            if *command_args.get(0).unwrap() == "cd" && command_args.len() == 2 {
                let mut root = env::current_dir().unwrap().clone();
    
                while !root.as_path().has_root() {
                    root = root.parent().unwrap().to_path_buf();
                }
                let string_root=root.display().to_string();
    
                let folder_to_access = (*command_args.get(1).unwrap()).to_string().clone();
                let is_relative = if !folder_to_access.starts_with(&string_root.clone()) { true } else { false };
                let new_path = if is_relative { format!("{}\\{}",ref_folder.display(),folder_to_access) } else { format!("{}",folder_to_access)};
                path = Some(new_path.clone());
    
                if let Ok(metadata) = metadata(path.clone().unwrap()) {
    
                    // Check that the second arg is a real folder
                    let is_dir = metadata.is_dir();
                    if is_dir {
    
                        change_folder = true;
                    }
    
    
                } 
            } else if *command_args.get(0).unwrap() == "cls" || *command_args.get(0).unwrap() == "clear" {
    
                self.commands_history.flush();
                self.current_command.flush();
    
            } else if command_args.len() > 0 { 
    
                let output = if cfg!(target_os = "windows") {
                    Command::new("powershell")
                            .args(&["-c", command_string.as_str().clone()])
                            .current_dir(ref_folder.display().to_string())
                            .stdout(Stdio::piped())
                            .stderr(Stdio::piped())
                            .output()
                            .expect("failed to execute process")
                } else {
                    Command::new("sh")
                            .arg("-c")
                            .arg(command_string.clone())
                            .current_dir(ref_folder.display().to_string())
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
        

        self.current_command.flush();
        self.commands_history.add(ExecutedTerminalCommand::new(command_string, ref_folder.clone(), command_output.clone()));
        if change_folder {
            ref_folder.push(path.unwrap());
        }

    }

    pub fn process(&mut self, event: Event, folder_ref: &mut PathBuf) {
        if let Event::Key(key) = event {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char(char) => {
                        if char == 'c' && key.modifiers.contains(KeyModifiers::CONTROL) {
                            //copy
                            let mut copy = String::from("");
                            if self.command_selection.is_selection_started() {
                                copy += &self.command_selection.get_selection().clone();
                            } else {
                                copy += &self.current_command.to_string().clone();
                            }
                            let mut ctx: ClipboardContext= ClipboardProvider::new().unwrap();
                            let _ = ctx.set_contents(self.command_selection.get_selection());    
                        } else if char == 'x' && key.modifiers.contains(KeyModifiers::CONTROL) {
                            //cut
                            let mut cut = String::from("");
                            if self.command_selection.is_selection_started() {
                                cut += &self.command_selection.get_selection().clone();
                            } else {
                                cut += &self.current_command.to_string().clone();
                            }
                            let _ = self.current_command.to_string().replace(&cut.clone(), "");
                            let mut ctx: ClipboardContext= ClipboardProvider::new().unwrap();
                            let _ = ctx.set_contents(cut.clone());
                        } else {
                            self.current_command+=char.to_string()
                        }
                    },
                    KeyCode::Up => {self.current_command = self.commands_history.up().to_command()},
                    KeyCode::Down => {self.current_command = self.commands_history.down().to_command()},
                    KeyCode::Left => {self.current_command.move_cursor_backward()},
                    KeyCode::Right => {self.current_command.move_cursor_forward()},
                    KeyCode::Enter => {self.run_command(folder_ref)},
                    KeyCode::Delete => {self.current_command.remove()},
                    KeyCode::Modifier(ModifierKeyCode::LeftShift) => {
                        self.command_selection.flush();
                        self.command_selection.command = self.current_command.to_string();
                        self.command_selection.start_selection(self.current_command.get_position());
                        self.command_selection.select_left();
                    },
                    KeyCode::Modifier(ModifierKeyCode::RightShift) => {
                        self.command_selection.flush();
                        self.command_selection.command = self.current_command.to_string();
                        self.command_selection.start_selection(self.current_command.get_position());
                        self.command_selection.select_right();
                    },
                    _ => {}
                }
            } else if key.kind == KeyEventKind::Repeat {
                match key.code {
                    KeyCode::Char(any) => {
                        self.current_command+=any.to_string()
                    },
                    KeyCode::Up => {self.current_command = self.commands_history.up().to_command()},
                    KeyCode::Down => {self.current_command = self.commands_history.down().to_command()},
                    KeyCode::Left => {self.current_command.move_cursor_backward()},
                    KeyCode::Right => {self.current_command.move_cursor_forward()},
                    KeyCode::Modifier(ModifierKeyCode::LeftShift) => {
                        if self.command_selection.is_selection_started() {
                            self.command_selection.select_left();
                        }
                    },
                    KeyCode::Modifier(ModifierKeyCode::RightShift) => {
                        if self.command_selection.is_selection_started() {
                            self.command_selection.select_right();
                        }
                    },
                    _ => {}
                }
            }
        } else if let Event::Paste(word) = event {
            self.current_command+=word;            
        }
    }
}