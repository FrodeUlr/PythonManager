use config::{Config, File, FileFormat};
use once_cell::sync::Lazy;
use std::{path::Path, sync::Mutex};

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Settings {
    #[serde(default = "default_venv_path")]
    pub venvs_path: String,
    pub default_pkgs: Vec<String>,
}

fn default_venv_path() -> String {
    String::from("~/pymngr/venvs")
}

static SETTINGS: Lazy<Mutex<Settings>> = Lazy::new(|| Mutex::new(Settings::default()));

impl Default for Settings {
    fn default() -> Self {
        Settings {
            venvs_path: default_venv_path(),
            default_pkgs: vec![],
        }
    }
}

impl Settings {
    pub async fn init() {
        let settings = Config::builder()
            .add_source(File::with_name("settings").format(FileFormat::Toml))
            .build()
            .unwrap_or_else(|_| {
                println!("Settings.toml missing or invalid");
                Config::default()
            });

        // Use the config to load the settings
        let new_settings: Settings = settings
            .try_deserialize()
            .unwrap_or_else(|_| Settings::default());

        // Validate the venv Path
        new_settings.validate_venv_path();

        let mut settings_lock = SETTINGS.lock().expect("Failed to lock settings");
        *settings_lock = new_settings;
    }

    pub fn get_settings() -> Settings {
        let settings_lock = SETTINGS.lock().expect("Failed to lock settings");
        settings_lock.clone()
    }

    fn validate_venv_path(&self) {
        let mut path = self.venvs_path.clone();
        if path.starts_with("~") {
            path = shellexpand::tilde(&path).to_string();
        }
        if !Path::new(&path).exists() {
            println!("Creating venvs folder: {}", path);
            std::fs::create_dir_all(&path).expect("Failed to create venvs folder");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_venv_path() {
        let settings = Settings::default();
        assert_eq!(settings.venvs_path, "~/pymngr/venvs");
    }

    #[test]
    fn test_validate_venv_path() {
        let settings = Settings {
            venvs_path: "~/pymngr/venvs".to_string(),
        };
        settings.validate_venv_path();
        let expected_path = shellexpand::tilde("~/pymngr/venvs").to_string();
        assert!(Path::new(&expected_path).exists());
    }

    #[test]
    fn test_get_settings() {
        let settings = Settings {
            venvs_path: "~/pymngr/venvs".to_string(),
        };
        let settings_lock = Mutex::new(settings);
        let settings = settings_lock.lock().unwrap();
        assert_eq!(settings.venvs_path, "~/pymngr/venvs");
    }
}
