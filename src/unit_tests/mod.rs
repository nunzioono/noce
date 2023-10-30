#[cfg(test)]
mod unit_tests {

    pub mod contructors_tests {
        use crate::state::{AppContext, App, ComponentType, project::ProjectComponent, code::{CodeComponent, code::{Line, Code}, code_utils::Point, code_selection::CodeSelection}, terminal::{TerminalComponent, terminal_history::ExecutedTerminalCommand}};
        use std::{env, path::PathBuf};

        // Define a test for the construction of the Line struct
        #[test]
        fn test_new_line() {
            let line = Line::new(1, "Test Line".to_string());
            assert_eq!(line.get_number(), 1);
            assert_eq!(line.get_string(), "Test Line");
        }

        // Define a test for the construction of the CodeSelection struct
        #[test]
        fn test_new_code_selection() {
            let start = Point::new(0, 0);
            let end = Point::new(1, 1);
            let code_selection = CodeSelection::new(start.clone(), end.clone());
            assert_eq!(code_selection.get_start().clone(), start);
            assert_eq!(code_selection.get_end().clone(), end);
        }

        // Define a test for the construction of the Code struct
        #[test]
        fn test_new_code() {
            let code = Code::new();
            assert_eq!(code.get_content().len(), 0);
            assert_eq!(code.is_cursor_displayed(), false);
            assert_eq!(code.get_cursor().clone(), Point::default());
            assert_eq!(code.get_selection().clone(), None);
        }

        #[test]
        // Ensure the code component is created correctly
        fn test_new_code_component() {
            let code_component = CodeComponent::new();
            assert_eq!(code_component.get_current().get_content().len(), 0);
            assert_eq!(code_component.get_history().get_current_code().get_content().len(), 0);
        }

        // Define a test for the construction of the ProjectComponent struct
        #[test]
        fn test_new_project_component() {
            // Create a new ProjectComponent instance
            let path = tempfile::tempdir().unwrap().into_path();
            let project_component = ProjectComponent::new(path);

            // Assert that the struct fields have the expected initial values
            assert_eq!(project_component.get_contents().len(), 0);
            assert_eq!(project_component.get_hover().clone(), 0);
            assert_eq!(project_component.get_focus().clone(), None);
            assert_eq!(project_component.get_popup(), false);
            assert_eq!(project_component.get_popup_decision(), true);
        }

        // Define a test for the construction of the ExecutedTerminalCommand struct
        #[test]
        fn test_new_executed_terminal_command() {
            // Sample values for the constructor
            let command = "ls -l".to_string();
            let folder = PathBuf::from("/some/directory");
            let output = "File1\nFile2\nFile3".to_string();

            // Create a new ExecutedTerminalCommand instance
            let executed_command = ExecutedTerminalCommand::new(command.clone(), folder.clone(), output.clone());

            // Assert that the struct fields have the expected initial values
            assert_eq!(executed_command.get_command().clone(), command);
            assert_eq!(executed_command.get_folder().clone(), folder);
            assert_eq!(executed_command.get_output().clone(), output);
        }

        #[test]
        pub fn app_context_default() {
            let context = AppContext::new(
                env::current_dir().unwrap(),
                None,
                None,
                ComponentType::Project
            );
            assert_eq!(context, AppContext::default())
        }
    
        #[test]
        pub fn app_state_default() {
            let context = AppContext::default();
            let app: App = App::default();
            assert_eq!(app, App::new(
                ProjectComponent::new(context.active_folder().to_path_buf()),
                CodeComponent::new(),
                TerminalComponent::new(),
                env::current_dir().unwrap()
            ))
        }
    

    }

    pub mod methods_tests {
        
        pub mod code_tests {
            pub mod point_tests {
                use crate::state::code::code_utils::Point;
    
    
                #[test]
                fn test_move_up() {
                    let mut point = Point::new(3, 4);
                    point.move_up(false, 5); // Not exceeding the limit
                    assert_eq!(point.get_x(), 2);
                    assert_eq!(point.get_y(), 4);
    
                    let mut point = Point::new(3, 4);
                    point.move_up(true, 5); // Exceeding the limit
                    assert_eq!(point.get_x(), 3);
                    assert_eq!(point.get_y(), 0);
                }
    
