mod cli;

use bon::builder;
use clap::Parser;
use hypr_toolbox::config::ExternalConfig;
use hypr_toolbox::profile::{self, Config};
use hypr_toolbox::query::{active_workspace, keyboard, workspaces};
use hyprland::event_listener::EventListener;
use hyprland::shared::WorkspaceType;

use crate::cli::{Options, QueryType, Tool};

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

fn main() -> Result<(), color_eyre::eyre::Report> {
    let _ = color_eyre::install()
        .inspect_err(|error| eprintln!("Failed to set up color-eyre: {error}"));
    let options = Options::parse();
    let mut event_listener = EventListener::new();

    match options.tool {
        Tool::Profile {} => {
            let config = Config::setup_default_or_read_existing().unwrap_or_default();
            profile::setup_listener(&mut event_listener, config);
            event_listener.start_listener()?;
        }

        Tool::Query {
            query_type,
            subscribe,
        } => {
            match query_type {
                QueryType::ActiveWindow => {
                    unimplemented!("The hyprland crate seems to panic inside the `Client::get_active()` function.");
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
            }

            if subscribe {
                event_listener.start_listener()?;
            }
        }
    }

    Ok(())
}

#[builder]
fn handle_active_workspace(subscribe: bool, event_listener: &mut EventListener) {
    println!("{}", &active_workspace::get().unwrap().id);
    if subscribe {
        event_listener.add_workspace_changed_handler(|ws| println!("{}", ws.id));
        event_listener.add_active_monitor_changed_handler(|monitor_event| {
            let ws_id = monitor_event
                .workspace_name
                .and_then(|ws_type| match ws_type {
                    WorkspaceType::Regular(id) => Some(id),
                    WorkspaceType::Special(_) => None,
                });
            if let Some(id) = ws_id {
                println!("{id}");
            }
        });
    }
}

#[builder]
fn handle_keyboard_layout(
    name_pattern: String,
    subscribe: bool,
    event_listener: &mut EventListener,
) {
    let handler = create_and_call_handler!(move || {
        let kbd_layout_name = keyboard::get(name_pattern.as_str()).unwrap();
        let kbd_layout_json = serde_json::to_string_pretty(&kbd_layout_name).unwrap();
        println!("{kbd_layout_json}");
    });

    if subscribe {
        event_listener.add_layout_changed_handler(move |_| handler());
    }
}

#[builder]
fn handle_workspaces(skip_missing: bool, subscribe: bool, event_listener: &mut EventListener) {
    let handler = create_and_call_handler!(move || {
        let workspaces = workspaces::get(skip_missing).unwrap();
        let workspaces_json = serde_json::to_string_pretty(&workspaces).unwrap();
        println!("{workspaces_json}",);
    });

    if subscribe {
        event_listener.add_workspace_added_handler(move |_| handler());
    }
}
