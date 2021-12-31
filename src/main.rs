mod app;
mod deno_cli;
mod catalog;

use crate::app::build_app;
use colored::*;

fn main() {
    let app = build_app();
    let matches = app.get_matches();
    if matches.subcommand().is_none() {
        println!(
            "{}",
            "😂 Please use subcommand or --help to display help!".red()
        );
        return;
    }
    let (sub_command, args) = matches.subcommand().unwrap();
    if sub_command == "run" {
        let mut artifact_args = vec![];
        if let Some(params) = args.values_of("params") {
            artifact_args = params.collect::<Vec<&str>>()
        }
        let artifact_full_name = args.value_of("artifact").unwrap();
        let artifact_parts: Vec<&str> = artifact_full_name.split("@").collect();
        let github_user = artifact_parts[1];
        let artifact_name = artifact_parts[0];
        let artifact = catalog::get_artifact(github_user, artifact_name).unwrap();
        let script_url = artifact.get_runnable_script(github_user);
        let mut _permissions = vec![];
        deno_cli::run(&script_url,
                      &artifact_args,
                      &_permissions,
        );
    }
}