                #[test]
                fn test_move_left() {
                    let mut point = Point::new(3, 4);
                    point.move_left(false, 5); // Not exceeding the limit
                    assert_eq!(point.get_x(), 3);
                    assert_eq!(point.get_y(), 3);
    
                    let mut point = Point::new(1, 0);
                    point.move_left(true, 5); // Exceeding the limit
                    assert_eq!(point.get_x(), 0);
                    assert_eq!(point.get_y(), 4);
                }
    
                #[test]
                fn test_move_right() {
                    let mut point = Point::new(3, 4);
                    point.move_right(false, 5); // Not exceeding the limit
                    assert_eq!(point.get_x(), 3);
                    assert_eq!(point.get_y(), 5);
    
                    let mut point = Point::new(4, 4);
                    point.move_right(true, 4); // Exceeding the limit
                    assert_eq!(point.get_x(), 5);
                    assert_eq!(point.get_y(), 0);
                }
    
                #[test]
                fn test_move_down() {
                    let mut point = Point::new(3, 4);
                    point.move_down(false, 5, 4); // Not exceeding the limit
                    assert_eq!(point.get_x(), 4);
                    assert_eq!(point.get_y(), 4);
    
                    let mut point = Point::new(3, 5);
                    point.move_down(true, 5, 4); // Exceeding the limit
                    assert_eq!(point.get_x(), 3);
                    assert_eq!(point.get_y(), 5);
                }
    
                #[test]
                fn test_set_x_and_set_y() {
                    let mut point = Point::new(3, 4);
                    point.set_x(5);
                    point.set_y(6);
                    assert_eq!(point.get_x(), 5);
                    assert_eq!(point.get_y(), 6);
                }
    
            }
    
            pub mod selection_tests {
                use crate::state::code::{code_selection::CodeSelection, code_utils::Point};
    
    
                #[test]
                fn test_set_start_and_set_end() {
                    let mut selection = CodeSelection::new(Point::new(1, 2), Point::new(3, 4));
            
                    let new_start = Point::new(5, 6);
                    let new_end = Point::new(7, 8);
            
                    selection.set_start(new_start.clone());
                    selection.set_end(new_end.clone());
            
                    assert_eq!(selection.get_start(), &new_start);
                    assert_eq!(selection.get_end(), &new_end);
                }
    
            }
    
            pub mod change_tests {
    
                use crate::state::code::{code_history::Change, code::Line};
    
                #[test]
                fn test_create_change_with_strings() {
                    let number = 42;
                    let from = "Old Text".to_string();
                    let to = "New Text".to_string();
            
                    let change = Change::create_change_with_strings(number, from.clone(), to.clone());
            
                    assert_eq!(change.get_number(), number);
                    assert_eq!(change.get_from(), from);
                    assert_eq!(change.get_to(), to);
                }
            
                #[test]
                fn test_create_change_with_line() {
                    let line = Line::new(42, "Old Text".to_string());
                    let to = "New Text".to_string();
            
                    let change = Change::create_change_with_line(line.clone(), to.clone());
            
                    assert_eq!(change.get_number(), line.get_number());
                    assert_eq!(change.get_from(), line.get_string());
                    assert_eq!(change.get_to(), to);
                }
            
                #[test]
                fn test_get_old_line() {
                    let number = 42;
                    let from = "Old Text".to_string();
                    let to = "New Text".to_string();
            
                    let change = Change::create_change_with_strings(number, from.clone(), to.clone());
                    let old_line = change.get_old_line();
            
                    assert_eq!(old_line.get_number(), number);
                    assert_eq!(old_line.get_string(), from);
                }
            
                #[test]
                fn test_get_new_line() {
                    let number = 42;
                    let from = "Old Text".to_string();
                    let to = "New Text".to_string();
            
                    let change = Change::create_change_with_strings(number, from.clone(), to.clone());
                    let new_line = change.get_new_line();
            
                    assert_eq!(new_line.get_number(), number);
                    assert_eq!(new_line.get_string(), to);
                }
                
    
            }
    
            pub mod history_tests {
                use crate::state::code::{code::Code, code_history::{CodeHistory, Change}};
    
