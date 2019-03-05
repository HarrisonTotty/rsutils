//! cat - concatenate files and print on the standard output

use clap::{crate_authors, crate_description, crate_version};
use std::io::Read;

/// Handles when multiple files were passed to the program.
fn handle_files(file_paths: Vec<String>, show_nonprinting: bool, show_ends: bool, show_tabs: bool, show_numbers: bool, nonblank_numbers: bool, squeeze_blank: bool) {
    for file_path in file_paths {
        if file_path != "-" {   
            if let Ok(contents) = std::fs::read_to_string(&file_path) {
                let mut line_number = 1;
                for line in contents.lines() {
                    if line.is_empty() {
                        if !squeeze_blank {
                            if show_numbers && !nonblank_numbers {
                                println!("{} {}", line_number, line);
                            }
                            else {
                                println!("");
                            }
                        }
                    }
                    else {
                        if show_numbers {
                            println!("{} {}", line_number, line);
                        }
                        else {
                            println!("{}", line);
                        }
                    }
                    line_number += 1;
                }
            }
            else {
                eprintln!("Unable to read file \"{}\"...", &file_path);
            }
        }
        else {
            let mut buffer = String::new();
            std::io::stdin().read_to_string(&mut buffer).expect("foo");
            let mut line_number = 1;
            for line in buffer.lines() {
                if line.is_empty() {
                    if !squeeze_blank {
                        if show_numbers && !nonblank_numbers {
                            println!("{} {}", line_number, line);
                        }
                        else {
                            println!("");
                        }
                    }
                }
                else {
                    if show_numbers {
                        println!("{} {}", line_number, line);
                    }
                    else {
                        println!("{}", line);
                    }
                }
                line_number += 1;
            }
        }
    }
}

/// Handles when no files are passed to the program.
fn handle_stdin() {
    
}

/// The main entry point of the program.
fn main() {
    // Parse command-line arguments.
    let args = parse_arguments();

    // Pull out all of the flags.
    let show_nonprinting = args.is_present("show_nonprinting") || args.is_present("show_all") || args.is_present("e") || args.is_present("t");
    let show_ends        = args.is_present("show_all") || args.is_present("e") || args.is_present("show_ends");
    let show_tabs        = args.is_present("show_all") || args.is_present("t") || args.is_present("show_tabs");
    let show_numbers     = args.is_present("number") || args.is_present("number_nonblank");
    let nonblank_numbers = args.is_present("number_nonblank");
    let squeeze_blank    = args.is_present("squeeze_blank");

    // Get an iterator of file paths.
    if args.is_present("files") {
        handle_files(args.values_of_lossy("files").unwrap(), show_nonprinting, show_ends, show_tabs, show_numbers, nonblank_numbers, squeeze_blank);
    }
    else {
        handle_stdin();
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
             .help("The files(s) to concatenate and print to stdout.")
             .multiple(true)
        )
        .arg(clap::Arg::with_name("show_all")
             .help("Equivalent to \"-vET\".")
             .long("show-all")
             .short("A")
        )
        .arg(clap::Arg::with_name("number_nonblank")
             .help("Number nonempty output lines. Overrides \"-n\".")
             .long("number-noblank")
             .short("b")
        )
        .arg(clap::Arg::with_name("e")
             .help("Equivalent to \"-vE\".")
             .short("e")
        )
        .arg(clap::Arg::with_name("show_ends")
             .help("Display \"$\" at the end of each line.")
             .long("show-ends")
             .short("E")
        )
        .arg(clap::Arg::with_name("number")
             .help("Number all output lines.")
             .long("number")
             .short("n")
        )
        .arg(clap::Arg::with_name("squeeze_blank")
             .help("Suppress repeated output lines.")
             .long("squeeze-blank")
             .short("s")
        )
        .arg(clap::Arg::with_name("t")
             .help("Equivalent to \"-vT\".")
             .short("t")
        )
        .arg(clap::Arg::with_name("show_tabs")
             .help("Display tab characters as \"^I\".")
             .long("show-tabs")
             .short("T")
        )
        .arg(clap::Arg::with_name("u")
             .help("(ignored)")
             .short("u")
        )
        .arg(clap::Arg::with_name("show_nonprinting")
             .help("Use \"^\" and \"M-\" notation, except for LFD and TAB.")
             .long("show-nonprinting")
             .short("v")
        );
    return argument_parser.get_matches();
}
