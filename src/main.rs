pub mod systems;
pub mod state;
pub mod  unit_tests;

use std::env;

use state::{AppContext, App};
use systems::start;

use crate::state::{project::ProjectComponent, code::CodeComponent, terminal::TerminalComponent};

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let context = AppContext::default();
    let app = App::new(
        ProjectComponent::new(context.active_folder().to_path_buf()),
        CodeComponent::new(),
        TerminalComponent::new(),
        context.active_folder().clone());

    start(app, context)
    .expect("Error: panicked starting the systems");
}
