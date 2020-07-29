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

#[cfg(test)]
mod tests {
    const SINGLE_ASSETS_TEST_JSON: &str = r###"{
        "objects":{
            "icons/icon_16x16.png":{
                "hash":"bdf48ef6b5d0d23bbb02e17d04865216179f510a",
                "size":3665
            }
        }
    }"###;

    const MULTIPLE_ASSETS_TEST_JSON: &str = r###"{
        "objects":{
            "icons/icon_16x16.png":{
                "hash":"bdf48ef6b5d0d23bbb02e17d04865216179f510a",
                "size":3665
            },
            "icons/icon_32x32.png":{
                "hash":"92750c5f93c312ba9ab413d546f32190c56d6f1f",
                "size":5362
            }
        }
    }"###;

    #[test]
    fn test_single_asset() {
        let assets: crate::download::assets::MinecraftAssetsJson
            = serde_json::from_str(SINGLE_ASSETS_TEST_JSON).unwrap();
        
        assert_eq!(assets.objects.len(), 1);
        // single asset
        for (path, asset) in assets.objects {
            assert_eq!(path, "icons/icon_16x16.png");
            assert_eq!(asset.hash, "bdf48ef6b5d0d23bbb02e17d04865216179f510a");
            assert_eq!(asset.size, 3665);
        }
    }

    #[test]
    fn test_multiple_assets() {
        let assets: crate::download::assets::MinecraftAssetsJson
            = serde_json::from_str(MULTIPLE_ASSETS_TEST_JSON).unwrap();
        
        assert_eq!(assets.objects.len(), 2);
    }
}
