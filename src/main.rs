mod app;
mod deno_cli;
mod deno_versions;
mod catalog;
mod known_catalogs;
mod dbang_utils;

use std::io;
use std::io::Write;
use colored_json::ToColoredJson;
use crate::app::build_app;
// use colored::*;

fn main() {
    let app = build_app();
    let matches = app.get_matches();
    // run artifact without 'run' sub command
    if matches.is_present("script") {
        let artifact_full_name = matches.value_of("script").unwrap();
        let mut artifact_args = vec![];
        if let Some(params) = matches.values_of("params") {
            artifact_args = params.collect::<Vec<&str>>()
        }
        dbang_run(artifact_full_name, &artifact_args).unwrap();
        return;
    }
    if matches.subcommand().is_none() { //display help if no subcommand
        build_app().print_help().unwrap();
        return;
    }
    // make sure DBANG_DIR ~/.dbang exist
    let dbang_dir = dbang_utils::dbang_dir();
    if !dbang_dir.exists() {
        std::fs::create_dir(&dbang_dir).unwrap();
    }
    // parse subcommand and run
    let (sub_command, sub_command_args) = matches.subcommand().unwrap();
    if sub_command == "run" {
        let mut artifact_args = vec![];
        if let Some(params) = sub_command_args.values_of("params") {
            artifact_args = params.collect::<Vec<&str>>()
        }
        let artifact_full_name = sub_command_args.value_of("script").unwrap();
        dbang_run(artifact_full_name, &artifact_args).unwrap();
    } else if sub_command == "catalog" {
        if sub_command_args.subcommand().is_none() { // print help if no subcommand
            build_app().find_subcommand("catalog").unwrap().clone().print_help().unwrap();
            return;
        }
        let (catalog_sub_command, catalog_sub_command_args) = sub_command_args.subcommand().unwrap();
        if catalog_sub_command == "list" {
            for catalog_full_name in catalog::Catalog::list_local().unwrap() {
                println!("{}", catalog_full_name);
            };
        } else if catalog_sub_command == "add" || catalog_sub_command == "update" {
            let repo_name = catalog_sub_command_args.value_of("repo_name").unwrap();
            if confirm_remote_catalog(repo_name).unwrap() {
                println!("Catalog added successfully!");
            } else {
                println!("Abort to accept nbang catalog!");
            }
        } else if catalog_sub_command == "delete" {
            let repo_name = catalog_sub_command_args.value_of("repo_name").unwrap();
            catalog::Catalog::delete(repo_name).unwrap();
            println!("Catalog deleted successfully!");
        } else {
            println!("{}", "Unknown subcommand");
        }
    } else if sub_command == "deno" {
        if sub_command_args.subcommand().is_none() { // print help if no subcommand
            build_app().find_subcommand("deno").unwrap().clone().print_help().unwrap();
            return;
        }
        let (deno_sub_command, deno_sub_command_args) = sub_command_args.subcommand().unwrap();
        if deno_sub_command == "list" {
            println!("Local Deno versions:");
            for deno_version in deno_versions::list().unwrap() {
                println!("  {}", deno_version);
            };
        } else if deno_sub_command == "add" {
            let mut deno_version = deno_sub_command_args.value_of("version").unwrap().to_string();
            if deno_version.starts_with("v") {
                deno_version = deno_version[1..].to_string();
            }
            println!("Begin to install Deno...");
            deno_versions::install(&deno_version).unwrap();
            println!("Deno installed successfully!");
        } else if deno_sub_command == "delete" {
            let deno_version = deno_sub_command_args.value_of("version").unwrap();
            deno_versions::delete(deno_version).unwrap();
            println!("Deno deleted successfully!");
        } else {
            println!("{}", "Unknown subcommand");
        }
    } else if sub_command == "trust" {
        if sub_command_args.subcommand().is_none() { // print help if no subcommand
            build_app().find_subcommand("trust").unwrap().clone().print_help().unwrap();
            return;
        }
        let (trust_sub_command, trust_sub_command_args) = sub_command_args.subcommand().unwrap();
        if trust_sub_command == "list" {
            println!("Local trusted catalogs:");
            for catalog in known_catalogs::list().unwrap() {
                println!("  {}", catalog);
            };
        } else if trust_sub_command == "add" {
            let repo_name = trust_sub_command_args.value_of("repo_name").unwrap().to_string();
            known_catalogs::add(&repo_name).unwrap();
            println!("Catalog in trusted list now!");
        } else if trust_sub_command == "delete" {
            let repo_name = trust_sub_command_args.value_of("repo_name").unwrap().to_string();
            known_catalogs::remove(&repo_name).unwrap();
            println!("Catalog removed from trusted list!");
        } else {
            println!("{}", "Unknown subcommand");
        }
    } else {
        println!("{}", "Unknown subcommand");
    }
}

fn confirm_remote_catalog(repo_name: &str) -> anyhow::Result<bool> {
    // check trusted catalog or not
    let is_trusted = known_catalogs::include(repo_name)?;
    if is_trusted {
        return Ok(true);
    }
    let catalog = catalog::Catalog::fetch_from_github(repo_name)?;
    let catalog_json = serde_json::to_string(&catalog)?;
    println!("Detail of nbang-catalog.json:");
    println!("{}", catalog_json.to_colored_json_auto()?);
    print!("Do you accept above catalog?  y/n > ");
    io::stdout().flush()?;
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    return if buffer.trim().starts_with("y") {
        catalog.save(repo_name)?;
        catalog.cache_artifacts(repo_name)?;
        Ok(true)
    } else {
        Ok(false)
    };
}

fn dbang_run(artifact_full_name: &str, artifact_args: &[&str]) -> anyhow::Result<()> {
    let artifact_parts: Vec<&str> = artifact_full_name.split("@").collect();
    let repo_name = artifact_parts[1];
    let artifact_name = artifact_parts[0];
    // validate local catalog exists or not
    if !catalog::Catalog::local_exists(repo_name)? {
        if !confirm_remote_catalog(repo_name)? {
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

