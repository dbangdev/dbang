use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use reqwest::blocking::Client;
use std::{fs};
use std::path::Path;
use crate::{dbang_utils, deno_cli, deno_versions};

#[derive(Serialize, Deserialize, Debug)]
pub struct Catalog {
    pub scripts: HashMap<String, Artifact>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Artifact {
    #[serde(rename(serialize = "script-ref", deserialize = "script-ref"))]
    pub script_ref: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compat: Option<bool>,
    #[serde(rename(serialize = "import-map", deserialize = "import-map"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_map: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deno: Option<String>,
    /// platform os and arch, format as `os-arch`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platforms: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unstable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<String>>,
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

    pub fn get_import_map_http_url(&self, repo_name: &str) -> String {
        let import_map = self.import_map.as_ref().unwrap().clone();
        return if import_map.starts_with("https://") || import_map.starts_with("http://") {
            import_map
        } else {
            let catalog_repo = Catalog::get_full_repo_name(repo_name);
            format!("https://raw.githubusercontent.com/{}/HEAD/{}", catalog_repo, import_map)
        };
    }

    pub fn get_deno_config(&self, repo_name: &str) -> String {
        let catalog_repo = Catalog::get_full_repo_name(repo_name);
        let deno_config_file = dbang_utils::dbang_dir().join("catalogs/github").join(catalog_repo).join("deno.json");
        if !deno_config_file.exists() {
            fs::write(&deno_config_file, "{}").unwrap();
        }
        String::from(deno_config_file.to_string_lossy())
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
        let default_deno = deno_versions::get_default_deno();
        if default_deno.exists() {
            return String::from(default_deno.to_string_lossy());
        }
        return "deno".to_string();
    }

    pub fn is_platform_compatible(&self) -> bool {
        if let Some(platforms) = &self.platforms {
            let os = std::env::consts::OS;
            let arch = std::env::consts::ARCH;
            let full_name = format!("{}-{}", os, arch);
            return platforms.iter().any(|x| {
                x == os || x == &full_name
            });
        }
        return true;
    }
}

impl Catalog {
    pub fn cache_artifacts(&self, github_user: &str) -> anyhow::Result<()> {
        for (_k, v) in self.scripts.iter() {
            deno_cli::cache(&v.get_deno_bin_path(), &v.get_script_http_url(github_user), &v.import_map)?;
        };
        Ok(())
    }

    pub fn fetch_from_github(repo_name: &str) -> anyhow::Result<Catalog> {
        let catalog_full_name = Catalog::get_full_repo_name(repo_name);
        let url = get_dbang_catalog_url_on_github(&catalog_full_name);
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
        return Catalog::read_from_file(&dbang_catalog_json_file);
    }

    pub fn read_from_file(dbang_catalog_json_file: &Path) -> anyhow::Result<Catalog> {
        let data = fs::read_to_string(dbang_catalog_json_file)
            .expect(format!("Unable to read {}", dbang_catalog_json_file.to_string_lossy()).as_str());
        let catalog: Catalog = serde_json::from_str(&data)
            .expect(format!("Unable to parse {}", dbang_catalog_json_file.to_string_lossy()).as_str());
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
        let deno_config_file = dbang_catalog_dir.join("deno.json");
        if !deno_config_file.exists() {
            std::fs::write(&deno_config_file, "{}")?;
        }
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


pub fn save_remote_dbang_catalog(repo_name: &str) -> anyhow::Result<()> {
    let catalog_full_name = Catalog::get_full_repo_name(repo_name);
    let url = get_dbang_catalog_url_on_github(&catalog_full_name);
    let response = Client::builder()
        .build()?
        .get(&url)
        .header("Accept", "application/json")
        .send()?;
    let catalog = response.json::<Catalog>()?;
    catalog.save(&catalog_full_name)
}

fn get_dbang_catalog_url_on_github(catalog_full_name: &str) -> String {
    let github_auth_token = dbang_utils::github_auth_token();
    return if let Some(token) = github_auth_token {
        format!("https://{}@raw.githubusercontent.com/{}/HEAD/dbang-catalog.json", token, catalog_full_name)
    } else {
        format!("https://raw.githubusercontent.com/{}/HEAD/dbang-catalog.json", catalog_full_name)
    };
}

#[cfg(test)]
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
    fn test_save_dbang_catalog() -> anyhow::Result<()> {
        let github_user = "linux-china";
        save_remote_dbang_catalog(github_user)?;
        if Catalog::local_exists(github_user).is_ok() {
            let catalog = Catalog::read_from_local(github_user)?;
            println!("catalog = {:?}", catalog);
        }
        Ok(())
    }

    #[test]
    fn test_fetch_remote_dbang_catalog() -> anyhow::Result<()> {
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

    #[test]
    fn test_is_platform_compatible() {
        let artifact = Artifact {
            script_ref: "hello.ts".to_string(),
            description: Some("Hello world".to_string()),
            platforms: Some(vec!["macos".to_string()]),
            deno: None,
            import_map: None,
            unstable: None,
            permissions: None,
            compat: None,
            args: None,
        };
        if cfg!(target_os = "macos") {
            assert!(artifact.is_platform_compatible());
        } else {
            assert!(!artifact.is_platform_compatible());
        }
    }
}
