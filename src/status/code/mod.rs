mod code_utils;

use std::path::PathBuf;

use crossterm::event::Event;

use self::code_utils::{Code, CodeHistory};

#[derive(Default, Clone)]
pub struct CodeState {
    file: PathBuf,
    current: Code,
    history: CodeHistory,
    selection: Option<Code>,
}

impl CodeState {
    
    pub fn process(&mut self, event: Event) {

    }

}