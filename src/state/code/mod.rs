pub mod code_history;
pub mod code_selection;
pub mod code;

use std::{fs::{File, OpenOptions}, io::{Write, Read}, error::Error, path::PathBuf, vec};
use self::{code::{Code, Line}, code_history::CodeHistory, code_selection::{CodeSelection, Point}};
use clipboard::{ClipboardProvider, ClipboardContext};
use crossterm::event::{KeyEventKind, Event, KeyCode, KeyModifiers};

use super::{Component, ComponentType, AppContext};

#[derive(Debug, PartialEq, Eq)]
pub struct CodeComponent {
    current: Code,
    history: CodeHistory,
    selection: Option<CodeSelection>,
}

impl Component for CodeComponent {

    fn get_type(&self) -> ComponentType {
        ComponentType::Code
    }

    fn handle_event(&mut self, context: &mut AppContext, event: Event) {

        if let Event::Key(key) = event {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char(char) => {
                        let current_code = self.get_current().get_content();

                        let mut char_normalized = char.clone().to_string();
                        char_normalized = char_normalized.to_lowercase().to_string();
                        if char_normalized == "x" && key.modifiers.contains(KeyModifiers::CONTROL) {
                            let cut;
                            if let Some(selection) = self.get_selection() {
                                if selection.get_start() != selection.get_end() {
                                    let mut code: Vec<String> = vec![];
                                    for i in selection.get_start().get_x()..selection.get_end().get_x() {
                                        if let Some(line) = current_code.get(i) {
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
                                    self.selection = None;
                                }
                            }
                        } else if char_normalized == "c" && key.modifiers.contains(KeyModifiers::CONTROL) {
                            let copy;
                            if let Some(selection) = self.get_selection() {
                                if selection.get_start() != selection.get_end() {
                                    let mut code: Vec<String> = vec![];
                                    for i in selection.get_start().get_x()..selection.get_end().get_x() {
                                        if let Some(line) = self.get_current().get_line(i) {
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
                            
                        } else if char_normalized == "v" && key.modifiers.contains(KeyModifiers::CONTROL) {
                            let clipboard: Result<ClipboardContext, Box<dyn Error>> = ClipboardProvider::new();
                            if let Ok(mut context) =  clipboard {
                                if let Ok(contents) = context.get_contents() {
                                    contents.split("\n").for_each(|line| {
                                        let number = self.current.get_content().into_iter().map(|line| line.get_number()).max().take().unwrap() + 1;
                                        let line = Line::new(number, line.to_string());
                                        let _ = self.current.add_line(line);
                                    });                                
                                }
                            }
                        } else if char_normalized == "s" && key.modifiers.contains(KeyModifiers::CONTROL) {
                                self.history.use_last();
                                let code = self.history.get_current_code();
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
    
                        } else if char_normalized == "z" && key.modifiers.contains(KeyModifiers::CONTROL) {
                            self.history.use_previous();
                            let code = self.history.get_current_code();
                            self.current = code.clone();                            
                        } else if char_normalized == "y" && key.modifiers.contains(KeyModifiers::CONTROL) {
                            self.history.use_next();
                            let code = self.history.get_current_code();
                            self.current = code.clone();
                        } else {
                            self.current.remove_cursor();
                            if let Some(current_line) = self.current.get_line(self.get_current().get_x()) {
                                self.current.change_line_at_cursor(current_line.get_string()[..self.get_current().get_y()].to_string() + &char.to_string() + &current_line.get_string()[self.current.get_y()..].to_string());    
                            }
                            self.current.set_y(self.current.get_y()+1);
                            self.current.set_cursor();
                        }
                    },
                    KeyCode::Delete => {
                        let last_number = self.current.get_content().into_iter().map(|x| x.get_number()).fold(0, |line1, line2| {
                            if line1 > line2 { line1 } else { line2 }
                        });
                        let last_line = self.current.get_line(last_number).unwrap();
                        self.current.change_line(last_line.get_number(), last_line.get_string()[..last_line.get_string().len()-1].to_string());
                    },
                    KeyCode::Enter => {
                        {
                            let mut_code = self.get_mut_current();
                            mut_code.remove_cursor();    
                        }
                        let code = self.get_current().clone();
                        let mut_code = self.get_mut_current();
                        if let Some(current_line) = code.get_content().get(code.get_x()) {
                            let line_number = current_line.get_number().clone();
                            let new_current_string = current_line.get_string()[..code.get_y()].to_string().clone();
                            let new_generated_string = current_line.get_string()[code.get_y()..].to_string().clone();
                            mut_code.flush();
                            for number in 0 .. line_number {
                                if let Some(line) = code.get_line(number) {
                                    mut_code.add_line(line.clone());                                    
                                }
                            }
                            mut_code.add_line(Line::new(current_line.get_number(), new_current_string));
                            mut_code.set_x(code.get_x());
                            mut_code.set_y(code.get_y());
                            mut_code.add_line(Line::new(current_line.get_number() + 1, new_generated_string));
                            for number in current_line.get_number() + 1.. code.get_content().len() {
                                if let Some(line) = code.get_line(number) {
                                    let mut new_line = line.clone();
                                    new_line.set_number(number + 1);
                                    mut_code.add_line(new_line.clone());                                    
                                }
                            }
                            mut_code.set_cursor();
                        }
                    },
                    KeyCode::Up => {
                        self.current.remove_cursor();
                        //Handles up arrow according to the state of the code, if something is selected has a different behaviour from the normal arrow key
                        if let Some(selection) = &mut self.selection {
                            //if something is selected and arrow up is pressed with shift extends the selection to the upper line
                            if key.modifiers.contains(KeyModifiers::SHIFT) {
                                //if the upper line exists moves the selection to the upper line at the same position occupied from cursor on the current line
                                if self.current.get_x()>0 {
                                    let mut current_end = selection.get_end().clone();
                                    current_end.set_x(current_end.get_x()- 1);
                                    //also moves the cursor to the new end
                                    self.current.set_x(current_end.get_x());
                                    if let Some(upper_line) = self.current.get_line(current_end.get_x()) {
                                        if let Some(current_line) = self.current.get_line(current_end.get_x() + 1) {
                                            if upper_line.get_string().len() < self.current.get_y() {
                                                current_end.set_y(upper_line.get_string().len());
                                                self.current.set_y(upper_line.get_string().len());    
                                            } else {
                                                current_end.set_y(current_end.get_y());
                                                self.current.set_y(current_end.get_y()-1);
                                            }
                                            selection.set_end(current_end.clone());
                                        }
                                    }
                                } else {
                                    //else moves the selection to the start of the current line
                                    let mut current_end = selection.get_end().clone();
                                    current_end.set_y(1);
                                    selection.set_end(current_end.clone());
                                    //also moves the cursor to the new end
                                    self.current.set_x(current_end.get_x());
                                    self.current.set_y(0);
                                }
                            } else {
                                //else if there is no shift key pressed removes the selection after setting the cursor to the selection end
                                let mut current_end = selection.get_end().clone();

                                current_end.set_y(current_end.get_y() - 1);
                                self.current.set_x(current_end.get_x());
                                self.current.set_y(current_end.get_y());
                                self.selection = None;
                            }
                        } else {
                            //if nothing is already selected and shift is pressed creates a selection from the current position of the cursor then moves the cursor to its end
                            if key.modifiers.contains(KeyModifiers::SHIFT) {
                                let current_x = self.current.get_x();
                                let current_y = self.current.get_y();
                                let start_point: Point = Point::new(current_x, current_y);
                                let mut end_point: Point;
                                //if an upper line set the ending point on the upper line
                                if self.current.get_x() > 0 {
                                    end_point= Point::new(current_x - 1, current_y);
                                } else {
                                    //else extend to the start of the current line
                                    end_point= Point::new(current_x, 0);
                                }
                                self.selection = Some(CodeSelection::new(start_point, end_point.clone()));
                                let mut upper_len = end_point.get_y() - 1;
                                if let Some(upper_line) = self.current.get_line(end_point.clone().get_x()) {
                                    upper_len = upper_line.get_string().len()-1;
                                }
                                self.current.set_x(end_point.get_x());
                                if upper_len > end_point.get_y() {
                                    if current_y > 0 {
                                        self.current.set_y(end_point.get_y()-1);    
                                    } else {
                                        self.current.set_y(end_point.get_y());    
                                    }
                                } else if upper_len <= end_point.get_y() {
                                    end_point.set_x(end_point.get_x()+1);
                                    end_point.set_y(0);
                                    self.current.set_y(upper_len);
                                }   

                            } else {
                                //else if shift is not pressed and a selection doesn't exist and an upper line exists just moves the cursor to the upper line
                                if self.current.get_x() > 0 {
                                    self.current.set_x(self.current.get_x() - 1);
                                    if let Some(upper_line) = self.current.get_line(self.current.get_x()) {
                                        if upper_line.get_string().len() < self.current.get_y() {
                                            self.current.set_y(upper_line.get_string().len());
                                        } else {
                                            let current_y = self.current.get_y();
                                            self.current.set_y(current_y);    
                                        }                                   
                                    }
    
                                }
                            }
                        }

                        self.current.set_cursor();

                    },
                    KeyCode::Down => {
                        let nlines = self.current.get_content().len();
                        self.current.remove_cursor();


                        //Handles up arrow according to the state of the code, if something is selected has a different behaviour from the normal arrow key
                        if let Some(selection) = &mut self.selection {
                            //if something is selected and arrow down is pressed with shift extends the selection to the lower line
                            if key.modifiers.contains(KeyModifiers::SHIFT) {
                                //if the lower line exists moves the selection to the lower line at the same position occupied from cursor on the current line
                                if self.current.get_x() < nlines - 1 {
                                    let mut current_end = selection.get_end().clone();
                                    current_end.set_x(current_end.get_x() + 1);
                                    self.current.set_x(current_end.get_x());

                                    if let Some(lower_line) = self.current.get_line(current_end.get_x()) {
                                        if let Some(current_line) = self.current.get_line(current_end.get_x() - 1) {
                                            //also moves the cursor to the new end
                                            if lower_line.get_string().len() < current_line.get_string().len() {
                                                self.current.set_y(lower_line.get_string().len()-1);
                                            } else {
                                                self.current.set_y(current_end.get_y());
            
                                            }
                                            let end_point = Point::new(self.current.get_x(), self.current.get_y());
                                            selection.set_end(end_point.clone());
                                        }
                                    }
                                } else {
                                    let mut end_y = self.current.get_y();
                                    if let Some(current_line) = self.current.get_content().get(self.current.get_x()) {
                                        end_y = current_line.get_string().len();
                                    }
                                    //else moves the selection to the end of the current line
                                    let mut current_end = selection.get_end().clone();
                                    current_end.set_y(end_y);
                                    selection.set_end(current_end.clone());
                                    //also moves the cursor to the new end
                                    self.current.set_x(current_end.get_x());
                                    self.current.set_y(current_end.get_y());
                            }
                            } else {
                                //else if there is no shift key pressed removes the selection after setting the cursor to the selection end
                                let mut current_end = selection.get_end().clone();
                                current_end.set_y(current_end.get_y() + 1);
                                self.current.set_x(current_end.get_x());
                                self.current.set_y(current_end.get_y());
                                self.selection = None;
                            }
                        } else {
                            //if nothing is already selected and shift is pressed creates a selection from the current position of the cursor then moves the cursor to its end
                            if key.modifiers.contains(KeyModifiers::SHIFT) {
                                let current_x = self.current.get_x();
                                let current_y = self.current.get_y();
                                let start_point: Point = Point::new(current_x, current_y);
                                let end_point: Point;
                                //if a lower line exists set the ending point on the lower line
                                if let Some(_lower_line) = self.current.get_content().get(self.current.get_x() + 1) {
                                    end_point= Point::new(current_x + 1, current_y);
                                } else {
                                    //else extend to the end of the current line
                                    let mut end_y = self.current.get_y();
                                    if let Some(current_line) = self.current.get_content().get(self.current.get_x()) {
                                        end_y = current_line.get_string().len()-1;
                                    }
                                    end_point= Point::new(current_x, end_y);
                                }
                                self.selection = Some(CodeSelection::new(start_point, end_point.clone()));
                                self.current.set_x(end_point.get_x());
                                self.current.set_y(end_point.get_y());
                            } else {
                                //else if shift is not pressed and a selection doesn't exist and an lower line exists just moves the cursor to the lower line
                                let current_x = self.current.get_x();
                                self.current.set_x(current_x + 1);
                                let mut current_y = self.current.get_y();
                                if let Some(lower_line) = self.current.get_content().get(self.current.get_x()) {
                                    if lower_line.get_string().len() < current_y {
                                        self.current.set_y(lower_line.get_string().len());
                                    } else {
                                        self.current.set_y(current_y);
                                    }
                                } else {
                                    //else if a lower line doesn't exist move the cursor to the end of the current line
                                    if let Some(current_line) = self.current.get_content().get(self.current.get_x()) {
                                        current_y = current_line.get_string().len() - 1;
                                    }
                                    self.current.set_x(current_x);
                                    self.current.set_y(current_y);
                                }
                            }
                        }

                        self.current.set_cursor();
                    },
                    KeyCode::Left => {
                        self.current.remove_cursor();

                        //if a selection exist
                        let mut selection_exists = false;
                        let mut start_point = Point::new(self.current.get_x(), self.current.get_y());
                        let mut end_point = Point::new(self.current.get_x(), self.current.get_y());
                        if let Some(selection) = &self.selection {
                            selection_exists = true;
                            start_point = selection.get_start().clone();
                            end_point = selection.get_end().clone();
                        }

                        if selection_exists {
                            if let Some(mutable_selection) = &mut self.selection {
                                //  and shift is also pressed
                                if key.modifiers.contains(KeyModifiers::SHIFT) {

                                    if end_point.get_y() > 0 {
                                        //      if the start x != end x change the end of the selection to the current y to the current y - 1 (the end point)
                                        if start_point.get_x() != end_point.get_x() {
                                        //      then if start x > end x the cursor goes on the left of the end point
                                            if start_point.get_x() > end_point.get_x() && end_point.get_y() > 0{
                                                self.current.set_x(end_point.get_x());    
                                                end_point.set_y(end_point.get_y()-1);
                                                self.current.set_y(end_point.get_y()-1);
                                            }
                                        //      else if the start x < end x the cursor goes on the right of the end point
                                            else if start_point.get_x() < end_point.get_x() {
                                                self.current.set_y(end_point.get_y() + 1);
                                            }

                                            mutable_selection.set_end(end_point.clone());
                                        }
                                        //      else if the start x == end x
                                        else if start_point.get_x() == end_point.get_x() {
                                    //      if start y != end y move the end on the left
                                            if start_point.get_y() != end_point.get_y() {


                                    //          if start y > end y set the cursor on the left of the end point
                                                if start_point.get_y() > end_point.get_y() && self.current.get_y() > 0{
                                                    end_point.set_y(self.current.get_y());
                                                    self.current.set_y(end_point.get_y() - 1);
                                                }
                                    //          else if start y < end y set the cursor on the right of the end point
                                                else if start_point.get_y() < end_point.get_y() && self.current.get_y() > 0{
                                                    end_point.set_y(self.current.get_y() - 1);
                                                    self.current.set_y(end_point.get_y());
                                                }

                                                mutable_selection.set_end(end_point.clone());

                                            } else {
                                                self.selection = None;
                                            }
                                        }
                                    }
                                
                                }
                                //  and shift is not pressed
                                else {
                                    //move the cursor the left of the selection and then delete the selection
                                        self.current.set_x(end_point.get_x());
                                        self.current.set_y(end_point.get_y()-1);
                                    //finally delete the selection
                                    self.selection = None;
                                }
                            }
                        } else {
                            //else the selection doesn't exist
                            //and shift is pressed
                            if key.modifiers.contains(KeyModifiers::SHIFT) {
                            //  create a selection from current position of the cursor to 1 on his left and set the cursor
                                if self.current.get_y() > 0 {
                                    let start_point = Point::new(self.current.get_x(),self.current.get_y() + 1);
                                    let end_point = Point::new(self.current.get_x(), self.current.get_y());
                                    self.selection = Some(CodeSelection::new(start_point, end_point.clone())); 
                                    self.current.set_y(end_point.get_y() - 1);   
                                }
                            }
                            //  and shift is not pressed
                            else {
                            //      move the cursor by 1 on his left
                                if self.current.get_y() > 0 {
                                    self.current.set_y(self.current.get_y() - 1);
                                    self.selection = None;
                                }
                            }
                        }

                        self.current.set_cursor();

                    },
                    KeyCode::Right => {
                        self.current.remove_cursor();

                        //if a selection exist
                        let mut selection_exists = false;
                        let mut start_point = Point::new(self.current.get_x(), self.current.get_y());
                        let mut end_point = Point::new(self.current.get_x(), self.current.get_y());
                        if let Some(selection) = &self.selection {
                            selection_exists = true;
                            start_point = selection.get_start().clone();
                            end_point = selection.get_end().clone();
                        }

                        if selection_exists {
                            if let Some(mutable_selection) = &mut self.selection {
                                //  and shift is also pressed
                                if key.modifiers.contains(KeyModifiers::SHIFT) {
                                //      if the start x != end x change the end of the selection to the current y to the current y + 1 (the end point)
                                if let Some(current_line) = self.current.get_line(end_point.get_x()) {
                                        if start_point.get_x() != end_point.get_x() && end_point.get_y() < current_line.get_string().len() - 1{
                                            end_point.set_y(end_point.get_y() + 1);
                                            mutable_selection.set_end(end_point.clone());
                                            self.current.set_x(end_point.get_x());
                                    //      then if start x > end x the cursor goes on the left of the end point
                                            if start_point.get_x() > end_point.get_x() {
                                                self.current.set_y(end_point.get_y());
                                            }
                                    //      else if the start x < end x the cursor goes on the right of the end point
                                            else if start_point.get_x() < end_point.get_x() {
                                                self.current.set_y(end_point.get_y());
                                            }
                                        }
                                    //      else if the start x == end x
                                        else if start_point.get_x() == end_point.get_x() {
                                    //      if start y != end y move the end on the left
                                            if let Some(current_line) = self.current.get_line(end_point.get_x()) {
                                                if start_point.get_y() != end_point.get_y() {
                                                    if end_point.get_y() < current_line.get_string().len() - 1 {
                                                        end_point.set_y(end_point.get_y() + 1);
                                                        mutable_selection.set_end(end_point.clone());
                                            //          if start y > end y set the cursor on the left of the end point
                                                        if start_point.get_y() > end_point.get_y() {
                                                            self.current.set_y(end_point.get_y() - 1);
                                                        }
                                            //          else if start y < end y set the cursor on the right of the end point
                                                        else if start_point.get_y() < end_point.get_y() {
                                                            self.current.set_y(end_point.get_y());
                                                        }
                                                    }
                                                } else {
                                                    self.selection = None;
                                                }
                                            }
                                        }
                                    }
                                }
                                //  and shift is not pressed
                                else {
                                    //move the cursor the right of the selection and then delete the selection
                                    //if the start x > end x
                                    if start_point.get_x() > end_point.get_x() {
                                    //  the cursor goes to the start
                                        self.current.set_x(start_point.get_x());
                                        self.current.set_y(start_point.get_y());
                                    }
                                    //else if start x < end x
                                    else if start_point.get_x() < end_point.get_x() {
                                    //  the cursor goes to the start
                                        self.current.set_x(end_point.get_x());
                                        self.current.set_y(end_point.get_y());
                                    }
                                    //else if start x == end x
                                    else if start_point.get_x() == end_point.get_x() {
                                    //  and start y > end y
                                        if start_point.get_y() > end_point.get_y() {
                                    //      the cursor goes on the end
                                            self.current.set_x(start_point.get_x());
                                            self.current.set_y(start_point.get_y());
                                        }
                                    //  and start y < end y
                                        else if start_point.get_y() < end_point.get_y() {
                                    //      the cursor goes on the start                 
                                            self.current.set_x(end_point.get_x());
                                            self.current.set_y(end_point.get_y());                       
                                        }
                                    }
                                    //finally delete the selection
                                    self.selection = None;
                                }
                            }
                        } else {
                            //else the selection doesn't exist
                            //and shift is pressed
                            if key.modifiers.contains(KeyModifiers::SHIFT) {
                            //  create a selection from current position of the cursor to 1 on his right and set the cursor
                                let start_point = Point::new(self.current.get_x(),self.current.get_y());
                                let end_point = Point::new(self.current.get_x(), self.current.get_y() + 1);
                                self.selection = Some(CodeSelection::new(start_point, end_point.clone()));
                                self.current.set_y(end_point.get_y());
                            }
                            //  and shift is not pressed
                            else {
                            //      move the cursor by 1 on his right
                                if let Some(upper_line) = self.current.get_line(self.current.get_x()) {
                                    if self.current.get_y() < upper_line.get_string().len() {
                                        self.current.set_y(self.current.get_y() + 1);
                                        self.selection = None;
                                    }
                                }
                            }
                        }

                        self.current.set_cursor();

                    },
                    KeyCode::Esc => {
                        context.set_focus(None);
                        context.set_hover(self.get_type());             
                    },
                    _ => {}
                }
            } if key.kind == KeyEventKind::Repeat {
                match key.code {
                    KeyCode::Char(char) => {
                        self.current.remove_cursor();
                        if let Some(current_line) = self.current.get_line(self.get_current().get_x()) {
                            self.current.change_line_at_cursor(current_line.get_string()[..self.get_current().get_y()].to_string() + &char.to_string() + &current_line.get_string()[self.current.get_y()..].to_string());    
                        }
                        self.current.set_y(self.current.get_y()+1);
                        self.current.set_cursor();
                    },
                    KeyCode::Delete => {
                        let last_number = self.current.get_content().into_iter().map(|x| x.get_number()).fold(0, |line1, line2| {
                            if line1 > line2 { line1 } else { line2 }
                        });
                        let last_line = self.current.get_line(last_number).unwrap();
                        self.current.change_line(last_line.get_number(), last_line.get_string()[..last_line.get_string().len()-1].to_string());
                    },
                    KeyCode::Enter => {
                        {
                            let mut_code = self.get_mut_current();
                            mut_code.remove_cursor();    
                        }
                        let code = self.get_current().clone();
                        let mut_code = self.get_mut_current();
                        if let Some(current_line) = code.get_content().get(code.get_x()) {
                            let line_number = current_line.get_number().clone();
                            let new_current_string = current_line.get_string()[..code.get_y()].to_string().clone();
                            let new_generated_string = current_line.get_string()[code.get_y()..].to_string().clone();
                            mut_code.flush();
                            for number in 0 .. line_number {
                                if let Some(line) = code.get_line(number) {
                                    mut_code.add_line(line.clone());                                    
                                }
                            }
                            mut_code.add_line(Line::new(current_line.get_number(), new_current_string));
                            mut_code.set_x(code.get_x());
                            mut_code.set_y(code.get_y());
                            mut_code.add_line(Line::new(current_line.get_number() + 1, new_generated_string));
                            for number in current_line.get_number() + 1.. code.get_content().len() {
                                if let Some(line) = code.get_line(number) {
                                    let mut new_line = line.clone();
                                    new_line.set_number(number + 1);
                                    mut_code.add_line(new_line.clone());                                    
                                }
                            }
                            mut_code.set_cursor();
                        }
                    },
                    KeyCode::Up => {
                        self.current.remove_cursor();
                        //Handles up arrow according to the state of the code, if something is selected has a different behaviour from the normal arrow key
                        if let Some(selection) = &mut self.selection {
                            //if something is selected and arrow up is pressed with shift extends the selection to the upper line
                            if key.modifiers.contains(KeyModifiers::SHIFT) {
                                //if the upper line exists moves the selection to the upper line at the same position occupied from cursor on the current line
                                if self.current.get_x()>0 {
                                    let mut current_end = selection.get_end().clone();
                                    current_end.set_x(current_end.get_x() - 1);
                                    //also moves the cursor to the new end
                                    self.current.set_x(current_end.get_x());
                                    if let Some(upper_line) = self.current.get_line(current_end.get_x()) {
                                        if let Some(current_line) = self.current.get_line(current_end.get_x() + 1) {
                                            if upper_line.get_string().len() < current_line.get_string().len() {
                                                current_end.set_y(upper_line.get_string().len());
                                                self.current.set_y(upper_line.get_string().len());    
                                            }
                                            selection.set_end(current_end.clone());
                                        }
                                    }
                                } else {
                                    //else moves the selection to the start of the current line
                                    let mut current_end = selection.get_end().clone();
                                    current_end.set_y(0);
                                    selection.set_end(current_end.clone());
                                    //also moves the cursor to the new end
                                    self.current.set_x(current_end.get_x());
                                    self.current.set_y(current_end.get_y());
                                }
                            } else {
                                //else if there is no shift key pressed removes the selection after setting the cursor to the selection end
                                let mut current_end = selection.get_end().clone();

                                current_end.set_y(current_end.get_y() - 1);
                                self.current.set_x(current_end.get_x());
                                self.current.set_y(current_end.get_y());
                                self.selection = None;
                            }
                        } else {
                            //if nothing is already selected and shift is pressed creates a selection from the current position of the cursor then moves the cursor to its end
                            if key.modifiers.contains(KeyModifiers::SHIFT) {
                                let current_x = self.current.get_x();
                                let current_y = self.current.get_y();
                                let start_point: Point = Point::new(current_x, current_y);
                                let end_point: Point;
                                //if an upper line set the ending point on the upper line
                                if self.current.get_x() > 0 {
                                    end_point= Point::new(current_x - 1, current_y);
                                } else {
                                    //else extend to the start of the current line
                                    end_point= Point::new(current_x, 0);
                                }
                                self.selection = Some(CodeSelection::new(start_point, end_point.clone()));
                                if current_y > 0 {
                                    self.current.set_x(end_point.get_x());
                                    self.current.set_y(end_point.get_y());    
                                }
                            } else {
                                //else if shift is not pressed and a selection doesn't exist and an upper line exists just moves the cursor to the upper line
                                if self.current.get_x() > 0 {
                                    self.current.set_x(self.current.get_x() - 1);
                                    if let Some(upper_line) = self.current.get_line(self.current.get_x()) {
                                        if upper_line.get_string().len() < self.current.get_y() {
                                            self.current.set_y(upper_line.get_string().len());
                                        } else {
                                            let current_y = self.current.get_y();
                                            self.current.set_y(current_y);    
                                        }                                   
                                    }
    
                                }
                            }
                        }

                        self.current.set_cursor();

                    },
                    KeyCode::Down => {
                        let nlines = self.current.get_content().len();
                        self.current.remove_cursor();


                        //Handles up arrow according to the state of the code, if something is selected has a different behaviour from the normal arrow key
                        if let Some(selection) = &mut self.selection {
                            //if something is selected and arrow down is pressed with shift extends the selection to the lower line
                            if key.modifiers.contains(KeyModifiers::SHIFT) {
                                //if the lower line exists moves the selection to the lower line at the same position occupied from cursor on the current line
                                if self.current.get_x() < nlines - 1 {
                                    let mut current_end = selection.get_end().clone();
                                    current_end.set_x(current_end.get_x() + 1);
                                    self.current.set_x(current_end.get_x());

                                    if let Some(lower_line) = self.current.get_line(current_end.get_x()) {
                                        if let Some(current_line) = self.current.get_line(current_end.get_x() - 1) {
                                            //also moves the cursor to the new end
                                            if lower_line.get_string().len() < current_line.get_string().len() {
                                                self.current.set_y(lower_line.get_string().len()-1);
                                            } else {
                                                self.current.set_y(current_end.get_y());
            
                                            }
                                            let end_point = Point::new(self.current.get_x(), self.current.get_y());
                                            selection.set_end(end_point.clone());
                                        }
                                    }
                                } else {
                                    let mut end_y = self.current.get_y();
                                    if let Some(current_line) = self.current.get_content().get(self.current.get_x()) {
                                        end_y = current_line.get_string().len();
                                    }
                                    //else moves the selection to the end of the current line
                                    let mut current_end = selection.get_end().clone();
                                    current_end.set_y(end_y);
                                    selection.set_end(current_end.clone());
                                    //also moves the cursor to the new end
                                    self.current.set_x(current_end.get_x());
                                    self.current.set_y(current_end.get_y());
                            }
                            } else {
                                //else if there is no shift key pressed removes the selection after setting the cursor to the selection end
                                let mut current_end = selection.get_end().clone();
                                current_end.set_y(current_end.get_y() + 1);
                                self.current.set_x(current_end.get_x());
                                self.current.set_y(current_end.get_y());
                                self.selection = None;
                            }
                        } else {
                            //if nothing is already selected and shift is pressed creates a selection from the current position of the cursor then moves the cursor to its end
                            if key.modifiers.contains(KeyModifiers::SHIFT) {
                                let current_x = self.current.get_x();
                                let current_y = self.current.get_y();
                                let start_point: Point = Point::new(current_x, current_y);
                                let end_point: Point;
                                //if a lower line exists set the ending point on the lower line
                                if let Some(_lower_line) = self.current.get_content().get(self.current.get_x() + 1) {
                                    end_point= Point::new(current_x + 1, current_y);
                                } else {
                                    //else extend to the end of the current line
                                    let mut end_y = self.current.get_y();
                                    if let Some(current_line) = self.current.get_content().get(self.current.get_x()) {
                                        end_y = current_line.get_string().len()-1;
                                    }
                                    end_point= Point::new(current_x, end_y);
                                }
                                self.selection = Some(CodeSelection::new(start_point, end_point.clone()));
                                self.current.set_x(end_point.get_x());
                                self.current.set_y(end_point.get_y());
                            } else {
                                //else if shift is not pressed and a selection doesn't exist and an lower line exists just moves the cursor to the lower line
                                let current_x = self.current.get_x();
                                self.current.set_x(current_x + 1);
                                let mut current_y = self.current.get_y();
                                if let Some(lower_line) = self.current.get_content().get(self.current.get_x()) {
                                    if lower_line.get_string().len() < current_y {
                                        self.current.set_y(lower_line.get_string().len());
                                    } else {
                                        self.current.set_y(current_y);
                                    }
                                } else {
                                    //else if a lower line doesn't exist move the cursor to the end of the current line
                                    if let Some(current_line) = self.current.get_content().get(self.current.get_x()) {
                                        current_y = current_line.get_string().len() - 1;
                                    }
                                    self.current.set_x(current_x);
                                    self.current.set_y(current_y);
                                }
                            }
                        }

                        self.current.set_cursor();
                    },
                    KeyCode::Left => {
                        self.current.remove_cursor();

                        //if a selection exist
                        let mut selection_exists = false;
                        let mut start_point = Point::new(self.current.get_x(), self.current.get_y());
                        let mut end_point = Point::new(self.current.get_x(), self.current.get_y());
                        if let Some(selection) = &self.selection {
                            selection_exists = true;
                            start_point = selection.get_start().clone();
                            end_point = selection.get_end().clone();
                        }

                        if selection_exists {
                            if let Some(mutable_selection) = &mut self.selection {
                                //  and shift is also pressed
                                if key.modifiers.contains(KeyModifiers::SHIFT) {

                                    if self.current.get_y() > 0 {
                                        //      if the start x != end x change the end of the selection to the current y to the current y - 1 (the end point)
                                        if start_point.get_x() != end_point.get_x() {
                                            end_point.set_y(end_point.get_y() - 1);
                                            mutable_selection.set_end(end_point.clone());
                                            self.current.set_x(end_point.get_x());
                                        //      then if start x > end x the cursor goes on the left of the end point
                                            if start_point.get_x() > end_point.get_x() {
                                                self.current.set_y(end_point.get_y() - 1);
                                            }
                                        //      else if the start x < end x the cursor goes on the right of the end point
                                            else if start_point.get_x() < end_point.get_x() {
                                                self.current.set_y(end_point.get_y() + 1);
                                            }
                                        }
                                        //      else if the start x == end x
                                        else if start_point.get_x() == end_point.get_x() {
                                    //      if start y != end y move the end on the left
                                            if start_point.get_y() != end_point.get_y() {
                                                end_point.set_y(end_point.get_y() - 1);
                                    //          if start y > end y set the cursor on the left of the end point
                                                if start_point.get_y() > end_point.get_y() {
                                                    self.current.set_y(end_point.get_y() - 1);
                                                }
                                    //          else if start y < end y set the cursor on the right of the end point
                                                else if start_point.get_y() < end_point.get_y() {
                                                    self.current.set_y(end_point.get_y() + 1);
                                                }
                                            }
                                        }
                                    }
                                
                                }
                                //  and shift is not pressed
                                else {
                                    //move the cursor the left of the selection and then delete the selection
                                    //if the start x > end x
                                    if start_point.get_x() > end_point.get_x() {
                                    //  the cursor goes to the end
                                        self.current.set_x(end_point.get_x());
                                        self.current.set_y(end_point.get_y());
                                    }
                                    //else if start x < end x
                                    else if start_point.get_x() < end_point.get_x() {
                                    //  the cursor goes to the start
                                        self.current.set_x(start_point.get_x());
                                        self.current.set_y(start_point.get_y());
                                    }
                                    //else if start x == end x
                                    else if start_point.get_x() == end_point.get_x() {
                                    //  and start y > end y
                                        if start_point.get_y() > end_point.get_y() {
                                    //      the cursor goes on the end
                                            self.current.set_x(end_point.get_x());
                                            self.current.set_y(end_point.get_y());
                                        }
                                    //  and start y < end y
                                        else if start_point.get_y() < end_point.get_y() {
                                    //      the cursor goes on the start                 
                                            self.current.set_x(start_point.get_x());
                                            self.current.set_y(start_point.get_y());                       
                                        }
                                    }
                                    //finally delete the selection
                                    self.selection = None;
                                }
                            }
                        } else {
                            //else the selection doesn't exist
                            //and shift is pressed
                            if key.modifiers.contains(KeyModifiers::SHIFT) {
                            //  create a selection from current position of the cursor to 1 on his left and set the cursor
                                if self.current.get_y() > 0 {
                                    let start_point = Point::new(self.current.get_x(),self.current.get_y());
                                    let end_point = Point::new(self.current.get_x(), self.current.get_y() - 1);
                                    self.selection = Some(CodeSelection::new(start_point, end_point));    
                                }
                            }
                            //  and shift is not pressed
                            else {
                            //      move the cursor by 1 on his left and set selection to None
                                if self.current.get_y() > 0 {
                                    self.current.set_y(self.current.get_y() - 1);
                                    self.selection = None;
                                }
                            }
                        }

                        self.current.set_cursor();

                    },
                    KeyCode::Right => {
                        self.current.remove_cursor();

                        //if a selection exist
                        let mut selection_exists = false;
                        let mut start_point = Point::new(self.current.get_x(), self.current.get_y());
                        let mut end_point = Point::new(self.current.get_x(), self.current.get_y());
                        if let Some(selection) = &self.selection {
                            selection_exists = true;
                            start_point = selection.get_start().clone();
                            end_point = selection.get_end().clone();
                        }
                        if selection_exists {
                            if let Some(mutable_selection) = &mut self.selection {
                                //  and shift is also pressed
                                if key.modifiers.contains(KeyModifiers::SHIFT) {
                                //      if the start x != end x change the end of the selection to the current y to the current y + 1 (the end point)
                                    if start_point.get_x() != end_point.get_x() {
                                        if let Some(current_line) = self.current.get_line(end_point.get_x()) {
                                            if self.current.get_y() < current_line.get_string().len() - 1 {
                                                end_point.set_y(end_point.get_y() + 1);   
                                                mutable_selection.set_end(end_point.clone());
                                                self.current.set_x(end_point.get_x());
                                        //      then if start x > end x the cursor goes on the left of the end point
                                                if start_point.get_x() > end_point.get_x() {
                                                    self.current.set_y(end_point.get_y());
                                                }
                                        //      else if the start x < end x the cursor goes on the right of the end point
                                                else if start_point.get_x() < end_point.get_x() {
                                                    self.current.set_y(end_point.get_y());
                                                }
                                            }
                                        }
                                    }
                                //      else if the start x == end x
                                    else if start_point.get_x() == end_point.get_x() {
                                //      if start y != end y move the end on the left
                                        if start_point.get_y() != end_point.get_y() {
                                            end_point.set_y(end_point.get_y() + 1);
                                //          if start y > end y set the cursor on the left of the end point
                                            if start_point.get_y() > end_point.get_y() {
                                                self.current.set_y(start_point.get_y() - 1);
                                            }
                                //          else if start y < end y set the cursor on the right of the end point
                                            else if start_point.get_y() < end_point.get_y() {
                                                self.current.set_y(end_point.get_y() + 1);
                                            }
                                            mutable_selection.set_end(end_point);
                                        }
                                    }
                                }
                                //  and shift is not pressed
                                else {
                                    //move the cursor the right of the selection and then delete the selection
                                    //if the start x > end x
                                    if start_point.get_x() > end_point.get_x() {
                                    //  the cursor goes to the start
                                        self.current.set_x(start_point.get_x());
                                        self.current.set_y(start_point.get_y());
                                    }
                                    //else if start x < end x
                                    else if start_point.get_x() < end_point.get_x() {
                                    //  the cursor goes to the start
                                        self.current.set_x(end_point.get_x());
                                        self.current.set_y(end_point.get_y());
                                    }
                                    //else if start x == end x
                                    else if start_point.get_x() == end_point.get_x() {
                                    //  and start y > end y
                                        if start_point.get_y() > end_point.get_y() {
                                    //      the cursor goes on the end
                                            self.current.set_x(start_point.get_x());
                                            self.current.set_y(start_point.get_y());
                                        }
                                    //  and start y < end y
                                        else if start_point.get_y() < end_point.get_y() {
                                    //      the cursor goes on the start                 
                                            self.current.set_x(end_point.get_x());
                                            self.current.set_y(end_point.get_y());                       
                                        }
                                    }
                                    //finally delete the selection
                                    self.selection = None;
                                }
                            }
                        } else {
                            //else the selection doesn't exist
                            //and shift is pressed
                            if key.modifiers.contains(KeyModifiers::SHIFT) {
                            //  create a selection from current position of the cursor to 1 on his right and set the cursor
                                let start_point = Point::new(self.current.get_x(),self.current.get_y());
                                let end_point = Point::new(self.current.get_x(), self.current.get_y() + 1);
                                self.selection = Some(CodeSelection::new(start_point, end_point.clone()));
                                self.current.set_y(end_point.get_y() + 1);
                            }
                            //  and shift is not pressed
                            else {
                            //      move the cursor by 1 on his right
                                if let Some(upper_line) = self.current.get_line(self.current.get_x()) {
                            //      if is not already at the end of the line
                                    if self.current.get_y() < upper_line.get_string().len() - 1 {
                                        self.current.set_y(self.current.get_y() + 1);
                                    }
                                }
                            }
                        }

                        self.current.set_cursor();

                    },
                    _ => {}
                }
            }
        }
    }
}

impl CodeComponent {

    pub fn new() -> Self {
        let code = Code::new();
        CodeComponent {
            current: code.clone(),
            history: CodeHistory::new(code.clone()),
            selection: None,
        }
    }

    pub fn set_current(&mut self, active_file: Option<PathBuf>) {
        if let Some(path) = active_file {
            let file = File::open(path);
            if let Ok(mut file) = file {
                let mut contents = String::new();
                let _ = file.read_to_string(&mut contents);
                contents
                .split("\n")
                .enumerate()
                .for_each(|tuple| {
                    let line = Line::new(tuple.0, tuple.1.to_string());
                    self.current.add_line(line);
                })
            }
            self.current.set_cursor();
        }
    }

    pub fn get_current(&self) -> &Code {
        &self.current
    }

    pub fn get_mut_current(&mut self) -> &mut Code {
        &mut self.current
    }

    pub fn get_history(&self) -> &CodeHistory {
        &self.history
    }

    pub fn get_selection(&self) -> &Option<CodeSelection> {
        &self.selection
    }

    pub fn get_mut_selection(&mut self) -> Option<&mut CodeSelection> {
        self.selection.as_mut()
    }
}