mod app;
mod deno_cli;
mod deno_versions;
mod catalog;
mod known_catalogs;
mod dbang_utils;
mod aliases;

use std::collections::HashMap;
use std::io;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::Duration;
use colored_json::ToColoredJson;
use which::which;
use crate::app::build_app;
use colored::*;
use crate::catalog::Catalog;
use update_informer::{registry::GitHub, Check, UpdateInformer};

fn main() {
    let app = build_app();
    let matches = app.get_matches();
    let verbose = matches.is_present("verbose");
    let quiet = matches.is_present("quiet");
    if !quiet {
        //update informer: dbang new version
        let dbang_informer = UpdateInformer::new(GitHub, "dbangdev/dbang", app::VERSION, Duration::from_secs(60 * 60 * 24));
        if let Ok(Some(version)) = dbang_informer.check_version() {
            println!("DBang new version available: {}", version);
        }
        //update informer: deno new version
        if let Some(deno_version) = deno_versions::get_default_deno_version() {
            let deno_informer = UpdateInformer::new(GitHub, "denoland/deno", &deno_version, Duration::from_secs(60 * 60 * 24));
            if let Ok(Some(version)) = deno_informer.check_version() {
                println!("Deno new version available: {}, please use `dbang deno install --default {}` to update!", version, version);
            }
        }
    }
    // run artifact without 'run' sub command
    if matches.is_present("script") {
        let artifact_full_name = matches.value_of("script").unwrap();
        let mut artifact_args = vec![];
        if let Some(params) = matches.values_of("params") {
            artifact_args = params.collect::<Vec<&str>>()
        }
        dbang_run(artifact_full_name, &artifact_args, verbose).unwrap();
        return;
    }
    if matches.subcommand().is_none() { //display help if no subcommand
        build_app().print_help().unwrap();
        return;
    }
    // make sure DBANG_DIR ~/.dbang/bin exist
    let dbang_bin_dir = dbang_utils::dbang_dir().join("bin");
    if !dbang_bin_dir.exists() {
        std::fs::create_dir_all(&dbang_bin_dir).unwrap();
    }
    // parse subcommand and run
    let (sub_command, sub_command_args) = matches.subcommand().unwrap();
    if sub_command == "run" {
        let mut artifact_args = vec![];
        if let Some(params) = sub_command_args.values_of("params") {
            artifact_args = params.collect::<Vec<&str>>()
        }
        let artifact_full_name = sub_command_args.value_of("script").unwrap();
        dbang_run(artifact_full_name, &artifact_args, verbose).unwrap();
    }
    if sub_command == "open" {
        let artifact_full_name = sub_command_args.value_of("script").unwrap();
        let url = format!("https://github.com/{}", artifact_full_name);
        dbang_utils::open_url(&url).unwrap();
    } else if sub_command == "install" {
        let artifact_full_name = sub_command_args.value_of("script").unwrap();
        let app_name = if let Some(name) = sub_command_args.value_of("name") {
            name.to_string()
        } else {
            if artifact_full_name.starts_with("http://") || artifact_full_name.starts_with("https://") {
                artifact_full_name.split("/").last().unwrap().to_string()
            } else if artifact_full_name.contains("@") {
                artifact_full_name.split("@").next().unwrap().to_string()
            } else {
                artifact_full_name.to_string()
            }
        };
        if app_name == "dbang" || app_name.starts_with("dbang-") || app_name == "deno" {
            println!("{}", "dbang, deno and dbang-* are reserved names, please use other names".red());
            return;
        }
        aliases::add(app_name.clone(), artifact_full_name.to_string()).unwrap();
        //create soft link
        let dbang_shim_path = which("dbang-shim").unwrap();
        let app_link = dbang_bin_dir.join(&app_name);
        if app_link.exists() {
            symlink::remove_symlink_file(&app_link).unwrap();
        }
        symlink::symlink_file(dbang_shim_path, app_link).unwrap();
        println!("{} app installed", app_name);
    } else if sub_command == "uninstall" {
        let app_name = sub_command_args.value_of("name").unwrap();
        aliases::remove(app_name).unwrap();
        let app_link = dbang_bin_dir.join(&app_name);
        if app_link.exists() {
            symlink::remove_symlink_file(app_link).unwrap();
        }
        println!("{} uninstalled successfully", app_name);
    } else if sub_command == "apps" {
        let apps: HashMap<String, String> = aliases::all().unwrap();
        if apps.is_empty() {
            println!("No apps installed");
        } else {
            println!("Local installed apps:");
            for pair in apps {
                println!("  {} -> {}", pair.0, pair.1);
            }
        }
    } else if sub_command == "catalog" {
        if sub_command_args.subcommand().is_none() { // print help if no subcommand
            build_app().find_subcommand("catalog").unwrap().clone().print_help().unwrap();
            return;
        }
        let (catalog_sub_command, catalog_sub_command_args) = sub_command_args.subcommand().unwrap();
        if catalog_sub_command == "list" {
            println!("Local installed catalogs:");
            for catalog_full_name in catalog::Catalog::list_local().unwrap() {
                println!("  {}", catalog_full_name);
            };
        } else if catalog_sub_command == "add" || catalog_sub_command == "update" {
            let repo_name = catalog_sub_command_args.value_of("repo_name").unwrap();
            let repo_full_name = Catalog::get_full_repo_name(repo_name);
            if confirm_remote_catalog(&repo_full_name).unwrap() {
                if catalog_sub_command == "add" {
                    println!("Catalog added successfully!");
                } else {
                    println!("Catalog updated successfully!");
                }
            } else {
                println!("{}", "Abort to accept dbang catalog!".red());
            }
        } else if catalog_sub_command == "delete" {
            let repo_name = catalog_sub_command_args.value_of("repo_name").unwrap();
            let repo_full_name = Catalog::get_full_repo_name(repo_name);
            catalog::Catalog::delete(&repo_full_name).unwrap();
            known_catalogs::remove(&repo_full_name).unwrap();
            aliases::remove_by_repo_name(&repo_full_name).unwrap();
            println!("Catalog deleted successfully!");
        } else if catalog_sub_command == "show" {
            let repo_name = catalog_sub_command_args.value_of("repo_name").unwrap();
            let catalog = catalog::Catalog::read_from_local(repo_name).unwrap();
            let catalog_json = serde_json::to_string(&catalog).unwrap();
            println!("{}", catalog_json.to_colored_json_auto().unwrap());
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
            let as_default = deno_sub_command_args.is_present("default");
            if deno_version.starts_with("v") {
                deno_version = deno_version[1..].to_string();
            }
            println!("Begin to install Deno {} ...", deno_version);
            deno_versions::install(&deno_version).unwrap();
            println!("Deno {} installed successfully!", deno_version);
            if as_default {
                deno_versions::link_as_default(&deno_version).unwrap();
                println!("Default deno switched to {}", deno_version);
            }
        } else if deno_sub_command == "delete" {
            let deno_version = deno_sub_command_args.value_of("version").unwrap();
            deno_versions::delete(deno_version).unwrap();
            println!("Deno deleted successfully!");
        } else if deno_sub_command == "default" {
            let deno_version = deno_sub_command_args.value_of("version").unwrap();
            deno_versions::link_as_default(deno_version).unwrap();
            println!("Default deno switched to {}", deno_version);
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
            known_catalogs::add(&Catalog::get_full_repo_name(&repo_name)).unwrap();
            println!("Catalog in trusted list now!");
        } else if trust_sub_command == "delete" {
            let repo_name = trust_sub_command_args.value_of("repo_name").unwrap().to_string();
            known_catalogs::remove(&Catalog::get_full_repo_name(&repo_name)).unwrap();
            println!("Catalog removed from trusted list!");
        } else {
            println!("{}", "Unknown subcommand");
        }
    }
}

