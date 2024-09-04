mod cli;

use crate::cli::{Options, QueryType, Tool};
use clap::Parser;
use hypr_toolbox::profile::{self, Config};
use hypr_toolbox::query::{active_workspace, keyboard, workspaces};
use hyprland::event_listener::EventListener;
use serde_json::to_string_pretty as to_json;
use std::fs;

// With the "subscribe" behaviour, its often needed to create a "handler"
// for the desired event, call it once right away and reuse it in hyprland's
// [`EventListener`], so here's a macro to create and call one nicely.
macro_rules! create_and_call_handler {
    ($handler:expr) => {{
        let closure = $handler;
        closure();
        closure
    }};
}

fn main() {
    let _ =
        color_eyre::install().inspect_err(|error| eprintln!("Couldn't set up color-eyre: {error}"));
    let options = Options::parse();

    let mut event_listener = EventListener::new();
    match options.tool {
        Tool::Profile {} => {
            let config = match dirs::config_dir() {
                None => Config::default(),
                Some(mut xdg_config_dir) => {
                    xdg_config_dir.push("hypr_toolbox");
                    let _ = fs::create_dir_all(&xdg_config_dir);

                    let mut config_file_path = xdg_config_dir.clone();
                    config_file_path.push("profile.json");

                    if fs::exists(&config_file_path).is_ok_and(|file_exists| !file_exists) {
                        let default_config = Config::default();
                        let default_config = serde_json::to_string_pretty(&default_config).unwrap();
                        let _ = fs::write(&config_file_path, default_config).inspect_err(|error| {
                            eprintln!("Could not create default config file: {error:?}");
                        });
                    }

                    let config = fs::read_to_string(&config_file_path)
                        .inspect_err(|error| eprintln!("Could not read config file: {error:?}"))
                        .unwrap_or_default();
                    serde_json::from_str(&config)
                        .inspect_err(|error| eprintln!("Could not deserialize config: {error:?}"))
                        .unwrap_or_default()
                }
            };

            profile::setup_listener(&mut event_listener, config);
            event_listener.start_listener().unwrap();
        }

        Tool::Query {
            query_type,
            subscribe,
        } => {
            match query_type {
                QueryType::ActiveWindow => {
                    panic!("This version of the hyprland crate seems to panic inside the `Client::get_active()` function.");
                    // dbg!(&active_window::get());
                }

                QueryType::ActiveWorkspace => handle_active_workspace()
                    .subscribe(subscribe)
                    .event_listener(&mut event_listener)
                    .call(),

                QueryType::KeyboardLayout { name_pattern } => handle_keyboard_layout()
                    .subscribe(subscribe)
                    .name_pattern(name_pattern)
                    .event_listener(&mut event_listener)
                    .call(),

                QueryType::Workspaces { skip_missing } => handle_workspaces()
                    .subscribe(subscribe)
                    .skip_missing(skip_missing)
                    .event_listener(&mut event_listener)
                    .call(),
            };

            if subscribe {
                event_listener.start_listener().unwrap();
            }
        }
    }
}

#[bon::builder]
fn handle_active_workspace(subscribe: bool, event_listener: &mut EventListener) {
    println!("{}", &active_workspace::get().unwrap().id);

    if subscribe {
        event_listener.add_workspace_change_handler(|ws| println!("{ws}"));
        event_listener.add_active_monitor_change_handler(|ws| println!("{}", ws.workspace));
    }
}

#[bon::builder]
fn handle_keyboard_layout(
    name_pattern: String,
    subscribe: bool,
    event_listener: &mut EventListener,
) {
    let handler = create_and_call_handler!(move || {
        println!(
            "{}",
            to_json(&keyboard::get(name_pattern.as_str()).unwrap()).unwrap()
        );
    });

    if subscribe {
        event_listener.add_keyboard_layout_change_handler(move |_| handler());
    }
}

#[bon::builder]
fn handle_workspaces(skip_missing: bool, subscribe: bool, event_listener: &mut EventListener) {
    let handler = create_and_call_handler!(move || {
        println!(
            "{}",
            to_json(&workspaces::get(skip_missing).unwrap()).unwrap()
        );
    });

    if subscribe {
        event_listener.add_workspace_added_handler(move |_| handler());
    }
}
