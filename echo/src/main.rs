//! echo - display a line of text

use clap::{crate_authors, crate_description, crate_version};

fn main() {
    let args = parse_arguments();
    let include_newline = !args.is_present("no_newline");
    let to_stdout       = !args.is_present("stderr");
    match args.values_of_lossy("strings") {
        Some(vals) => {
            let full = vals.join(" ");
            if include_newline {
                if to_stdout {
                    println!("{}", full);
                }
                else {
                    eprintln!("{}", full);
                }
            }
            else {
                if to_stdout {
                    print!("{}", full);
                }
                else {
                    eprint!("{}", full);
                }
            }
        },
        _ => {
            if include_newline {
                if to_stdout {
                    println!("");
                }
                else {
                    eprintln!("");
                }
            }
            else {
                if to_stdout {
                    print!("");
                }
                else {
                    eprint!("")
                }
            }
        }
    }
}

/// Parses the command-line arguments, returning the collection of matches.
pub fn parse_arguments<'a>() -> clap::ArgMatches<'a> {
    let argument_parser = clap::App::new("echo")
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
        .arg(clap::Arg::with_name("strings")
             .help("The string(s) to write to stdout/stderr.")
             .multiple(true)
        )
        .arg(clap::Arg::with_name("no_newline")
             .help("Specifies that the program should not output the trailing newline character.")
             .long("no-newline")
             .short("n")
        )
        .arg(clap::Arg::with_name("stderr")
             .help("Specifies that the program should print to stderr instead of stdout.")
             .long("stderr")
             .short("e")
        );
    return argument_parser.get_matches();
}
