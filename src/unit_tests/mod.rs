#[cfg(test)]
mod unit_tests {
    use std::env;

    use crate::state::{AppContext, App, ComponentType, project::ProjectComponent, code::{CodeComponent, code::Code}, terminal::TerminalComponent};


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
            CodeComponent::new(Code::new(context.active_file().clone())),
            TerminalComponent::new(),
            env::current_dir().unwrap()
        ))
    }
}