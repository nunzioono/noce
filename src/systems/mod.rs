use std::{error::Error, io::stdout};

use crossterm::{terminal::{enable_raw_mode, EnterAlternateScreen, disable_raw_mode, LeaveAlternateScreen}, execute};
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



pub fn start(mut app: App, mut context: AppContext, debugging_events: bool) -> Result<(), Box<dyn Error>>{
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend);
//    let fps = 60;
    let ui_system = UiSystem::new();
    let event_system = EventSystem::new();

    if let Ok(mut terminal) = terminal {
        let mut nframe = 0;

        loop {
            {
                let current_code = app.get_mut_code(); 
                //blink the cursor on frame update
                if context.active_file().is_some() {
                    if nframe%30 == 0 {
                        current_code.get_mut_current().remove_cursor();
                    } else {
                        current_code.get_mut_current().set_cursor();
                    }
                }
            

                //set the new file if the active file changed
                if context.active_file_changed() {
                    current_code.get_mut_current().flush();
                    current_code.set_current(context.active_file().clone());
                    context.set_active_file_changed(false);
                }

            }

            //update the ui on the upcoming frame (if debugging events do not display the ui)
            if !debugging_events {
                let _ = ui_system.tick(&mut terminal, &app, &context);    
            }


            //receive the next user input event
            let res = event_system.tick(&mut app, &mut context);  
            if let Ok(res) = res {
                if !res {
                    break;
                }
            }
            
            nframe += 1;
        }

        // restore terminal
        disable_raw_mode()?;
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen
        )?;
        terminal.show_cursor()?;
    }
        
    Ok(())
}