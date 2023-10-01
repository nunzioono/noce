use std::path::PathBuf;

use crossterm::event::Event;

#[derive(Default, Clone)]
pub struct ProjectState {
    contents: Vec<PathBuf>,
    hover: Option<u16>,
    focus: Option<u16>
}

impl ProjectState {
    pub fn process(&mut self, event: Event) {
    }
}