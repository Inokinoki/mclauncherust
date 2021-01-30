use std::path::{ Path, PathBuf };
use std::{ env, fs };

use crate::launcher_config;
use crate::download::version_list::MinecraftVersionListJson;

use tokio::runtime::Runtime;

#[derive(Debug)]
pub struct MinecraftVersion {
    pub id: String,
    pub path: String,
    pub has_json: bool,
    pub has_jar: bool,
    // pub json_path: String,
    // pub jar_path: String,
}

#[derive(Debug)]
pub struct MinecraftInstance {
    base_path: String,

    runtime: tokio::runtime::Runtime,
}

impl MinecraftInstance {
    pub fn new() -> MinecraftInstance {
        MinecraftInstance {
            base_path: env::current_dir().unwrap().to_str().unwrap_or_else(|| { "" }).to_string(),

            runtime: Runtime::new().unwrap(),
        }
    }

    pub fn from(path: &str) -> MinecraftInstance {
        MinecraftInstance {
            base_path: path.to_string(),

            runtime: Runtime::new().unwrap(),
        }
    }

    pub fn existing_versions(&self) -> Vec<MinecraftVersion> {
        let mut installed_versions = Vec::new();

        if !self.has_versions_dir() {
            self.create_versions_dir();
        }

        let versions_dir = self.versions_dir();
        let versions_folder_path = Path::new(&versions_dir);
        for entry in fs::read_dir(&versions_folder_path).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_dir() {
                let mut has_json = false;
                let mut has_jar = false;

                // Check jar file
                let mut jar_file_name = entry.file_name();
                jar_file_name.push(".jar");
                let mut jar_file_path = path.with_file_name(entry.file_name());
                jar_file_path.push(jar_file_name);
                if jar_file_path.exists() && jar_file_path.is_file() {
                    has_jar = true;
                }

                // Check json file
                let mut json_file_name = entry.file_name();
                json_file_name.push(".json");
                let mut json_file_path = path.with_file_name(entry.file_name());
                json_file_path.push(json_file_name);
                if json_file_path.exists() && json_file_path.is_file() {
                    has_json = true;
                }

                let version = MinecraftVersion {
                    id: path.file_name().unwrap().to_str().unwrap().to_string(),
                    path: path.to_str().unwrap().to_string(),
                    has_json: has_json,
                    has_jar: has_jar,
                    // json_path: json_file_path.to_str().unwrap().to_string(),
                    // jar_path: jar_file_path.to_str().unwrap().to_string(),
                };
                installed_versions.push(version);
            }
        }
        installed_versions
    }

    pub fn path(&self) -> &Path {
        Path::new(&self.base_path)
    }

    pub fn download_version(&self, v: &MinecraftVersion) {
        if !v.has_json {
            // TODO: Download JSON
        }

        if !v.has_jar {
            // TODO: Download JAR
        }

        // TODO: Download assets, libraries, etc 
    }

    pub async fn download_manifest(&self) -> Result<MinecraftVersionListJson, Box<dyn std::error::Error>> {
        let resp = reqwest::get(launcher_config::URL_JSON_VERSION_LIST_INOKI)
            .await?
            .json::<MinecraftVersionListJson>()
            .await?;
        Ok(resp)
    }

    /* functions to detect dir */
    fn has_versions_dir(&self) -> bool {
        // TODO
        true
    }

    fn has_assets_dir(&self) -> bool {
        // TODO
        true
    }

    fn has_libraries_dir(&self) -> bool {
        // TODO
        true
    }

    fn has_mods_dir(&self) -> bool {
        // TODO
        true
    }

    /* functions to create dir */
    fn create_versions_dir(&self) {
        // TODO
    }

    fn create_assets_dir(&self) {
        // TODO
    }

    fn create_libraries_dir(&self) {
        // TODO
    }

    fn create_mods_dir(&self) {
        // TODO
    }

    fn create_version_dir(&self, v: &MinecraftVersion) {
        if !self.has_versions_dir() {
            self.create_versions_dir();
        }

        // TODO: create versions/id
    }

    fn versions_dir(&self) -> String {
        let versions_path: PathBuf = Path::new(&self.base_path).join("versions");
        versions_path.to_str().unwrap().to_string()
    }

    fn assets_dir(&self) -> String {
        let versions_path: PathBuf = Path::new(&self.base_path).join("assets");
        versions_path.to_str().unwrap().to_string()
    }

    fn libraries_dir(&self) -> String {
        let versions_path: PathBuf = Path::new(&self.base_path).join("libraries");
        versions_path.to_str().unwrap().to_string()
    }

    fn mods_dir(&self) -> String {
        let versions_path: PathBuf = Path::new(&self.base_path).join("mods");
        versions_path.to_str().unwrap().to_string()
    }
}
