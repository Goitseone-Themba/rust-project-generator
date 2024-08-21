use clap::{Arg, Command};
use std::fs;
use std::path::Path;

fn main() {
    let matches = Command::new("rust_generator")
        .version("1.0")
        .about("creates a small project structure")
        .arg(
            Arg::new("name")
            .help("The name of your project")
            .required(true)
            .index(1),
        )
        .get_matches();

    let project_name = matches.get_one::<String>("name").expect("required argument");
    create_project(project_name);
}

fn create_project(name: &str) {
    let project_dir = Path::new(name);

    fs::create_dir(project_dir).expect("Failed to create project directory");

    let readme_path = project_dir.join("README.md");
    fs::write(readme_path, format!("# {}\n\nProject created with the project_creator", name))
        .expect("Failed to create README.md");

    let src_dir = project_dir.join("src");
    fs::create_dir(&src_dir).expect("Failed to create src directory");

    let main_rs_file = src_dir.join("main.rs");
    fs::write(main_rs_file, r#"fn main() {
    println!("Hello, World!")
    }"#,
    ).expect("Failed to create main.rs");

    let cargo_toml_file = project_dir.join("cargo.toml");
    fs::write(cargo_toml_file, format!("[package]
            name = {}
            version = 0.1.0
            edition = 2021

            [dependencies]", name))
        .expect("Failed to create cargo.toml");


}
