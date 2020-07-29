use serde::{Serialize, Deserialize};
use std::collections::HashMap;

// This declaration will look for a file named `launcher_config.rs` or
// `launcher_config/mod.rs` and will insert its contents inside a module
// named `launcher_config` under this scope
mod launcher_config;


#[derive(Deserialize, Debug)]
struct MinecraftVersionInfoDownloadsFileJson {
    sha1: String,
    size: u128,
    url: String,
}

#[derive(Deserialize, Debug)]
struct MinecraftVersionInfoFileJson {
    id: String,
    sha1: String,
    size: u128,
    url: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Deserialize, Debug)]
struct MinecraftVersionJson {
    r#type: String,
    time: String,
    url: String,
    releaseTime: String,
    id: String,
}

#[derive(Deserialize, Debug)]
struct MinecraftVersionListJson {
    latest: HashMap<String, String>,
    versions: Vec<MinecraftVersionJson>,
}

#[derive(Deserialize, Debug)]
struct MinecraftVersionInfoLibrariesDownloadsArtifactJson {
    path: String,
    sha1: String,
    size: u128,
    url: String,
}

#[derive(Deserialize, Debug)]
struct MinecraftVersionInfoLibrariesDownloadsJson {
    artifact: MinecraftVersionInfoLibrariesDownloadsArtifactJson,
}

#[derive(Deserialize, Debug)]
struct MinecraftVersionInfoLibrariesJson {
    downloads: MinecraftVersionInfoLibrariesDownloadsJson,
    name: String,
}

#[derive(Deserialize, Debug)]
struct MinecraftVersionInfoLoggingClientJson {
    argument: String,
    file: MinecraftVersionInfoFileJson,
    r#type: String,
}

#[derive(Deserialize, Debug)]
struct MinecraftVersionInfoLoggingJson {
    client: MinecraftVersionInfoLoggingClientJson,
}

#[derive(Deserialize, Debug)]
struct MinecraftVersionInfoAssertIndexJson {
    id: String,
    sha1: String,
    size: u128,
    totalSize: u128,
    url: String,
}

#[derive(Deserialize, Debug)]
struct MinecraftVersionInfoDownloadsJson {
    client: MinecraftVersionInfoDownloadsFileJson,
    client_mappings: MinecraftVersionInfoDownloadsFileJson,
    server: MinecraftVersionInfoDownloadsFileJson,
    server_mappings: MinecraftVersionInfoDownloadsFileJson,
}

#[derive(Deserialize, Debug)]
struct MinecraftVersionInfoJson {
    arguments: MinecraftVersionInfoArgumentsArrayJson,
    assetIndex: MinecraftVersionInfoAssertIndexJson,
    assets: String,
    downloads: MinecraftVersionInfoDownloadsJson,
    id: String,
    libraries: Vec<MinecraftVersionInfoLibrariesJson>,
    logging: MinecraftVersionInfoLoggingJson,
    mainClass: String,
    minimumLauncherVersion: u64,
    releaseTime: String,
    time: String,
    r#type: String,
}


#[derive(Deserialize, Debug)]
struct MinecraftVersionInfoArgumentsJson {
    game: MinecraftVersionInfoArgumentsArrayJson,
    jvm: MinecraftVersionInfoArgumentsArrayJson,
}

#[derive(Deserialize, Debug)]
struct MinecraftVersionInfoArgumentsArrayRuleOSWithoutVersionJson {
    name: String,
}

#[derive(Deserialize, Debug)]
struct MinecraftVersionInfoArgumentsArrayRuleOSWithVersionJson {
    name: String,
    version: String,
}

#[derive(Deserialize, Debug)]
struct MinecraftVersionInfoArgumentsArrayRuleOSWithArchJson {
    arch: String,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum MinecraftVersionInfoArgumentsArrayRuleOSJson {
    OSWithoutVersion(MinecraftVersionInfoArgumentsArrayRuleOSWithoutVersionJson),
    OSWithVersion(MinecraftVersionInfoArgumentsArrayRuleOSWithVersionJson),
    OSWithArch(MinecraftVersionInfoArgumentsArrayRuleOSWithArchJson),
}

#[derive(Deserialize, Debug)]
struct MinecraftVersionInfoArgumentsArrayRuleWithOSJson {
    action: String,
    os: MinecraftVersionInfoArgumentsArrayRuleOSJson,
}

#[derive(Deserialize, Debug)]
struct MinecraftVersionInfoArgumentsArrayRuleWithFeaturesJson {
    action: String,
    features: HashMap<String, bool>,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum MinecraftVersionInfoArgumentsArrayRuleJson {
    OS(MinecraftVersionInfoArgumentsArrayRuleWithOSJson),
    Features(MinecraftVersionInfoArgumentsArrayRuleWithFeaturesJson),
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum StringOrList {
    SimpleString(String),
    StringVector(Vec<String>),
}

#[derive(Deserialize, Debug)]
struct MinecraftVersionInfoArgumentsArrayGameRulesJson {
    rules: Vec<MinecraftVersionInfoArgumentsArrayRuleWithFeaturesJson>,
    value: StringOrList,
}

#[derive(Deserialize, Debug)]
struct MinecraftVersionInfoArgumentsArrayJVMRulesJson {
    rules: Vec<MinecraftVersionInfoArgumentsArrayRuleWithOSJson>,
    value: StringOrList,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum StringOrMinecraftVersionInfoArgumentsArrayGameRulesJson {
    SimpleString(String),
    Rules(MinecraftVersionInfoArgumentsArrayGameRulesJson),
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum StringOrMinecraftVersionInfoArgumentsArrayJVMRulesJson {
    SimpleString(String),
    Rules(MinecraftVersionInfoArgumentsArrayJVMRulesJson),
}

#[derive(Deserialize, Debug)]
struct MinecraftVersionInfoArgumentsArrayJson {
    game: Vec<StringOrMinecraftVersionInfoArgumentsArrayGameRulesJson>,
    jvm: Vec<StringOrMinecraftVersionInfoArgumentsArrayJVMRulesJson>,
}

#[derive(Deserialize, Debug)]
struct MinecraftAssetsObjectJson {
    hash: String,
    size: u128,
}

#[derive(Deserialize, Debug)]
struct MinecraftAssetsJson {
    objects: HashMap<String, MinecraftAssetsObjectJson>,
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get(launcher_config::URL_JSON_VERSION_LIST_INOKI)
        .await?
        .json::<MinecraftVersionListJson>()
        .await?;
    println!("{:#?}", resp.versions.get(0).unwrap());
    let resp2 = reqwest::get(&resp.versions.get(0).unwrap().url)
        .await?
        .json::<MinecraftVersionInfoJson>()
        .await?;
    println!("{:#?}", resp2);
    let resp3 = reqwest::get("https://launchermeta.mojang.com/v1/packages/96dd0d11c96498cea7d82df597ee27d6b84df182/1.16.json")
        .await?
        .json::<MinecraftAssetsJson>()
        .await?;
    Ok(())
}

/*fn main() {
    let point = Point { x: 1, y: 2 };

    let serialized = serde_json::to_string(&point).unwrap();
    println!("serialized = {}", serialized);

    let deserialized: Point = serde_json::from_str(&serialized).unwrap();
    println!("deserialized = {:?}", deserialized);

    println!("version {} url {}", launcher_config::LAUNCHER_VERSION_SEMANTIC, launcher_config::URL_JSON_VERSION_LIST);

    get_versions();
}
*/
