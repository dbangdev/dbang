use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use reqwest::blocking::Client;
use std::path::{Path, PathBuf};
use std::{fs};
use crate::deno_cli;

#[derive(Serialize, Deserialize, Debug)]
pub struct Catalog {
    pub scripts: HashMap<String, Artifact>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Artifact {
    #[serde(rename(serialize = "script-ref", deserialize = "script-ref"))]
    pub script_ref: String,
    pub description: Option<String>,
    pub deno: Option<String>,
    pub permissions: Option<Vec<String>>,
}

impl Artifact {
    pub fn read_from_local(github_user: &str, artifact_name: &str) -> anyhow::Result<Artifact> {
        let catalog = Catalog::read_from_local(github_user)?;
        let artifact = catalog.scripts.get(artifact_name).unwrap();
        Ok(artifact.clone())
    }

    pub fn get_script_http_url(&self, github_user: &str) -> String {
        return if self.script_ref.starts_with("https://") || self.script_ref.starts_with("http://") {
            self.script_ref.to_string()
        } else {
            format!("https://raw.githubusercontent.com/{}/dbang-catalog/main/{}", github_user, self.script_ref)
        };
    }

    pub fn get_deno_permissions(&self) -> Vec<String> {
        if let Some(permissions) = &self.permissions {
            return permissions.iter().map(|x| {
                if x.starts_with("--") {
                    x.clone()
                } else if x.starts_with("-") {
                    format!("-{}", x)
                } else {
                    format!("--{}", x)
                }
            }).collect();
        }
        return vec![];
    }
}

impl Catalog {
    pub fn cache_artifacts(&self, github_user: &str) -> anyhow::Result<()> {
        for (_k, v) in self.scripts.iter() {
            deno_cli::cache(&v.get_script_http_url(github_user))?;
        };
        Ok(())
    }

    pub fn fetch_from_github(github_user: &str) -> anyhow::Result<Catalog> {
        let url = format!("https://raw.githubusercontent.com/{}/dbang-catalog/main/dbang-catalog.json", github_user);
        let client = Client::new();
        let response = client.get(&url).send()?;
        let catalog: Catalog = response.json()?;
        Ok(catalog)
    }

    pub fn read_from_local(github_user: &str) -> anyhow::Result<Catalog> {
        let home_dir: PathBuf = dirs::home_dir().unwrap();
        let dbang_catalog_json_file = Path::new(&home_dir)
            .join(".dbang")
            .join("catalogs/github")
            .join(github_user)
            .join("dbang-catalog")
            .join("dbang-catalog.json");
        let data = fs::read_to_string(dbang_catalog_json_file).expect("Unable to read dbang-catalog.json");
        let catalog: Catalog = serde_json::from_str(&data).expect("Unable to parse dbang-catalog.json");
        Ok(catalog)
    }

    pub fn local_exists(github_user: &str) -> anyhow::Result<bool> {
        let home_dir: PathBuf = dirs::home_dir().unwrap();
        let dbang_catalog_json_file = Path::new(&home_dir)
            .join(".dbang")
            .join("catalogs/github")
            .join(github_user)
            .join("dbang-catalog")
            .join("dbang-catalog.json");
        Ok(dbang_catalog_json_file.exists())
    }
}


pub fn save_nbang_catalog_from_json(github_user: &str, json_text: &str) -> anyhow::Result<()> {
    let home_dir: PathBuf = dirs::home_dir().unwrap();
    let dbang_catalog_dir = Path::new(&home_dir)
        .join(".dbang")
        .join("catalogs/github")
        .join(github_user)
        .join("dbang-catalog");
    std::fs::create_dir_all(&dbang_catalog_dir)?;
    let dbang_catalog_file = dbang_catalog_dir.join("dbang-catalog.json");
    std::fs::write(&dbang_catalog_file, json_text)?;
    Ok(())
}

pub fn save_remote_nbang_catalog(github_user: &str) -> anyhow::Result<()> {
    let url = format!("https://raw.githubusercontent.com/{}/dbang-catalog/main/dbang-catalog.json", github_user);
    let response = Client::builder()
        .build()?
        .get(&url)
        .header("Accept", "application/json")
        .send()?;
    let json_text = response.text()?;
    save_nbang_catalog_from_json(github_user, &json_text)
}


mod tests {
    use super::*;

    #[test]
    fn test_deserialize() {
        //language=json
        let json_text = r#"
        {
          "aliases": {
            "hello": {
              "script-ref": "hello.ts",
              "description": "Hello world"
            },
            "myip": {
              "script-ref": "myip.ts",
              "description": "Display your IP address by https://httpbin.org/ip",
              "deno": "1.17.1",
              "permissions": [
                "allow-net"
              ]
            }
          }
        }"#;
        let catalog: Catalog = serde_json::from_str(json_text).unwrap();
        println!("catalog = {:?}", catalog);
    }

    #[test]
    fn test_save_nbang_catalog() -> anyhow::Result<()> {
        let github_user = "linux-china";
        save_remote_nbang_catalog(github_user)?;
        if Catalog::local_exists(github_user).is_ok() {
            let catalog = Catalog::read_from_local(github_user)?;
            println!("catalog = {:?}", catalog);
        }
        Ok(())
    }

    #[test]
    fn test_fetch_remote_nbang_catalog() -> anyhow::Result<()> {
        let catalog = Catalog::fetch_from_github("linux-china")?;
        println!("catalog = {:?}", catalog);
        Ok(())
    }

    #[test]
    fn test_read_local_dbang_catalog() -> anyhow::Result<()> {
        let catalog = Catalog::read_from_local("linux-china")?;
        println!("catalog = {:?}", catalog);
        Ok(())
    }

    #[test]
    fn test_get_artifact() {
        let artifact = Artifact::read_from_local("linux-china", "hello").unwrap();
        println!("artifact = {:?}", artifact);
        println!("url = {}", artifact.get_script_http_url("linux-china"));
    }
}
