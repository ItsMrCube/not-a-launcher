use dialoguer::{theme::ColorfulTheme, Input};
use serde::{Deserialize, Serialize};
use std::{error::Error, fs};

pub fn reconfig() {
    let config = Config {
        dir: ask("Minecraft directory", "dir"),
        version: ask("Minecraft version", "version"),

        mods: ask_list("Mods", "mods"),
        mod_urls: ask_list("Mod URLs", "mod_urls"),

        resourcepacks: ask_list("Resourcepacks", "resourcepacks"),
        resourcepack_urls: ask_list("Resourcepack URLs", "resourcepack_urls"),

        shaderpacks: ask_list("Shaderpacks", "shaderpacks"),
        shaderpack_urls: ask_list("Shaderpack URLs", "shaderpack_urls"),
    };

    let toml = toml::to_string(&config).unwrap();

    fs::write("config/config.toml", toml).unwrap();
}

pub fn read() -> Result<Config, Box<dyn Error>> {
    let toml = fs::read_to_string("config/config.toml")?;

    toml::from_str(&toml).map_err(|e| e.into())
}

fn ask(prompt: &str, prop: &str) -> String {
    let default = match read() {
        Ok(config) => match prop {
            "dir" => config.dir,
            "version" => config.version,
            "mods" => config.mods.join(" "),
            "mod_urls" => config.mod_urls.join(" "),
            "resourcepacks" => config.resourcepacks.join(" "),
            "resourcepack_urls" => config.resourcepack_urls.join(" "),
            "shaderpacks" => config.shaderpacks.join(" "),
            "shaderpack_urls" => config.shaderpack_urls.join(" "),
            _ => panic!("Unknown property: {}", prop),
        },
        Err(_) => "".to_string(),
    };

    let show_default = default.len() > 0;

    Input::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .allow_empty(true)
        .default(default)
        .show_default(show_default)
        .interact()
        .unwrap()
}

fn ask_list(prompt: &str, prop: &str) -> Vec<String> {
    ask(prompt, prop)
        .split(" ")
        .map(|s| s.to_string())
        .filter(|s| s.len() > 0)
        .collect()
}

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub dir: String,
    pub version: String,

    pub mods: Vec<String>,
    pub mod_urls: Vec<String>,

    pub resourcepacks: Vec<String>,
    pub resourcepack_urls: Vec<String>,

    pub shaderpacks: Vec<String>,
    pub shaderpack_urls: Vec<String>,
}
