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

impl Clone for MinecraftVersionJson {
    fn clone(&self) -> Self {
        MinecraftVersionJson {
            r#type: self.r#type.clone(),
            time: self.time.clone(),
            url: self.url.clone(),
            releaseTime: self.releaseTime.clone(),
            id: self.id.clone()
        }
    }
}

#[cfg(test)]
mod tests {
    const VERSION_LIST_TEST_JSON: &str =
        r###"{
            "latest": {"release": "1.16.1", "snapshot": "20w30a"},
            "versions": [
                {
                    "id": "20w30a",
                    "type": "snapshot",
                    "url": "https://launchermeta.mojang.com/v1/packages/97b01440f8964c8eee644953ea9eecc3d83b2c64/20w30a.json",
                    "time": "2020-07-22T15:08:32+00:00",
                    "releaseTime": "2020-07-22T15:05:15+00:00"
                }
            ]
        }"###;

    #[test]
    fn test_monoversion_version_list() {
        let version_list: crate::download::version_list::MinecraftVersionListJson
            = serde_json::from_str(VERSION_LIST_TEST_JSON).unwrap();
        
        assert_eq!(version_list.versions.len(), 1);
        // version_list
        for version in version_list.versions.iter() {
            assert_eq!(version.id, "20w30a");
            assert_eq!(version.r#type, "snapshot");
            assert_eq!(version.url, "https://launchermeta.mojang.com/v1/packages/97b01440f8964c8eee644953ea9eecc3d83b2c64/20w30a.json");
            assert_eq!(version.time, "2020-07-22T15:08:32+00:00");
            assert_eq!(version.releaseTime, "2020-07-22T15:05:15+00:00");
        }
    }
}