                #[test]
                fn test_new_code_history() {
                    let code = Code::new();
                    let history = CodeHistory::new(code.clone());
    
                    assert_eq!(history.get_current_code(), code);
                }
    
                #[test]
                fn test_new_code_history_with_changes() {
                    let code = Code::new();
                    let changes = vec![Change::create_change_with_strings(1, "Old Text".to_string(), "New Text".to_string())];
                    let history = CodeHistory::new_with_changes(code.clone(), changes.clone());
    
                    assert_eq!(history.get_current_code(), code);
                    assert_eq!(history.get_changes(), changes);
                }
    
                #[test]
                fn test_add_change() {
                    let mut code_history = CodeHistory::new(Code::new());
                    let change = Change::create_change_with_strings(1, "Old Text".to_string(), "New Text".to_string());
                    code_history.add_change(change.clone());
    
                    assert_eq!(code_history.get_changes().len(), 1);
                    assert_eq!(code_history.get_changes()[0], change);
                }
    
                #[test]
                fn test_add_change_at_position() {
                    let mut code_history = CodeHistory::new(Code::new());
                    let change1 = Change::create_change_with_strings(1, "Old Text 1".to_string(), "New Text 1".to_string());
                    let change2 = Change::create_change_with_strings(2, "Old Text 2".to_string(), "New Text 2".to_string());
                    code_history.add_change(change1.clone());
                    code_history.add_change_at_position(0, change2.clone());
    
                    assert_eq!(code_history.get_changes().len(), 2);
                    assert_eq!(code_history.get_changes()[0], change2);
                    assert_eq!(code_history.get_changes()[1], change1);
                }
    
            }
    
    
        }

        pub mod project_tests {
            use std::{path::PathBuf, fs, io};

            use tempfile::TempDir;

            use crate::state::project::{ContentType, ProjectComponent};

            // Helper function to assert the contents of a directory
            fn assert_directory_contents(parent: &PathBuf, expected_contents: Vec<&str>) -> io::Result<()> {
                let actual_contents: io::Result<Vec<_>> = fs::read_dir(parent)?
                    .map(|entry| entry.map(|entry| entry.file_name()))
                    .collect();
                let actual_contents: Vec<_> = actual_contents?
                .into_iter()
                .map(|content| content.to_str().unwrap().to_string())
                .collect();

                let expected_contents: Vec<_> = expected_contents.into_iter().map(|s| s.to_string()).collect();

                assert_eq!(actual_contents, expected_contents);
                Ok(())
            }

            #[test]
            fn test_add_content_file() {
                let temp_dir = TempDir::new().expect("Failed to create temp directory");
                let parent = temp_dir.into_path();

                let mut manager = ProjectComponent::new(parent.clone());
                let file_name = "test_file.txt";
                manager.add_content(&parent, file_name.to_string(), ContentType::FILE);

                assert_directory_contents(&parent, vec![file_name]).unwrap();
            }

            #[test]
            fn test_add_content_folder() {
                let temp_dir = TempDir::new().expect("Failed to create temp directory");
                let parent = temp_dir.into_path();

                let mut manager = ProjectComponent::new(parent.clone());
                let folder_name = "test_folder";
                manager.add_content(&parent, folder_name.to_string(), ContentType::FOLDER);

                assert_directory_contents(&parent, vec![folder_name]).unwrap();
            }
            
        }

        pub mod terminal_tests {
            use std::path::{Path, PathBuf};

            use crate::state::terminal::{terminal_command::TerminalCommand, terminal_history::{ExecutedTerminalCommand, ExecutedTerminalHistory}};

            #[test]
            fn test_default_constructor() {
                let terminal_command = TerminalCommand::default();

                assert_eq!(terminal_command.get_position(), 0);
                assert_eq!(terminal_command.get_buffer(), "");
            }

            #[test]
            fn test_flush_with_default_constructor() {
                let mut terminal_command = TerminalCommand::default();
                terminal_command.flush();

                assert_eq!(terminal_command.get_position(), 0);
                assert_eq!(terminal_command.get_buffer(), "");
            }

