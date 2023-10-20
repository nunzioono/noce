use std::{error::Error, path::MAIN_SEPARATOR};

use ratatui::{Terminal, prelude::{Backend, Rect, Alignment, Layout, Direction, Constraint}, Frame, style::{Style, Stylize, Styled}, widgets::{Paragraph, Block, BorderType, Borders, ListItem, List, ListState, Clear}, text::{Line, Span}};

use crate::state::{App, AppContext, ComponentType, code::code_selection::Point};

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
    pub fn tick<B: Backend>(&self, terminal: &mut Terminal<B>,
        app: &App,
        context: &AppContext,
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
        self.render_project(app, context, f, project_area);
        self.render_code(app, context, f, code_area);
        self.render_terminal(app, context, f, terminal_area);
        self.render_popup(f, app);
    }
    
    fn render_project<B: Backend>(&self, app: &App, context: &AppContext, frame: &mut Frame<B>, project_area:Rect) {
        let context_focus: Option<ComponentType> = context.focus().clone();
        let context_hover: ComponentType = context.hover().clone();

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

        let mut block = Block::new()
        .title("Project")
        .borders(Borders::ALL)
        .border_type(BorderType::Plain);

        let style = Style::new().blue().on_white().bold().italic();
        if context_focus == Some(ComponentType::Project) && !app.get_project().get_popup() {
            block = block.style(style);
        } else if context_focus.is_none() && context_hover == ComponentType::Project {
            block = block.border_style(style);
        }

        // Create a List from all list items and highlight the currently selected one
        let list: List = List::new(items)
        .block(block)
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
        
    }

    fn render_code<B: Backend>(&self, app: &App, context: &AppContext, frame: &mut Frame<B>, code_area: Rect) {
        //Prepare data to conditionally render different variants of the same ui
        let context_focus: Option<ComponentType> = context.focus().clone();
        let context_hover: ComponentType = context.hover().clone();
        let area = self.layout_center(90, 94, code_area);
        let layout_code = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Ratio(1, 10),Constraint::Ratio(9, 10)])
        .split(area)
        .to_vec();
    
        let mut block = Block::new()
        .title("Code")
        .borders(Borders::ALL)
        .border_type(BorderType::Plain);

        let style = Style::new().blue().on_white().bold().italic();
        if context_focus == Some(ComponentType::Code){
            block = block.style(style);
        } else if context_focus.is_none() && context_hover == ComponentType::Code {
            block = block.border_style(style);
        }

        let selection_style = Style::new().white().on_blue().bold().italic();

        let code = app
        .get_code()
        .get_current()
        .get_content();

        //Retrieve if something is selected the starting and ending point of the selection
        let selection = app.get_code().get_selection();
        let mut start_point: Point = Point::new(0,0);
        let mut end_point: Point = Point::new(0,0);
        if let Some(selection) = selection {
            start_point = selection.get_start().clone();
            end_point = selection.get_end().clone();
        }

        //Get the text style according to his status (selected or not)
        let lines: Vec<ListItem> = code
        .into_iter()
        .map(move |line| {
            let mut vec: Vec<Span<'_>> = vec![];
            // the row belongs to the start of the selection it needs to be splitted in non-selected text and selected text
            if line.get_number() == start_point.get_x() {
                let string;
                if start_point.get_y() > end_point.get_y() {
                    string = line.get_string()[..end_point.get_y()].to_string();                
                } else {
                    string = line.get_string()[..start_point.get_y()].to_string();
                }
                string.set_style(style);
                if !string.is_empty() {
                    vec.push(string.into());
                }

                //if the selection is on multiple lines the selection on the first line should be calculated here
                if start_point.get_x() != end_point.get_x() {
                    let first = line.get_string()[start_point.get_y()..].to_string();
                    vec.push(Span::styled(first.clone(),selection_style));
                }
            }
            //if the row belongs to the last line of the selection needs to be splitted in selected text and non-selected text
            //note that selection can start in a line and end on the same line so it needs to not push the selection 2 times
            if line.get_number() == end_point.get_x() {
                let string;
                if start_point.get_y() > end_point.get_y() {
                    string = line.get_string()[start_point.get_y()..].to_string();                
                } else {
                    string = line.get_string()[end_point.get_y()..].to_string();
                }
                string.set_style(style);
                let last: String;
                if start_point.get_x() == end_point.get_x() {
                    if start_point.get_y() > end_point.get_y() {
                        //since the slice of a string includes the start and excludes the end to exclude the cursor from the selection
                        //right to left it's needed a +1 on the starting point
                        last = line.get_string()[end_point.get_y()..start_point.get_y()+1].to_string();
                    } else {
                        last = line.get_string()[start_point.get_y()..end_point.get_y()].to_string();
                    }
                } else {
                    last = line.get_string()[..end_point.get_y()].to_string();
                }

                if !last.is_empty() {
                    vec.push(Span::styled(last.clone(),selection_style));
                }

                if !string.is_empty() {
                    vec.push(string.into());
                }
            }
            
            //the if the line is selected as whole must be pushed as styled and if not as non styled
            if line.get_number() > start_point.get_x() && line.get_number() < end_point.get_x() {
                vec.push(Span::styled(line.get_string(),selection_style));
            } else if line.get_number() < start_point.get_x() || line.get_number() > end_point.get_x() {
                let string = line.get_string();
                string.set_style(style);
                vec.push(string.into());
            }

            let ratatui_line = Line::from(vec);
            let item = ListItem::new(ratatui_line);
            item
        })
        .collect();

        let numbers: Vec<ListItem>= code
        .into_iter()
        .map(|line| {
            ListItem::new((line.get_number()+1).to_string())
        })
        .collect();


        let list_lines = List::new(lines);

        let list_numbers = List::new(numbers);

        frame.render_widget(block, code_area);
        frame.render_widget(list_numbers, layout_code.get(0).unwrap().clone());
        frame.render_widget(list_lines, layout_code.get(1).unwrap().clone());




    }

    fn render_terminal<B: Backend>(&self,_app: &App, context: &AppContext, frame: &mut Frame<B>, terminal_area: Rect) {
        let context_focus: Option<ComponentType> = context.focus().clone();
        let context_hover: ComponentType = context.hover().clone();

        let mut block = Block::new()
        .title("Terminal")
        .borders(Borders::ALL)
        .border_type(BorderType::Plain);

        let style = Style::new().blue().on_white().bold().italic();
        if context_focus == Some(ComponentType::Terminal){
            block = block.style(style);
        } else if context_focus.is_none() && context_hover == ComponentType::Terminal {
            block = block.border_style(style);
        }

        frame.render_widget(block, terminal_area);
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