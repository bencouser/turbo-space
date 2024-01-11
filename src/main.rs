use std::fs;

fn main() {
    let project_name = "project_name";

    create_dir(&format!("{}/data/", project_name));
    create_dir(&format!("{}/notebooks/", project_name));
}

fn create_dir(path: &str) {
    match fs::create_dir_all(path) {
        Ok(_) => println!("Created directory: {}", path),
        Err(e) => println!("Error creating directory: {}", e),
    }
}
