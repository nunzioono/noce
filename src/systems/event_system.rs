use std::{
    error::Error,
    time::Duration,
    sync::{RwLock, Arc},
};


use crossterm::event;

use crate::status::App;

pub fn event_system(app: Arc<RwLock<App>>, quit: Arc<RwLock<bool>>) -> Result<(),Box<dyn Error>>{
    loop {
        if event::poll(Duration::from_millis(50))? {
            let event = event::read()?;
            if let Ok(mut write_guard) = app.write() {
                write_guard.process(event, quit.clone());
            }     
        }                        
    }
}  