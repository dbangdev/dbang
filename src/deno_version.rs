use std::fs::File;
use reqwest::blocking::Client;
use std::path::{Path, PathBuf};
use std::fs;
use std::io;

pub fn get_deno_binary(version: &str) -> PathBuf {
    get_deno_home(version).join("deno")
}

pub fn get_deno_home(version: &str) -> PathBuf {
    let home_dir: PathBuf = dirs::home_dir().unwrap();
    Path::new(&home_dir)
        .join(".dbang")
        .join("deno")
        .join(version)
}

pub fn list() -> anyhow::Result<Vec<String>> {
    let home_dir: PathBuf = dirs::home_dir().unwrap();
    let deno_dir = Path::new(&home_dir)
        .join(".dbang")
        .join("deno");
    let mut files = fs::read_dir(deno_dir)?;
    let mut versions = Vec::new();
    while let Some(file) = files.next() {
        let dir = file?;
        if dir.path().is_dir() && dir.path().join("deno").exists() {
            let file_name = dir.file_name();
            let file_name = file_name.to_str().unwrap();
            versions.push(file_name.to_string());
        }
    }
    Ok(versions)
}

pub fn install(version: &str) -> anyhow::Result<()> {
    let deno_bin_path = get_deno_binary(version);
    if !deno_bin_path.exists() {
        download(version)?;
        unzip_deno(version)?;
    }
    Ok(())
}

pub fn download(version: &str) -> anyhow::Result<()> {
    let deno_version_dir = get_deno_home(version);
    std::fs::create_dir_all(&deno_version_dir)?;
    let temp_zip_file = deno_version_dir.join("deno.zip");
    let download_url = if cfg!(target_os = "macos") && cfg!(target_arch = "aarch64") {
        format!("https://github.com/denoland/deno/releases/download/v{}/deno-aarch64-apple-darwin.zip", version)
    } else if cfg!(target_os = "macos") {
        format!("https://github.com/denoland/deno/releases/download/v{}/deno-x86_64-apple-darwin.zip", version)
    } else if cfg!(target_os = "windows") {
        format!("https://github.com/denoland/deno/releases/download/v{}/deno-x86_64-pc-windows-msvc.zip", version)
    } else {
        format!("https://github.com/denoland/deno/releases/download/v{}/deno-x86_64-unknown-linux-gnu.zip", version)
    };
    let mut response = Client::builder()
        .build()?
        .get(download_url)
        .send()?;
    let mut zip_file = std::fs::File::create(&temp_zip_file)?;
    io::copy(&mut response, &mut zip_file)?;
    Ok(())
}

pub fn delete(version: &str) -> anyhow::Result<()> {
    let deno_version_dir = get_deno_home(version);
    if deno_version_dir.exists() {
        fs::remove_dir_all(&deno_version_dir)?;
    }
    Ok(())
}

pub fn unzip_deno(version: &str) -> anyhow::Result<()> {
    let deno_version_dir = get_deno_home(version);
    let deno_zip_file = deno_version_dir.join("deno.zip");
    //unzip zip_file to deno_version_dir
    let mut zip = zip::ZipArchive::new(File::open(deno_zip_file)?)?;
    for i in 0..zip.len() {
        let mut file = zip.by_index(i)?;
        let outpath = deno_version_dir.join(file.name());
        let outpath = match outpath.parent() {
            Some(p) => p,
            None => &deno_version_dir,
        };
        if !outpath.exists() {
            std::fs::create_dir_all(outpath)?;
        }
        let mut outfile = File::create(outpath.join(file.name()))?;
        io::copy(&mut file, &mut outfile)?;
        // Get and Set permissions
        #[cfg(any(unix, macos, linux))]
            {
                use std::os::unix::fs::PermissionsExt;
                outfile.set_permissions(fs::Permissions::from_mode(file.unix_mode().unwrap())).unwrap();
            }
    }
    Ok(())
}

mod tests {
    use super::*;

    #[test]
    fn test_deno_exists() {
        println!("{}", get_deno_binary("1.17.1").exists());
    }

    #[test]
    fn test_os_name() {
        let arch = std::env::consts::ARCH;
        let os_name = std::env::consts::OS;
        println!("arch: {}", arch);
        println!("os: {}", os_name);
    }

    #[test]
    fn test_deno_download() {
        download("1.17.1").unwrap();
    }

    #[test]
    fn test_unzip_deno() {
        unzip_deno("1.17.1").unwrap();
    }

    #[test]
    fn test_delete() {
        delete("1.17.1").unwrap();
    }

    #[test]
    fn test_install() {
        install("1.17.1").unwrap();
    }

    #[test]
    fn test_list() {
        for x in list().unwrap() {
            println!("{}", x);
        }
    }
}
