use audiotags::{MimeType, Picture, Tag};
use std::env;
use std::ffi::OsStr;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <directory>", args[0]);
        std::process::exit(1);
    }

    let dir_path = &args[1];
    let dir = std::fs::read_dir(dir_path)?;

    for entry in dir {
        let entry = entry?;
        let path = entry.path();

        if path.extension() == Some(OsStr::new("m4a")) {
            process_file(&path)?;
        }
    }

    Ok(())
}

fn process_file(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let filename = path
        .file_stem()
        .and_then(|s| s.to_str())
        .ok_or("Invalid filename")?;

    let parts: Vec<&str> = filename.split(" - ").collect();
    if parts.len() != 2 {
        println!(
            "Skipped: {} (expected exactly one ' - ' separator)",
            filename
        );
        return Ok(());
    }

    let artist = parts[0].trim();
    let title = parts[1].trim();

    let mut tag = Tag::new().read_from_path(path)?;

    tag.set_artist(artist);
    tag.set_title(title);

    tag.write_to_path(path.as_os_str().to_str().unwrap())?;
    // println!(
    //     "Processed: {} -> Artist: '{}', Title: '{}'",
    //     filename, artist, title
    // );

    Ok(())
}
