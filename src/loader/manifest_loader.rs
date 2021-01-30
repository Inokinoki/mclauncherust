use crate::launcher_config;
use crate::download::version_list::MinecraftVersionListJson;

pub async fn download_manifest_async_impl() -> Result<MinecraftVersionListJson, Box<dyn std::error::Error>> {
    let resp = reqwest::get(launcher_config::URL_JSON_VERSION_LIST_INOKI)
        .await?
        .json::<MinecraftVersionListJson>()
        .await?;
    Ok(resp)
}
