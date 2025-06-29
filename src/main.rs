use clap::{Parser, Subcommand};
use std::fs;
use std::path::Path;
use std::process::Command;

#[derive(Parser, Debug)]
#[command(name = "koshi", version, about = "The Kopi Package Manager")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    New {
        name: String,
    },
    Run,
}

fn handle_new_project(name: &str) {
    let project_path = Path::new(name);
    if project_path.exists() {
        eprintln!("Error: directory '{}' already exists.", name);
        return;
    }

    let src_path = project_path.join("src");
    fs::create_dir_all(&src_path).expect("Failed to create project directories.");

    let toml_content = format!(
        "[package]\n\
         name = \"{}\"\n\
         version = \"0.1.0\"\n",
        name
    );
    fs::write(project_path.join("Kopi.toml"), toml_content)
        .expect("Failed to write Kopi.toml.");

    let main_kp_content = "put(\"Hello, Kopi!\")\n";
    fs::write(src_path.join("main.kp"), main_kp_content)
        .expect("Failed to write src/main.kp.");

    println!("Created new Kopi project '{}'", name);
}

fn handle_run_project() {
    let main_file = Path::new("src/main.kp");
    if !main_file.exists() {
        eprintln!("Error: could not find 'src/main.kp'. Are you in a Kopi project directory?");
        return;
    }

    println!("Running project...");

    let status = Command::new("kopi")
        .arg(main_file)
        .status()
        .expect("Failed to execute 'kopi'. Is it installed and in your PATH?");

    if !status.success() {
        eprintln!("Project finished with an error.");
    }
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::New { name } => {
            handle_new_project(&name);
        }
        Commands::Run => {
            handle_run_project();
        }
    }
}
