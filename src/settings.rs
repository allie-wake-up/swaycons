use config::{Config, ConfigError, File, FileFormat};
use xdg::BaseDirectories;

static DEFAULT_SETTINGS: &str = include_str!(r"./config.toml");

pub fn get_settings() -> Result<Config, ConfigError> {
    let xdg_dirs = BaseDirectories::with_prefix("swaycons").unwrap();
    let config_path = xdg_dirs
        .place_config_file("config.toml")
        .expect("cannot create configuration directory");
    let mut settings = Config::new();
    settings.merge(File::from_str(DEFAULT_SETTINGS, FileFormat::Toml))?;
    settings.merge(File::from(config_path).required(false))?;
    Ok(settings)
}
