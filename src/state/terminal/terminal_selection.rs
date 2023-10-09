use super::terminal_command::TerminalCommand;

// Terminal Selectionù
#[derive(Debug, PartialEq, Eq)]
pub struct TerminalSelection {
    command: Option<TerminalCommand>,
    start: usize,
    end: usize,
}

impl TerminalSelection {
    pub fn new() -> Self {
        TerminalSelection {
            command: None,
            start: 0,
            end: 0,
        }
    }

    pub fn set_command(&mut self, cmd: TerminalCommand) {
        self.command = Some(cmd);
    }

    pub fn start_selection(&mut self, start: usize, end: usize) {
        if let Some(cmd) = self.command.clone() {
            self.start = if cmd.get_buffer().chars().enumerate().filter(|tuple| tuple.0 == start).count() > 0 { start } else { 0 };
            self.end = if cmd.get_buffer().chars().enumerate().filter(|tuple| tuple.0 == end).count() > 0 { end } else { 0 };
        }
    }

    pub fn clear_selection(&mut self) {
        self.start = 0;
        self.end = 0;
    }

    pub fn is_empty(&self) -> bool {
        self.start == self.end || self.command.is_none()
    }

    pub fn get_start(&self) -> usize {
        self.start
    }

    pub fn get_end(&self) -> usize {
        self.end
    }

    pub fn get_selection(&self) -> String {
        let mut result = String::default();
        if let Some(command) = &self.command {
            result = command.get_buffer().clone();
        }
        result
    }
}