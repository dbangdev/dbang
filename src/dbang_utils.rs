use std::path::{Path, PathBuf};

pub fn dbang_dir() -> PathBuf {
    let home_dir: PathBuf = dirs::home_dir().unwrap();
    Path::new(&home_dir)
        .join(".dbang")
}

pub fn github_auth_token() -> Option<String> {
    if let Ok(tokens) = std::env::var("DENO_AUTH_TOKENS") {
        for pair in tokens.split(";") {
            if pair.contains("@raw.githubusercontent.com") {
                return pair.split("@").nth(0).map(|s| s.to_string());
            }
        }
    }
    None
}

mod tests {
    use super::*;

    #[test]
    fn test_github_auth_token() {
        let token = github_auth_token();
        println!("{:?}", token.ok_or("not found"));
    }
}
