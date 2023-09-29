mod systems;
mod status;

use std::collections::VecDeque;

use crossterm::event::Event;
use status::code::CodeStatus;
use status::project::ProjectStatus;
use status::terminal::TerminalStatus;

use crate::status::{Status, app::AppStatus, code, project, terminal};
use crate::systems::{System, event_system, ui_system};

pub trait Mediator {
    fn notify_event(&mut self, event: Event);
    fn notify_ui(&mut self);
    fn read_status(&mut self);
}

#[derive(Default)]
pub struct MainMediator {
    app: AppStatus,
    code: CodeStatus,
    project: ProjectStatus,
    terminal: TerminalStatus,
    systems: Vec<Box<dyn System>>,
}

impl Mediator for MainMediator {
    //to be called by the event system
    fn notify_event(&mut self, event: Event) {
        
    }

    //to be called by a status
    fn notify_ui(&mut self) {
        
    }

    //to be called by the ui system
    fn read_status(&mut self) {}
}

impl MainMediator {
    
    pub fn accept_component(&mut self, mut component: impl Status + 'static) {
    }

    pub fn accept_system(&mut self, mut system: impl System + 'static) {
        self.systems.push(Box::new(system));
    }
}

fn main() {
    let mut mediator = MainMediator::default();
    let app = AppStatus::default();

    mediator.accept_component(app);
}
