use std::process::{Command, Stdio};

pub fn run(script_name: &str, args: &[&str], permissions: &[String]) {
    Command::new("deno")
        .arg("run")
        .args(permissions)
        .arg(script_name)
        .args(args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("failed to execute Deno");
}