            #[test]
            fn test_remove_with_default_constructor() {
                let mut terminal_command = TerminalCommand::default();
                terminal_command.remove();

                assert_eq!(terminal_command.get_position(), 0);
                assert_eq!(terminal_command.get_buffer(), "");
            }

            // Similar tests for other methods...

            #[test]
            fn test_add_with_default_constructor() {
                let mut terminal_command = TerminalCommand::default();
                terminal_command.add('X');

                assert_eq!(terminal_command.get_position(), 0);
                assert_eq!(terminal_command.get_buffer(), "X");
            }

            #[test]
            fn test_move_cursor_forward_with_default_constructor() {
                let mut terminal_command = TerminalCommand::default();
                terminal_command.move_cursor_forward();

                assert_eq!(terminal_command.get_position(), 0);
            }

            #[test]
            fn test_move_cursor_backward_with_default_constructor() {
                let mut terminal_command = TerminalCommand::default();
                terminal_command.move_cursor_backward();

                assert_eq!(terminal_command.get_position(), 0);
            }

            #[test]
            fn test_set_position_with_default_constructor() {
                let mut terminal_command = TerminalCommand::default();
                terminal_command.set_position(3);

                assert_eq!(terminal_command.get_position(), 3);
            }

            #[test]
            fn test_set_buffer_with_default_constructor() {
                let mut terminal_command = TerminalCommand::default();
                terminal_command.set_buffer("new_buffer".to_string());

                assert_eq!(terminal_command.get_position(), 0);
                assert_eq!(terminal_command.get_buffer(), "new_buffer");
            }

            #[test]
            fn test_new() {
                let command = "ls -l".to_string();
                let folder = Path::new("/some/directory").to_path_buf();
                let output = "Listing of files".to_string();
        
                let executed_command = ExecutedTerminalCommand::new(command.clone(), folder.clone(), output.clone());
        
                assert_eq!(executed_command.get_command(), &command);
                assert_eq!(executed_command.get_folder(), &folder);
                assert_eq!(executed_command.get_output(), &output);
            }
        
            #[test]
            fn test_get_command() {
                let command = "echo 'Hello, World!'".to_string();
                let folder = Path::new("/some/directory").to_path_buf();
                let output = "Hello, World!".to_string();
        
                let executed_command = ExecutedTerminalCommand::new(command.clone(), folder.clone(), output.clone());
        
                assert_eq!(executed_command.get_command(), &command);
            }
        
            #[test]
            fn test_get_folder() {
                let command = "pwd".to_string();
                let folder = Path::new("/current/directory").to_path_buf();
                let output = "/current/directory".to_string();
        
                let executed_command = ExecutedTerminalCommand::new(command.clone(), folder.clone(), output.clone());
        
                assert_eq!(executed_command.get_folder(), &folder);
            }
        
            #[test]
            fn test_get_output() {
                let command = "ls".to_string();
                let folder = Path::new("/some/directory").to_path_buf();
                let output = "file1\nfile2\nfile3".to_string();
        
                let executed_command = ExecutedTerminalCommand::new(command.clone(), folder.clone(), output.clone());
        
                assert_eq!(executed_command.get_output(), &output);
            }

            #[test]
            fn test_history_default_constructor() {
                let history = ExecutedTerminalHistory::default();

                assert!(history.get_history().is_empty());
            }

            #[test]
            fn test_up() {
                let mut history = ExecutedTerminalHistory::default();
                let command1 = ExecutedTerminalCommand::new("ls".to_string(), PathBuf::from("/dir1"), "file1 file2".to_string());
                let command2 = ExecutedTerminalCommand::new("cd ..".to_string(), PathBuf::from("/dir2"), "".to_string());

                history.add(command1);
                history.add(command2);

                let result1 = history.up();
                assert_eq!(result1, Some(&ExecutedTerminalCommand::new("ls".to_string(), PathBuf::from("/dir1"), "file1 file2".to_string())));
                let result2 = history.up();
                assert_eq!(result2, None);
                let result3 = history.up();
                assert_eq!(result3, None);
            }

