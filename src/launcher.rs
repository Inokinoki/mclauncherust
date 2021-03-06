
use crate::args_generator::ArgsGenerator;
use crate::download::version::MinecraftVersionInfoJson;

use crate::tuiapp::TUIApp;

use which::which;
use std::process::Command;
use std::path::{ Path, PathBuf };

pub struct Launcher {
    args_generator: ArgsGenerator,
    java_path: Option<PathBuf>,
    app: TUIApp,

    // TODO: auth
    // TODO: validator ?
    // TODO: instance ?
}

impl Launcher {
    pub fn new(app: TUIApp) -> Launcher {
        let mut generator = ArgsGenerator::new();
        generator.add_env("user_type", "mojang");               // --userType

        generator.add_env("version_name", "Minecheruster-0.1"); // --version
        generator.add_env("launcher_name", "Minecheruster");    // -Dminecraft.launcher.brand
        generator.add_env("launcher_name", "0.1");              // -Dminecraft.launcher.version

        generator.add_env("assets_root", "");                   // TODO: get from instance --assetsDir
        generator.add_env("game_directory", "Inokinoki");       // TODO: get from instance --gameDir
        generator.add_env("natives_directory", "");             // TODO: get from instance -Djava.library.path
        generator.add_env("classpath", "");                     // TODO: get from instance -cp

        generator.add_env("path", "");                          // TODO: get from instance -Dlog4j.configurationFile

        let java_path = which::which("java");

        Launcher {
            args_generator: generator,
            java_path: match java_path {
                Ok(path) => Some(path),
                Err(e) => None,
            },
            app: app,
        }
    }

    // Load a version
    pub fn load(&mut self, version: &MinecraftVersionInfoJson) {
        self.args_generator.add_env("auth_player_name", "Inokinoki");           // TODO: get from auth info --username        
        self.args_generator.add_env("auth_uuid", "Inokinoki");                  // --uuid
        self.args_generator.add_env("auth_access_token", "Inokinoki");          // --accessToken

        self.args_generator.add_env("assets_index_name", &version.assets);      // --assetIndex
        self.args_generator.add_env("version_type", &version.r#type);           // --versionType

        // TODO: mainClass ?
    }

    /* 
    user env:
        - resolution_width
        - resolution_height
    */

    pub fn configure(&mut self) -> bool {
        let version = self.app.main_loop();
        match version {
            Some(v) => {
                self.load(&v);
                return true;
            },
            None => {
                return true;
            },
        }
        false
    }

    pub fn launch(&self) {
    }
}
