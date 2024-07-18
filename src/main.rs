use std::time::Duration;
use walkdir::{DirEntry, WalkDir};

const SECONDS_IN_DAY: f64 = 86_400.0;

// converts Duration to days by converting it to f64 and dividing by 86_400
fn duration_to_days(duration: Duration) -> f64 {
    (duration.as_secs_f64() / SECONDS_IN_DAY).round()
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry.file_name()
         .to_str()
         .map(|s| s.starts_with("."))
         .unwrap_or(false)
}

fn main() {
    let path = "/home/stefano/Pictures/";

    let walker = WalkDir::new(path).into_iter();

    println!(
            "{0: <150} | {1: >15} | {2: >15} | {3: >15} |",
            "file-name", "created", "last modified", "last access"
    );

    for _ in 0..=205 {
        print!("-");
    }

    println!("");

    //for entry in walker.filter_entry(|e| is_hidden(e)) {
    for entry in walker.filter_map(|e| e.ok()) {
    /*for entry in walker.filter_map(|e| {
        duration_to_days(e.unwrap().metadata().unwrap().accessed()) > 30
    }){*/
        //let entry = entry.unwrap();
        let name = entry.file_name().to_str().unwrap();
        //let metadata = fs::metadata(entry.path()).unwrap();
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

        let name = format!("{} {}", "    ".repeat(entry.depth()), name);

        println!(
            "{: <150} | {: >15} | {: >15} | {: >15} |",
            name,
            created,
            modified,
            last_access
        );
    }
}

