pub mod systems;
pub mod state;
pub mod  unit_tests;
use std::{sync::{Arc, RwLock}, env, path::PathBuf};

use state::{AppContext, App};
use systems::start;

use crate::state::{project::ProjectComponent, code::{CodeComponent, code::Code}, terminal::TerminalComponent};

fn main() {
    let context = AppContext::default();
    let vec_contents: Vec<PathBuf> = context.active_folder().read_dir().unwrap().into_iter().map(|entry| entry.unwrap().path()).collect();
    let app = App::new(
        ProjectComponent::new(vec_contents),
        CodeComponent::new(Code::new()),
        TerminalComponent::new(),
        context.active_folder().clone());

    start(Arc::new(RwLock::new(app)), Arc::new(RwLock::new(context)))
    .expect("Error: panicked starting the systems");
}
