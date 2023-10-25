use std::{error::Error, fs::{OpenOptions, File}, io::Write};

use clipboard::{ClipboardContext, ClipboardProvider};
use crossterm::event::{Event, KeyModifiers};

use crate::state::AppContext;

use super::{CodeComponent, code_selection::CodeSelection, code::Line};
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

pub fn handle_cut (code_component: &mut CodeComponent) {
    let cut;
    if let Some(selection) = code_component.get_current().get_selection() {
        if selection.get_start() != selection.get_end() {
            let mut code: Vec<String> = vec![];
            for i in selection.get_start().get_x()..selection.get_end().get_x() {
                if let Some(line) = code_component.get_current().get_line(i) {
                    let selected_line: String;
                    if line.get_number() == selection.get_start().get_x() {
                        selected_line = line.get_string()[selection.get_start().get_y()..].to_string();
                        if let Some(current_string) = code.get(line.get_number()) {
                            let new_string: String = current_string.replace(&selected_line, "").clone();
                            code.push(new_string.clone());    
                        }
                    } else if line.get_number() == selection.get_end().get_x() {
                        selected_line = line.get_string()[..selection.get_end().get_y()].to_string();
                        if let Some(current_string) = code.get(line.get_number()) {
                            let new_string: String = current_string.replace(&selected_line, "").clone();
                            code.push(new_string.clone());    
                        }
                    } else {
                        code.push(line.get_string());
                    }
                }
            }
            cut = code.join("\n").to_string();
            let clipboard: Result<ClipboardContext, Box<dyn Error>> = ClipboardProvider::new();
            if let Ok(mut context) =  clipboard {
                let _ = context.set_contents(cut);
            } 

            code_component.get_mut_current().flush_selection();
        }
    }
}

pub fn handle_copy(code_component: &mut CodeComponent) {
    let copy;
    if let Some(selection) = code_component.get_current().get_selection() {
        if selection.get_start() != selection.get_end() {
            let mut code: Vec<String> = vec![];
            for i in selection.get_start().get_x()..selection.get_end().get_x() {
                if let Some(line) = code_component.get_current().get_line(i) {
                    let selected_line: String;
                    if line.get_number() == selection.get_start().get_x() {
                        selected_line = line.get_string()[selection.get_start().get_y()..].to_string();
                        if let Some(current_string) = code.get(line.get_number()) {
                            let new_string: String = current_string.replace(&selected_line, "").clone();
                            code.push(new_string.clone());    
                        }
                    } else if line.get_number() == selection.get_end().get_x() {
                        selected_line = line.get_string()[..selection.get_end().get_y()].to_string();
                        if let Some(current_string) = code.get(line.get_number()) {
                            let new_string: String = current_string.replace(&selected_line, "").clone();
                            code.push(new_string.clone());    
                        }
                    } else {
                        code.push(line.get_string());
                    }
                }
            }
            copy = code.join("\n").to_string();
            let clipboard: Result<ClipboardContext, Box<dyn Error>> = ClipboardProvider::new();
            if let Ok(mut context) =  clipboard {
                let _ = context.set_contents(copy);
            } 
        }
    }
}

pub fn handle_paste(code_component: &mut CodeComponent) {
    let clipboard: Result<ClipboardContext, Box<dyn Error>> = ClipboardProvider::new();
    if let Ok(mut context) =  clipboard {
        if let Ok(contents) = context.get_contents() {
            contents.split("\n").for_each(|line| {
                let number = code_component.current.get_content().into_iter().map(|line| line.get_number()).max().take().unwrap() + 1;
                let line = Line::new(number, line.to_string());
                let _ = code_component.current.add_line(line);
            });                                
        }
    }
}

