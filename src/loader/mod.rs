use std::path::{ Path, PathBuf };
use std::{ env, fs };

#[derive(Debug)]
pub struct MinecraftVersion {
    pub id: String,
    pub path: String,
    pub has_json: bool,
    pub has_jar: bool,
}

#[derive(Debug)]
pub struct MinecraftInstance {
    base_path: String,
}

impl MinecraftInstance {
    pub fn new() -> MinecraftInstance {
        MinecraftInstance {
            base_path: env::current_dir().unwrap().to_str().unwrap_or_else(|| { "" }).to_string(),
        }
    }

    pub fn from(path: &str) -> MinecraftInstance {
        MinecraftInstance {
            base_path: path.to_string(),
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
                let version = MinecraftVersion {
                    id: path.file_name().unwrap().to_str().unwrap().to_string(),
                    path: path.to_str().unwrap().to_string(),
                    has_json: false,
                    has_jar: false,
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
