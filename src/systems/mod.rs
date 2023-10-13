use std::{sync::{Arc, RwLock}, error::Error, io::stdout, thread};

use crossterm::{terminal::{enable_raw_mode, EnterAlternateScreen, disable_raw_mode, LeaveAlternateScreen}, execute, event::{EnableMouseCapture, DisableMouseCapture}};
use ratatui::{prelude::CrosstermBackend, Terminal};

use crate::state::{AppContext, App, terminal};

use self::{event_system::EventSystem, ui_system::UiSystem};

pub mod event_system;
pub mod ui_system;

// Define an enum for different types of systems.
#[derive(PartialEq, Eq, Hash)]
pub enum SystemType {
    Event,
    Ui,
    // Add more system types here if needed.
}

// Define a trait for systems.
pub trait System {
}



pub fn start(mut app: Arc<RwLock<App>>, context: Arc<RwLock<AppContext>>) -> Result<(), Box<dyn Error>>{
    // setup terminal
    let debugging_events = false;
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    if let Ok(terminal) = Terminal::new(backend) {
        let mut terminal_arc = Arc::new(RwLock::new(terminal));
        let terminal_clone = Arc::clone(&mut terminal_arc);
    
        let app_read_ward = Arc::clone(&app);
        let app_write_ward = Arc::clone(&mut app);
        let context_read_ward = Arc::clone(&context);
        
        let mut handle= None;   
        if !debugging_events {
            handle = Some(thread::spawn(move || {
                let _ = UiSystem::new().start(terminal_clone, app_read_ward, context_read_ward);
            }));    
        }
    
        let _ = EventSystem::new().start(app_write_ward, Arc::clone(&context));        
    
        if !debugging_events {
            handle.unwrap().join().unwrap();
        }
    
        // restore terminal
        disable_raw_mode()?;
        execute!(
            terminal_arc.write().unwrap().backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        terminal_arc.write().unwrap().show_cursor()?;
            
    }
 
    Ok(())
}