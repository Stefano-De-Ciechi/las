/*
* author: Stefano De Ciechi
* purpose: create a cli utility to scan directories and files and print metadata (like creation, modification and last_access dates) to help decide if an entry is worth eliminating or not
* date: 2024-07-17
*/

use std::process::exit;
use std::time::Duration;
use walkdir::{DirEntry, WalkDir};
use clap::{App, Arg};

#[macro_use] extern crate prettytable;
use prettytable::Table;
use prettytable::format;

const SECONDS_IN_DAY: f64 = 86_400.0;

// converts Duration to days by converting it to f64 and dividing by 86_400
fn duration_to_days(duration: Duration) -> f64 {
    ( duration.as_secs_f64() / SECONDS_IN_DAY ).round()
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry.file_name()
         .to_str()
         .map(|s| s.starts_with("."))
         .unwrap_or(false)
}

fn main() {

    let args = App::new("las")
        .version("0.1")
        .about("LAS - Last Access Scanner\n inspect entries in a path to check for creation, modification and last access dates; useful to decide if files are unused by a long time and can be removed from your system")

        .arg(Arg::with_name("path")
            .help("root folder to begin scanning")
            .takes_value(true)
            .required(true)
        )

        .arg(Arg::with_name("max_depth")
            .help("limit the recursion level of the scanning")
            .takes_value(true)
            .short("m")
            .default_value("1")
        )

        .arg(Arg::with_name("skip_hidden")
            .help("skip hidden files and directories")
            .takes_value(true)
            .default_value("true")
            .short("s")
            .possible_values(&["0", "1", "f", "t", "false", "true"])
        )

        .get_matches();

    let path = match args.value_of("path") {
        Some(p) => p,
        None => {
            eprintln!("error with the specified path");
            exit(-1);
        }
    };

    let max_depth_str: &str = args.value_of("max_depth")
        .to_owned()
        .unwrap();

    let max_depth: usize = match max_depth_str.parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("can't parse max_depth into a valid unsigned int");
            exit(-1);
        }
    };

    let skip_hidden = match args.value_of("skip_hidden").unwrap() {
        "0" | "f" | "false" => false,
        "1" | "t" | "true" => true,
        _ => true
    };

    let walker = WalkDir::new(path)
        .max_depth(max_depth)
        .into_iter();

    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);

    table.set_titles(row!["entry-name", "created", "last modified", "last access"]);

    let entries: Box<dyn Iterator<Item = walkdir::DirEntry>> = match skip_hidden {
        false => Box::new(walker.filter_map(|e| e.ok())),

        true => Box::new(walker
            .filter_entry(|e| !is_hidden(e))
            .filter_map(|e| e.ok())),
    };

    for entry in entries {
        let name = entry.file_name()
            .to_str()
            .unwrap();

        let metadata = match entry.metadata() {
            Ok(m) => m,
            Err(_) => {
                eprintln!("can't retrieve metadata for the file {}", name);
                exit(-1);
            }
        };
        
        // Creation time
        let created = match metadata.created() {
            Ok(time) => {
                let ct = match time.elapsed() {
                    Ok(t) => t,
                    Err(_) => Duration::new(0, 0),
                };

                duration_to_days(ct)
            },
            Err(_) => -1.0,
        };

        // Last Modification time
        let modified = match metadata.modified() {
            Ok(time) => {
                let mt = match time.elapsed() {
                    Ok(t) => t,
                    Err(_) => Duration::new(0, 0),
                };

                duration_to_days(mt)
            },
            Err(_) => -1.0,
        };

        // Last Access time
        let last_access = match metadata.accessed() {
            Ok(time) => {
                let las = match time.elapsed() {
                    Ok(t) => t,
                    Err(_) => Duration::new(0, 0),
                };

                duration_to_days(las)
            },
            Err(_) => -1.0,
        };

        let name = format!("{}{}", "    ".repeat(entry.depth()), name);

        match metadata.is_dir() {
            true => table.add_row(row![FY => name, created, modified, last_access]),
            false => table.add_row(row![name, created, modified, last_access])
        };
    }

    table.printstd();
}

