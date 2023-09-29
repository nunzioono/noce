use self::terminal::{ExecutedCommandHistory, TerminalCommand};

mod terminal;

#[derive(Default)]
pub struct TerminalStatus {
    pub current_command: TerminalCommand,
    commands_history: ExecutedCommandHistory
}