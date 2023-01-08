use dialoguer::{theme::ColorfulTheme, Input};
use serde::{Deserialize, Serialize};
use std::fs;

pub fn reconfig() {
    let config = Config {
        dir: ask("Minecraft directory"),
        version: ask("Minecraft version"),

        mods: ask_list("Mods"),
        mod_urls: ask_list("Mod URLs"),
        resourcepack_urls: ask_list("Resourcepack URLs"),
        shaderpacks_urls: ask_list("Shaderpack URLs"),
    };

    let toml = toml::to_string(&config).unwrap();

    fs::write("config/config.toml", toml).unwrap();
}

pub fn read() -> Config {
    let toml = fs::read_to_string("config/config.toml").unwrap();

    toml::from_str(&toml).unwrap()
}

fn ask(prompt: &str) -> String {
    Input::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .allow_empty(true)
        .interact()
        .unwrap()
}

fn ask_list(prompt: &str) -> Vec<String> {
    ask(prompt)
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
    pub resourcepack_urls: Vec<String>,
    pub shaderpacks_urls: Vec<String>,
}
