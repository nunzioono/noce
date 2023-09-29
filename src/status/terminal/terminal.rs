use std::path::PathBuf;

#[derive(Default)]
pub struct TerminalCommand {
    command_buffer: String,
    position: u16,
}

#[derive(Default)]
pub struct ExecutedTerminalCommand {
    command: String,
    folder: PathBuf,
    output: String
}

#[derive(Default)]
pub struct ExecutedCommandHistory {
    history: Vec<ExecutedTerminalCommand>,
    position: u8
}