use crossterm::event::{Event, KeyModifiers};

use super::{CodeComponent, code_selection::CodeSelection};
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

    pub fn move_up(&mut self, exceed: bool, limit: usize) {
        if !exceed && self.x > 0 {
            self.x -=1;
            if self.y > limit - 1 {
                self.y = limit - 1;
            }
        } else if exceed {
            self.y = 0;
        }
    } 

    pub fn move_left(&mut self, exceed: bool, limit: usize) {
        if self.y > 0 {
            self.y -= 1;
        } else if exceed && self.y == 0 {
            self.x -= 1;
            self.y = limit - 1;
        }
    }

    pub fn move_right(&mut self, exceed: bool, limit: usize) {
        if self.y < limit {
            self.y += 1;
        } else if exceed && self.y == limit {
            self.x += 1;
            self.y = 0;
        }
    }

    pub fn move_down(&mut self, exceed: bool, limit: usize, length: usize) {
        if !exceed {
            self.x += 1;
            if self.y > length {
                self.y = length;
            }
        } else if exceed {
            self.y = limit;
        }
    }

    pub fn set_x(&mut self, x: usize) {
        self.x = x;
    }

    pub fn set_y(&mut self, y: usize) {
        self.y = y;
    }
}

pub fn handle_up(code_component: &mut CodeComponent, event: Event) {
    let readable_current_code = code_component.get_current().clone();
    let readable_cursor = readable_current_code.get_cursor().clone();
    let mut is_selecting = false;
    let mut readable_selection = CodeSelection::default();
    let mut is_shift = false;
    let mut current_size = 0;
    let mut upper_size = 0;

    if let Some(selection) = readable_current_code.get_selection() {
        is_selecting = true;
        readable_selection = selection.clone();
    }

    if let Some(current) = readable_current_code.get_line(readable_cursor.get_x()) {
        current_size = current.get_string().len();
    } 

    if readable_current_code.get_cursor().get_x() > 0 {
       if let Some(upper) = readable_current_code.get_line(readable_cursor.get_x() - 1) {
            upper_size = upper.get_string().len();
       } 
    }

    if let Event::Key(key) = event {
    
        if key.modifiers.contains(KeyModifiers::SHIFT) {
            is_shift = true;
        }

        let mutable_code = code_component.get_mut_current();
        let mut current_selection_end = readable_selection.get_end().clone();

        if is_selecting && is_shift {

            if readable_cursor.get_x() > 0 {
                mutable_code.get_mut_cursor().move_up(false, upper_size);
                let current_selection_start = readable_selection.get_start().clone();
                current_selection_end.move_up(false, upper_size);
                if current_selection_start == current_selection_end {
                    mutable_code.flush_selection();
                } else {
                    mutable_code.set_selection_end(current_selection_end);
                }
            } else {
                current_selection_end.move_up(true, upper_size);
                current_selection_end.move_right(true, current_size);
                mutable_code.get_mut_cursor().move_up(true, upper_size);
                mutable_code.set_selection_end(current_selection_end);
            }


        } else if is_selecting && !is_shift {

            
            let end = readable_selection.get_end();
            let start = readable_selection.get_start();

            if start.get_x() > end.get_x() {
                mutable_code.get_mut_cursor().set_x(end.get_x());
                mutable_code.get_mut_cursor().set_y(end.get_y());    
            } else {
                mutable_code.get_mut_cursor().set_x(start.get_x());
                mutable_code.get_mut_cursor().set_y(start.get_y());    
            }
            mutable_code.flush_selection();
            

        } else if !is_selecting && is_shift {

            if readable_cursor.get_x() > 0 {
                current_selection_end = readable_cursor.clone();
                current_selection_end.move_up(false, upper_size);
                current_selection_end.move_right(false, current_size);
                mutable_code.create_selection(readable_cursor, current_selection_end.clone());
                mutable_code.get_mut_cursor().move_up(false, upper_size);
            } else {
                current_selection_end = readable_cursor.clone();
                current_selection_end.move_up(true, upper_size);
                current_selection_end.move_right(false, current_size);
                mutable_code.create_selection(readable_cursor, current_selection_end.clone());
                mutable_code.get_mut_cursor().move_up(true, upper_size);
            }

        } else if !is_selecting && !is_shift {

            if readable_cursor.get_x() > 0 {
                mutable_code.get_mut_cursor().move_up(false, upper_size);
            } else {
                mutable_code.get_mut_cursor().move_up(true, upper_size);
            }
        }


    }

}

