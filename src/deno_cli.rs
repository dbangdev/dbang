use std::path::Path;
use std::process::{Command, Output, Stdio};
use crate::catalog::Artifact;

pub fn run(repo_name: &str, artifact: &Artifact, args: &[&str], verbose: bool) -> anyhow::Result<Output> {
    let mut command = Command::new(artifact.get_deno_bin_path());
    command.arg("run").arg("--no-check").arg("--cached-only");
    if let Some(unstable) = artifact.unstable {
        if unstable {
            command.arg("--unstable");
        }
    }
    if let Some(permissions) = &artifact.permissions {
        command.args(permissions);
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
    command.arg("--config");
    command.arg(artifact.get_deno_config(repo_name));
    command.arg(artifact.get_script_http_url(repo_name));
    command.args(args);
    if verbose {
        println!("[dbang] command line:  {:?}", command);
    }
    let output = command
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()?;
    Ok(output)
}

pub fn run_local(working_dir: &Path, artifact: &Artifact, args: &[&str], verbose: bool) -> anyhow::Result<Output> {
    let mut command = Command::new(artifact.get_deno_bin_path());
    command.arg("run").arg("--no-check");
    if let Some(unstable) = artifact.unstable {
        if unstable {
            command.arg("--unstable");
        }
    }
    if artifact.permissions.is_some() {
        command.args(artifact.get_deno_permissions());
    }
    if artifact.import_map.is_some() {
        command.arg("--import-map");
        command.arg(artifact.import_map.as_ref().unwrap());
    }
    if let Some(compat) = artifact.compat {
        if compat {
            command.arg("--compat");
        }
    }
    command.arg(&artifact.script_ref);
    command.args(args);
    if verbose {
        println!("[dbang] command line:  {:?}", command);
    }
    let output = command
        .current_dir(working_dir)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()?;
    Ok(output)
}

pub fn cache(deno_bin_path: &str, script_name: &str, import_map: &Option<String>) -> anyhow::Result<Output> {
    let mut command = Command::new(deno_bin_path);
    command.arg("cache")
        .arg("--no-check")
        .arg("--unstable")
        .arg("--reload")
        .arg("--quiet");
    if let Some(import_map) = import_map {
        command.arg("--import-map");
        command.arg(import_map);
    }
    let output = command
        .arg(script_name)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()?;
    Ok(output)
}
