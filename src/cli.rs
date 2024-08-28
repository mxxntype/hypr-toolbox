use crate::tools::Tool;
use clap::builder::styling::{AnsiColor, Effects, Styles};
use clap::Parser;

#[derive(Parser, Clone, Debug)]
#[command(version, author, about)]
#[command(styles(Styles::styled()
    .usage(AnsiColor::Magenta.on_default()  | Effects::BOLD)
    .header(AnsiColor::Magenta.on_default() | Effects::BOLD)))]
pub struct Options {
    /// Which tool to run this time.
    #[command(subcommand)]
    tool: Tool,

    /// Whether to enable debug logging of the tools' internal events.
    #[arg(long, short, default_value_t = false)]
    verbose: bool,
}
