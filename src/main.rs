use std::env;
use std::fs;
use std::fs::File;
use std::io::{Result, Write};
use std::process::Command;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Please provide a project name!");
        return Ok(());
    }

    let project_name = &args[1];

    // Create env file eventually

    let env_name = format!("{}-env", project_name);

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

    create_file(&format!("{}/.env", project_name), "")?;

    create_file(&format!("{}/.gitignore", project_name), ".env")?;

    create_file(
        &format!("{}/requirements.txt", project_name),
        "Requirements",
    )?;

    // Create Conda Environment
    create_conda_env(&env_name);

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

fn create_conda_env(env_name: &str) {
    let conda_create = Command::new("conda")
        .args(&[
            "create",
            "--name",
            env_name,
            "python=3.10",
            "numpy",
            "pandas",
            "tensorflow",
            "grpcio >=1.37.0,<2.0",
            "h5py >=3.6.0,<3.7",
            "-y",
        ])
        .output();

    match conda_create {
        Ok(..) => {
            println!("Conda env created with: Python 3.10");
            // Install other packages
            let pip_install = Command::new("conda")
                .args(&[
                    "run",
                    "-n",
                    env_name,
                    "--no-capture-output",
                    "pip",
                    "install",
                    "tensorflow-macos",
                    "tensorflow-metal",
                ])
                .output();

            match pip_install {
                Ok(..) => println!("Installed tensorflow-macos and tensorflow-metal"),
                Err(e) => println!(
                    "Error installing tensorflow-macos and tensorflow-metal: {:?}",
                    e
                ),
            }
        }
        Err(e) => println!("Error creating Conda environment: {:?}", e),
    }
}
