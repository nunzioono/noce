use std::{path::PathBuf, env::{current_dir, self}};

use crossterm::event::{Event, KeyEventKind, KeyCode};

use self::{project::ProjectComponent, code::CodeComponent, terminal::TerminalComponent};

pub mod code;
pub mod project;
pub mod terminal;

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum ComponentType {
    Project,
    Code,
    Terminal
}

pub trait Component {
    fn get_type(&self) -> ComponentType;
    fn handle_event(&mut self, context: &mut AppContext, event: Event);
}

#[derive(PartialEq, Eq, Debug)]
pub struct AppContext {
    active_folder: PathBuf,
    active_file: Option<PathBuf>,
    active_file_changed: bool,
    focus: Option<ComponentType>,
    hover: ComponentType,
}

impl Default for AppContext {
    fn default() -> AppContext {
        let result = current_dir();
        let folder: PathBuf;
        if let Ok(result) = result {
            folder = result;
        } else {
            folder = PathBuf::default();
        }
        AppContext {
            active_folder: folder,
            active_file: None,
            active_file_changed: false,
            focus: None,
            hover: ComponentType::Project,
        }
    }
}

impl AppContext {
    pub fn new(active_folder: PathBuf, active_file: Option<PathBuf>, focus: Option<ComponentType>, hover: ComponentType) -> AppContext {
        AppContext {
            active_folder: active_folder,
            active_file: active_file,
            active_file_changed: false,
            focus: focus,
            hover: hover,
        }
    }

   // Getter for active_folder
   pub fn active_folder(&self) -> &PathBuf {
        &self.active_folder
    }

    // Setter for active_folder
    pub fn set_active_folder(&mut self, path: PathBuf) {
        self.active_folder = path;
    }

    // Getter for active_file
    pub fn active_file(&self) -> &Option<PathBuf> {
        &self.active_file
    }

    // Setter for active_file
    pub fn set_active_file(&mut self, path: Option<PathBuf>) {
        self.active_file = path;
        self.active_file_changed = true;
    }

    pub fn active_file_changed(&self) -> bool {
        self.active_file_changed
    }

    pub fn set_active_file_changed(&mut self, change: bool) {
        self.active_file_changed = change;
    }
    
    // Getter for focus
    pub fn focus(&self) -> &Option<ComponentType> {
        &self.focus
    }

    // Setter for focus
    pub fn set_focus(&mut self, component: Option<ComponentType>) {
        self.focus = component;
    }

    // Getter for hover
    pub fn hover(&self) -> &ComponentType {
        &self.hover
    }

    // Setter for hover
    pub fn set_hover(&mut self, component: ComponentType) {
        self.hover = component;
    }
    
}

#[derive(Debug, PartialEq, Eq)]
pub struct App {
    project: ProjectComponent,
    code: CodeComponent,
    terminal: TerminalComponent,    
}

impl Default for App {
    fn default() -> App {

        App {
            project: ProjectComponent::new(env::current_dir().unwrap().to_path_buf()),
            code: CodeComponent::new(),
            terminal: TerminalComponent::new()
        }
    }
}

impl App {

    pub fn new(_project: ProjectComponent, code: CodeComponent, terminal: TerminalComponent, path: PathBuf) -> App {

        App {
            project: ProjectComponent::new(path),
            code: code,
            terminal: terminal 
        }
    }

    pub fn get_project(&self) -> &ProjectComponent {
        &self.project
    }

    pub fn get_code(&self) -> &CodeComponent {
        &self.code
    }

    pub fn get_terminal(&self) -> &TerminalComponent {
        &self.terminal
    }

    pub fn get_mut_code(&mut self) -> &mut CodeComponent {
        &mut self.code
    }

    pub fn handle_event(&mut self, context: &mut AppContext, focus: Option<ComponentType>, event: Event) -> bool {


        if focus.is_some() {
            if let Some(focus) = focus {
                match focus {
                    ComponentType::Project => {
                        self.project.handle_event(context, event);
                    },
                    ComponentType::Code => {
                        self.code.handle_event(context, event);
                    },
                    ComponentType::Terminal => {
                        self.terminal.handle_event(context, event);
                    }
                }
            }
        } else {
            if let Event::Key(key) = event {
                if key.kind == KeyEventKind::Press {
                    let hover: Option<ComponentType> = Some(context.hover().clone());
                    match key.code {
                        KeyCode::Tab => {
                            if focus.is_none() {
                                if let Some(hover) = hover {
                                    let next_hover = match hover {
                                        ComponentType::Project => {
                                            ComponentType::Code
                                        },
                                        ComponentType::Code => {
                                            ComponentType::Terminal
                                        },
                                        ComponentType::Terminal => {
                                            ComponentType::Project
                                        },
                                    };
                                    context.set_hover(next_hover);
                                }
                            }
                        },
                        KeyCode::Enter => {
                            context.set_focus(Some(context.hover().clone()));
                        },
                        KeyCode::Esc => {
                            return false;
                        },
                        _ => {
                        }
                        
                    }
                }    
            }
        }
        true
  

    }
}