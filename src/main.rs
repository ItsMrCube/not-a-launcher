use dialoguer::{theme::ColorfulTheme, Confirm};
use std::{fs, path::Path};
use uuid::Uuid;
mod config;
mod modrinth;

fn main() {
    let reconfig = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Do you want to configure?")
        .interact()
        .unwrap();

    if reconfig {
        config::reconfig();
    }

    let mut config = config::read();

    config
        .mod_urls
        .append(&mut modrinth::mod_to_url(config.mods, config.version));

    download(
        //
        config.mod_urls,
        &config.dir,
        "mods",
        "jar",
    );
    download(
        //
        config.resourcepack_urls,
        &config.dir,
        "resourcepacks",
        "zip",
    );
    download(
        //
        config.shaderpacks_urls,
        &config.dir,
        "shaderpacks",
        "zip",
    );
}

fn download(urls: Vec<String>, dir: &str, sub_dir: &str, ext: &str) {
    let path = Path::new(dir).join(sub_dir);

    fs::remove_dir_all(&path).ok();
    fs::create_dir(&path).ok();

    for url in urls {
        let response = reqwest::blocking::get(url).unwrap().bytes().unwrap();

        let file_name = format!("{}.{}", Uuid::new_v4(), ext);

        fs::write(path.join(file_name), response).unwrap();
    }
}
