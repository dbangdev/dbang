//! clap App for command cli
use clap::{App, Arg};

const VERSION: &str = "0.1.0";

pub fn build_app() -> App<'static> {
    let run_command = App::new("run")
        .about("Run scripts from catalog")
        .arg(
            Arg::new("artifact")
                .required(true)
                .help("artifact name")
                .index(1)
        ).arg(
        Arg::new("params")
            .required(false)
            .help("params")
            .index(2)
            .multiple_values(true)
    );
    let deno_command = App::new("deno")
        .about("Deno version management")
        .arg(
            Arg::new("name")
                .long("name") // allow --name
                .takes_value(true)
                .help("template name")
                .required(true),
        )
        .arg(
            Arg::new("repo")
                .long("repo") // allow --name
                .takes_value(true)
                .help("git repository url")
                .required(true),
        )
        .arg(
            Arg::new("desc")
                .long("desc") // allow --name
                .takes_value(true)
                .help("template description")
                .required(true),
        );
    let trust_command = App::new("trust")
        .about("Catalog trust management")
        .arg(
            Arg::new("name")
                //.long("name") // allow --name
                .takes_value(true)
                .help("template name")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("dir")
                //.long("dir") // allow --name
                .takes_value(true)
                .help("App's directory")
                .required(true)
                .index(2),
        );
    let install_command = App::new("install")
        .about("Install app from catalog")
        .arg(
            Arg::new("name")
                .takes_value(true)
                .help("template name")
                .required(true),
        );
    let catalog_command = App::new("catalog").about("Catalog").arg(
        Arg::new("remote")
            .long("remote")
            .takes_value(false)
            .help("remotes template")
            .required(false),
    );
    let complete_command = App::new("complete")
        .about("Generate shell completion for zsh & bash")
        .arg(
            Arg::new("zsh")
                .long("zsh")
                .takes_value(false)
                .help("Zsh completion")
                .required(false),
        )
        .arg(
            Arg::new("oh_my_zsh")
                .long("oh_my_zsh")
                .takes_value(false)
                .help("Oh My Zsh")
                .required(false),
        )
        .arg(
            Arg::new("bash")
                .long("bash")
                .takes_value(false)
                .help("Bash completion")
                .required(false),
        );
    // init Clap
    App::new("dbang")
        .version(VERSION)
        .about("CLI to manage Deno scripts: https://dbang.dev")
        .subcommand(run_command)
        .subcommand(deno_command)
        .subcommand(trust_command)
        .subcommand(install_command)
        .subcommand(catalog_command)
        .subcommand(complete_command)
        .arg(Arg::new("artifact")
            .required(false)
            .help("artifact name")
            .index(1))
        .arg(
            Arg::new("params")
                .required(false)
                .help("params")
                .index(2)
                .multiple_values(true)
        )
}
