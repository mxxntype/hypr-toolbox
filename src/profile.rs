use hyprland::{event_listener::EventListener, shared::WorkspaceType};
use std::{collections::HashMap, process::Command};

pub struct Config {
    output: String,
    default_scale: f64,
    workspace_map: HashMap<u8, f64>,
}

impl Default for Config {
    fn default() -> Self {
        let mut workspace_map = HashMap::new();
        let _ = workspace_map.insert(7, 2.0);
        let _ = workspace_map.insert(10, 1.0);

        Self {
            output: String::from("eDP-1"),
            workspace_map,
            default_scale: 1.5,
        }
    }
}

pub fn setup_listener(event_listener: &mut EventListener, config: Config) {
    event_listener.add_workspace_change_handler(move |ws_type| match ws_type {
        WorkspaceType::Special(_) => { /* Do nothing I guess? */ }
        WorkspaceType::Regular(id_string) => {
            let id = id_string.parse::<u8>().unwrap_or(1);
            let scale = match config.workspace_map.get(&id) {
                Some(scale) => *scale,
                None => config.default_scale,
            };

            let _ = Command::new("wlr-randr")
                .arg("--output")
                .arg(config.output.clone())
                .arg("--scale")
                .arg(scale.to_string())
                .output()
                .inspect_err(|error| eprintln!("Couldn't set display scale: {error:?}"));
        }
    });
}
