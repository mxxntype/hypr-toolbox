use clap::Parser;
use hypr_toolbox::cli::{Options, QueryType, Tool};
use hypr_toolbox::query::{keyboard, workspaces};
use hyprland::event_listener::EventListener;
use serde_json::to_string_pretty as to_json;

macro_rules! create_and_call_handler {
    ($closure_definition:expr) => {{
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
        } => {
            match query_type {
                QueryType::ActiveWindow => todo!(),

                QueryType::ActiveWorkspace => todo!(),

                QueryType::KeyboardLayout { name_pattern } => {
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

                QueryType::Workspaces { skip_missing } => {
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
            };

            if subscribe {
                event_listener.start_listener().unwrap();
            }
        }
    }
}
