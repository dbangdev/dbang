use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use reqwest::blocking::Client;

#[derive(Serialize, Deserialize, Debug)]
pub struct Catalog {
    pub aliases: HashMap<String, Artifact>,
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

pub fn get_artifact(github_user: &str, artifact_name: &str) -> reqwest::Result<Artifact> {
    let catalog = fetch_nbang_catalog(github_user)?;
    let artifact = catalog.aliases.get(artifact_name).unwrap();
    Ok(artifact.clone())
}

pub fn fetch_nbang_catalog(github_user: &str) -> reqwest::Result<Catalog> {
    let url = format!("https://raw.githubusercontent.com/{}/dbang-catalog/main/dbang-catalog.json", github_user);
    let response = Client::builder()
        .build()?
        .get(&url)
        .header("Accept", "application/json")
        .send()?;
    let catalog = response.json::<Catalog>()?;
    Ok(catalog)
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
    fn test_fetch_nbang_catalog() {
        let catalog = fetch_nbang_catalog("linux-china").unwrap();
        println!("{:#?}", catalog);
    }

    #[test]
    fn test_get_artifact() {
        let artifact = get_artifact("linux-china", "hello").unwrap();
        println!("artifact = {:?}", artifact);
        println!("url = {}", artifact.get_script_http_url("linux-china"));
    }
}
