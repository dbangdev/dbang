mod app;
mod deno_cli;
mod catalog;

use crate::app::build_app;
use colored::*;
use std::env;

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
        let artifact_name = args.value_of("artifact").unwrap();
        println!("{}", artifact_name);
        let mut artifact_args = vec![];
        if let Some(params) = args.values_of("params") {
            artifact_args = params.collect::<Vec<&str>>()
        }
        let mut permissions = vec![];
        deno_cli::run("https://raw.githubusercontent.com/linux-china/dbang-catalog/main/hello.ts",
                      &artifact_args,
                      &permissions,
        );
    }
}
