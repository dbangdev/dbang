mod dbang_utils;
mod aliases;
mod catalog;
mod deno_cli;
mod deno_versions;

pub fn main() {
    let mut alias: String = std::env::args().nth(0).unwrap();
    if alias.contains("/") {
        alias = alias.split("/").last().unwrap().to_string();
    }
    let script_args = std::env::args().skip(1).collect::<Vec<String>>();
    let script_args: Vec<&str> = script_args.iter().map(std::ops::Deref::deref).collect();
    if let Some(script_name) = aliases::find_script_name_by_alias(&alias) {
        dbang_run(&script_name, &script_args).unwrap();
    } else {
        println!("{} not found!", alias);
    }
}

fn dbang_run(script_full_name: &str, script_args: &[&str]) -> anyhow::Result<()> {
    let artifact_parts: Vec<&str> = script_full_name.split("@").collect();
    let repo_name = artifact_parts[1];
    let script_name = artifact_parts[0];
    let artifact = catalog::Artifact::read_from_local(repo_name, script_name).unwrap();
    let script_url = artifact.get_script_http_url(repo_name);
    let permissions: Vec<String> = artifact.get_deno_permissions();
    deno_cli::run(&artifact.get_deno_bin_path(), &script_url, script_args, &permissions)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_find_script_name() {}
}
