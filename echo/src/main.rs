//! echo - display a line of text

use clap::{crate_authors, crate_description, crate_version};
//use std::io::{self, Write};

fn main() {
    let args = parse_arguments();
    let include_newline = !args.is_present("no_newline");
    let enable_escapes  =  args.is_present("enable_escapes");
    match args.values_of_lossy("string") {
        Some(vals) => {
            let full = vals.join(" ");
            if include_newline {
                if enable_escapes {
                    println!("{}", full);
                    //writeln!(&mut io::stdout(), "{}", full);
                }
                else {
                    println!("{}", full);
                }
            }
            else {
                if enable_escapes {
                    print!("{}", full);
                    //write!(&mut io::stdout(), "{}", full);
                }
                else {
                    print!("{}", full);
                }
            }
        },
        _ => {
            if include_newline {
                println!("");
            }
            else {
                print!("");
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
        .arg(clap::Arg::with_name("string")
             .help("The string(s) to write to stdout.")
             .multiple(true)
        )
        .arg(clap::Arg::with_name("no_newline")
             .help("Do not output the trailing newline.")
             .short("n")
        )
        .arg(clap::Arg::with_name("enable_escapes")
             .help("Enable interpretation of backslash escapes.")
             .short("e")
        )
        .arg(clap::Arg::with_name("disable_escapes")
             .help("Disable interpretation of backslash escapes (default).")
             .short("E")
        );
    return argument_parser.get_matches();
}
