mod app;

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
}
