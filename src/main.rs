use colored::*;
use std::path::PathBuf;
use std::process::Command;
use std::{env, fs, io};

fn main() {
    markdown_to_html();
}

fn markdown_to_html() {
    let output_dir = create_output_dir();

    let output = output_dir.join("output.html");

    let current_dir = env::current_dir().expect("Could not get current directory");

    let templates_dir = current_dir.join("templates");
    let template = templates_dir.join("github-markdown-template.html");

    let input_dir = current_dir.join("input");
    let input = input_dir.join("RaspberryPi_Installation.md");

    run_pandoc(&input, &output, &template);
}

fn create_output_dir() -> PathBuf {
    let current_dir = env::current_dir().expect("Could not get current directory");

    let output_dir = current_dir.join("output");

    match fs::create_dir(&output_dir) {
        Ok(_) => println!("Created output directory."),
        Err(e) => {
            if e.kind() == io::ErrorKind::AlreadyExists {
                println!("Output directory already exists.");
            } else {
                eprintln!("{} {}", "Could not create output directory:".red(), e);
            }
        }
    }

    output_dir
}

fn run_pandoc(input: &PathBuf, output: &PathBuf, template: &PathBuf) {
    let status = Command::new("pandoc")
        .arg(input)
        .arg("-o")
        .arg(output)
        .arg("--standalone")
        .arg("--embed-resources")
        .arg("--template")
        .arg(template)
        .status()
        .expect("Could not execute pandoc");

    if status.success() {
        println!("Conversion successful.");
    } else {
        eprintln!("Conversion failed.");
    }
}
