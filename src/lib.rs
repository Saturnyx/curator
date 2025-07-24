use reqwest::blocking::get;
use serde::Deserialize;

pub fn search (key, data){
}

pub fn get_licenses (license_type){
    let api_url = "https://api.github.com/repos/spdx/license-list-data/git/trees/main:text";
    let client = reqwest::blocking::Client::new();
    let response = client
        .get(api_url)
        .header("User-Agent", "curator-app")
        .send()?
        .json::<ApiResponse>()?;

    let files = response
        .tree
        .into_iter()
        .filter(|item| item.item_type == "blob")
        .map(|item| item.path)
        .collect();

    Ok(files)
}