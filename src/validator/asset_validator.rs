use std::path::{ Path, PathBuf };
use std::fs;
use std::io;

use sha1::{ Sha1, Digest };
use hex;

use crate::download::assets::{ MinecraftAssetsJson, MinecraftAssetsObjectJson };


pub fn validate_asset(asset_path: &str, index: &MinecraftAssetsJson) -> bool {
    let mut all_ok = true;

    let asset_objects_path = Path::new(asset_path).join("objects");

    for (asset_name, object) in index.objects.iter() {
        let hash = &object.hash;
        let size = &object.size;
        
        if hash.len() < 2 {
            all_ok = false;
            break;
        }

        let mut dir = hash.to_string();
        dir.truncate(2);

        // Check file existence
        let mut object_path = asset_objects_path.clone();
        object_path.push(dir);
        object_path.push(hash);

        if !object_path.exists() || !object_path.is_file() {
            all_ok = false;
            break;
        }

        // Check file size
        let metadata = fs::metadata(&object_path).unwrap();
        if metadata.len() as u128 != object.size {
            all_ok = false;
            break;
        }

        // Check file hash sha1
        let mut file = fs::File::open(&object_path).unwrap();
        let mut hasher = Sha1::new();
        io::copy(&mut file, &mut hasher).unwrap();
        let calculated_hash = hasher.finalize();
        let object_hash = hex::encode(calculated_hash);

        if &object_hash != hash {
            all_ok = false;
            break;
        }
    }

    all_ok
}


#[cfg(test)]
mod tests {
    use crate::validator::asset_validator::validate_asset;
    use crate::download::assets::{ MinecraftAssetsJson, MinecraftAssetsObjectJson };

    #[test]
    fn test_asset_validator_all_ok() {
        let asset_list: MinecraftAssetsJson = serde_json::from_str(ASSET_STRING).unwrap();

        assert_eq!(validate_asset("asset_tests", &asset_list), true);
    }

    fn test_asset_validator_not_ok() {
        let asset_list: MinecraftAssetsJson = serde_json::from_str(ASSET_NOT_CONTAINED_STRING).unwrap();

        assert_eq!(validate_asset("asset_tests", &asset_list), false);
    }

    const ASSET_STRING: &str = r###"{
        "objects":{
            "icons/icon_16x16.png":{
                "hash":"bdf48ef6b5d0d23bbb02e17d04865216179f510a",
                "size":3665
            },
            "icons/icon_32x32.png":{
                "hash":"92750c5f93c312ba9ab413d546f32190c56d6f1f",
                "size":5362
            },
            "icons/minecraft.icns":{
                "hash":"991b421dfd401f115241601b2b373140a8d78572",
                "size":114786
            }
        }
    }"###;

    const ASSET_NOT_CONTAINED_STRING: &str = r###"{
        "objects":{
            "minecraft/icons/icon_16x16.png":{
                "hash":"bdf48ef6b5d0d23bbb02e17d04865216179f510a",
                "size":3665
            }
        }
    }"###;
}
