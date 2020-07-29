use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct MinecraftVersionJson {
    pub r#type: String,
    pub time: String,
    pub url: String,
    pub releaseTime: String,
    pub id: String,
}

#[derive(Deserialize, Debug)]
pub struct MinecraftVersionListJson {
    pub latest: HashMap<String, String>,
    pub versions: Vec<MinecraftVersionJson>,
}
