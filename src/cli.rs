use clap::builder::styling::{AnsiColor, Effects, Styles};
use clap::{Parser, Subcommand};

#[derive(Parser, Clone, Debug)]
#[command(version, author, about)]
#[command(styles(Styles::styled()
    .usage(AnsiColor::Magenta.on_default()  | Effects::BOLD)
    .header(AnsiColor::Magenta.on_default() | Effects::BOLD)))]
pub struct Options {
    /// Which tool to run this time.
    #[command(subcommand)]
    pub tool: Tool,

    /// Whether to enable debug logging of the tools' internal events.
    #[arg(long, short, default_value_t = false)]
    pub verbose: bool,
}

/// The set of tools this crate provides.
#[derive(Subcommand, Debug, Clone, PartialEq, Eq)]
pub enum Tool {
    /// Detect user actions and adapt to them on-the-fly.
    ///
    /// This tool can be used for associating actions with a specific workspace.
    ///
    /// # Examples
    ///
    /// It can set the display scale to 2 (using `wlr-randr`) and set
    /// `xwayland:force_zero_scaling = true` when and only when you're
    /// on workspace #7.
    ///
    /// TODO:
    /// - [ ] More conditions
    /// - [ ] Run arbitrary commands
    Profile {},

    /// Query the compositor for information, once or subscribe-style.
    Query {
        /// The type of query that should be performed.
        #[command(subcommand)]
        query_type: QueryType,

        /// Whether to wait for subsequent changes or query once and exit.
        ///
        /// Without this flag, the tool will only check the check the state once,
        /// return it to the issuer and exit. If this flag will passed, changes
        /// in relevant state will be streamed back to the user, an the tool will
        /// not exit until told to.
        ///
        /// Most useful for listen-like data in status bars and stuff alike.
        #[arg(short, long, default_value_t = false)]
        subscribe: bool,
    },
}

/// Types of queries that the `query` tool can perform.
#[derive(Debug, Clone, PartialEq, Eq, Subcommand)]
pub enum QueryType {
    /// Get the currently focused window.
    ///
    /// WARN: Currently unusable because of an issue with the [`hyprland`] crate.
    ActiveWindow,

    /// Get the ID of the currently focused workspace.
    ActiveWorkspace,

    /// Get the current keyboard layout name.
    KeyboardLayout {
        /// A pattern that will be used to find a specific keyboard's
        /// layout; if none is passed, the function will look for a
        /// keyboard with the name "at-translated-set-2-keyboard",
        /// which seems to be the name for the main keyboard.
        #[arg(short('p'), long, default_value = "at-translated-set-2-keyboard")]
        name_pattern: String,
    },

    /// Get the list of workspaces the compositor holds.
    Workspaces {
        #[arg(long, default_value_t = false)]
        skip_missing: bool,
    },
}
