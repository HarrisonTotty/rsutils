//! cat - concatenate files and print on the standard output

use clap::{crate_authors, crate_description, crate_version};
use std::io::Read;

/// The main entry point of the program.
fn main() {
    // Parse command-line arguments.
    let args = parse_arguments();

    // Pull out all of the flags.
    let to_stdout = !args.is_present("stderr");
    let no_blank  =  args.is_present("no_blank");

    let mut contents = String::new();

    // Get an iterator of file paths.
    if args.is_present("files") {
        for path in args.values_of_lossy("files").unwrap() {
            if let Ok(fc) = std::fs::read_to_string(&path) {
                contents.push_str(&fc)
            }
        }
    }
    else {
        std::io::stdin().read_to_string(&mut contents).expect("Unable to read from stdin");
    }
    if !no_blank {
        if to_stdout {
            print!("{}", contents)
        }
        else {
            eprint!("{}", contents)
        }
    }
    else {
        for line in contents.lines() {
            if line.is_empty() {
                continue;
            }
            if to_stdout {
                println!("{}", line);
            }
            else {
                eprintln!("{}", line);
            }
        }
    }
}

/// Parses the command-line arguments, returning the collection of matches.
pub fn parse_arguments<'a>() -> clap::ArgMatches<'a> {
    let argument_parser = clap::App::new("cat")
        // General Information
        .about(crate_description!())
        .author(crate_authors!())
        .version(crate_version!())
        // General Parser Settings
        .settings(
            &[
                clap::AppSettings::ColoredHelp,
            ]
        )
        // Global Arguments
        .arg(clap::Arg::with_name("files")
             .help("The files(s) to concatenate and print to stdout. If no files are specified, the program will read from stdin.")
             .multiple(true)
        )
        .arg(clap::Arg::with_name("stderr")
             .help("Send the contents of the files to stderr instead of stdout.")
             .long("stderr")
             .short("e")
        )
        .arg(clap::Arg::with_name("no_blank")
             .help("Specifies that the program should not print lines that are empty or only contain whitespace.")
             .long("no-blank")
             .short("b")
        );
    return argument_parser.get_matches();
}
