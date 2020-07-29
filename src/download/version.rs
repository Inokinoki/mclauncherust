use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct MinecraftVersionInfoDownloadsFileJson {
    pub sha1: String,
    pub size: u128,
    pub url: String,
}

#[derive(Deserialize, Debug)]
pub struct MinecraftVersionInfoFileJson {
    pub id: String,
    pub sha1: String,
    pub size: u128,
    pub url: String,
}


#[derive(Deserialize, Debug)]
pub struct MinecraftVersionInfoLibrariesDownloadsArtifactJson {
    pub path: String,
    pub sha1: String,
    pub size: u128,
    pub url: String,
}

#[derive(Deserialize, Debug)]
pub struct MinecraftVersionInfoLibrariesDownloadsJson {
    pub artifact: MinecraftVersionInfoLibrariesDownloadsArtifactJson,
}

#[derive(Deserialize, Debug)]
pub struct MinecraftVersionInfoLibrariesJson {
    pub downloads: MinecraftVersionInfoLibrariesDownloadsJson,
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct MinecraftVersionInfoLoggingClientJson {
    pub argument: String,
    pub file: MinecraftVersionInfoFileJson,
    pub r#type: String,
}

#[derive(Deserialize, Debug)]
pub struct MinecraftVersionInfoLoggingJson {
    pub client: MinecraftVersionInfoLoggingClientJson,
}

#[derive(Deserialize, Debug)]
pub struct MinecraftVersionInfoAssertIndexJson {
    pub id: String,
    pub sha1: String,
    pub size: u128,
    pub totalSize: u128,
    pub url: String,
}

#[derive(Deserialize, Debug)]
pub struct MinecraftVersionInfoDownloadsJson {
    pub client: MinecraftVersionInfoDownloadsFileJson,
    pub client_mappings: MinecraftVersionInfoDownloadsFileJson,
    pub server: MinecraftVersionInfoDownloadsFileJson,
    pub server_mappings: MinecraftVersionInfoDownloadsFileJson,
}

#[derive(Deserialize, Debug)]
pub struct MinecraftVersionInfoJson {
    pub arguments: MinecraftVersionInfoArgumentsArrayJson,
    pub assetIndex: MinecraftVersionInfoAssertIndexJson,
    pub assets: String,
    pub downloads: MinecraftVersionInfoDownloadsJson,
    pub id: String,
    pub libraries: Vec<MinecraftVersionInfoLibrariesJson>,
    pub logging: MinecraftVersionInfoLoggingJson,
    pub mainClass: String,
    pub minimumLauncherVersion: u64,
    pub releaseTime: String,
    pub time: String,
    pub r#type: String,
}


#[derive(Deserialize, Debug)]
pub struct MinecraftVersionInfoArgumentsJson {
    pub game: MinecraftVersionInfoArgumentsArrayJson,
    pub jvm: MinecraftVersionInfoArgumentsArrayJson,
}

#[derive(Deserialize, Debug)]
pub struct MinecraftVersionInfoArgumentsArrayRuleOSWithoutVersionJson {
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct MinecraftVersionInfoArgumentsArrayRuleOSWithVersionJson {
    pub name: String,
    pub version: String,
}

#[derive(Deserialize, Debug)]
pub struct MinecraftVersionInfoArgumentsArrayRuleOSWithArchJson {
    pub arch: String,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum MinecraftVersionInfoArgumentsArrayRuleOSJson {
    OSWithoutVersion(MinecraftVersionInfoArgumentsArrayRuleOSWithoutVersionJson),
    OSWithVersion(MinecraftVersionInfoArgumentsArrayRuleOSWithVersionJson),
    OSWithArch(MinecraftVersionInfoArgumentsArrayRuleOSWithArchJson),
}

#[derive(Deserialize, Debug)]
pub struct MinecraftVersionInfoArgumentsArrayRuleWithOSJson {
    pub action: String,
    pub os: MinecraftVersionInfoArgumentsArrayRuleOSJson,
}

#[derive(Deserialize, Debug)]
pub struct MinecraftVersionInfoArgumentsArrayRuleWithFeaturesJson {
    pub action: String,
    pub features: HashMap<String, bool>,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum MinecraftVersionInfoArgumentsArrayRuleJson {
    OS(MinecraftVersionInfoArgumentsArrayRuleWithOSJson),
    Features(MinecraftVersionInfoArgumentsArrayRuleWithFeaturesJson),
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum StringOrList {
    SimpleString(String),
    StringVector(Vec<String>),
}

#[derive(Deserialize, Debug)]
pub struct MinecraftVersionInfoArgumentsArrayGameRulesJson {
    pub rules: Vec<MinecraftVersionInfoArgumentsArrayRuleWithFeaturesJson>,
    pub value: StringOrList,
}

#[derive(Deserialize, Debug)]
pub struct MinecraftVersionInfoArgumentsArrayJVMRulesJson {
    pub rules: Vec<MinecraftVersionInfoArgumentsArrayRuleWithOSJson>,
    pub value: StringOrList,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum StringOrMinecraftVersionInfoArgumentsArrayGameRulesJson {
    SimpleString(String),
    Rules(MinecraftVersionInfoArgumentsArrayGameRulesJson),
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum StringOrMinecraftVersionInfoArgumentsArrayJVMRulesJson {
    SimpleString(String),
    Rules(MinecraftVersionInfoArgumentsArrayJVMRulesJson),
}

#[derive(Deserialize, Debug)]
pub struct MinecraftVersionInfoArgumentsArrayJson {
    pub game: Vec<StringOrMinecraftVersionInfoArgumentsArrayGameRulesJson>,
    pub jvm: Vec<StringOrMinecraftVersionInfoArgumentsArrayJVMRulesJson>,
}

#[cfg(test)]
mod tests {
    mod test_version_constant;

    #[test]
    fn test_single_version() {
        let version: crate::download::version::MinecraftVersionInfoJson
            = serde_json::from_str(test_version_constant::VERSION_TEST_JSON).unwrap();
    }
}
