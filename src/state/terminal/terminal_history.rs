use std::{fmt, path::PathBuf};

// Executed Terminal Command
#[derive(Debug, PartialEq, Eq)]
pub struct ExecutedTerminalCommand {
    command: String,
    folder: PathBuf,
    output: String,
}

impl ExecutedTerminalCommand {
    pub fn new(command: String, folder: PathBuf, output: String) -> Self {
        ExecutedTerminalCommand {
            command,
            folder,
            output,
        }
    }

    pub fn get_command(&self) -> &String {
        &self.command
    }

    pub fn get_folder(&self) -> &PathBuf {
        &self.folder
    }

    pub fn get_output(&self) -> &String {
        &self.output
    }
}

impl fmt::Display for ExecutedTerminalCommand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}> {}\n{}",
            self.get_folder().display(),
            self.get_command(),
            self.get_output()
        )
    }
}

// Executed Terminal History
#[derive(Debug, PartialEq, Eq)]
pub struct ExecutedTerminalHistory {
    history: Vec<ExecutedTerminalCommand>,
    position: isize,
}

impl Default for ExecutedTerminalHistory {
    fn default() -> Self {
        ExecutedTerminalHistory {
            history: Vec::new(),
            position: 0,
        }
    }
}

impl ExecutedTerminalHistory {
    pub fn up(&mut self) -> Option<&ExecutedTerminalCommand> {
        if self.position >= 0 {
            self.position -= 1;
            self.history.get(self.position as usize )
        } else {
            None
        }
    }

    pub fn down(&mut self) -> Option<&ExecutedTerminalCommand> {
        if self.position < self.history.len().try_into().unwrap() {
            self.position += 1;
            self.history.get(self.position as usize)
        } else {
            None
        }
    }

    pub fn add(&mut self, command: ExecutedTerminalCommand) {
        self.history.push(command);
        self.position =( self.history.len()-1) as isize;
    }

    pub fn flush(&mut self) {
        self.history.clear();
        self.position = 0;
    }

    pub fn get_history(&self) -> &Vec<ExecutedTerminalCommand> {
        &self.history
    } 
}

impl fmt::Display for ExecutedTerminalHistory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (index, command) in self.history.iter().enumerate() {
            if index > 0 {
                write!(f, "\n")?;
            }
            write!(f, "{}", command)?;
        }
        Ok(())
    }
}
