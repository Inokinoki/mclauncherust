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
    pub artifact: Option<MinecraftVersionInfoLibrariesDownloadsArtifactJson>,
    pub classifiers: Option<HashMap<String, MinecraftVersionInfoLibrariesDownloadsArtifactJson>>,
}

#[derive(Deserialize, Debug)]
pub struct MinecraftVersionInfoLibrariesExtractJson {
    exclude: Option<Vec<String>>,
}

#[derive(Deserialize, Debug)]
pub struct MinecraftVersionInfoLibrariesJson {
    pub downloads: MinecraftVersionInfoLibrariesDownloadsJson,
    pub name: String,
    pub extract: Option<MinecraftVersionInfoLibrariesExtractJson>,
    pub natives: Option<HashMap<String, String>>,
    pub rules: Option<Vec<MinecraftVersionInfoArgumentsArrayRuleWithOSJson>>,
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

/*
// Fix missing client_mappings
#[derive(Deserialize, Debug)]
pub struct MinecraftVersionInfoDownloadsJson {
    pub client: MinecraftVersionInfoDownloadsFileJson,
    pub client_mappings: MinecraftVersionInfoDownloadsFileJson,
    pub server: MinecraftVersionInfoDownloadsFileJson,
    pub server_mappings: MinecraftVersionInfoDownloadsFileJson,
}
*/

#[derive(Deserialize, Debug)]
pub struct MinecraftVersionInfoJson {
    pub arguments: Option<MinecraftVersionInfoArgumentsArrayJson>,
    pub assetIndex: MinecraftVersionInfoAssertIndexJson,
    pub assets: String,
    pub downloads: HashMap<String, MinecraftVersionInfoDownloadsFileJson>,
    pub id: String,
    pub libraries: Vec<MinecraftVersionInfoLibrariesJson>,
    pub logging: Option<MinecraftVersionInfoLoggingJson>,
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
    pub os: Option<MinecraftVersionInfoArgumentsArrayRuleOSJson>,
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

    #[test]
    #[ignore]
    fn test_all_versions() {
        let start = 0;
        let mut counter = start + 1;

        let version_list_response = reqwest::blocking::get(crate::launcher_config::URL_JSON_VERSION_LIST_INOKI)
            .unwrap().json::<crate::download::version_list::MinecraftVersionListJson>().unwrap();

        for version in version_list_response.versions[start..].iter() {
            println!("{}. Testing {}", counter, &version.url);
            let version_response = reqwest::blocking::get(&version.url)
                .unwrap().json::<crate::download::version::MinecraftVersionInfoJson>().unwrap();
            counter = counter + 1;
        }
        
        /*
        // Uncomment this if you want to test a single case
        let version_response = reqwest::blocking::get("https://launchermeta.mojang.com/v1/packages/551af51dcb3c047908f4a175233436f32b56b1c7/14w27b.json")
                .unwrap().json::<crate::download::version::MinecraftVersionInfoJson>().unwrap();
        */
    }
}
