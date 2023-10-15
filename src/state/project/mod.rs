use std::{path::{PathBuf, Path}, fs::{File, create_dir, read_dir, rename, remove_file, remove_dir_all}};

use crossterm::event::{Event, KeyEventKind, KeyCode, KeyModifiers};

use super::{Component, ComponentType, AppContext};

#[derive(PartialEq, Eq)]
pub enum ContentType {
    FILE,
    FOLDER
}

#[derive(Debug, PartialEq, Eq)]
pub struct ProjectComponent {
    contents: Vec<PathBuf>,
    hover: usize,
    focus: Option<usize>,
    edit: bool,
    first_edit: bool,
    edit_extension: bool,
    popup: bool,
    popup_decision: bool,
}

impl ProjectComponent {

    pub fn update_contents(&mut self, active_folder: &PathBuf) {
        self.contents.clear();

        if let Ok(entries) = read_dir(active_folder) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    self.contents.push(path);
                }
            }
        }

    }


    pub fn get_contents(&self) -> &Vec<PathBuf> {
        &self.contents
    }
    
    pub fn get_hover(&self) -> &usize {
        &self.hover
    }

    pub fn get_focus(&self) -> &Option<usize> {
        &self.focus
    }

    pub fn set_hover(&mut self, hover: usize) -> &mut Self {
        self.hover = hover;
        self
    }

    pub fn set_focus(&mut self, focus: usize) -> &mut Self {
        self.focus = Some(focus);
        self
    }

    pub fn get_popup(&self) -> bool {
        self.popup
    }

    pub fn set_popup(&mut self, value: bool) {
        self.popup = value;
    }

    pub fn get_popup_decision(&self) -> bool {
        self.popup_decision
    }

}

impl Component for ProjectComponent {

    fn get_type(&self) -> ComponentType {
        ComponentType::Project
    }

    fn handle_event(&mut self, context: &mut AppContext, event: Event) {
        if let Event::Key(key) = event {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Left => {
                        if self.popup {
                            self.popup_decision = true;
                        }
                    },
                    KeyCode::Right => {
                        if self.popup {
                            self.popup_decision = false;
                        }
                    },
                    KeyCode::Up => {
                        if !self.edit {
                            if self.get_hover() > &0 {
                                self.set_hover(self.get_hover() - 1);
                            } else {
                                self.set_hover(0);
                            }    
                        }
                    },
                    KeyCode::Down => {
                        if !self.edit {
                            if self.get_hover() < &(self.contents.len() - 1){
                                self.set_hover(self.get_hover() + 1);
                            } else {
                                self.set_hover(self.contents.len() - 1);
                            }    
                        }
                    },
                    KeyCode::Enter => {
                        if !self.popup {
                                                    if !self.edit {
                            self.set_focus(self.get_hover().clone());

                            if let Some(focus) = self.get_focus() {
                                let selected_item = self.contents[focus.clone()].clone();
    
                                if selected_item.is_dir() {
                                    context.set_active_folder(selected_item.clone());
                                    self.update_contents(&selected_item)
                                } else if  selected_item.is_file() {
                                    context.set_active_file(Some(selected_item));
                                }                                    
                            }    
                        } else {
                            self.edit = false;
                            self.edit_extension = false;
                        }
                    } else {
                        if self.popup_decision {
                            let hover = self.contents[self.get_hover().clone()].clone();
                            if hover.is_file() {
                                let _ = remove_file(hover);
                            } else if hover.is_dir() {
                                let _ = remove_dir_all(hover);
                            }
                        }
                        self.popup = false;

                        self.update_contents(context.active_folder());

                    }


                    },
                    KeyCode::Char(char) => {
                        let char = char.to_lowercase().last();
                        if let Some(char) = char {
                            if char == 'f' && key.modifiers.contains(KeyModifiers::CONTROL) {
                                self.add_content(context.active_folder(), String::from("new_file"), ContentType::FILE);
                            }
                            else if char == 'd' && key.modifiers.contains(KeyModifiers::CONTROL) {
                                self.add_content(context.active_folder(), String::from("new_folder"), ContentType::FOLDER);
                            }
                            else if char == 'r' && key.modifiers.contains(KeyModifiers::CONTROL) {
                                if self.get_focus().is_none() && !self.edit {
                                    self.edit = true;
                                    self.first_edit = true;
                                }
                            }
                            else if char == 'x' && key.modifiers.contains(KeyModifiers::CONTROL) {
                                //eliminate folder or file which is hovered
                                if self.get_focus().is_none() {
                                    self.popup = true;
                                }
                            } else {
                                if self.edit {
                                    if let Some(thing) = self.contents.get(self.get_hover().clone()) {
                                        let path = thing.as_path();
                                        if let Some(name) = thing.file_name() {
                                            if let Some(name_as_str) = name.to_str() {
                                                let from = name_as_str.to_string();
                                                let mut to = from.clone();

                                                if !self.edit_extension {
                                                    //if is the first char the user digits the name goes resetted
                                                    if self.first_edit {
                                                        to.clear();
                                                        self.first_edit = false;
                                                    } else if char == '.' && path.is_file() {
                                                        //if the user pressed . and the name is not empty we want to enter in extension mode to add an extension
                                                        self.edit_extension = true;
                                                    } 
                                                    to.push(char);
                                                    let _ = rename(from, to);
    
                                                } else {

                                                    if let Some(extension) = path.extension() {
                                                        if let Some(extension) = extension.to_str() {
                                                            let mut extension = extension.to_string();
                                                            extension.push(char);
                                                            let to_path = Path::new(&name_as_str.to_string()).with_extension(extension);
                                                            let _ = rename(from, to_path);
                                                        }
                                                    } else {
                                                        let to_path = Path::new(&name_as_str.to_string()).with_extension(char.to_string());
                                                        let _ = rename(from, to_path);
                                                    }
                                                }
                                                self.update_contents(context.active_folder());
                                            } 
                                        }
                                    }
                                }
                            }   
                        }

                    },
                    KeyCode::Esc => {
                        context.set_focus(None);
                        context.set_hover(self.get_type());                   
                    }
                    _ => {}
                }
            }
            else if key.kind == KeyEventKind::Repeat {
                match key.code {
                    KeyCode::Up => {
                        if self.get_hover() > &0 {
                            self.set_hover(self.get_hover() - 1);
                        } else {
                            self.set_hover(0);
                        }
                    },
                    KeyCode::Down => {
                        if self.get_hover() < &self.contents.len() {
                            self.set_hover(self.get_hover() + 1);
                        } else {
                            self.set_hover(0);
                        }
                    },
                    _ => {}
                }
            }
        }
    }
}

impl ProjectComponent {
    pub fn new(active_folder: PathBuf) -> Self {
        let mut contents: Vec<PathBuf> = vec![];

        if let Ok(entries) = read_dir(active_folder) {
            for entry in entries {
                if let Ok(entry) = entry {
                    contents.push(entry.path());
                }
            }
        }

        ProjectComponent {
            contents: contents,
            hover: 0,
            focus: None,
            edit: false,
            first_edit: false,
            edit_extension: false,
            popup: false,
            popup_decision: true
        }
    }

    pub fn add_content(&mut self, parent: &PathBuf, content: String, content_type: ContentType) {
        let mut new_file_path = parent.clone();
        new_file_path.push(content);
        if content_type == ContentType::FILE {
            if let Ok(_file) = File::create(new_file_path) {
                self.update_contents(parent);
                return;
            }
        }
        else if content_type == ContentType::FOLDER {
            if let Ok(_) = create_dir(new_file_path) {
                self.update_contents(parent);
                return;
            }
        }
    }

}
