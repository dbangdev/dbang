mod app;
mod deno_cli;
mod deno_version;
mod catalog;

use std::io;
use std::io::Write;
use colored_json::ToColoredJson;
use crate::app::build_app;
// use colored::*;

fn main() {
    let app = build_app();
    let matches = app.get_matches();
    if matches.is_present("artifact") { // run artifact without run sub command
        let artifact_full_name = matches.value_of("artifact").unwrap();
        let mut artifact_args = vec![];
        if let Some(params) = matches.values_of("params") {
            artifact_args = params.collect::<Vec<&str>>()
        }
        dbang_run(artifact_full_name, &artifact_args).unwrap();
        return;
    }
    if matches.subcommand().is_none() { //display help if no subcommand
        build_app().print_help().unwrap();
    }
    let (sub_command, args) = matches.subcommand().unwrap();
    if sub_command == "run" {
        let mut artifact_args = vec![];
        if let Some(params) = args.values_of("params") {
            artifact_args = params.collect::<Vec<&str>>()
        }
        let artifact_full_name = args.value_of("artifact").unwrap();
        dbang_run(artifact_full_name, &artifact_args).unwrap();
    } else if sub_command == "catalog" {

    } else if sub_command == "deno" {
        //dbang_version();
    } else {
        println!("{}", "Unknown subcommand");
    }
}

fn dbang_run(artifact_full_name: &str, artifact_args: &[&str]) -> anyhow::Result<()> {
    let artifact_parts: Vec<&str> = artifact_full_name.split("@").collect();
    let repo_name = artifact_parts[1];
    let artifact_name = artifact_parts[0];
    // validate local catalog exists or not
    if !catalog::Catalog::local_exists(repo_name)? {
        let catalog = catalog::Catalog::fetch_from_github(repo_name)?;
        let catalog_json = serde_json::to_string(&catalog)?;
        println!("Detail of nbang-catalog.json:");
        println!("{}", catalog_json.to_colored_json_auto()?);
        print!("Do you accept above catalog?  y/n > ");
        io::stdout().flush()?;
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer)?;
        if buffer.trim().starts_with("y") {
            catalog::save_nbang_catalog_from_json(repo_name, &catalog_json)?;
            catalog.cache_artifacts(repo_name)?;
        } else {
            println!("Abort to accept nbang catalog!");
            return Ok(());
        }
    }
    let artifact = catalog::Artifact::read_from_local(repo_name, artifact_name).unwrap();
    let script_url = artifact.get_script_http_url(repo_name);
    let permissions: Vec<String> = artifact.get_deno_permissions();
    deno_cli::run(&script_url,
                  artifact_args,
                  &permissions,
    )?;
    Ok(())
}

