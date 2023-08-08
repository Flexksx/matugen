use directories::ProjectDirs;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use color_eyre::{Help, Report};

use serde::{Deserialize, Serialize};

use super::arguments::Cli;
use crate::Template;

#[derive(Serialize, Deserialize, Debug)]
pub enum WallpaperTool {
    Swaybg,
    Swww,
    Nitrogen,
    Feh,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub reload_apps: Option<bool>,
    pub reload_apps_list: Option<Apps>,
    pub set_wallpaper: Option<bool>,
    pub wallpaper_tool: Option<WallpaperTool>,
    // TODO: Add a `Command` struct
    pub swww_options: Option<Vec<String>>,
    pub feh_options: Option<Vec<String>>,
    pub run_after: Option<Vec<Vec<String>>>,
    pub prefix: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Apps {
   pub kitty: Option<bool>,
   pub waybar: Option<bool>,
   pub gtk_theme: Option<bool>,
   pub dunst: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigFile {
    pub config: Config,
    pub templates: HashMap<String, Template>,
}

const ERROR_TEXT: &str = "Error reading config file, check https://github.com/InioX/Matugen#configuration for help";

impl ConfigFile {
    pub fn read(args: &Cli) -> Result<ConfigFile, Report> {
        match &args.config {
            Some(config_file) => Ok(Self::read_from_custom_path(config_file)?),
            None => Ok(Self::read_from_proj_path()?),
        }
    }

    fn read_from_custom_path(config_file: &PathBuf) -> Result<ConfigFile, Report> {
        let content: String = match fs::read_to_string(config_file) {
            Ok(content) => content,
            Err(e) => {
                return Err(Report::new(e).wrap_err("Could not find the provided config file."))
            }
        };

        match toml::from_str(&content) {
            Ok(res) => return Ok(res),
            Err(e) => return Err(Report::new(e)
            .suggestion(ERROR_TEXT))
        }
    }

    fn read_from_proj_path() -> Result<ConfigFile, Report> {
        if let Some(proj_dirs) = ProjectDirs::from("com", "InioX", "matugen") {
            let proj_dir = proj_dirs.config_dir();
            let config_file = PathBuf::from(proj_dir).join("config.toml");

            if !config_file.exists() {
                return Self::read_from_fallback_path();
            }

            let content: String = fs::read_to_string(config_file).unwrap();
            match toml::from_str(&content) {
                Ok(res) => return Ok(res),
                Err(e) => return Err(Report::new(e)
                .suggestion(ERROR_TEXT))
            }
        } else {
            Ok(Self::read_from_fallback_path()?)
        }
    }

    fn read_from_fallback_path() -> Result<ConfigFile, Report> {
        let content: String = String::from(
            r#"
            [config]
            [templates]
        "#,
        );
        Ok(toml::from_str(&content)?)
    }
}
