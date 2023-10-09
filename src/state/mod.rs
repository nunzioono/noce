use std::{path::PathBuf, ops::ControlFlow, env::current_dir};

use crossterm::event::{Event, KeyEventKind, KeyCode};

use self::{project::ProjectComponent, code::{CodeComponent, code::Code}, terminal::TerminalComponent};

pub mod code;
pub mod project;
pub mod terminal;

#[derive(PartialEq, Eq, Clone)]
pub enum ComponentType {
    Project,
    Code,
    Terminal
}

pub trait Component {
    fn get_type(&self) -> ComponentType;
    fn handle_event(&mut self, context: &mut AppContext, event: Event);
}

pub struct AppContext {
    active_folder: PathBuf,
    active_file: Option<PathBuf>,
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
            focus: None,
            hover: ComponentType::Project,
        }
    }
}

impl AppContext {
    
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

pub struct App {
    project: ProjectComponent,
    code: CodeComponent,
    terminal: TerminalComponent,    
}

impl Default for App {
    fn default() -> App {
        App {
            project: ProjectComponent::new(),
            code: CodeComponent::new(Code::new()),
            terminal: TerminalComponent::new()
        }
    }
}

impl App {

    pub fn get_project(&self) -> &ProjectComponent {
        &self.project
    }

    pub fn get_code(&self) -> &CodeComponent {
        &self.code
    }

    pub fn get_terminal(&self) -> &TerminalComponent {
        &self.terminal
    }

    pub fn handle_event(&mut self, context: &mut AppContext, focus: Option<ComponentType>, event: Event) -> ControlFlow<()>{
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
        }else {
            if let Event::Key(key) = event {
                if key.kind == KeyEventKind::Press {
                    if key.code == KeyCode::Tab {
                        if focus.is_none() {
                            context.hover = match context.hover() {
                                ComponentType::Project => {
                                    ComponentType::Code
                                },
                                ComponentType::Code => {
                                    ComponentType::Terminal
                                },
                                ComponentType::Terminal => {
                                    ComponentType::Project
                                }
                            };
                        }
                    }
                    if key.code == KeyCode::Enter {
                        context.focus = Some(context.hover().clone());
                    }
                    if key.code == KeyCode::Esc {
                        return ControlFlow::Break(());
                    }
                }    
            }
        }
        ControlFlow::Continue(())   

    }

 
}