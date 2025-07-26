use serde::Deserialize;
use std::{collections::HashMap, env, path::PathBuf, process::Command};

type AppKeyMap = HashMap<String, String>;

pub struct AppState {
    pub config: Config,
    pub config_path: PathBuf,
    current_window_class: String,
}

#[derive(Deserialize)]
pub struct Config {
    #[serde(flatten)]
    pub apps: HashMap<String, AppKeyMap>,
}

impl AppState {
    pub fn new() -> AppState {
        let home = env::var("HOME").expect("HOME variable is not found");
        let config_path = PathBuf::from(home)
            .join(".config")
            .join("hyprkey")
            .join("config.toml");

        return AppState {
            config: toml::from_str::<Config>(
                std::fs::read_to_string(&config_path)
                    .as_deref()
                    .unwrap_or(""),
            )
            .expect("couldn't convert to toml"),
            config_path,
            current_window_class: String::new(),
        };
    }

    pub fn bind_current_window(&self) {
        if let Some(app_binds) = self
            .config
            .apps
            .iter()
            .find(|(app_name, ..)| **app_name == self.current_window_class)
        {
            for (from_key, to_key) in app_binds.1 {
                Command::new("hyprctl")
                    .arg("keyword")
                    .arg("bind")
                    .arg(format!("{from_key}, sendshortcut, {to_key}, active"))
                    .output()
                    // .status()
                    .expect("Failed to run script");
            }
        };
    }

    pub fn unbind_current_window(&self) {
        if let Some(app_binds) = self
            .config
            .apps
            .iter()
            .find(|(app_name, ..)| **app_name == self.current_window_class)
        {
            for (from_key, ..) in app_binds.1 {
                Command::new("hyprctl")
                    .arg("keyword")
                    .arg("unbind")
                    .arg(from_key)
                    .output()
                    // .status()
                    .expect("Failed to run script");
            }
        };
    }

    pub fn enter_window(&mut self, new_window_class: &str) {
        if new_window_class == self.current_window_class {
            return;
        }

        self.unbind_current_window();
        self.current_window_class = new_window_class.to_owned();
        self.bind_current_window();
    }

    pub fn reload_config(&mut self) {
        self.unbind_current_window();
        self.config = toml::from_str::<Config>(
            std::fs::read_to_string(&self.config_path)
                .as_deref()
                .unwrap_or(""),
        )
        .expect("couldn't convert to string");
        self.bind_current_window();
    }
}
