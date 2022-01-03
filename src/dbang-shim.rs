mod dbang_utils;
mod aliases;

use std::process::{Command, Stdio};

pub fn main() {
    let mut alias: String = std::env::args().nth(0).unwrap();
    if alias.contains("/") {
        alias = alias.split("/").last().unwrap().to_string();
    }
    let script_args = std::env::args().skip(1).collect::<Vec<String>>();
    if let Some(script_name) = aliases::find_script_name_by_alias(&alias) {
        dbang_run(&script_name, &script_args).unwrap();
    } else {
        println!("{} not found!", alias);
    }
}

fn dbang_run(script_full_name: &str, args: &[String]) -> anyhow::Result<()> {
    Command::new("dbang")
        .arg("run")
        .arg(script_full_name)
        .args(args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .unwrap();
    Ok(())
}

mod tests {
    #[test]
    fn test_find_script_name() {}
}
