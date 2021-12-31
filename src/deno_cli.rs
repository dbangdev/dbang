use std::process::Command;

pub fn run(script_name: &str, args: &[&str], permissions: &[&str]) {
    Command::new("deno")
        .arg("run")
        .args(permissions)
        .arg(script_name)
        .args(args)
        .spawn()
        .expect("failed to execute Deno");
}
