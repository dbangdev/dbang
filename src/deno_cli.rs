use std::process::{Command, Output, Stdio};

pub fn run(deno_bin_path: &str, script_name: &str, args: &[&str], permissions: &[String]) -> anyhow::Result<Output> {
    let output = Command::new(deno_bin_path)
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

pub fn cache(deno_bin_path: &str, script_name: &str) -> anyhow::Result<Output> {
    let output = Command::new(deno_bin_path)
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
