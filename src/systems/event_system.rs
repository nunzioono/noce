use std::{time::Duration, sync::{Arc, RwLock}, error::Error};

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
    pub fn start(&self, app: Arc<RwLock<App>>, context: Arc<RwLock<AppContext>>,) -> Result<(),Box<dyn Error>>{
        loop {
            if let Ok(_) = event::poll(Duration::from_millis(50)) {
                let event = event::read();
                if let Ok(event) = event {
                    let focus;
                    if let Ok(read_guard) = context.read() {
                        focus = read_guard.focus();
                        if let Ok(mut context_write_guard) = context.write() {
                            if let Ok(mut app_write_guard) = app.write() {                        
                                app_write_guard.handle_event(&mut context_write_guard, focus.clone(), event);
                            }
                        }   
                    }
                }
            }                        
        }
    }
}