            #[test]
            fn test_down() {
                let mut history = ExecutedTerminalHistory::default();
                let command1 = ExecutedTerminalCommand::new("ls".to_string(), PathBuf::from("/dir1"), "file1 file2".to_string());
                let command2 = ExecutedTerminalCommand::new("cd ..".to_string(), PathBuf::from("/dir2"), "".to_string());

                history.add(command1);
                history.add(command2);

                history.up(); // Move to the previous command
                let result1 = history.down();
                assert_eq!(result1, Some(&ExecutedTerminalCommand::new("cd ..".to_string(), PathBuf::from("/dir2"), "".to_string())));
                let _result2 = history.up();
                let _result3 = history.up();
                let result4 = history.down();
                assert_eq!(result4, Some(&ExecutedTerminalCommand::new("ls".to_string(), PathBuf::from("/dir1"), "file1 file2".to_string())));
            }

            #[test]
            fn test_add() {
                let mut history = ExecutedTerminalHistory::default();
                let command = ExecutedTerminalCommand::new("ls".to_string(), PathBuf::from("/dir1"), "file1 file2".to_string());

                history.add(command);

                assert_eq!(history.get_history().len(), 1);
            }

            #[test]
            fn test_flush() {
                let mut history = ExecutedTerminalHistory::default();
                let command = ExecutedTerminalCommand::new("ls".to_string(), PathBuf::from("/dir1"), "file1 file2".to_string());

                history.add(command);
                history.flush();

                assert!(history.get_history().is_empty());
            }

        }

    }


    pub mod event_related_tests {

        pub mod context_tests {
            use crossterm::event::{KeyEvent, KeyCode, KeyModifiers, Event};
            use crate::state::{AppContext, App, project::ProjectComponent, code::CodeComponent, terminal::TerminalComponent, ComponentType};


            #[test]
            pub fn hover_test() {
                let mut context = AppContext::default();
                let mut app = App::new(
                    ProjectComponent::new(context.active_folder().to_path_buf().clone()),
                    CodeComponent::new(),
                    TerminalComponent::new(),
                    context.active_folder().clone());
                
                let fake_event = Event::Key(KeyEvent::new(KeyCode::Tab, KeyModifiers::empty()));

                assert_eq!(context.hover().clone(),ComponentType::Project);
                app.handle_event(&mut context, None, fake_event.clone());
                assert_eq!(context.hover().clone(),ComponentType::Code);
                app.handle_event(&mut context, None, fake_event.clone());
                assert_eq!(context.hover().clone(),ComponentType::Terminal);
                app.handle_event(&mut context, None, fake_event);
                assert_eq!(context.hover().clone(),ComponentType::Project);
            }
        
            #[test]
            pub fn focus_test() {
                let mut context = AppContext::default();
                let mut app = App::new(
                    ProjectComponent::new(context.active_folder().to_path_buf().clone()),
                    CodeComponent::new(),
                    TerminalComponent::new(),
                    context.active_folder().clone());
                
                let fake_tab_event = Event::Key(KeyEvent::new(KeyCode::Tab, KeyModifiers::empty()));
                let fake_enter_event = Event::Key(KeyEvent::new(KeyCode::Enter, KeyModifiers::empty()));
                let fake_esc_event = Event::Key(KeyEvent::new(KeyCode::Esc, KeyModifiers::empty()));

                assert_eq!(context.focus().clone(),None);
                app.handle_event(&mut context, None, fake_enter_event.clone());
                assert_eq!(context.focus().clone(),Some(ComponentType::Project));
                app.handle_event(&mut context, Some(ComponentType::Project), fake_esc_event.clone());
                app.handle_event(&mut context, None, fake_tab_event.clone());
                app.handle_event(&mut context, None, fake_enter_event.clone());
                assert_eq!(context.focus().clone(),Some(ComponentType::Code));
                app.handle_event(&mut context, Some(ComponentType::Code), fake_esc_event.clone());
                app.handle_event(&mut context, None, fake_tab_event.clone());
                app.handle_event(&mut context, None, fake_enter_event.clone());
                assert_eq!(context.focus().clone(),Some(ComponentType::Terminal));
                app.handle_event(&mut context, Some(ComponentType::Terminal), fake_esc_event.clone());
                app.handle_event(&mut context, None, fake_tab_event.clone());
                app.handle_event(&mut context, None, fake_enter_event.clone());
                assert_eq!(context.focus().clone(),Some(ComponentType::Project));

            }
    

        }

