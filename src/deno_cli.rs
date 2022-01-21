use std::process::{Command, Output, Stdio};
use crate::catalog::Artifact;

pub fn run(repo_name: &str, artifact: &Artifact, args: &[&str]) -> anyhow::Result<Output> {
    let mut command = Command::new(artifact.get_deno_bin_path());
    command.arg("run").arg("--no-check");
    if artifact.permissions.is_some() {
        command.args(artifact.get_deno_permissions());
    }
    if artifact.import_map.is_some() {
        command.arg("--import-map");
        command.arg(artifact.get_import_map_http_url(repo_name));
    }
    if let Some(compat) = artifact.compat {
        if compat {
            command.arg("--compat");
        }
    }
    let output = command
        .arg(artifact.get_script_http_url(repo_name))
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
