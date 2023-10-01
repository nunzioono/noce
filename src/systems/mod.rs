pub mod event_system;
pub mod ui_system;

use crate::{status::App, systems::{event_system::event_system, ui_system::ui_system}};

use std::{error::Error, io::stdout, sync::{RwLock, Arc}, thread::spawn};

use crossterm::{terminal::{enable_raw_mode, EnterAlternateScreen, disable_raw_mode, LeaveAlternateScreen}, execute, event::{EnableMouseCapture, DisableMouseCapture}};

use ratatui::{prelude::CrosstermBackend, Terminal};

pub fn start(app: Arc<RwLock<App>>) -> Result<(), Box<dyn Error>>{
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let quit = Arc::new(RwLock::new(false));
    let quit_r = Arc::clone(&quit);
    let app_r = Arc::clone(&app);

    let handle = spawn(move || {
        let _ = event_system(app, quit); 
    });

    let res = ui_system(&mut terminal, app_r, quit_r);
    handle.join().unwrap();

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}