        pub mod project_events_tests {
            use crossterm::event::{KeyEvent, KeyCode, KeyModifiers, Event};
            use crate::state::{AppContext, App, project::ProjectComponent, code::CodeComponent, terminal::TerminalComponent, ComponentType};
            use std::{fs::File, path::Path};
            use std::io::Write;
            use tempfile::TempDir;

            #[test]
            pub fn hover_entry_test() {
                let mut context = AppContext::default();
                let mut app = App::new(
                    ProjectComponent::new(context.active_folder().to_path_buf().clone()),
                    CodeComponent::new(),
                    TerminalComponent::new(),
                    context.active_folder().clone());
                let fake_enter_event = Event::Key(KeyEvent::new(KeyCode::Enter, KeyModifiers::empty()));

                app.handle_event(&mut context, None, fake_enter_event.clone());
                let current_contents = app.get_project().get_contents().clone();
                assert_eq!(current_contents.get(0).unwrap(),app.get_project().get_contents().get(app.get_project().get_hover().clone()).unwrap());
            }
    


            pub fn scroll_to(context: &mut AppContext, app: &mut App, prefix: &str) {
                let contents = app.get_project().get_contents().clone();
                let contents_with_prefix: Vec<usize> = contents.clone().into_iter().enumerate().filter(|(_i,content)| {
                    content.clone().as_path().file_name().unwrap().to_str().unwrap().contains(prefix)
                }).map(|(i,_entry)| i).collect();
                if contents_with_prefix.len() >= 1 {
                    let index_to = contents_with_prefix.get(0).unwrap().clone();
                    if index_to > 0 {
                        let fake_down_event = Event::Key(KeyEvent::new(KeyCode::Down, KeyModifiers::empty()));
                        for _ in 0..index_to {
                            app.handle_event(context, Some(ComponentType::Project), fake_down_event.clone());
                        }        
                    }
                }
            }

            #[test]
            pub fn focus_entry_test() {
                let tmp_dir = TempDir::with_prefix_in("tmp-", ".").ok().unwrap();
                let tmp_file_path = tmp_dir.path().join(Path::new("tmp_file.txt"));
                let mut tmp_file = File::create(tmp_file_path).ok().unwrap();
                let _ = writeln!(tmp_file," temporary file!");


                let mut context = AppContext::default();
                let mut app = App::new(
                    ProjectComponent::new(context.active_folder().to_path_buf().clone()),
                    CodeComponent::new(),
                    TerminalComponent::new(),
                    context.active_folder().clone());
                let fake_enter_event = Event::Key(KeyEvent::new(KeyCode::Enter, KeyModifiers::empty()));

                app.handle_event(&mut context, None, fake_enter_event.clone());
                scroll_to(&mut context, &mut app, "tmp-");
                app.handle_event(&mut context, Some(ComponentType::Project), fake_enter_event.clone());
                scroll_to(&mut context, &mut app, "tmp_file.txt");
                app.handle_event(&mut context, Some(ComponentType::Project), fake_enter_event.clone());
                assert_eq!(app.get_project().get_contents().get(0).unwrap(),app.get_project().get_contents().get(app.get_project().get_focus().unwrap().clone()).unwrap())

            }

        }

        pub mod code_events_tests {
            use std::env;
            use std::{fs::File, path::Path};
            use std::io::Write;
            use crossterm::event::{KeyEvent, KeyModifiers, Event, KeyCode};
            use tempfile::TempDir;

            use crate::state::code::code_utils::Point;
            use crate::state::{AppContext, App, project::ProjectComponent, code::CodeComponent, terminal::TerminalComponent, ComponentType};

            pub fn setup_test(prefix: &str) -> (AppContext,App,TempDir) {
                let tmp_dir = TempDir::with_prefix_in(prefix, ".").ok().unwrap();
                let tmp_file_path = tmp_dir.path().join(Path::new("tmp_file.txt"));
                let mut tmp_file = File::create(tmp_file_path).ok().unwrap();
                let _ = writeln!(tmp_file," temporary file!");

                let context = AppContext::default();
                let app = App::new(
                    ProjectComponent::new(context.active_folder().to_path_buf().clone()),
                    CodeComponent::new(),
                    TerminalComponent::new(),
                    context.active_folder().clone());
                (context,app,tmp_dir)
            }

