use serde::{Deserialize, Serialize};

pub fn to_url(project_names: Vec<String>, game_version: &str) -> Vec<String> {
    let mut project_urls = Vec::new();

    for project_name in project_names {
        let api_url = format!(
            "https://api.modrinth.com/v2/project/{project_name}/version?game_versions=[\"{game_version}\"]"
        );

        let versions: Vec<Version> = reqwest::blocking::get(api_url).unwrap().json().unwrap();

        let version = versions.first().unwrap();

        let file = version
            .files
            .iter()
            .find(|f| f.primary)
            .unwrap_or(version.files.first().unwrap());

        project_urls.push(file.url.clone());
    }

    return project_urls;
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