fn confirm_remote_catalog(repo_name: &str) -> anyhow::Result<bool> {
    // check trusted catalog or not
    let is_trusted = known_catalogs::include(repo_name)?;
    if is_trusted {
        catalog::save_remote_nbang_catalog(repo_name)?;
        return Ok(true);
    }
    let catalog = catalog::Catalog::fetch_from_github(repo_name)?;
    let catalog_json = serde_json::to_string(&catalog)?;
    println!("Detail of dbang-catalog.json:");
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

fn dbang_run(artifact_full_name: &str, artifact_args: &[&str], verbose: bool) -> anyhow::Result<()> {
    if !artifact_full_name.contains('@') { // run from local dbang-catalog.json
        return dbang_run_script(artifact_full_name, artifact_args, verbose);
    }
    let artifact_parts: Vec<&str> = artifact_full_name.split("@").collect();
    let repo_name = artifact_parts[1];
    let script_name = artifact_parts[0];
    // validate local catalog exists or not
    if !catalog::Catalog::local_exists(repo_name)? {
        if !confirm_remote_catalog(repo_name)? {
            println!("Abort to accept dbang catalog!");
            return Ok(());
        }
    }
    let artifact = catalog::Artifact::read_from_local(repo_name, script_name).unwrap();
    if !artifact.is_platform_compatible() {
        eprintln!("Script is not compatible with this platform: {:?}", artifact.platforms.as_ref().unwrap());
        return Ok(());
    }
    let script_url = artifact.get_script_http_url(repo_name);
    let permissions: Vec<String> = artifact.get_deno_permissions();
    if verbose {
        println!("[dbang] begin to run {}/{}", script_name, artifact_full_name);
        println!("[dbang] script url:  {}", script_url);
        if let Some(ref description) = artifact.description {
            println!("[dbang] script description:  {}", description);
        }
        if !permissions.is_empty() {
            println!("[dbang] script permissions:  {}", permissions.join(","));
        }
    }
    deno_cli::run(repo_name, &artifact, artifact_args, verbose)?;
    Ok(())
}

fn dbang_run_script(artifact_full_name: &str, artifact_args: &[&str], verbose: bool) -> anyhow::Result<()> {
    let current_dir = std::env::current_dir()?;
    let dbang_catalog_file = find_local_dbang_catalog(Some(current_dir.as_path()));
    if dbang_catalog_file.is_none() {
        eprintln!("dbang-catalog.json not found in current directory, parent directories or $HOME/.dbang");
        return Ok(());
    } else {
        let dbang_catalog_json_file = dbang_catalog_file.unwrap();
        let catalog = catalog::Catalog::read_from_file(&dbang_catalog_json_file)?;
        if let Some(artifact) = catalog.scripts.get(artifact_full_name) {
            deno_cli::run_local(dbang_catalog_json_file.parent().unwrap(), artifact, artifact_args, verbose)?;
            Ok(())
        } else {
            Err(anyhow::anyhow!("{} is not in dbang-catalog.json!", artifact_full_name))
        }
    }
}

fn find_local_dbang_catalog(base_dir: Option<&Path>) -> Option<PathBuf> {
    if let Some(dir) = base_dir {
        let dbang_catalog_file = dir.join("dbang-catalog.json");
        return if dbang_catalog_file.exists() {
            Some(dbang_catalog_file)
        } else {
            find_local_dbang_catalog(dir.parent())
        };
    }
    let default_dbang_catalog = dirs::home_dir().unwrap().join(".dbang").join("dbang-catalog.json");
    return if default_dbang_catalog.exists() {
        Some(default_dbang_catalog)
    } else {
        None
    };
}
