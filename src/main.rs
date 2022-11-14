use serde::Deserialize;
use std::fs;

fn main() {
    // Get config
    let config: Config = {
        let slice = fs::read("config/mods.json").unwrap();
        serde_json::from_slice(slice.as_slice()).unwrap()
    };

    // Empty dir
    {
        fs::read_dir(&config.dir).unwrap().for_each(|entry| {
            fs::remove_file(entry.unwrap().path()).unwrap();
        });
    }

    let mut mods = vec![];

    // Fetch mods
    for m in &config.mods {
        let version_url = format!("https://api.modrinth.com/v2/project/{}/version", m);
        let versions: Vec<Version> = reqwest::blocking::get(&version_url)
            .unwrap()
            .json()
            .unwrap();

        let version = &versions[0];

        let file_info = &version.files.iter().find(|f| f.primary).unwrap();

        mods.push(reqwest::blocking::get(&file_info.url).unwrap());
    }

    // Fetch raw mods
    for m in &config.raw_mods {
        mods.push(reqwest::blocking::get(m).unwrap());
    }

    // Write mods
    let mut i = 0;
    for m in mods {
        let path = format!("{}/{}.jar", &config.dir, &i.to_string());

        fs::write(&path, &m.bytes().unwrap().to_vec().as_slice()).unwrap();

        i += 1;
    }

    println!("Downloaded {} mods.", i);
}

#[derive(Deserialize)]
struct Config {
    mods: Vec<String>,
    raw_mods: Vec<String>,
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
