//! clap App for command cli
use clap::{App, Arg};

const VERSION: &str = "0.1.7";

pub fn build_app() -> App<'static> {
    let run_command = App::new("run")
        .about("Run scripts from catalog")
        .arg(
            Arg::new("script")
                .required(true)
                .help("script name")
                .index(1)
        ).arg(
        Arg::new("params")
            .required(false)
            .help("params")
            .index(2)
            .multiple_values(true)
    );
    let open_command = App::new("open")
        .about("Open catalog on GitHub")
        .arg(
            Arg::new("script")
                .required(true)
                .help("script name")
                .index(1)
        );
    let deno_command = App::new("deno")
        .about("Deno version management")
        .subcommand(App::new("list")
            .about("List installed deno versions")
        )
        .subcommand(App::new("add")
            .about("Install Deno with version")
            .arg(
                Arg::new("default")
                    .long("default")
                    .takes_value(false)
                    .help("Set as default version")
                    .required(false)
            )
            .arg(Arg::new("version")
                .required(true)
                .help("Deno version")
                .index(1)
            )
        )
        .subcommand(App::new("default")
            .about("Set default Deno version")
            .arg(Arg::new("version")
                .required(true)
                .help("Default Deno version for DBang")
                .index(1)
            )
        )
        .subcommand(App::new("delete")
            .about("Delete local installed Deno")
            .arg(Arg::new("version")
                .required(true)
                .help("Deno version")
                .index(1)
            )
        );
    let trust_command = App::new("trust")
        .about("Trust management for catalogs")
        .subcommand(App::new("list")
            .about("List trusted catalogs")
        )
        .subcommand(App::new("add")
            .about("Add new trusted catalog")
            .arg(Arg::new("repo_name")
                .required(true)
                .help("GitHub repo name, e.g. github_user or github_user/repo")
                .index(1)
            )
        )
        .subcommand(App::new("delete")
            .about("Delete trusted catalog")
            .arg(Arg::new("repo_name")
                .required(true)
                .help("GitHub repo name, e.g. github_user or github_user/repo")
                .index(1)
            )
        );
    let install_command = App::new("install")
        .about("Install app from catalog")
        .arg(
            Arg::new("name")
                .long("name")
                .takes_value(true)
                .help("Custom app name for script")
                .required(false),
        )
        .arg(
            Arg::new("script")
                .help("script full name")
                .required(true)
                .index(1)
        );
    let uninstall_command = App::new("uninstall")
        .about("Uninstall app")
        .arg(
            Arg::new("name")
                .help("App name for script")
                .required(true)
                .index(1)
        );
    let apps_command = App::new("apps")
        .about("List installed apps");
    let catalog_command = App::new("catalog")
        .about("Catalog management")
        .subcommand(App::new("list")
            .about("List installed catalogs")
        )
        .subcommand(App::new("show")
            .about("Display catalog info")
            .arg(Arg::new("repo_name")
                .required(true)
                .help("GitHub repo name, e.g. github_user or github_user/repo")
                .index(1)
            )
        )
        .subcommand(App::new("add")
            .about("Add new catalog")
            .arg(Arg::new("repo_name")
                .required(true)
                .help("GitHub repo name, e.g. github_user or github_user/repo")
                .index(1)
            )
        )
        .subcommand(App::new("update")
            .about("Update local catalog")
            .arg(Arg::new("repo_name")
                .required(true)
                .help("GitHub repo name, e.g. github_user or github_user/repo")
                .index(1)
            )
        )
        .subcommand(App::new("delete")
            .about("Delete local catalog")
            .arg(Arg::new("repo_name")
                .required(true)
                .help("GitHub repo name, e.g. github_user or github_user/repo")
                .index(1)
            )
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
        .arg(
            Arg::new("verbose")
                .long("verbose")
                .short('v')
                .takes_value(false)
                .help("Verbose output")
                .required(false),
        )
        .subcommand(run_command)
        .subcommand(open_command)
        .subcommand(deno_command)
        .subcommand(trust_command)
        .subcommand(install_command)
        .subcommand(uninstall_command)
        .subcommand(apps_command)
        .subcommand(catalog_command)
        .subcommand(complete_command)
        .arg(Arg::new("script")
            .required(false)
            .help("script name")
            .index(1))
        .arg(
            Arg::new("params")
                .required(false)
                .help("params")
                .index(2)
                .multiple_values(true)
        )
}
