#[cfg(test)]
mod unit_tests {
    use std::{env, path::PathBuf};

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
        let app: App = App::default();
        let vec_contents: Vec<PathBuf> = env::current_dir().unwrap().read_dir().unwrap().into_iter().map(|entry| entry.unwrap().path()).collect();
        assert_eq!(app, App::new(
            ProjectComponent::new(vec_contents),
            CodeComponent::new(Code::new()),
            TerminalComponent::new(),
            env::current_dir().unwrap()
        ))
    }
}