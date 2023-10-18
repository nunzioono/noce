#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Point {
    x: usize,
    y: usize
}

impl Point {
    pub fn new(x: usize, y: usize) -> Point {
        Point { x: x, y: y }
    }

    pub fn get_x(&self) -> usize {
        self.x
    }

    pub fn get_y(&self) -> usize {
        self.y
    } 

    pub fn set_x(&mut self, x: usize) {
        self.x = x;
    }

    pub fn set_y(&mut self, y: usize) {
        self.y = y;
    }
}

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

