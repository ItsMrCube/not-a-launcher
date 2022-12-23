use std::{error::Error, fs, path::Path};

use serde::Deserialize;

fn main() -> Result<(), Box<dyn Error>> {
    let config: Config = {
        let file = fs::read("config/mods.json")?;
        serde_json::from_slice(file.as_slice())?
    };

    // Empty dir
    for dir in ["mods", "resourcepacks", "shaderpacks"] {
        let path = Path::new(&config.dir).join(dir);

        fs::remove_dir_all(&path).ok();

        fs::create_dir(&path).ok();
    }

    let mut i = 0;

    // Download mods
    for mod_name in &config.mods {
        let version_url = format!("https://api.modrinth.com/v2/project/{mod_name}/version");
        let versions: Vec<Version> = reqwest::blocking::get(&version_url)?.json()?;
        let url = &versions[0]
            .files
            .iter()
            .find(|v| v.primary)
            .map(|v| &v.url)
            .unwrap();

        write(format!("{}/mods/{}.jar", &config.dir, &i), &url)?;

        i += 1;
    }

    for url in &config.resourcepacks {
        write(format!("{}/resourcepacks/{}.zip", &config.dir, &i), &url)?;

        i += 1;
    }

    for url in &config.shaderpacks {
        write(format!("{}/shaderpacks/{}.zip", &config.dir, &i), &url)?;

        i += 1;
    }

    Ok(())
}

fn write(path: String, url: &str) -> Result<(), Box<dyn Error>> {
    let mut response = reqwest::blocking::get(url)?;

    let mut file = fs::File::create(path)?;

    std::io::copy(&mut response, &mut file)?;

    Ok(())
}

#[derive(Deserialize)]
struct Config {
    mods: Vec<String>,
    resourcepacks: Vec<String>,
    shaderpacks: Vec<String>,
    dir: String,
}

#[derive(Deserialize, Debug)]
struct Version {
    files: Vec<File>,
}

#[derive(Deserialize, Debug)]
struct File {
    url: String,
    primary: bool,
}
