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

        if !self.has_versions() {
            self.create_versions();
        }

        let versions_folder_path: PathBuf = Path::new(&self.base_path).join("versions");
        {
            for entry in fs::read_dir(&versions_folder_path).unwrap() {
                let entry = entry.unwrap();
                let path = entry.path();
                if path.is_dir() {
                    installed_versions.push(MinecraftVersion {
                        id: path.file_name().unwrap().to_str().unwrap().to_string(),
                        path: path.to_str().unwrap().to_string(),
                        has_json: false,
                        has_jar: false,
                    });
                }
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
    fn has_versions(&self) -> bool {
        // TODO
        true
    }

    fn has_assets(&self) -> bool {
        // TODO
        true
    }

    fn has_libraries(&self) -> bool {
        // TODO
        true
    }

    fn has_mods(&self) -> bool {
        // TODO
        true
    }

    /* functions to create dir */
    fn create_versions(&self) {
        // TODO
    }

    fn create_assets(&self) {
        // TODO
    }

    fn create_libraries(&self) {
        // TODO
    }

    fn create_mods(&self) {
        // TODO
    }

    fn create_version_dir(&self, v: &MinecraftVersion) {
        if !self.has_versions() {
            self.create_versions();
        }

        // TODO: create versions/id
    }
}
