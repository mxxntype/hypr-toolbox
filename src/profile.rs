use crate::config::ExternalConfig;
use hyprland::{event_listener::EventListener, keyword::Keyword, shared::WorkspaceType};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, process::Command};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Config {
    output: String,
    default_scale: f64,
    workspace_scale_map: HashMap<u8, f64>,
    xwayland_scaling_workspaces: Vec<u8>,
}

impl Default for Config {
    fn default() -> Self {
        let mut map = HashMap::new();
        let _ = map.insert(7, 2.0);
        let _ = map.insert(10, 1.0);

        Self {
            output: String::from("eDP-1"),
            default_scale: 1.5,
            workspace_scale_map: map,
            xwayland_scaling_workspaces: vec![7],
        }
    }
}

impl ExternalConfig for Config {
    type Config = Self;
    const FILENAME: &'static str = "profile.json";
}

pub fn setup_listener(event_listener: &mut EventListener, config: Config) {
    event_listener.add_workspace_change_handler(move |ws_type| match ws_type {
        WorkspaceType::Special(_) => { /* Do nothing I guess? */ }
        WorkspaceType::Regular(id_string) => {
            let id = id_string.parse::<u8>().unwrap_or(1);
            let scale = match config.workspace_scale_map.get(&id) {
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

            let scale_this_ws = config.xwayland_scaling_workspaces.contains(&id);
            let _ = Keyword::set("xwayland:force_zero_scaling", (!scale_this_ws).to_string())
                .inspect_err(|error| {
                    eprintln!("Couldn't set `xwayland:force_zero_scaling` keyword: {error:?}");
                });
        }
    });
}
