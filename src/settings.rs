use std::collections::HashMap;
use config::{Config, ConfigError, File, FileFormat, Value};
use xdg::BaseDirectories;

#[derive(Clone, Debug)]
pub struct GlobalSettings {
    pub color: String,
    pub focused_color: Option<String>,
    pub icon: String,
    pub size: String,
}

impl GlobalSettings {
    pub fn new(color: String, focused_color: Option<String>, icon: String, size: String) -> GlobalSettings {
        GlobalSettings { color, focused_color, icon, size }
    }

    pub fn from(mut config: HashMap<String, String>) -> GlobalSettings {
        let color = config.remove("color").unwrap();
        let focused_color = config.remove("focused_color");
        let icon = config.remove("icon").unwrap();
        let size = config.remove("size").unwrap();
        GlobalSettings { color, focused_color, icon, size }
    }
}

#[derive(Clone, Debug)]
pub struct BaseSettings {
    pub color: Option<String>,
    pub icon: Option<String>,
    pub size: Option<String>,
}

impl BaseSettings {
    pub fn new(mut config: HashMap<String, Value>) -> BaseSettings {
        let color = match config.remove("color") {
            Some(color) => match color.into_str() {
                Ok(color) => Some(color),
                Err(_e) => None,
            },
            None => None,
        };
        let icon = match config.remove("icon") {
            Some(icon) => match icon.into_str() {
                Ok(icon) => Some(icon),
                Err(_e) => None,
            },
            None => None,
        };
        let size = match config.remove("size") {
            Some(size) => match size.into_str() {
                Ok(size) => Some(size),
                Err(_e) => None,
            },
            None => None,
        };
        BaseSettings { color, icon, size }
    }
}

#[derive(Clone, Debug)]
pub struct TitleSettings {
    pub app_id: Option<Vec<String>>,
    pub base: BaseSettings,
}

impl TitleSettings {
    pub fn new(mut config: HashMap<String, Value>) -> TitleSettings {
        let app_id: Option<Vec<String>> = match config.remove("app_id") {
            Some(app_id) => {
                match app_id.try_into() {
                    Ok(app_id) => Some(app_id),
                    Err(_e) => None,
                }
            }
            None => None,
        };
        let base = BaseSettings::new(config);
        TitleSettings { app_id, base }
    }
}

#[derive(Clone, Debug)]
pub struct Settings {
    pub global: GlobalSettings,
    pub title: HashMap<String, TitleSettings>,
    pub app_id: HashMap<String, BaseSettings>,
}

static DEFAULT_SETTINGS: &str = include_str!(r"./config.toml");

pub fn get_settings() -> Result<Settings, ConfigError> {
    let xdg_dirs = BaseDirectories::with_prefix("swaycons").unwrap();
    let config_path = xdg_dirs
        .place_config_file("config.toml").unwrap();
    let mut settings = Config::new();
    settings.merge(File::from_str(DEFAULT_SETTINGS, FileFormat::Toml))?;
    settings.merge(File::from(config_path).required(false))?;
    let global_map: HashMap<String, String> = settings.get("global").unwrap();
    let global = GlobalSettings::from(global_map);
    let title_config: HashMap<String, HashMap<String, Value>> = settings.get("title").unwrap();
    let mut title = HashMap::new();
    for (key, value) in title_config {
        title.insert(key, TitleSettings::new(value));
    }
    let mut app_id = HashMap::new();
    let app_id_config: HashMap<String, HashMap<String, Value>> = settings.get("app_id").unwrap();
    for (key, value) in app_id_config {
        app_id.insert(key, BaseSettings::new(value));
    }
    Ok(Settings {
        global,
        title,
        app_id
    })
}