            pub fn scroll_to(context: &mut AppContext, app: &mut App, prefix: &str) {
                let contents = app.get_project().get_contents().clone();
                let contents_with_prefix: Vec<usize> = contents.clone().into_iter().enumerate().filter(|(_i,content)| {
                    content.clone().as_path().file_name().unwrap().to_str().unwrap().contains(prefix)
                }).map(|(i,_entry)| i).collect();
                if contents_with_prefix.len() >= 1 {
                    let index_to = contents_with_prefix.get(0).unwrap().clone();
                    if index_to > 0 {
                        let fake_down_event = Event::Key(KeyEvent::new(KeyCode::Down, KeyModifiers::empty()));
                        for _ in 0..index_to {
                            app.handle_event(context, Some(ComponentType::Project), fake_down_event.clone());
                        }        
                    }
                }
            }

            #[test]
            pub fn chars_events_test() {
                let (mut context, mut app, _dir) = setup_test("tmp-chars-");

                let fake_enter_event = Event::Key(KeyEvent::new(KeyCode::Enter, KeyModifiers::empty()));
                let fake_tab_event = Event::Key(KeyEvent::new(KeyCode::Tab, KeyModifiers::empty()));
                let fake_esc_event = Event::Key(KeyEvent::new(KeyCode::Esc, KeyModifiers::empty()));
                let fake_char_event = Event::Key(KeyEvent::new(KeyCode::Char('a'), KeyModifiers::empty()));

                app.handle_event(&mut context, None, fake_enter_event.clone());
                scroll_to(&mut context, &mut app, "tmp-chars-");
                app.handle_event(&mut context, Some(ComponentType::Project), fake_enter_event.clone());
                assert!(context.active_folder().clone().as_path().file_name().unwrap().to_str().unwrap().contains("tmp-chars-"));
                scroll_to(&mut context, &mut app, "tmp_file.txt");
                app.handle_event(&mut context, Some(ComponentType::Project), fake_enter_event.clone());
                assert_eq!(context.active_file().clone().unwrap().file_name().unwrap().to_str().unwrap(),"tmp_file.txt");
                //set the new file if the active file changed
                if context.active_file_changed() {
                    app.get_mut_code().get_mut_current().flush();
                    app.get_mut_code().set_current(context.active_file().clone());
                    context.set_active_file_changed(false);
                }
                app.handle_event(&mut context, Some(ComponentType::Project), fake_esc_event.clone());
                app.handle_event(&mut context, None, fake_tab_event.clone());
                app.handle_event(&mut context, None, fake_enter_event.clone());
                app.handle_event(&mut context, Some(ComponentType::Code), fake_char_event.clone());
                app.get_mut_code().get_mut_current().remove_cursor();
                let code_content = app.get_code().get_current().get_content().get(0).unwrap().get_string();
                assert_eq!(code_content.as_str(),"a temporary file!");
            }