pub fn handle_down(code_component: &mut CodeComponent, event: Event) {
    
    let readable_current_code = code_component.get_current().clone();
    let readable_cursor = readable_current_code.get_cursor().clone();
    let mut is_selecting = false;
    let mut readable_selection = CodeSelection::default();
    let mut is_shift = false;
    let mut current_size = 0;
    let mut lower_size = 0;
    let nlines = readable_current_code.get_content().len();

    if let Some(selection) = readable_current_code.get_selection() {
        is_selecting = true;
        readable_selection = selection.clone();
    }

    if let Some(current) = readable_current_code.get_line(readable_cursor.get_x()) {
        current_size = current.get_string().len();
    }

    if let Some(lower) = readable_current_code.get_line(readable_cursor.get_x() + 1) {
        lower_size = lower.get_string().len();
    }

    if let Event::Key(key) = event {
    
        if key.modifiers.contains(KeyModifiers::SHIFT) {
            is_shift = true;
        }

        let mutable_code = code_component.get_mut_current();
        let mut current_selection_end = readable_selection.get_end().clone();

        if is_selecting && is_shift {

            if readable_cursor.get_x() < nlines - 1 {
                current_selection_end.move_down(false, current_size, lower_size);
                mutable_code.get_mut_cursor().move_down(false, current_size, lower_size);
                let current_selection_start = readable_selection.get_start().clone();
                if current_selection_start.get_x() == current_selection_end.get_x() && current_selection_start.get_y().abs_diff(current_selection_end.get_y()) <= 1 {
                    mutable_code.flush_selection();
                } else {
                    mutable_code.set_selection_end(current_selection_end);
                }
            } else {
                current_selection_end.move_down(true, current_size, lower_size);
                mutable_code.get_mut_cursor().move_down(true, current_size, lower_size);
                mutable_code.set_selection_end(current_selection_end);
            }



        } else if is_selecting && !is_shift {

            let end = readable_selection.get_end();
            let start = readable_selection.get_start();

            if start.get_x() < end.get_x() {
                mutable_code.get_mut_cursor().set_x(end.get_x());
                mutable_code.get_mut_cursor().set_y(end.get_y());    
            } else {
                mutable_code.get_mut_cursor().set_x(start.get_x());
                mutable_code.get_mut_cursor().set_y(start.get_y());    
            }
            mutable_code.flush_selection();

        } else if !is_selecting && is_shift {

            if readable_cursor.get_x() < nlines - 1 {
                current_selection_end = readable_cursor.clone();
                current_selection_end.move_down(false, current_size, lower_size);
                mutable_code.create_selection(readable_cursor, current_selection_end.clone());
                mutable_code.get_mut_cursor().move_down(false, current_size, lower_size);
            } else {
                current_selection_end = readable_cursor.clone();
                current_selection_end.move_right(true, lower_size);
                mutable_code.create_selection(readable_cursor, current_selection_end.clone());
                mutable_code.get_mut_cursor().move_down(true, current_size, lower_size);
            }

        } else if !is_selecting && !is_shift {

            if readable_cursor.get_x() < nlines - 1 {
                mutable_code.get_mut_cursor().move_down(false, current_size, lower_size);
            } else {
                mutable_code.get_mut_cursor().move_down(true, current_size, lower_size);
            }
        }


    }

}

pub fn handle_left(code_component: &mut CodeComponent, event: Event) {
    let readable_current_code = code_component.get_current().clone();
    let readable_cursor = readable_current_code.get_cursor().clone();
    let mut is_selecting = false;
    let mut readable_selection = CodeSelection::default();
    let mut is_shift = false;
    let mut upper_size = 0;
    let mut current_size = 0;

    if let Some(selection) = readable_current_code.get_selection() {
        is_selecting = true;
        readable_selection = selection.clone();
    }

    if let Some(current) = readable_current_code.get_line(readable_cursor.get_x()) {
        current_size = current.get_string().len();
    }

    if readable_current_code.get_cursor().get_x() > 0 {
       if let Some(upper) = readable_current_code.get_line(readable_cursor.get_x() - 1) {
            upper_size = upper.get_string().len();
       } 
    }

    if let Event::Key(key) = event {
    
        if key.modifiers.contains(KeyModifiers::SHIFT) {
            is_shift = true;
        }

        let mutable_code = code_component.get_mut_current();
        let mut current_selection_end = readable_selection.get_end().clone();

        if is_selecting && is_shift {

            if readable_cursor.get_x() == 0 {
                mutable_code.get_mut_cursor().move_left(false, upper_size);
                if current_selection_end.get_y() > 1 {
                    current_selection_end.move_left(false, upper_size);
                }
                let current_selection_start = readable_selection.get_start().clone();
                if current_selection_start == current_selection_end {
                    mutable_code.flush_selection();
                } else {
                    mutable_code.set_selection_end(current_selection_end);
                }
            } else {
                current_selection_end.move_left(true, upper_size);
                mutable_code.get_mut_cursor().move_left(true, upper_size);
                mutable_code.set_selection_end(current_selection_end);
            }


        } else if is_selecting && !is_shift {

            let end = readable_selection.get_end();
            let start = readable_selection.get_start();

            if start.get_x() > end.get_x() {
                mutable_code.get_mut_cursor().set_x(end.get_x());
                mutable_code.get_mut_cursor().set_y(end.get_y()-1);    
            } else if start.get_x() < end.get_x() {
                mutable_code.get_mut_cursor().set_x(start.get_x());
                mutable_code.get_mut_cursor().set_y(start.get_y());    
            } else if start.get_x() == end.get_x() {
                if start.get_y() < end.get_y() {
                    mutable_code.get_mut_cursor().set_x(start.get_x());
                    mutable_code.get_mut_cursor().set_y(start.get_y());
                } else if start.get_y() > end.get_y() {
                    mutable_code.get_mut_cursor().set_x(end.get_x());
                    mutable_code.get_mut_cursor().set_y(end.get_y()-1);
                }
            }
            mutable_code.flush_selection();

        } else if !is_selecting && is_shift {

            let mut current_selection_start = readable_cursor.clone();
            current_selection_start.move_right(false, current_size);
            current_selection_end = readable_cursor.clone();
            mutable_code.create_selection(current_selection_start, current_selection_end.clone());
            mutable_code.get_mut_cursor().move_left(false, upper_size); 

        } else if !is_selecting && !is_shift {

            if readable_cursor.get_x() == 0 {
                mutable_code.get_mut_cursor().move_left(false, upper_size);
            } else {
                mutable_code.get_mut_cursor().move_left(true, upper_size);
            }
        }


    }
}