pub fn handle_save(code_component: &mut CodeComponent, context: &mut AppContext) {
    code_component.history.use_last();
    let code = code_component.history.get_current_code();
    let utf8_code = code.to_string().chars().map(|char| char as u8).fold(vec![], |mut vec, char| {
        vec.push(char);
        vec
    });
    if let Some(path) = context.active_file() {
        if path.is_file() {
            let f = OpenOptions::new().append(true).open(path);
            if let Ok(mut file) = f {
                let _ = file.write_all(&utf8_code);
            }    
        }
    } else if let Some(path) = context.active_file() {
        let f = File::create(path);
        if let Ok(mut file) = f {
            let _ = file.write_all(&utf8_code);
        }
    } 

}

pub fn handle_undo (code_component: &mut CodeComponent) {
    code_component.history.use_previous();
    let code = code_component.history.get_current_code();
    code_component.current = code.clone();
}

pub fn handle_redo(code_component: &mut CodeComponent) {
    code_component.history.use_next();
    let code = code_component.history.get_current_code();
    code_component.current = code.clone();
}

pub fn handle_char(code_component: &mut CodeComponent, char: String) {
    code_component.current.remove_cursor();
    if let Some(current_line) = code_component.current.get_line(code_component.current.get_cursor().get_x()) {
        code_component.current.change_line_at_cursor(current_line.get_string()[..code_component.current.get_cursor().get_y()].to_string() + &char.to_string() + &current_line.get_string()[code_component.current.get_cursor().get_y()..].to_string());    
    }
    let y = code_component.current.get_cursor().get_y();
    code_component.current.get_mut_cursor().set_y(y+1);
    code_component.current.set_cursor();
}

pub fn handle_delete(code_component: &mut CodeComponent) {
    let readable_code = code_component.get_current().clone();
    let mutable_code = code_component.get_mut_current();
    let readable_cursor = readable_code.get_cursor().clone();
    let mut upper_size = 0;

    if readable_cursor.get_x() > 0 {
        if let Some(upper_line) = readable_code.get_line(readable_cursor.get_x() - 1) {
            upper_size = upper_line.get_string().len();
        }
    }
    if let Some(current_line) = readable_code.get_line(readable_cursor.get_x()).clone() {
        let line = current_line.get_string().clone();
        if readable_cursor.get_y() == 0 && readable_cursor.get_x() > 0 {
            mutable_code.get_mut_cursor().move_left(true, upper_size);
            mutable_code.change_line(readable_cursor.get_x()-1, line);
            mutable_code.remove_line(readable_cursor.get_x());
            for i in readable_cursor.get_x()..readable_code.get_content().len() {
                mutable_code.set_line_number(i);
            }    
        } else if readable_cursor.get_y() > 0 {
            let mut new_string = line[..readable_cursor.get_y()-1].to_string();
            new_string.push_str(&line[readable_cursor.get_y()..]);
            mutable_code.replace_line(current_line.get_number(), line, new_string.clone());
            mutable_code.get_mut_cursor().move_left(false, new_string.len());
        }
    }    
}

pub fn handle_enter(code_component: &mut CodeComponent) {
    let code = code_component.get_current().clone();
    let mut_code = code_component.get_mut_current();

    if let Some(current_line) = code.get_content().get(code.get_cursor().get_x()) {
        let new_current_string = current_line.get_string()[..code.get_cursor().get_y()].to_string().clone();
        let new_generated_string = current_line.get_string()[code.get_cursor().get_y()..].to_string().clone();
        
        mut_code.replace_line(code.get_cursor().get_x(),current_line.get_string(), new_current_string.clone());
        mut_code.get_mut_cursor().move_right(true, current_line.get_string().len() - new_current_string.len());
        mut_code.get_mut_content().insert(code.get_cursor().get_x()+1, Line::new(code.get_cursor().get_x()+1, new_generated_string));
        for number in current_line.get_number()+1.. code.get_content().len() {
            if let Some(line) = mut_code.get_mut_content().get_mut(number) {
                line.set_number(number + 1);
            }
        }
    }

}