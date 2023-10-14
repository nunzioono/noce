pub mod systems;
pub mod state;
pub mod  unit_tests;

use state::{AppContext, App};
use systems::start;

use crate::state::{project::ProjectComponent, code::{CodeComponent, code::Code}, terminal::TerminalComponent};

fn main() {
    let context = AppContext::default();
    let app = App::new(
        ProjectComponent::new(context.active_folder().to_path_buf()),
        CodeComponent::new(Code::new()),
        TerminalComponent::new(),
        context.active_folder().clone());

    start(app, context)
    .expect("Error: panicked starting the systems");
}