pub fn handle_right(code_component: &mut CodeComponent, event: Event) {
    let readable_current_code = code_component.get_current().clone();
    let readable_cursor = readable_current_code.get_cursor().clone();
    let mut is_selecting = false;
    let mut readable_selection = CodeSelection::default();
    let mut is_shift = false;
    let mut current_size = 0;

    if let Some(selection) = readable_current_code.get_selection() {
        is_selecting = true;
        readable_selection = selection.clone();
    }

    if let Some(current) = readable_current_code.get_line(readable_cursor.get_x()) {
        current_size = current.get_string().len();
    }

    if let Event::Key(key) = event {
    
        if key.modifiers.contains(KeyModifiers::SHIFT) {
            is_shift = true;
        }

        let mutable_code = code_component.get_mut_current();
        let mut current_selection_end = readable_selection.get_end().clone();

        if is_selecting && is_shift {

            if readable_cursor.get_x() > 0 {
                current_selection_end.move_right(false, current_size);
                mutable_code.get_mut_cursor().move_right(false, current_size);
                let current_selection_start = readable_selection.get_start().clone();
                if current_selection_start == current_selection_end {
                    mutable_code.flush_selection();
                } else {
                    mutable_code.set_selection_end(current_selection_end);
                }
            } else {
                current_selection_end.move_right(true, current_size);
                mutable_code.get_mut_cursor().move_right(true, current_size);
                mutable_code.set_selection_end(current_selection_end);
            }


        } else if is_selecting && !is_shift {

            let end = readable_selection.get_end();
            let start = readable_selection.get_start();

            if start.get_x() < end.get_x() {
                mutable_code.get_mut_cursor().set_x(end.get_x());
                mutable_code.get_mut_cursor().set_y(end.get_y());    
            } else if start.get_x() > end.get_x() {
                mutable_code.get_mut_cursor().set_x(start.get_x());
                mutable_code.get_mut_cursor().set_y(start.get_y());    
            } else if start.get_x() == end.get_x() {
                if start.get_y() > end.get_y() {
                    mutable_code.get_mut_cursor().set_x(start.get_x());
                    mutable_code.get_mut_cursor().set_y(start.get_y());
                } else if start.get_y() < end.get_y() {
                    mutable_code.get_mut_cursor().set_x(end.get_x());
                    mutable_code.get_mut_cursor().set_y(end.get_y());
                }
            }
            mutable_code.flush_selection();

        } else if !is_selecting && is_shift {

            if readable_cursor.get_x() > 0 {
                current_selection_end = readable_cursor.clone();
                current_selection_end.move_right(false, current_size);
                mutable_code.create_selection(readable_cursor, current_selection_end.clone());
                mutable_code.get_mut_cursor().move_right(false, current_size);
            } else {
                current_selection_end = readable_cursor.clone();
                current_selection_end.move_right(true, current_size);
                mutable_code.create_selection(readable_cursor, current_selection_end.clone());
                mutable_code.get_mut_cursor().move_right(true, current_size);
            }

        } else if !is_selecting && !is_shift {

            if readable_cursor.get_x() == readable_current_code.get_content().len() - 1 {
                mutable_code.get_mut_cursor().move_right(false, current_size);
            } else {
                mutable_code.get_mut_cursor().move_right(true, current_size);
            }
        }


    }
}