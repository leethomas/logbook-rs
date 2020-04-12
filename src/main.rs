use clap::{Arg, App, ArgMatches, SubCommand, value_t, values_t};

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
            .takes_value(true))
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
        .subcommand(SubCommand::with_name("read")
            .about("Read back your log entries")
            .arg(Arg::with_name("after")
                .long("after")
                .value_name("YYYY|YYYY-MM|YYYY-MM-DD")
                .help("Read all entries after this date"))
            .arg(Arg::with_name("before")
                .long("before")
                .value_name("YYYY|YYYY-MM|YYYY-MM-DD")
                .help("Read all entries after this date")))
        .get_matches();

    let result = match get_operation(&matches) {
        logbook::LogbookOperationKind::Read => run_reader(&matches),
        logbook::LogbookOperationKind::Write => run_writer(&matches)
    };

    if result.is_err() {
        println!("Error: {:?}", result.err().take());
    }
}

/// Transforms raw CLI inputs into options that the app can use.
fn parse_writer_args(cli_args: &ArgMatches)
    -> Result<logbook::writer::Options, String> {
    let utc_offset = value_t!(cli_args, "utc_offset", f32).ok();
    let tags = values_t!(cli_args, "tags", String).ok();
    let message = value_t!(cli_args, "message", String).ok();

    logbook::writer::Options::new(message, tags, utc_offset)
}

fn parse_reader_args(cli_args: &ArgMatches) ->
    Result<logbook::reader::Options, String> {
        let before = value_t!(cli_args, "before", String).ok();
        let after = value_t!(cli_args, "after", String).ok();

        logbook::reader::Options::new(before, after)
    }

fn get_operation(cli_args: &ArgMatches) -> logbook::LogbookOperationKind {
    if cli_args.is_present("read") {
        logbook::LogbookOperationKind::Read
    } else {
        logbook::LogbookOperationKind::Write
    }
}

fn run_writer(cli_args: &ArgMatches) -> Result<(), String> {
    let config = logbook::Logbook::app_config()?;
    let options = parse_writer_args(cli_args)?;

    logbook::writer::Writer::run(config, options)
}

fn run_reader(cli_args: &ArgMatches) -> Result<(), String> {
    println!("Reading is not yet implemented!");
    let options = parse_reader_args(cli_args)?;

    logbook::reader::Reader::run(options)
}