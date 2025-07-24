use std::io::{BufRead, BufReader};
use std::os::unix::net::UnixStream;
use std::path::PathBuf;
use std::process::Command;
use std::sync::{Arc, RwLock};
use std::{env, thread};

use notify::event::ModifyKind;
use notify::{EventKind, Watcher};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
struct Config {
    #[serde(flatten)]
    apps: HashMap<String, AppKeyMap>,
}

type AppKeyMap = HashMap<String, String>;

fn main() -> anyhow::Result<()> {
    let runtime_dir = env::var("XDG_RUNTIME_DIR").expect("XDG_RUNTIME_DIR not set");
    let hypr_instance =
        env::var("HYPRLAND_INSTANCE_SIGNATURE").expect("HYPRLAND_INSTANCE_SIGNATURE not set");

    let socket_path = PathBuf::from(runtime_dir)
        .join("hypr")
        .join(hypr_instance)
        .join(".socket2.sock");

    let home = env::var("HOME")?;
    let config_path = PathBuf::from(home)
        .join(".config")
        .join("hyprkey")
        .join("config.toml");

    let config = Arc::new(RwLock::new(toml::from_str::<Config>(
        std::fs::read_to_string(&config_path)
            .as_deref()
            .unwrap_or(""),
    )?));

    let thread_config = Arc::clone(&config);

    let watch_thread = thread::spawn(move || -> anyhow::Result<()> {
        let (tx, rx) = std::sync::mpsc::channel();

        let mut watcher = notify::recommended_watcher(tx)?;

        watcher.watch(
            config_path.parent().unwrap(),
            notify::RecursiveMode::NonRecursive,
        )?;

        for res in rx {
            match res {
                Ok(notify::Event { kind, .. }) => {
                    if let EventKind::Modify(ModifyKind::Data(_)) = kind {
                        let mut cfg = thread_config.write().expect("Poisoned RwLock during write");

                        *cfg = toml::from_str::<Config>(
                            std::fs::read_to_string(&config_path)
                                .as_deref()
                                .unwrap_or(""),
                        )?;
                    }
                }
                Err(e) => {
                    panic!("Error watching config file:\n{e}");
                }
            }
        }

        Ok(())
    });

    let stream = UnixStream::connect(&socket_path).expect("Failed to connect to Hyprland socket");
    let reader = BufReader::new(stream);
    let mut previous_class = String::from("");

    for line in reader.lines() {
        match line {
            Ok(mut line) => {
                if line.starts_with("activewindow>>") {
                    line = line.replace("activewindow>>", "");
                    let current_class = line.split(",").next().unwrap();

                    if current_class == previous_class {
                        continue;
                    };

                    let config = config.read().expect("Poisoned RwLock during read");
                    if config.apps.is_empty() {
                        continue;
                    }

                    for (app_name, binds) in config.apps.iter().filter(|(app_name, ..)| {
                        *app_name == current_class || **app_name == previous_class
                    }) {
                        for (from_key, to_key) in binds {
                            let binding = format!("{from_key}, sendshortcut, {to_key}, active");
                            let app_status = if *app_name == current_class {
                                "active"
                            } else {
                                "former"
                            };

                            Command::new("hyprctl")
                                .arg("keyword")
                                .arg(if app_status == "active" {
                                    "bind"
                                } else {
                                    "unbind"
                                })
                                .arg(if app_status == "active" {
                                    &binding
                                } else {
                                    from_key
                                })
                                .status()
                                .expect("Failed to run script");
                        }
                    }

                    previous_class = current_class.to_owned();
                }
            }
            Err(e) => {
                eprintln!("Error reading line: {e}");
                break;
            }
        }
    }

    watch_thread.join().unwrap()?;

    Ok(())
}
