use std::{env, error, fs};

fn main() -> Result<(), Box<dyn error::Error>> {
    let current_dir = env::current_dir()?;
    println!(
        "Entries modified in the last 24 hours in {:?}:",
        current_dir
    );

    for entry in fs::read_dir(current_dir)? {
        let entry = entry?;
        let path = entry.path();

        let metadata = fs::metadata(&path)?;

        if metadata.is_file() {
            let last_modified = metadata.modified()?.elapsed()?.as_secs();
            if last_modified < 24 * 3600 {
                println!(
                    "Last modified: {:?} seconds, is read only: {:?}, size: {:?} bytes, filename: {:?}",
                    last_modified,
                    metadata.permissions().readonly(),
                    metadata.len(),
                    path.file_name().ok_or("No filename")?
                );
            }
        }
    }

    Ok(())
}