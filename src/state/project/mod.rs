use std::{path::PathBuf, fs::{File, create_dir}};

use crossterm::event::{Event, KeyEventKind, KeyCode, KeyModifiers};

use super::{Component, ComponentType};

#[derive(PartialEq, Eq)]
pub enum ContentType {
    FILE,
    FOLDER
}

pub struct ProjectComponent {
    contents: Vec<PathBuf>,
    hover: Option<u16>,
    focus: Option<u16>,
}

impl ProjectComponent {
    
    pub fn get_hover(&self) -> &Option<u16> {
        &self.hover
    }

    pub fn get_focus(&self) -> &Option<u16> {
        &self.focus
    }

    pub fn set_hover(&mut self, hover: u16) -> &mut Self {
        self.hover = Some(hover);
        self
    }

    pub fn set_focus(&mut self, focus: u16) -> &mut Self {
        self.focus = Some(focus);
        self
    }

}

impl Component for ProjectComponent {

    fn get_type(&self) -> ComponentType {
        ComponentType::Project
    }

    fn handle_event(&mut self, context: &mut super::AppContext, event: Event) {
        if let Event::Key(key) = event {
            if key.kind == KeyEventKind::Press || key.kind == KeyEventKind::Repeat {
                match key.code {
                    KeyCode::Up => {
                        if let Some(value) = self.get_hover().clone() {
                            if value > 0 {
                                self.set_hover(value - 1);
                            }
                        } else {
                            self.set_hover(0);
                        }
                    },
                    KeyCode::Down => {
                        if let Some(value) = self.focus {
                            if value < self.contents.len() as u16 {
                                self.focus = Some(value + 1);
                            }
                        } else {
                            self.focus = Some(0);
                        }
                    },
                    _ => {}
                }
            }
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Enter => {
                        self.focus = self.hover;
                        self.hover = None;
                        let selected_item = self.contents[self.focus.unwrap() as usize].clone();
                        if selected_item.is_dir() {
                            context.set_active_folder(selected_item);
                        } else if  selected_item.is_file() {
                            context.set_active_file(Some(selected_item));
                        }
                    },
                    KeyCode::Char(char) => {
                        if char == 'f' && key.modifiers.contains(KeyModifiers::CONTROL) {
                            self.add_content(context.active_folder(), String::from("new_file"), ContentType::FILE);
                        }
                        if char == 'd' && key.modifiers.contains(KeyModifiers::CONTROL) {
                            self.add_content(context.active_folder(), String::from("new_folder"), ContentType::FOLDER);
                        }
                    },
                    KeyCode::Esc => {
                        context.set_focus(None);
                        context.set_hover(self.get_type());                   
                    }
                    _ => {}
                }
            }
        }
    }
}

impl ProjectComponent {
    pub fn new() -> Self {
        ProjectComponent {
            contents: Vec::new(),
            hover: None,
            focus: None,
        }
    }

    pub fn add_content(&mut self, parent: &PathBuf, content: String, content_type: ContentType) {
        let mut new_file_path = parent.clone();
        new_file_path.push(content);
        if content_type == ContentType::FILE {
            if let Ok(_file) = File::create(new_file_path) {
                return;
            }
        }
        else if content_type == ContentType::FOLDER {
            if let Ok(_) = create_dir(new_file_path) {
                return;
            }
        }
    }

}