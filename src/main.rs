use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::Result;

fn main() -> Result<()> {
    let project_name = "project_name";

    // Data Directory
    create_dir(&format!("{}/data/", project_name));
    create_dir(&format!("{}/data/raw/", project_name));
    create_dir(&format!("{}/data/processed/", project_name));

    // Notebooks Directory
    create_dir(&format!("{}/notebooks/", project_name));

    // Root Files
    create_file(
        &format!("{}/README.md", project_name),
        &format!("# {}", project_name),
    )?;

    // Scripts Directory
    // Images Directory
    // Models Directory
    // ...

    Ok(())
}

fn create_dir(path: &str) {
    match fs::create_dir_all(path) {
        Ok(_) => println!("Created directory: {}", path),
        Err(e) => println!("Error creating directory: {}", e),
    }
}

fn create_file(path: &str, contents: &str) -> Result<()> {
    let mut file = File::create(path)?;
    file.write_all(contents.as_bytes())?;
    println!("Created file: {}", path);
    Ok(())
}
