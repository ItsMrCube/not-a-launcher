use serde::{Deserialize, Serialize};

pub fn mod_to_url(mod_names: Vec<String>, game_version: String) -> Vec<String> {
    let mut mod_urls = Vec::new();

    for mod_name in mod_names {
        let api_url = format!(
            "https://api.modrinth.com/v2/project/{mod_name}/version?game_versions=[\"{game_version}\"]"
        );

        let versions: Vec<Version> = reqwest::blocking::get(api_url).unwrap().json().unwrap();

        let version = versions.first().unwrap();

        let file = version.files.iter().find(|f| f.primary).unwrap();

        mod_urls.push(file.url.clone());
    }

    return mod_urls;
}

#[derive(Serialize, Deserialize)]
struct Version {
    files: Vec<File>,
}

#[derive(Serialize, Deserialize)]
struct File {
    url: String,
    primary: bool,
}
