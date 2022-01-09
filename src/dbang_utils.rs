use std::path::{Path, PathBuf};
use std::process::{Command};

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

pub fn open_url(url: &str) -> anyhow::Result<()> {
    if cfg!(target_os = "macos") {
        Command::new("open")
            .arg(url)
            .spawn()?;
    } else if cfg!(target_os = "windows") {
        Command::new("cmd")
            .arg("/c")
            .arg("start")
            .arg(url)
            .spawn()?;
    } else {
        Command::new("xdg-open")
            .arg(url)
            .spawn()?;
    };
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_github_auth_token() {
        let token = github_auth_token();
        println!("{:?}", token.ok_or("not found"));
    }
}
