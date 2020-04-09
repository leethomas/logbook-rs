use clap::{Arg, App, ArgMatches, value_t, values_t};

mod config;
mod entry;
mod logbook;

static APP_NAME: &str = "logbook";

fn main() {
    let matches = App::new(APP_NAME)
        .version("0.1.0")
        .author("Lee T. <leet944@gmail.com>")
        .about("Take daily timestamped notes")
        .arg(Arg::with_name("message")
            .short("m")
            .long("message")
            .value_name("TEXT")
            .help("The message to be recorded")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("tags")
            .short("t")
            .long("tag")
            .value_name("TEXT")
            .takes_value(true)
            .multiple(true)
            .help("Tag associated with this message. Pass this option for each tag you have."))
        .arg(Arg::with_name("utc_offset")
            .short("u")
            .long("utc_offset")
            .value_name("NUMBER")
            .allow_hyphen_values(true)
            .help("The UTC offset to use for this message's timestamp. Defaults to the current
            machine's offset."))
        .get_matches();

    match get_logbook_options(matches).and_then(logbook::Logbook::run) {
        Err(e) => println!("Error: {:?}", e),
        Ok(_) => println!("Successfully recorded message.")
    }
}

/// Transforms raw CLI inputs into options that the app can use.
fn get_logbook_options(cli_args: ArgMatches) -> Result<logbook::Options, String> {
    let utc_offset = value_t!(cli_args, "utc_offset", f32).ok();
    let tags = values_t!(cli_args, "tags", String).ok();
    // Unwrapping b/c this is required and should be enforced by clap before we
    // reach here.
    let message = value_t!(cli_args, "message", String).unwrap();

    logbook::Options::new(message, tags, utc_offset)
}
