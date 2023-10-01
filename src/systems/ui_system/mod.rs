use std::{sync::{Arc, RwLock}, error::Error};

use ratatui::{prelude::Backend, Terminal, Frame};

use crate::status::App;

pub fn ui_system<B: Backend>(
    terminal: &mut Terminal<B>,
    app: Arc<RwLock<App>>,
    quit: Arc<RwLock<bool>>
) -> Result<(),Box<dyn Error>> {
    loop {
        if let Ok(read_guard) = quit.read() {
            if *read_guard {
                return Ok(())
            }
        }
        if let Ok(read_guard) = app.read() {
            terminal.draw(|f| ui(f, &read_guard))?;
        }     
    }
 
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    
}