extern crate clap;

use clap::{Arg, App, ArgMatches, SubCommand};

pub fn run_app() -> ArgMatches<'static> {
    App::new("Vocabulearn")
        .version("0.1")
        .author("Will Taylor")
        .subcommand(quiz_subcommand())
        .subcommand(vocab_subcommand())
        .get_matches()
}


fn quiz_subcommand() -> App<'static, 'static> {
    SubCommand::with_name("quiz")
        .about("Run a quiz")
        .arg(Arg::with_name("type")
            .long("type")
            .short("t")
            .possible_values(&["ntf", "ftn", "both"])
            .default_value("both")
            .takes_value(true))
}

fn vocab_subcommand() -> App<'static, 'static> {
    SubCommand::with_name("vocab")
        .about("Manage vocab items")
        .arg(Arg::with_name("add")
            .short("a")
            .long("add"))
}
