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
        if self.x > 0 {
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
        } else if exceed {
            self.x -= 1;
            self.y = limit - 1;
        }
    }

    pub fn move_right(&mut self, exceed: bool, limit: usize) {
        if self.y < limit - 1 {
            self.y += 1;
        } else if exceed {
            self.x += 1;
        }
    }

    pub fn move_down(&mut self, exceed: bool, limit: usize, length: usize) {
        if self.x < limit - 1 {
            self.x += 1;
        } else if exceed {
            self.y = length - 1;
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
    let mut upper_size = 0;

    if let Some(selection) = readable_current_code.get_selection() {
        is_selecting = true;
        readable_selection = selection.clone();
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
                current_selection_end.move_up(false, upper_size);
                mutable_code.get_mut_cursor().move_up(false, upper_size);
            } else {
                current_selection_end.move_up(true, upper_size);
                mutable_code.get_mut_cursor().move_up(true, upper_size);
            }

            mutable_code.set_selection_end(current_selection_end);

        } else if is_selecting && !is_shift {

            if readable_cursor.get_x() > 0 {
                mutable_code.flush_selection();
                mutable_code.get_mut_cursor().move_up(false, upper_size);
            } else {
                mutable_code.flush_selection();
                mutable_code.get_mut_cursor().move_up(true, upper_size);
            }

        } else if !is_selecting && is_shift {

            if readable_cursor.get_x() > 0 {
                current_selection_end = readable_cursor.clone();
                current_selection_end.move_up(false, upper_size);
                mutable_code.create_selection(readable_cursor, current_selection_end.clone());
                mutable_code.get_mut_cursor().move_up(false, upper_size);
            } else {
                current_selection_end = readable_cursor.clone();
                current_selection_end.move_up(true, upper_size);
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
    let nlines = readable_current_code.get_content().len();

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
                current_selection_end.move_down(false, nlines, current_size);
                mutable_code.get_mut_cursor().move_down(false, nlines, current_size);
            } else {
                current_selection_end.move_right(true, current_size);
                mutable_code.get_mut_cursor().move_right(true, current_size);
            }

            mutable_code.set_selection_end(current_selection_end);


        } else if is_selecting && !is_shift {

            if readable_cursor.get_x() > 0 {
                mutable_code.flush_selection();
                mutable_code.get_mut_cursor().move_down(false, nlines, current_size);
            } else {
                mutable_code.flush_selection();
                mutable_code.get_mut_cursor().move_down(true, nlines, current_size);
            }

        } else if !is_selecting && is_shift {

            if readable_cursor.get_x() > 0 {
                current_selection_end = readable_cursor.clone();
                current_selection_end.move_down(false, nlines, current_size);
                mutable_code.create_selection(readable_cursor, current_selection_end.clone());
                mutable_code.get_mut_cursor().move_down(false, nlines, current_size);
            } else {
                current_selection_end = readable_cursor.clone();
                current_selection_end.move_right(true, current_size);
                mutable_code.create_selection(readable_cursor, current_selection_end.clone());
                mutable_code.get_mut_cursor().move_down(true, nlines, current_size);
            }

        } else if !is_selecting && !is_shift {

            if readable_cursor.get_x() > 0 {
                mutable_code.get_mut_cursor().move_down(false, nlines, current_size);
            } else {
                mutable_code.get_mut_cursor().move_down(true, nlines, current_size);
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

    if let Some(selection) = readable_current_code.get_selection() {
        is_selecting = true;
        readable_selection = selection.clone();
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

            if readable_cursor.get_y() > 0 {
                current_selection_end.move_up(false, upper_size);
                mutable_code.get_mut_cursor().move_up(false, upper_size);
            } else {
                current_selection_end.move_up(true, upper_size);
                mutable_code.get_mut_cursor().move_up(true, upper_size);
            }

            mutable_code.set_selection_end(current_selection_end);

        } else if is_selecting && !is_shift {

            if readable_cursor.get_y() > 0 {
                mutable_code.flush_selection();
                mutable_code.get_mut_cursor().move_up(false, upper_size);
            } else {
                mutable_code.flush_selection();
                mutable_code.get_mut_cursor().move_up(true, upper_size);
            }

        } else if !is_selecting && is_shift {

            if readable_cursor.get_y() > 0 {
                current_selection_end = readable_cursor.clone();
                current_selection_end.move_up(false, upper_size);
                mutable_code.create_selection(readable_cursor, current_selection_end.clone());
                mutable_code.get_mut_cursor().move_up(false, upper_size);
            } else {
                current_selection_end = readable_cursor.clone();
                current_selection_end.move_up(true, upper_size);
                mutable_code.create_selection(readable_cursor, current_selection_end.clone());
                mutable_code.get_mut_cursor().move_up(true, upper_size);
            }

        } else if !is_selecting && !is_shift {

            if readable_cursor.get_y() > 0 {
                mutable_code.get_mut_cursor().move_up(false, upper_size);
            } else {
                mutable_code.get_mut_cursor().move_up(true, upper_size);
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
            } else {
                current_selection_end.move_right(true, current_size);
                mutable_code.get_mut_cursor().move_right(true, current_size);
            }

            mutable_code.set_selection_end(current_selection_end);

        } else if is_selecting && !is_shift {

            if readable_cursor.get_x() > 0 {
                mutable_code.flush_selection();
                mutable_code.get_mut_cursor().move_right(false, current_size);
            } else {
                mutable_code.flush_selection();
                mutable_code.get_mut_cursor().move_right(true, current_size);
            }

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

            if readable_cursor.get_x() > 0 {
                mutable_code.get_mut_cursor().move_right(false, current_size);
            } else {
                mutable_code.get_mut_cursor().move_right(true, current_size);
            }
        }


    }
}