pub mod app;
pub mod code;
pub mod project;
pub mod terminal;

use crate::Mediator;

pub trait Status {
    fn process(&mut self, mediator: &mut dyn Mediator);
}