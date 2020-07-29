use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct MinecraftAssetsObjectJson {
    pub hash: String,
    pub size: u128,
}

#[derive(Deserialize, Debug)]
pub struct MinecraftAssetsJson {
    pub objects: HashMap<String, MinecraftAssetsObjectJson>,
}
