use clap::Parser;
use hypr_toolbox::cli::Options;
use hypr_toolbox::tools::{query::workspaces, QueryType, Tool};
use hyprland::event_listener::EventListener;
use serde_json::to_string_pretty as to_json;

macro_rules! create_and_call_handler {
    ($name:ident, $closure_definition:expr) => {{
        let closure = $closure_definition;
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
        Tool::Profile {} => todo!(),

        Tool::Query {
            query_type,
            subscribe,
        } => match query_type {
            QueryType::ActiveWindow => todo!(),

            QueryType::ActiveWorkspace => todo!(),

            QueryType::KeyboardLayout => todo!(),

            QueryType::Workspaces { skip_missing } => {
                let handler = create_and_call_handler!(handler, move || {
                    println!(
                        "{}",
                        to_json(&workspaces::get(skip_missing).unwrap()).unwrap()
                    );
                });

                if subscribe {
                    event_listener.add_workspace_added_handler(move |_| handler());
                    event_listener.start_listener().unwrap();
                }
            }
        },
    }
}
