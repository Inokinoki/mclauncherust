use std::path::{ Path, PathBuf };
use std::{ env, fs };

use crate::launcher_config;
use crate::download::version_list::MinecraftVersionListJson;
use crate::download::version::MinecraftVersionInfoJson;

use tokio::runtime::Runtime;

mod manifest_loader;
mod version_loader;

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

    pub fn get_version(&self, version: &MinecraftVersion) -> Option<MinecraftVersionInfoJson> {
        if version.has_jar && version.has_json {
            let mut json_file_path = PathBuf::from(&version.path);
            json_file_path.push(format!("{}.json", version.id));

            let version_json_str = fs::read_to_string(json_file_path);

            match version_json_str {
                Ok(v) => {
                    let version_info: MinecraftVersionInfoJson
                        = serde_json::from_str(&v).unwrap();
                    return Some(version_info);
                }
                _ => {}
            }
        }
        // TODO: add more error handling
        None
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

    pub fn download_version(&mut self, v: &MinecraftVersion) {
        if !v.has_json {
            // Download JSON
            // let task = crate::loader::version_loader::download_version_async_impl();
            // match self.runtime.block_on(task) {
            //     Ok(version_info) => {}
            //     Err(e) => {}
            // }
        }

        if !v.has_jar {
            // TODO: Download JAR
        }

        // TODO: Download assets, libraries, etc 
    }

    pub fn download_manifest(&mut self) -> Option<MinecraftVersionListJson> {
        let task = manifest_loader::download_manifest_async_impl();
        match self.runtime.block_on(task) {
            Ok(manifest) => {
                Some(manifest)
            }
            Err(e) => { None }
        }
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
