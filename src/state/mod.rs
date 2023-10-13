use std::{path::PathBuf, ops::ControlFlow, env::{current_dir, self}};

use crossterm::event::{Event, KeyEventKind, KeyCode};

use self::{project::ProjectComponent, code::{CodeComponent, code::Code}, terminal::TerminalComponent};

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
    pub fn new(active_folder: PathBuf, active_file: Option<PathBuf>, focus: Option<ComponentType>, hover: ComponentType) -> AppContext {
        AppContext {
            active_folder: active_folder,
            active_file: active_file,
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
        let path = env::current_dir();
        let mut contents = vec![];
        if let Ok(path) = path {
            if let Ok(path) = path.read_dir() {
                path.into_iter().for_each(|entry| {
                    if let Ok(entry) = entry {
                        contents.push(entry.path());                        
                    }
                }); 
            }
        }
        App {
            project: ProjectComponent::new(contents),
            code: CodeComponent::new(Code::new()),
            terminal: TerminalComponent::new()
        }
    }
}

impl App {

    pub fn new(project: ProjectComponent, code: CodeComponent, terminal: TerminalComponent, path: PathBuf) -> App {
        let vec_contents: Vec<PathBuf> = path.read_dir().unwrap().into_iter().map(|entry| entry.unwrap().path()).collect();

        App {
            project: ProjectComponent::new(vec_contents),
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
                            return ControlFlow::Break(());
                        },
                        _ => {
                            println!("Event got ignored!")
                        }
                        
                    }
                }    
            }
        }
        ControlFlow::Continue(()) 
  

    }
}