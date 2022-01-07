use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use reqwest::blocking::Client;
use std::{fs};
use crate::{dbang_utils, deno_cli, deno_versions};

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
    pub fn read_from_local(repo_name: &str, artifact_name: &str) -> anyhow::Result<Artifact> {
        let catalog_repo = Catalog::get_full_repo_name(repo_name);
        let catalog = Catalog::read_from_local(&catalog_repo)?;
        let artifact = catalog.scripts.get(artifact_name).unwrap();
        Ok(artifact.clone())
    }

    pub fn get_script_http_url(&self, repo_name: &str) -> String {
        return if self.script_ref.starts_with("https://") || self.script_ref.starts_with("http://") {
            self.script_ref.to_string()
        } else {
            let catalog_repo = Catalog::get_full_repo_name(repo_name);
            format!("https://raw.githubusercontent.com/{}/HEAD/{}", catalog_repo, self.script_ref)
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

    pub fn get_deno_bin_path(&self) -> String {
        if let Some(deno_version) = &self.deno {
            return String::from(deno_versions::get_deno_binary(deno_version).to_string_lossy());
        }
        return "deno".to_string();
    }
}

impl Catalog {
    pub fn cache_artifacts(&self, github_user: &str) -> anyhow::Result<()> {
        for (_k, v) in self.scripts.iter() {
            deno_cli::cache(&v.get_deno_bin_path(), &v.get_script_http_url(github_user))?;
        };
        Ok(())
    }

    pub fn fetch_from_github(repo_name: &str) -> anyhow::Result<Catalog> {
        let catalog_full_name = Catalog::get_full_repo_name(repo_name);
        let url = format!("https://raw.githubusercontent.com/{}/HEAD/dbang-catalog.json", catalog_full_name);
        let client = Client::new();
        let response = client.get(&url).send()?;
        let catalog: Catalog = response.json()?;
        Ok(catalog)
    }

    pub fn read_from_local(repo_name: &str) -> anyhow::Result<Catalog> {
        let catalog_full_name = Catalog::get_full_repo_name(repo_name);
        let dbang_catalog_json_file = dbang_utils::dbang_dir()
            .join("catalogs/github")
            .join(catalog_full_name)
            .join("dbang-catalog.json");
        let data = fs::read_to_string(dbang_catalog_json_file).expect("Unable to read dbang-catalog.json");
        let catalog: Catalog = serde_json::from_str(&data).expect("Unable to parse dbang-catalog.json");
        Ok(catalog)
    }

    pub fn save(&self, repo_name: &str) -> anyhow::Result<()> {
        let catalog_full_name = Catalog::get_full_repo_name(repo_name);
        let dbang_catalog_dir = dbang_utils::dbang_dir()
            .join("catalogs/github")
            .join(catalog_full_name);
        std::fs::create_dir_all(&dbang_catalog_dir)?;
        let dbang_catalog_file = dbang_catalog_dir.join("dbang-catalog.json");
        let json_text = serde_json::to_string(self)?;
        std::fs::write(&dbang_catalog_file, json_text)?;
        Ok(())
    }

    pub fn local_exists(repo_name: &str) -> anyhow::Result<bool> {
        let catalog_repo = Catalog::get_full_repo_name(repo_name);
        let dbang_catalog_json_file = dbang_utils::dbang_dir()
            .join("catalogs/github")
            .join(catalog_repo)
            .join("dbang-catalog.json");
        Ok(dbang_catalog_json_file.exists())
    }

    pub fn delete(repo_name: &str) -> anyhow::Result<()> {
        let catalog_repo = Catalog::get_full_repo_name(repo_name);
        let dbang_catalog = dbang_utils::dbang_dir()
            .join("catalogs/github")
            .join(catalog_repo);
        fs::remove_dir_all(&dbang_catalog)?;
        Ok(())
    }

    pub fn list_local() -> anyhow::Result<Vec<String>> {
        let github_dir = dbang_utils::dbang_dir()
            .join("catalogs")
            .join("github");
        let mut users = fs::read_dir(github_dir)?;
        let mut user_list = Vec::new();
        while let Some(file) = users.next() {
            let user = file?;
            let user_path = user.path();
            if user_path.is_dir() {
                let github_user = user.file_name();
                let github_user = github_user.to_str().unwrap();
                let mut repos = fs::read_dir(user_path)?;
                while let Some(repo) = repos.next() {
                    let repo = repo?;
                    let repo_name = repo.file_name();
                    let repo_name = repo_name.to_str().unwrap();
                    user_list.push(format!("{}/{}", github_user, repo_name));
                }
            }
        }
        Ok(user_list)
    }

    pub fn get_full_repo_name(repo_name: &str) -> String {
        return if !repo_name.contains("/") {
            format!("{}/dbang-catalog", repo_name)
        } else {
            repo_name.to_string()
        };
    }
}


pub fn save_remote_nbang_catalog(repo_name: &str) -> anyhow::Result<()> {
    let catalog_full_name = Catalog::get_full_repo_name(repo_name);
    let github_auth_token = dbang_utils::github_auth_token();
    let url = if let Some(token) = github_auth_token {
        format!("https://{}@raw.githubusercontent.com/{}/HEAD/dbang-catalog.json", token, catalog_full_name)
    } else {
        format!("https://raw.githubusercontent.com/{}/HEAD/dbang-catalog.json", catalog_full_name)
    };
    let response = Client::builder()
        .build()?
        .get(&url)
        .header("Accept", "application/json")
        .send()?;
    let catalog = response.json::<Catalog>()?;
    catalog.save(&catalog_full_name)
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

    #[test]
    fn test_list_catalogs() {
        for user in Catalog::list_local().unwrap() {
            println!("user = {}", user);
        }
    }
}
