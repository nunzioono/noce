pub mod systems;
pub mod state;

use std::sync::{Arc, RwLock};

use state::{AppContext, App};
use systems::start;

fn main() {
    println!("Starting the program, creating the context...");
    let context = AppContext::default();
    println!("App state settings...");
    let app = App::default();
    println!("App created! Starting event and ui systems");
    start(Arc::new(RwLock::new(app)), Arc::new(RwLock::new(context)))
    .expect("Error: panicked starting the systems");
}
