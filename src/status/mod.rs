pub mod code;
pub mod project;
pub mod terminal;

use std::{path::PathBuf, env::current_dir, sync::{Arc, RwLock}};

use crossterm::event::{Event, KeyCode, KeyEventKind};

use self::{code::CodeState, project::ProjectState, terminal::{TerminalState, terminal_utils::TerminalCommand}};

#[derive(PartialEq, Eq, Clone)]
pub enum Panel {
    Code,
    Project,
    Terminal,
}

pub struct App {
    folder: Arc<RwLock<PathBuf>>,
    file: Option<PathBuf>,
    hover: Option<Panel>,
    focus: Option<Panel>,
    code_state: CodeState,
    project_state: ProjectState,
    terminal_state: TerminalState,
}

impl Default for App {
    fn default() -> App {
        App { folder: Arc::new(RwLock::new(current_dir().unwrap())), file: None, hover: None, focus: None, code_state: CodeState::default(), project_state: ProjectState::default(), terminal_state: TerminalState::new(TerminalCommand::default()) }
    }

}

impl App {

    pub fn get_folder(&self) -> PathBuf {
        (*self.folder.clone().read().unwrap()).clone()
    }

    pub fn is_active_file(&self) -> bool {
        self.file.is_some()
    }

    pub fn get_file(&self) -> PathBuf {
        self.file.clone().unwrap()
    }

    pub fn set_file(&mut self, file: PathBuf) {
        self.file = Some(file);
    }   

    pub fn is_hovering(&self) -> bool {
        self.hover.is_some()
    }

    pub fn is_focusing(&self) -> bool {
        self.focus.is_some()
    }

    pub fn get_hover_panel(&self) -> Panel {
        self.hover.clone().unwrap()
    }

    pub fn get_focus_panel(&self) -> Panel {
        self.focus.clone().unwrap()
    }

    pub fn set_hover_panel(&mut self, panel: Panel) {
        self.hover = Some(panel);
    }

    pub fn set_focus_panel(&mut self, panel: Panel) {
        self.focus = Some(panel);
    }

    pub fn process(&mut self, event: Event, quit: Arc<RwLock<bool>>) {
        if let Event::Key(key) = event {
            if !self.is_focusing() && key.kind == KeyEventKind::Press {
                if key.code == KeyCode::Tab {
                    self.set_hover_panel(Panel::Project);
                }
                if key.code == KeyCode::Enter {
                    self.set_focus_panel(self.get_hover_panel());
                }
                if key.code == KeyCode::Esc {
                    if let Ok(mut write_guard) = quit.write() {
                        *write_guard = true;
                    }          
                }
            }    
        } else if self.is_focusing() && self.get_focus_panel() == Panel::Project {
            self.project_state.process(event);
        } else if self.is_focusing() && self.get_focus_panel() == Panel::Code {
            self.code_state.process(event);
        } else if self.is_focusing() && self.get_focus_panel() == Panel::Terminal {
            self.terminal_state.process(event, Arc::clone(&self.folder));
        } 
    }

}