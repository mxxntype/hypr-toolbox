use clap::Parser;

fn main() {
    let _ =
        color_eyre::install().inspect_err(|error| eprintln!("Couldn't set up color-eyre: {error}"));

    let options = hypr_toolbox::cli::Options::parse();

    dbg!(&options);
}
