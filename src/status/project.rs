use std::{path::PathBuf, fs::{read_dir, File, self}};

use crossterm::event::{Event, KeyEventKind, KeyCode, KeyModifiers};

#[derive(PartialEq, Eq)]
enum ContentType {
    FILE,
    FOLDER
}


pub struct ProjectState {
    contents: Vec<PathBuf>,
    hover: Option<u16>,
    focus: Option<u16>
}

impl ProjectState {


    pub fn new(folder: &PathBuf) -> ProjectState {
        let mut contents: Vec<PathBuf> = vec![];
        if let Ok(entries) = read_dir(folder) {
            entries
            .filter(|entry| {
                entry.is_ok()
            })
            .map(|entry| {
                entry.unwrap().path()
            })
            .for_each(|entry_name| contents.push(entry_name));
        }

        ProjectState {
            contents: contents,
            hover: Some(0),
            focus: None
         }
    }

    fn add_content(&mut self, parent_folder: &PathBuf, child_name: String, child_type: ContentType) {
        let mut child_pathbuf = parent_folder.clone();
        child_pathbuf.push(child_name);

        let clone_child_path = child_pathbuf.clone();

        match child_type {
            ContentType::FILE => {
                let res = File::create(child_pathbuf);

                if res.is_ok() {
                    self.contents.push(clone_child_path);
                }
            },
            ContentType::FOLDER => {
                let res = fs::create_dir(child_pathbuf);
                if res.is_ok() {
                    self.contents.push(clone_child_path);
                }
            }
        }
        if child_type == ContentType::FILE {
        }
        if child_type == ContentType::FOLDER {

        }
    }

    pub fn process(&mut self, event: Event, mut folder_ref: PathBuf, mut file_ref: Option<PathBuf>) {
        if let Event::Key(key) = event {
            if key.kind == KeyEventKind::Press || key.kind == KeyEventKind::Repeat {
                match key.code {
                    KeyCode::Up => {
                        if let Some(value) = self.focus {
                            if value > 0 {
                                self.focus = Some(value - 1);
                            }
                        } else {
                            self.focus = Some(0);
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
                            folder_ref = selected_item.clone();
                        } else if  selected_item.is_file() {
                            file_ref = Some(selected_item);
                        }
                    },
                    KeyCode::Esc => {
                        self.hover = self.focus;
                        self.focus = None;
                        let selected_item = self.contents[self.focus.unwrap() as usize].clone();
                        if  selected_item.is_file() {
                            file_ref = None;
                        }
                    },
                    KeyCode::Char(char) => {
                        if char == 'f' && key.modifiers.contains(KeyModifiers::CONTROL) {
                            self.add_content(&folder_ref, String::from("new_file"), ContentType::FILE);
                        }
                        if char == 'd' && key.modifiers.contains(KeyModifiers::CONTROL) {
                            self.add_content(&folder_ref, String::from("new_folder"), ContentType::FOLDER);
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}