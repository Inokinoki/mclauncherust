use std::path::{ Path, PathBuf };

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
    pub fn from(path: &str) -> MinecraftInstance {
        MinecraftInstance {
            base_path: path.to_string(),
        }
    }

    pub fn existing_versions(&self) -> Vec<MinecraftVersion> {
        vec![]
    }

    pub fn path(&self) -> &Path {
        Path::new(&self.base_path)
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
}
