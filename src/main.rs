/*
* author: Stefano De Ciechi
* purpose: create a cli utility to scan directories and files and print metadata (like creation, modification and last_access dates) to help decide if an entry is worth eliminating or not
* date: 2024-07-17
*/

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

    // TODO update to latest version of clap
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
            .long("max_depth")
            .default_value("1")
        )

        .arg(Arg::with_name("skip_hidden")
            .help("skip hidden files and directories")
            .takes_value(true)
            .default_value("true")
            .short("s")
            .long("skip_hidden")
            .possible_values(&["0", "1", "f", "t", "false", "true"])
        )

        .get_matches();

    let path = args.value_of("path").unwrap();

    // TODO check for correct parse result
    let max_depth: usize = args.value_of("max_depth")
        .to_owned()
        .unwrap()
        .parse()
        .unwrap();

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

        let metadata = entry.metadata().unwrap();
        
        // Creation time
        let created = metadata.created().unwrap();
        let created = duration_to_days( created.elapsed().unwrap() );
        
        // Last Modification time
        let modified = metadata.modified().unwrap();
        let modified = duration_to_days( modified.elapsed().unwrap() );

        // Last Access time
        let last_access = metadata.accessed().unwrap();
        let last_access = duration_to_days( last_access.elapsed().unwrap() );

        let name = format!("{}{}", "    ".repeat(entry.depth()), name);

        match metadata.is_dir() {
            true => table.add_row(row![FY => name, created, modified, last_access]),
            false => table.add_row(row![name, created, modified, last_access])
        };
    }

    table.printstd();
}

