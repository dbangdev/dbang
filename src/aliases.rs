use std::collections::HashMap;
use std::path::PathBuf;
use crate::dbang_utils;

fn get_aliases_file() -> PathBuf {
    dbang_utils::dbang_dir().join("aliases.json")
}

pub fn find_script_name_by_alias(alias: &str) -> Option<String> {
    let aliases = all().unwrap();
    aliases.get(alias).map(|x| x.to_string())
}

pub fn all() -> anyhow::Result<HashMap<String, String>> {
    let aliases_file = get_aliases_file();
    if aliases_file.exists() {
        let data = std::fs::read_to_string(aliases_file).expect("Unable to read aliases.json");
        let aliases: HashMap<String, String> = serde_json::from_str(&data).expect("Unable to parse aliases.json");
        Ok(aliases)
    } else {
        Ok(HashMap::new())
    }
}

pub fn save(aliases: &HashMap<String, String>) -> anyhow::Result<()> {
    let aliases_file = get_aliases_file();
    let data = serde_json::to_string(aliases).expect("Unable to serialize aliases");
    std::fs::write(aliases_file, data)?;
    Ok(())
}

pub fn add(alias: String, script_name: String) -> anyhow::Result<()> {
    let mut aliases = all()?;
    aliases.insert(alias, script_name);
    save(&aliases)
}

pub fn remove(alias: &str) -> anyhow::Result<()> {
    let mut aliases = all()?;
    aliases.remove(alias);
    save(&aliases)
}

pub fn remove_by_repo_name(repo_name: &str) -> anyhow::Result<()> {
    let mut aliases = all()?;
    let empties: Vec<_> = aliases
        .iter()
        .filter(|&(_, v)| v == repo_name)
        .map(|(k, _)| k.clone())
        .collect();
    for empty in empties { aliases.remove(&empty); }
    save(&aliases)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        add("hello".to_string(), "hello@linux-china".to_string()).unwrap();
        let script_name = find_script_name_by_alias("hello").unwrap();
        assert_eq!(script_name, "hello@linux-china");
    }
}
