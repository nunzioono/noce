use std::path::PathBuf;

use crate::Mediator;

use super::Status;

enum Panel {
    Code,
    Project,
    Terminal,
}

#[derive(Default)]
pub struct AppStatus {
    pub folder: PathBuf,
    hover: Option<Panel>,
    focus: Option<Panel>,
}

impl Status for AppStatus {
    
    fn process(&mut self, mediator: &mut dyn Mediator) {
        
    }

}