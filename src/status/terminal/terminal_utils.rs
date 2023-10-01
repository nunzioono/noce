use std::{path::PathBuf, fmt::Display, ops::{Add, AddAssign}};

use itertools::Itertools;

#[derive(Default, Clone)]
pub struct TerminalCommand {
    command_buffer: String,
    position: u16,
}

impl Display for TerminalCommand {
    
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = f.write_str(&self.command_buffer.clone().as_str());
        Ok(())
    }

}

impl Add<String> for TerminalCommand {
    type Output = String;
    
    fn add(self, rhs: String) -> Self::Output {
        self.command_buffer + &rhs
    }

}

impl AddAssign<String> for TerminalCommand {
    fn add_assign(&mut self, rhs: String) {
        self.command_buffer += &rhs;
    }

}
impl Add<TerminalCommand> for TerminalCommand {
    type Output = String;
    
    fn add(self, rhs: TerminalCommand) -> Self::Output {
        self.command_buffer + &rhs.command_buffer
    }

}

impl TerminalCommand {

    pub fn flush(&mut self) {
        self.command_buffer.clear();
    }

    pub fn remove(&mut self) {
        if self.command_buffer.len() > 0 {
            let mut char_vec: Vec<char> = self.command_buffer.chars().collect();
            char_vec.remove(self.position as usize);
            self.command_buffer = char_vec.into_iter().collect();
        }
    }

    pub fn move_cursor_forward(&mut self) {
        self.position+=1;
    }

    pub fn move_cursor_backward(&mut self) {
        self.position-=1;
    }

    pub fn get_position(&self) -> u16 {
        self.position
    }
}

#[derive(Clone)]
pub struct ExecutedTerminalCommand {
    command: String,
    folder: PathBuf,
    output: String
}

impl Display for ExecutedTerminalCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&(self.folder.display().to_string().as_str().to_owned()+">"+self.command.as_str()+"\n"+self.output.as_str()))
    }
}

impl ExecutedTerminalCommand {
    pub fn new(command: String, folder: PathBuf, output: String) -> ExecutedTerminalCommand {
        ExecutedTerminalCommand { command: command, folder: folder, output: output }
    }

    pub fn get_command(&self) -> String {
        self.command.clone()
    }

    pub fn get_folder(&self) -> PathBuf {
        self.folder.clone()
    }

    pub fn get_output(&self) -> String {
        self.output.clone()
    }

    pub fn to_command(&self) -> TerminalCommand {
        TerminalCommand { command_buffer: self.command.clone(), position: 0 }
    }
}

#[derive(Default)]
pub struct ExecutedCommandHistory {
    history: Vec<ExecutedTerminalCommand>,
    position: u8
}

impl Display for ExecutedCommandHistory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.history.iter().map(|executed_command| executed_command.to_string()).join("\n").as_str())
    }
}

impl ExecutedCommandHistory {
    pub fn up(&mut self) -> ExecutedTerminalCommand {
        if self.position>0 {
            self.position-=1;
        }
        self.history[self.position as usize].clone()
    }

    pub fn down(&mut self) -> ExecutedTerminalCommand {
        if self.position<29 {
            self.position+=1;
        }
        self.history[self.position as usize].clone()
    }

    pub fn add(&mut self, command: ExecutedTerminalCommand) {
        if self.history.len() == 30 {
            self.history.remove(0);
        }
        self.history.push(command);
        self.position = self.history.len() as u8 -1 ;
    }

    pub fn flush(&mut self) {
        self.history.clear();
        self.position = 0;
    }
}