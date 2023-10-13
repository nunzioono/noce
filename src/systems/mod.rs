use std::{error::Error, io::stdout, ops::ControlFlow};

use crossterm::{terminal::{enable_raw_mode, EnterAlternateScreen, disable_raw_mode, LeaveAlternateScreen}, execute, event::{EnableMouseCapture, DisableMouseCapture}};
use ratatui::{prelude::CrosstermBackend, Terminal};

use crate::state::{AppContext, App};

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



pub fn start(mut app: App, mut context: AppContext) -> Result<(), Box<dyn Error>>{
    // setup terminal
    let debugging_events = false;

    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend);

    if let Ok(mut terminal) = terminal {

        loop {
            if !debugging_events {
                let _ = UiSystem::new().start(&mut terminal, &app, &context);    
            }
            let res = EventSystem::new().start(&mut app, &mut context);  
            match res {
                ControlFlow::Break(()) => {
                    break;
                },
                _ => {}
            }              
        }

        // restore terminal
        disable_raw_mode()?;
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        terminal.show_cursor()?;
    }
        
    Ok(())
}