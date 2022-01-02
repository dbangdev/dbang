use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;
use crate::catalog::Catalog;
use crate::dbang_utils;

fn get_known_catalogs_file() -> PathBuf {
    dbang_utils::dbang_dir().join("known_catalogs")
}

pub fn list() -> anyhow::Result<Vec<String>> {
    let known_catalogs_file = get_known_catalogs_file();
    if known_catalogs_file.exists() {
        let input = File::open(&known_catalogs_file)?;
        let lines = BufReader::new(input).lines().map(|l| l.unwrap()).collect::<Vec<String>>();
        Ok(lines)
    } else {
        Ok(Vec::new())
    }
}

pub fn include(repo_name: &str) -> anyhow::Result<bool> {
    let catalog_full_name = Catalog::get_full_repo_name(repo_name);
    let known_catalogs = list()?;
    Ok(known_catalogs.contains(&catalog_full_name))
}

pub fn add(repo_name: &str) -> anyhow::Result<()> {
    let catalog_full_name = Catalog::get_full_repo_name(repo_name);
    let mut known_catalogs = list().unwrap();
    if !known_catalogs.contains(&catalog_full_name) {
        known_catalogs.push(catalog_full_name);
        let mut output = File::create(get_known_catalogs_file())?;
        output.write_all(known_catalogs.join("\n").as_bytes())?;
    }
    Ok(())
}

pub fn remove(repo_name: &str) -> anyhow::Result<()> {
    let catalog_full_name = Catalog::get_full_repo_name(repo_name);
    let mut known_catalogs = list().unwrap();
    if known_catalogs.contains(&catalog_full_name) {
        let index = known_catalogs.iter().position(|x| *x == catalog_full_name).unwrap();
        known_catalogs.remove(index);
        let mut output = File::create(get_known_catalogs_file())?;
        output.write_all(known_catalogs.join("\n").as_bytes())?;
    }
    Ok(())
}

mod tests {
    use super::*;

    #[test]
    fn test_add_known_catalog() {
        add("linux-china/dbang-catalog").unwrap();
        add("linux-china/demo").unwrap();
        list().unwrap().iter().for_each(|x| println!("{}", x));
        assert!(include("linux-china/dbang-catalog").unwrap());
        remove("linux-china/demo").unwrap();
        println!("{}", list().unwrap().len());
    }
}
