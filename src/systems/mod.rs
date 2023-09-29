pub mod event_system;
pub mod ui_system;

pub trait System {
    
}

/*
fn main() -> Result<(), Error> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    loop {
        //ui
        terminal.draw(|f| ui(f, &app))?;

        //event
        if let Event::Key(key) = event::read()? {
            
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

    Ok(())
} */