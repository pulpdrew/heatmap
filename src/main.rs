#![feature(iterator_fold_self)]

use std::{
    fs::{self, DirEntry, File},
    io::Write,
    process,
};

use heatmap::{
    gpx_deserializer::parse_gpx,
    path::{Path, Points},
    tcx_deserializer::parse_tcx,
};

fn main() {
    // Extract the segments from each file in the input directory
    let mut all_paths: Vec<Path> = vec![];
    for entry in fs::read_dir("input_bkp").expect("Failed to read './input' directory") {
        if let Ok(entry) = entry {
            match parse_entry(&entry) {
                Ok(mut segments) => {
                    println!("INFO: Added {:?}", entry.path());
                    all_paths.append(&mut segments);
                }
                Err(error) => println!("WARN: Skipped {:?} ({})", entry.path(), error),
            };
        }
    }

    // Clean up the paths with smoothing and averaging nearby paths
    println!("INFO: Smoothing and cleaning paths...");
    let smoothed_paths: Vec<Path> = all_paths.iter().map(|p| p.smooth()).collect();
    let all_points = Points::new(
        smoothed_paths
            .iter()
            .flat_map(|s| s.points.clone())
            .collect(),
        0.00035,
    );
    let clean_paths: Vec<Path> = all_paths.iter().map(|p| p.combine(&all_points)).collect();

    // Generate the code to plot each polyline, insert it into the template HTML
    let data_js = format!("var paths = {};", as_json_arrays(&clean_paths));

    // Open the output file and write to it
    let mut file = if let Ok(output_file) = File::create("data.js") {
        output_file
    } else {
        eprintln!("Could not create output file 'data.js'");
        process::exit(4);
    };

    if write!(&mut file, "{}", data_js).is_ok() {
        println!("DONE: Wrote output to 'data.js.' Open 'index.html' in browser to view map.");
    } else {
        eprintln!("Could not write to output file 'data.js'");
        process::exit(5);
    }
}

fn parse_entry(entry: &DirEntry) -> Result<Vec<Path>, String> {
    let filename = entry.file_name().into_string();
    match filename {
        Ok(filename) => {
            let filename = filename.to_lowercase();
            if supports_file(&filename) {
                if filename.ends_with(".gpx") {
                    match std::fs::read_to_string(&entry.path()) {
                        Ok(contents) => Ok(parse_gpx(&contents)),
                        Err(_) => Err(format!("Failed to read file {:?}", &entry.path())),
                    }
                } else if filename.ends_with(".tcx") {
                    match std::fs::read_to_string(&entry.path()) {
                        Ok(contents) => Ok(parse_tcx(&contents)),
                        Err(_) => Err(format!("Failed to read file {:?}", &entry.path())),
                    }
                } else {
                    panic!(format!("Unexpected filetype on file {}", filename))
                }
            } else {
                Err(format!("Unsupported filetype on file {:?}", filename))
            }
        }
        Err(filename) => Err(format!("Bad file name {:?}", filename)),
    }
}

/// True iff the given filename ends with a supported filetype
fn supports_file(filename: &str) -> bool {
    filename.ends_with(".gpx") || filename.ends_with(".tcx")
}

fn as_json_arrays(paths: &[Path]) -> String {
    format!(
        "[\n  {}]",
        paths
            .iter()
            .map(Path::as_json_array)
            .fold_first(|array, next| array + "," + &next + "\n  ")
            .unwrap_or_else(String::new)
    )
}
