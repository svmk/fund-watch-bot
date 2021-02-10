use std::path::PathBuf;

pub trait AbsFile {
    fn resolve_abs_path(&self) -> PathBuf;
}