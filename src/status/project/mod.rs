use std::path::PathBuf;

#[derive(Default)]
pub struct ProjectStatus {
    contents: Vec<PathBuf>,
    hover: Option<u16>,
    focus: Option<u16>
}