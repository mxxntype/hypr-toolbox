pub mod query;

use clap::Subcommand;

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
    /// TODO: Well, actually implement.
    /// - [ ] More conditions
    /// - [ ] Run arbitrary commands
    Profile {},

    /// Query the compositor for information, once or subscribe-style.
    ///
    /// TODO: Well, actually implement.
    Query {
        /// The type of query that should be performed.
        #[command(subcommand)]
        query_type: QueryType,

        /// Whether to await wait for subsequent changes or query once and exit.
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Subcommand)]
pub enum QueryType {
    ActiveWindow,

    ActiveWorkspace,

    KeyboardLayout,

    Workspaces {
        #[arg(long, default_value_t = false)]
        skip_missing: bool,
    },
}
