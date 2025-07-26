mod config;

use std::io::{BufRead, BufReader};
use std::os::unix::net::UnixStream;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};
use std::{env, thread};

use notify::event::ModifyKind;
use notify::{EventKind, Watcher};

use crate::config::AppState;

fn main() -> anyhow::Result<()> {
    let app_state = Arc::new(RwLock::new(AppState::new()));

    let runtime_dir = env::var("XDG_RUNTIME_DIR").expect("XDG_RUNTIME_DIR not set");
    let hypr_instance =
        env::var("HYPRLAND_INSTANCE_SIGNATURE").expect("HYPRLAND_INSTANCE_SIGNATURE not set");
    let socket_path = PathBuf::from(runtime_dir)
        .join("hypr")
        .join(hypr_instance)
        .join(".socket2.sock");

    let thread_app_state = Arc::clone(&app_state);

    let watch_thread = thread::spawn(move || -> anyhow::Result<()> {
        let (tx, rx) = std::sync::mpsc::channel();

        let mut watcher = notify::recommended_watcher(tx)?;

        watcher.watch(
            thread_app_state
                .read()
                .expect("Poisoned RWLock during read")
                .config_path
                .parent()
                .unwrap(),
            notify::RecursiveMode::NonRecursive,
        )?;

        for res in rx {
            match res {
                Ok(notify::Event { kind, .. }) => {
                    if let EventKind::Modify(ModifyKind::Data(_)) = kind {
                        let mut cfg = thread_app_state
                            .write()
                            .expect("Poisoned RwLock during write");

                        cfg.reload_config();
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

    for line in reader.lines() {
        match line {
            Ok(mut line) => {
                if line.starts_with("activewindow>>") {
                    line = line.replace("activewindow>>", "");
                    let window_class = line.split(",").next().unwrap();
                    let mut app_state = app_state.write().expect("Poisoned RwLock during write");
                    app_state.enter_window(window_class);
                }
            }
            Err(e) => {
                eprintln!("Error reading line: {e}");
                break;
            }
        }
    }

    let app_state = Arc::clone(&app_state);
    ctrlc::set_handler(move || {
        let app_state = app_state.read().expect("Poisoned RwLock during read");
        app_state.unbind_current_window();
    })
    .expect("Error setting Ctrl-C handler");

    watch_thread.join().unwrap()?;

    Ok(())
}
