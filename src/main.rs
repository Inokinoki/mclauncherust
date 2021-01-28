use serde::{Serialize, Deserialize};
use std::collections::HashMap;

// This declaration will look for a file named `launcher_config.rs` or
// `launcher_config/mod.rs` and will insert its contents inside a module
// named `launcher_config` under this scope
mod launcher_config;
mod download;
mod tuiapp;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = tuiapp::app::TUIApp::new();
    app.main_loop();

    /*
    let resp = reqwest::get(launcher_config::URL_JSON_VERSION_LIST_INOKI)
        .await?
        .json::<download::version_list::MinecraftVersionListJson>()
        .await?;
    println!("{:#?}", resp.versions.get(0).unwrap());
    let resp2 = reqwest::get(&resp.versions.get(0).unwrap().url)
        .await?
        .json::<download::version::MinecraftVersionInfoJson>()
        .await?;
    println!("{:#?}", resp2);
    let resp3 = reqwest::get("https://launchermeta.mojang.com/v1/packages/96dd0d11c96498cea7d82df597ee27d6b84df182/1.16.json")
        .await?
        .json::<download::assets::MinecraftAssetsJson>()
        .await?;
    */
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
