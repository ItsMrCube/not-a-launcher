use serde::Deserialize;
use std::fs;

fn main() {
    // Get config
    let slice = fs::read("config/mods.json").unwrap();
    let cfg: Config = serde_json::from_slice(slice.as_slice()).unwrap();

    // Empty dir
    fs::read_dir(&cfg.dir).unwrap().for_each(|entry| {
        fs::remove_file(entry.unwrap().path()).unwrap();
    });

    // Download mods
    for m in &cfg.mods {
        let url = format!("https://api.modrinth.com/v2/project/{}/version", m);

        let versions: Vec<Version> = reqwest::blocking::get(url).unwrap().json().unwrap();
        let version = &versions[0];

        let file_info = version.files.iter().find(|f| f.primary).unwrap();

        let file = reqwest::blocking::get(&file_info.url)
            .unwrap()
            .bytes()
            .unwrap();

        let path = format!("{}/{}", &cfg.dir, &file_info.filename);
        fs::write(path, file.to_vec().as_slice()).unwrap();

        println!("Downloaded {}", &version.name);
    }
}

#[derive(Deserialize)]
struct Config {
    mods: Vec<String>,
    dir: String,
}

#[derive(Deserialize, Debug)]
struct Version {
    name: String,
    files: Vec<File>,
}

#[derive(Deserialize, Debug)]
struct File {
    url: String,
    filename: String,
    primary: bool,
}
