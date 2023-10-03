mod systems;
pub mod status;

use std::sync::{RwLock, Arc};

use systems::start;
use status::App;

fn main() {
    //setting the status of the app
    let app = Arc::new(RwLock::new(App::default()));

    start(app)
    .expect("Error: panicked starting the systems");
}
