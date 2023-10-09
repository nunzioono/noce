use std::{sync::{Arc, RwLock}, error::Error, io::stdout};

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

pub fn start(mut app: Arc<RwLock<App>>, context: Arc<RwLock<AppContext>>) -> Result<(), Box<dyn Error>>{
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let _ = EventSystem::new().start(Arc::clone(&mut app), Arc::clone(&context));
    let _ = UiSystem::new().start(&mut terminal, Arc::clone(&app), Arc::clone(&context));

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
 
    Ok(())
}