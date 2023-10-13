use std::{time::Duration, ops::ControlFlow};

use crossterm::event;

use crate::state::{AppContext, App};

// Define a generic EventSystem struct implementing the System trait.
pub struct EventSystem {
    // Add your fields here if needed.
}

impl EventSystem {
    pub fn new() -> Self {
        EventSystem {
            // Initialize your fields here if needed.
        }
    }
}

impl EventSystem {
    pub fn start(&self, app: &mut App, context: &mut AppContext,) -> ControlFlow<()>{
        if let Ok(_) = event::poll(Duration::from_millis(50)) {
            if let Ok(event) = event::read() {
                let focus = context.focus().clone();
                let res = app.handle_event(context, focus, event);
                return res;
            }
        }
        ControlFlow::Continue(())
    }
}
