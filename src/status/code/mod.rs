mod code;

use std::path::PathBuf;

use self::code::{Code, CodeHistory};

#[derive(Default)]
pub struct CodeStatus {
    file: PathBuf,
    current: Code,
    history: CodeHistory,
    selection: Option<Code>,
}