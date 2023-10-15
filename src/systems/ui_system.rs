use std::{error::Error, path::MAIN_SEPARATOR};

use ratatui::{Terminal, prelude::{Backend, Rect, Alignment, Layout, Direction, Constraint}, Frame, style::{Style, Modifier, Color, Stylize}, widgets::{Paragraph, Block, BorderType, Borders, ListItem, List, ListState, Clear}};
use ratatui_textarea::{CursorMove, TextArea};

use crate::state::{App, AppContext, ComponentType};

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
    pub fn start<B: Backend>(&self, terminal: &mut Terminal<B>,
        app: &App,
        context: &AppContext
    ) -> Result<(),Box<dyn Error>> {
        terminal.draw(|f| self.ui(f, app, context))?;
        Ok(())
    }

    fn ui<B: Backend>(&self, f: &mut Frame<B>, app: &App, context: &AppContext) {
        let (title_area, main_area) = self.calculate_layout(f.size());
        let project_area = main_area[0];
        let code_area = main_area[1];
        let terminal_area = main_area[2];
        
        self.render_title(context, f, title_area);
        self.render_editor(app, context, f, project_area, code_area, terminal_area);
    }
    
    fn render_editor<B: Backend>(&self, app: &App, context: &AppContext, frame: &mut Frame<B>, project_area:Rect, code_area:Rect, terminal_area:Rect) {
        let mut default_style: Style = Style::new();
        default_style = default_style.blue().on_white().bold().italic();
        let history: Vec<String>;
        let context_focus: Option<ComponentType> = context.focus().clone();
        let context_hover: ComponentType = context.hover().clone(); 

        history = (app.get_terminal().get_history()).to_string().split('\n').into_iter().map(|el| el.to_string()).collect();
    
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
                    if context_focus == Some(ComponentType::Project) && !app.get_project().get_popup() {
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

        // Iterate through all elements in the `items` app and append some debug text to it.
        let items: Vec<ListItem> = app.get_project().get_contents()
        .into_iter()
        .map(|path| {
                let path = path;
                let mut name = path.file_name().unwrap().to_str().unwrap().to_string();
                if path.is_dir() {
                    name = name + MAIN_SEPARATOR.to_string().as_str();                        
                }
                ListItem::new(name)                
            }
        )
        .collect();

        // Create a List from all list items and highlight the currently selected one
        let list: List = List::new(items)
        .block(blocks.get(0).unwrap().clone())
        .highlight_style(
            Style::default().white().on_blue().bold()
        );



        let offset = app.get_project().get_hover();
        let mut list_state = ListState::default();
        if context_focus == Some(ComponentType::Project) {
            list_state = list_state
            .with_offset(0)
            .with_selected(Some(offset.clone()));
        }



        // We can now render the item list
        frame.render_stateful_widget(list, project_area, &mut list_state);
        frame.render_widget(blocks.get(1).unwrap().clone(), code_area);
        frame.render_widget(terminal_text_area.widget(), terminal_area);
        self.render_popup(frame, app);
    
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
    
    fn render_title<B: Backend>(&self, context: &AppContext, frame: &mut Frame<B>, area: Rect) {
        let mut title = "NOCE".to_string();
        let active_file = context.active_file();
        if let Some(active) = active_file {
            if let Some(name) = active.file_name() {
                title = title + " - " + name.to_str().unwrap();
            }
        }
        frame.render_widget(
            Paragraph::new(title.as_str())
                .dark_gray()
                .alignment(Alignment::Center),
            area,
        );
    }

    fn render_popup<B: Backend>(&self, f: &mut Frame<B>, app: &App) {
        let size = f.size();
        let popup_size = self.layout_center(80, 40, size);
        let popup_content = self.layout_center(99, 90, popup_size);
        let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Ratio(8, 10), Constraint::Ratio(2, 10)])
        .split(popup_content)
        .to_vec();
        let buttons_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Ratio(1,10),Constraint::Ratio(4,10),Constraint::Ratio(4,10),Constraint::Ratio(1,10),])
        .split(popup_layout.get(1).unwrap().clone())
        .to_vec();

        if app.get_project().get_popup() {
            let block = Block::default().title("Cancel").title_alignment(Alignment::Center).style(Style::new().blue().on_white().bold().italic()).borders(Borders::ALL);
            let paragraph = Paragraph::new("Are you sure you want to cancel this?\n(The action is not revertable)").alignment(Alignment::Center);
            let selected_button_style = Style::new().white().on_blue().bold().italic();
            let mut button1 = Paragraph::new("Ok").alignment(Alignment::Center);
            let mut button2 = Paragraph::new("Go back").alignment(Alignment::Center);

            if app.get_project().get_popup_decision() {
                button1 = button1.style(selected_button_style);   
            } else {
                button2 = button2.style(selected_button_style);
            }

            f.render_widget(Clear, popup_size); //this clears out the background
            f.render_widget(block, popup_size);
            f.render_widget(paragraph, popup_layout.get(0).unwrap().clone());
            f.render_widget(button1, buttons_layout.get(1).unwrap().clone());
            f.render_widget(button2, buttons_layout.get(2).unwrap().clone());
        }
    }
    
    /// helper function to create a centered rect using up certain percentage of the available rect `r`
    fn layout_center(&self, percent_x: u16, percent_y: u16, r: Rect) -> Rect {
        let popup_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ])
            .split(r);
    
        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ])
            .split(popup_layout[1])[1]
    }
}