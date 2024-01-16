use std::env;
use std::fs;
use std::fs::File;
use std::io::Result;
use std::io::{self, Write};
use std::process::Command;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Please provide a project name!");
        return Ok(());
    }

    let project_name = &args[1];
    let env_name = format!("{}_env", project_name);
    let template_path = "./environment.yml";

    // Data Directory
    create_dir(&format!("{}/data/", project_name));
    create_dir(&format!("{}/data/raw/", project_name));
    create_dir(&format!("{}/data/processed/", project_name));

    // Notebooks Directory
    create_dir(&format!("{}/notebooks/", project_name));

    // Scripts Directory
    create_dir(&format!("{}/scripts/", project_name));

    // Models Directory
    create_dir(&format!("{}/models/", project_name));

    // src Directory
    create_dir(&format!("{}/src/", project_name));

    // Images Directory
    create_dir(&format!("{}/images/", project_name));

    // Root Files
    create_file(
        &format!("{}/README.md", project_name),
        &format!("# {}", project_name),
    )?;

    // make .env file
    create_file(&format!("{}/.env", project_name), "")?;

    create_file(&format!("{}/.gitignore", project_name), ".env")?;

    create_file(
        &format!("{}/requirements.txt", project_name),
        "Requirements",
    )?;

    create_conda_environment(&project_name, &env_name, template_path)?;

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

fn create_conda_environment(
    project_name: &str,
    env_name: &str,
    template_path: &str,
) -> io::Result<()> {
    println!("Creating conda environment...");

    // Read template file
    let template = fs::read_to_string(template_path)?;

    // print the env name
    println!("env_name: {}", env_name);

    // Replace placeholder with env name
    let config = template.replace("PLACEHOLDER", env_name);

    // Write to a new file
    create_file(&format!("{}/{}.yml", project_name, env_name), &config)?;

    // Create Environment
    let output = Command::new("conda")
        .arg("env")
        .arg("create")
        .arg("-f")
        .arg(&format!("{}/{}.yml", project_name, env_name))
        .output()?;

    if output.status.success() {
        println!("Created conda environment!");
    } else {
        println!("Error creating conda environment");
    }

    Ok(())
}