            #[test]
            pub fn arrows_events_test() {
                let (mut context, mut app, _dir) = setup_test("tmp-arrows-");

                let fake_enter_event = Event::Key(KeyEvent::new(KeyCode::Enter, KeyModifiers::empty()));
                let fake_tab_event = Event::Key(KeyEvent::new(KeyCode::Tab, KeyModifiers::empty()));
                let fake_esc_event = Event::Key(KeyEvent::new(KeyCode::Esc, KeyModifiers::empty()));
                let fake_down_event = Event::Key(KeyEvent::new(KeyCode::Down, KeyModifiers::empty()));
                let fake_up_event = Event::Key(KeyEvent::new(KeyCode::Up, KeyModifiers::empty()));
                let fake_left_event = Event::Key(KeyEvent::new(KeyCode::Left, KeyModifiers::empty()));
                let fake_right_event = Event::Key(KeyEvent::new(KeyCode::Right, KeyModifiers::empty()));

                app.handle_event(&mut context, None, fake_enter_event.clone());
                scroll_to(&mut context, &mut app, "tmp-arrows-");
                app.handle_event(&mut context, Some(ComponentType::Project), fake_enter_event.clone());
                assert!(context.active_folder().clone().as_path().file_name().unwrap().to_str().unwrap().contains("tmp-arrows-"));
                scroll_to(&mut context, &mut app, "tmp_file.txt");
                app.handle_event(&mut context, Some(ComponentType::Project), fake_enter_event.clone());
                assert_eq!(context.active_file().clone().unwrap().file_name().unwrap().to_str().unwrap(),"tmp_file.txt");
                //set the new file if the active file changed
                if context.active_file_changed() {
                    app.get_mut_code().get_mut_current().flush();
                    app.get_mut_code().set_current(context.active_file().clone());
                    context.set_active_file_changed(false);
                }
                app.handle_event(&mut context, Some(ComponentType::Project), fake_esc_event.clone());
                app.handle_event(&mut context, None, fake_tab_event.clone());
                app.handle_event(&mut context, None, fake_enter_event.clone());
                assert_eq!(app.get_code().get_current().get_cursor().clone(),Point::new(0, 0));                

                //test on up event with and without exceed
                app.handle_event(&mut context, Some(ComponentType::Code), fake_up_event.clone());
                assert_eq!(app.get_code().get_current().get_cursor().clone(),Point::new(0, 0));
                app.handle_event(&mut context, Some(ComponentType::Code), fake_down_event.clone());
                assert_eq!(app.get_code().get_current().get_cursor().clone(),Point::new(1, 0));
                app.handle_event(&mut context, Some(ComponentType::Code), fake_up_event.clone());
                assert_eq!(app.get_code().get_current().get_cursor().clone(),Point::new(0, 0));

                //test on down event with and without exceed
                app.handle_event(&mut context, Some(ComponentType::Code), fake_down_event.clone());
                assert_eq!(app.get_code().get_current().get_cursor().clone(),Point::new(1, 0));
                app.handle_event(&mut context, Some(ComponentType::Code), fake_down_event.clone());
                assert_eq!(app.get_code().get_current().get_cursor().clone(),Point::new(1, 0));

                app.handle_event(&mut context, Some(ComponentType::Code), fake_up_event.clone());
                assert_eq!(app.get_code().get_current().get_cursor().clone(),Point::new(0, 0));

                //test on right event with and without exceed
                let first_line_len = app.get_code().get_current().get_content().get(0).unwrap().get_string().len();
                app.handle_event(&mut context, Some(ComponentType::Code), fake_right_event.clone());
                assert_eq!(app.get_code().get_current().get_cursor().clone(),Point::new(0, 1));
                for _ in 0..first_line_len-2 {
                    let current_y = app.get_code().get_current().get_cursor().clone().get_y();
                    app.handle_event(&mut context, Some(ComponentType::Code), fake_right_event.clone());
                    assert_eq!(app.get_code().get_current().get_cursor().clone(),Point::new(0, current_y+1));        
                }
                app.handle_event(&mut context, Some(ComponentType::Code), fake_right_event.clone());
                assert_eq!(app.get_code().get_current().get_cursor().clone(),Point::new(1, 0));

                //test on left event with and without exceed
                app.handle_event(&mut context, Some(ComponentType::Code), fake_left_event.clone());
                assert_eq!(app.get_code().get_current().get_cursor().clone(),Point::new(0, first_line_len-2));
                let mut current_y = app.get_code().get_current().get_cursor().clone().get_y();
                for _ in 0..first_line_len-2 {
                    app.handle_event(&mut context, Some(ComponentType::Code), fake_left_event.clone());
                    assert_eq!(app.get_code().get_current().get_cursor().clone(),Point::new(0, current_y-1));
                    current_y-=1;        
                }
                app.handle_event(&mut context, Some(ComponentType::Code), fake_left_event.clone());
                assert_eq!(app.get_code().get_current().get_cursor().clone(),Point::new(0, 0));
            }

            #[test]
            pub fn selection_events_test() {

            }

            #[test]
            pub fn modifiers_events_test() {

            }

            #[test]
            pub fn enter_events_test() {

            }

            #[test]
            pub fn delete_events_test() {

            }

        }

    }

}