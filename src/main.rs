use std::path::PathBuf;

use langquiz_rs::drivers::logging::init_logging_with_level;
use langquiz_rs::*;

use clap::arg;
use clap::value_parser;
use clap::{Arg, ArgAction, ArgMatches, Command, command};

use log::LevelFilter;

pub enum QuizStatus {
    Correct,
    Incorrect,
    Quit,
}

pub fn try_quiz(
    record: &[String],
    header: &[String],
    display_mode: &str,
    fill_mode: &str,
) -> QuizStatus {
    let display_mode_in_header = header.iter().any(|s| *s == display_mode);

    let fill_mode_in_header = header.iter().any(|s| *s == fill_mode);

    if !display_mode_in_header {
        panic!("Display mode {display_mode} was not found in the csv header {header:?}");
    }

    if !fill_mode_in_header {
        panic!("Fill mode {fill_mode} was not found in the csv header {header:?}");
    }

    let display_message = format!(
        "the corresponding {fill_mode} for {}",
        get_csv_record_column(record, header, display_mode).expect("Could not fetch record")
    );

    let opt_answer = drivers::user_prompt::read_str_or_quit(&display_message);

    match opt_answer {
        Some(answer) => {
            if answer
                != get_csv_record_column(record, header, fill_mode).expect("Could not fetch record")
            {
                QuizStatus::Incorrect
            } else {
                QuizStatus::Correct
            }
        }
        None => QuizStatus::Quit,
    }
}

fn app_quiz_vocab(matches: &ArgMatches) {
    let quiz_file_path = matches.get_one::<PathBuf>("quiz_file_path").unwrap();
    let display_mode = matches.get_one::<String>("display_mode").unwrap();
    let fill_mode = matches.get_one::<String>("fill_mode").unwrap();

    let csv_records = parse_csv_file(quiz_file_path)
        .unwrap_or_else(|e| panic!("Failed to parse vocab file csv: {e}"));

    loop {
        let random_record =
            get_random_csv_record(&csv_records).expect("Failed to get random record");

        loop {
            let quiz_status = try_quiz(random_record, &csv_records.header, display_mode, fill_mode);

            match quiz_status {
                QuizStatus::Correct => {
                    println!("That is correct!");
                    break;
                }
                QuizStatus::Incorrect => {
                    let retry = drivers::user_prompt::read_binary_choice_from_user("retry");

                    if !retry {
                        break;
                    }
                }
                QuizStatus::Quit => {
                    println!("Thank you for playing!");
                    return;
                }
            }
        }
    }
}

fn parse_args() -> ArgMatches {
    command!()
        .arg(
            Arg::new("verbose")
                .help("1: info, 2: debug, 3: trace")
                .short('v')
                .action(ArgAction::Count),
        )
        // .arg(
        //     arg!([sentence_file_path] "Csv with sentence in pinyin, pinyin_pitch, hanzi, english translation.")
        //         .required(true)
        //         .value_parser(value_parser!(PathBuf)),
        // )
        .subcommand(
            Command::new("quiz_convert")
                .about("Quiz on converting representations")
                .arg(
                    arg!([quiz_file_path] "Csv with Different columns to convert between")
                        .required(true)
                        .value_parser(value_parser!(PathBuf)),
                )
                .arg(arg!([display_mode] "Test displays words of the given column").required(true))
                .arg(
                    arg!([fill_mode] "User required to fill in valid vocab of the given column")
                        .required(true),
                ), // .arg(
                   //     Arg::new("regen")
                   //         .long("regen")
                   //         .help("Regenerates all intermediates")
                   //         .action(clap::ArgAction::SetTrue),
                   // )
        )
        .subcommand_required(true)
        .get_matches()
}

fn main() {
    let matches = parse_args();

    let verbose = matches.get_count("verbose");

    let level_filter = {
        if verbose == 0 {
            LevelFilter::Off
        } else if verbose == 1 {
            LevelFilter::Info
        } else if verbose == 2 {
            LevelFilter::Debug
        } else {
            LevelFilter::Trace
        }
    };

    init_logging_with_level(level_filter).expect("Failed to set logging");

    match matches.subcommand() {
        Some(("quiz_convert", sub_matches)) => {
            app_quiz_vocab(sub_matches);
        }

        _ => unreachable!(),
    }
}
