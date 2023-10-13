use std::{sync::{Arc, RwLock}, error::Error};

use ratatui::{Terminal, prelude::{Backend, Rect, Alignment, Layout, Direction, Constraint}, Frame, style::{Style, Modifier, Color, Stylize}, widgets::{Paragraph, Block, BorderType, Borders}};
use ratatui_textarea::{CursorMove, TextArea};

use crate::state::{App, project::ProjectComponent, code::CodeComponent, terminal::{TerminalComponent, self}, AppContext, ComponentType};

use super::System;

// Define a generic UiSystem struct implementing the System trait.
pub struct UiSystem {
    // Add your fields here if needed.
}

impl UiSystem {
    pub fn new() -> Self {
        UiSystem {
            // Initialize your fields here if needed.
        }
    }
}

impl System for UiSystem {
    
}

impl UiSystem {
    pub fn start<B: Backend>(&self, terminal: Arc<RwLock<Terminal<B>>>,
        app: Arc<RwLock<App>>,
        context: Arc<RwLock<AppContext>>
    ) -> Result<(),Box<dyn Error>> {
        loop {
            if let Ok(app_read_guard) = app.read()  {
                if let Ok(context_read_guard) = context.read() {
                    terminal.write().unwrap().draw(|f| self.ui(f, &app_read_guard, &context_read_guard))?;
                }
            }     
        }
    }

    fn ui<B: Backend>(&self, f: &mut Frame<B>, app: &App, context: &AppContext) {
        let (title_area, main_area) = self.calculate_layout(f.size());
        let project_area = main_area[0];
        let code_area = main_area[1];
        let terminal_area = main_area[2];
        
        self.render_title(f, title_area);
        self.render_editor(app, context, f, project_area, code_area, terminal_area, &app.get_project(), &app.get_code(), &app.get_terminal());
    }
    
    fn render_editor<B: Backend>(&self, app: &App, context: &AppContext, frame: &mut Frame<B>, project_area:Rect, code_area:Rect, terminal_area:Rect, project: &ProjectComponent, code: &CodeComponent, terminal: &TerminalComponent) {
        let mut default_style: Style = Style::new();
        default_style = default_style.blue().on_white().bold().italic();
        let history: Vec<String>;
        let context_focus: Option<ComponentType> = context.focus().clone();
        let context_hover: ComponentType = context.hover().clone(); 

        history = (*terminal.get_history()).to_string().split('\n').into_iter().map(|el| el.to_string()).collect();
    
        let mut terminal_text_area = TextArea::new(history.clone());
        terminal_text_area.set_cursor_line_style(Style::default().add_modifier(Modifier::BOLD));
        if context_focus.is_some() && context_focus != Some(ComponentType::Terminal) {
            terminal_text_area.set_cursor_style(Style::default().fg(Color::Black))
        }
        terminal_text_area.move_cursor(CursorMove::End);
        /*if app.terminal_updated {
            terminal_text_area.scroll(Scrolling::Delta { rows: vec_content.len() as i16 - 2, cols: 0 });
            terminal_text_area.move_cursor(CursorMove::Jump(vec_content.len() as u16 - 2, (vec_content.get(vec_content.len() - 2).unwrap().len() as u16)+1))
        }*/
    
        let standard_block = Block::new()
        .borders(Borders::ALL)
        .border_type(BorderType::Plain);

        let mut blocks: Vec<Block<'_>> = vec![];

        for i in 0..3 {     
    
            let new_block: Option<Block<'_>> = match i {
                0 => {
                    let mut tmp = standard_block.clone().title("Project");
                    if context_focus == Some(ComponentType::Project) {
                        tmp = tmp.style(default_style.clone());
                    } else if context_focus.is_none() && context_hover == ComponentType::Project {
                        tmp = tmp.border_style(default_style.clone());
                    }
                    Some(tmp)
                },
                1 => {
                    let mut tmp = standard_block.clone().title("Code");
                    if context_focus == Some(ComponentType::Code) {
                        tmp = tmp.style(default_style.clone());
                    } else if context_focus.is_none() && context_hover == ComponentType::Code {
                        tmp = tmp.border_style(default_style.clone());
                    }
                    Some(tmp)
                },
                2 => {
                    let mut tmp = standard_block.clone().title("Terminal");
                    if context_focus == Some(ComponentType::Terminal) {
                        tmp = tmp.style(default_style.clone());
                    } else if context_focus.is_none() && context_hover == ComponentType::Terminal {
                        tmp = tmp.border_style(default_style.clone());
                    }
                    Some(tmp)
                },
                _ => {None}
            };
            
            if let Some(new_block) = new_block {
                blocks.push(new_block.clone());
            }
    
        }
        if let Some(block) = blocks.get(2) {
            terminal_text_area.set_block(block.clone());            
        }
    
        frame.render_widget(blocks.get(0).unwrap().clone(), project_area);
        frame.render_widget(blocks.get(1).unwrap().clone(), code_area);
        frame.render_widget(terminal_text_area.widget(), terminal_area);
    
    }
    
    /// Calculate the layout of the UI elements.
    ///
    /// Returns a tuple of the title area and the main areas.
    fn calculate_layout(&self, area: Rect) -> (Rect, Vec<Rect>) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Length(1), Constraint::Min(0)])
            .split(area);
        let title_area = layout[0];
        let main_areas = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Ratio(8, 10),Constraint::Ratio(2, 10)])
        .split(layout[1])
        .to_vec();
        let upper_area = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Ratio(2, 10),Constraint::Ratio(8, 10)])
        .split(main_areas[0])
        .to_vec();
        let project_area = upper_area[0];
        let code_area = upper_area[1];
        let terminal_area = main_areas[1];
        (title_area, vec![project_area,code_area,terminal_area])
    }
    
    fn render_title<B: Backend>(&self, frame: &mut Frame<B>, area: Rect) {
        frame.render_widget(
            Paragraph::new("NOCE")
                .dark_gray()
                .alignment(Alignment::Center),
            area,
        );
    }
}