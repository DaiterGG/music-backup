use std::fs;
use std::process;

const DIR_PATH: &str = r#"C:\music\main playlist"#;
const OUTPUT_PATH: &str = r#"C:\music\backup\ALL TRACKS.txt"#;

fn main() {
    // Read directory entries
    let entries = match fs::read_dir(DIR_PATH) {
        Ok(e) => e,
        Err(err) => {
            eprintln!("Error reading directory '{}': {}", DIR_PATH, err);
            process::exit(1);
        }
    };

    // Collect file names
    let mut file_names = Vec::new();
    for entry in entries {
        match entry {
            Ok(entry) => {
                let file_name = entry.file_name().to_string_lossy().into_owned();
                let file_name = file_name.replace(".m4a", "");
                file_names.push(file_name);
            }
            Err(err) => eprintln!("Error reading directory entry: {}", err),
        }
    }

    // Write to output file
    let content = file_names.join("\n");
    if let Err(err) = fs::write(OUTPUT_PATH, content) {
        eprintln!("Error writing to file '{}': {}", OUTPUT_PATH, err);
        process::exit(1);
    }
}
