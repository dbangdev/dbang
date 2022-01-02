use std::path::{Path, PathBuf};

pub fn dbang_dir() -> PathBuf {
    let home_dir: PathBuf = dirs::home_dir().unwrap();
    Path::new(&home_dir)
        .join(".dbang")
}
