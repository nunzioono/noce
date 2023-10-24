
use crate::state::code::code_utils::Point;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct CodeSelection {
    start: Point,
    end: Point
}

impl CodeSelection {

    pub fn new(start: Point, end: Point) -> CodeSelection {
        CodeSelection {
            start,
            end
        }
    }

    pub fn get_start(&self) -> &Point {
        &self.start
    }

    pub fn get_end(&self) -> &Point {
        &self.end
    }

    pub fn set_start(&mut self, start: Point) {
        self.start = start;
    }

    pub fn set_end(&mut self, end: Point) {
        self.end = end;
    }

}

