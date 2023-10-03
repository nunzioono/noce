pub mod code;
pub mod project;
pub mod terminal;

use std::{path::PathBuf, env::current_dir, sync::{Arc, RwLock}, cell::RefCell};

use crossterm::event::{Event, KeyCode, KeyEventKind};

use self::{code::CodeState, project::ProjectState, terminal::TerminalState};

#[derive(PartialEq, Eq, Clone)]
pub enum Panel {
    Code,
    Project,
    Terminal,
}

pub struct App {
    folder: Arc<RwLock<PathBuf>>,
    file: RefCell<Option<PathBuf>>,
    hover: Option<Panel>,
    focus: Option<Panel>,
    code_state: CodeState,
    project_state: ProjectState,
    terminal_state: TerminalState,
}

impl Default for App {

    fn default() -> App {
        let folder = Arc::new(RwLock::new(current_dir().unwrap()));
        let folder_w = Arc::clone(&folder);
        let mut path_w = folder_w.write().unwrap().to_path_buf();
        App { folder: folder, file: RefCell::new(None), hover: None, focus: None, code_state: CodeState::new(&None), project_state: ProjectState::new(&mut path_w), terminal_state: TerminalState::new() }
    }

}

impl App {

    pub fn get_folder(&self) -> PathBuf {
        (*self.folder.clone().read().unwrap()).clone()
    }

    pub fn is_active_file(&self) -> bool {
        self.file.take().is_some()
    }

    pub fn get_file(&self) -> PathBuf {
        self.file.take().unwrap()
    }

    pub fn set_file(&mut self, file: PathBuf) {
        self.file = RefCell::new(Some(file));
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
            self.project_state.process(event, self.folder.write().unwrap().to_path_buf(), self.file.take());
        } else if self.is_focusing() && self.get_focus_panel() == Panel::Code {
            self.code_state.process(event, self.file.take());
        } else if self.is_focusing() && self.get_focus_panel() == Panel::Terminal {
            self.terminal_state.process(event, &mut self.folder.write().unwrap().to_path_buf());
        } 
    }

}