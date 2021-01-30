use crate::download::version::MinecraftVersionInfoJson;
use crate::download::version_list::MinecraftVersionJson;

use std::{
    path::Path,
    fs::File,
    io::prelude::*,
};

pub async fn download_version_async_impl(version: &MinecraftVersionJson)
    -> Result<MinecraftVersionInfoJson, Box<dyn std::error::Error>> {
    let resp = reqwest::get(&version.url)
        .await?
        .json::<MinecraftVersionInfoJson>()
        .await?;
    Ok(resp)
}

pub async fn download_version_jar_async_impl(url: &str, dst: &str)
    -> Result<(), Box<dyn std::error::Error>> {
    let mut res = reqwest::get(url).await?;
    let mut file = match File::create(dst) {
        Err(why) => panic!("couldn't create: {}", why),
        Ok(file) => file,
    };

    while let Some(chunk) = res.chunk().await? {
        match file.write_all(&chunk) {
            Err(why) => panic!("couldn't write: {}", why),
            Ok(_) => { /* TODO: display percentage */ },
        }
    }
    Ok(())
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
        
        let mut rt = tokio::runtime::Runtime::new().unwrap();
        for version in version_list.versions.iter() {
            assert_eq!(version.id, "20w30a");
            assert_eq!(version.r#type, "snapshot");
            assert_eq!(version.url, "https://launchermeta.mojang.com/v1/packages/97b01440f8964c8eee644953ea9eecc3d83b2c64/20w30a.json");
            assert_eq!(version.time, "2020-07-22T15:08:32+00:00");
            assert_eq!(version.releaseTime, "2020-07-22T15:05:15+00:00");

            let task = crate::loader::version_loader::download_version_async_impl(version);
            match rt.block_on(task) {
                Ok(version_info) => {
                    assert_eq!(version_info.id, version.id);
                }
                Err(e) => {}
            }
        }
    }
}
