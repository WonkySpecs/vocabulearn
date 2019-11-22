extern crate clap;

use clap::{Arg, App, ArgMatches, SubCommand};

pub const QUIZ: &str = "quiz";
pub const VOCAB: &str = "vocab";

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
        .arg(Arg::with_name("num-questions")
            .long("num-questions")
            .short("n")
            .default_value("10")
            .takes_value(true))
}

fn vocab_subcommand() -> App<'static, 'static> {
    SubCommand::with_name(VOCAB)
        .about("Manage vocab items")
        .subcommand(SubCommand::with_name("add")
            .arg(Arg::with_name("native")
                .required(true)
                .index(1))
            .arg(Arg::with_name("transliterated")
                .required(true)
                .index(2)))
}
