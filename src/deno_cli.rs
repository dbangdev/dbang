use std::process::{Command, Output, Stdio};

pub fn run(script_name: &str, args: &[&str], permissions: &[String]) -> anyhow::Result<Output> {
    let output = Command::new("deno")
        .arg("run")
        .arg("--no-check")
        .args(permissions)
        .arg(script_name)
        .args(args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()?;
    Ok(output)
}

pub fn cache(script_name: &str) -> anyhow::Result<Output> {
    let output = Command::new("deno")
        .arg("cache")
        .arg("--no-check")
        .arg("--unstable")
        .arg("--reload")
        .arg("--quiet")
        .arg(script_name)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()?;
    Ok(output)
}
