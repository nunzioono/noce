use std::{time::Duration, io};

use crossterm::event::{self, poll};


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
    pub fn tick(&self, app: &mut App, context: &mut AppContext) -> io::Result<bool> {
            if poll(Duration::from_millis(16))? {
                if let Ok(event) = event::read() {
                    let focus = context.focus().clone();
                    Ok(app.handle_event(context, focus, event))
                } else {
                    Ok(true)
                }
            } else {
                // Timeout expired, no `Event` is available
                Ok(true)
            }
    }
}
