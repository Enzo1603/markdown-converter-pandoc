use colored::*;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use std::{env, fs, io};

fn main() {
    let theme = get_theme();

    match theme {
        Theme::Light | Theme::Dark => markdown_to_html(&theme),
        Theme::Both => {
            markdown_to_html(&Theme::Dark);
            markdown_to_html(&Theme::Light);
        }
    }
}

enum Theme {
    Light,
    Dark,
    Both,
}

impl Theme {
    fn from_input(input: &str) -> Result<Theme, String> {
        match input.trim().to_lowercase().as_str() {
            "light" => Ok(Theme::Light),
            "dark" => Ok(Theme::Dark),
            "both" => Ok(Theme::Both),
            _ => Err("Invalid theme".to_string()),
        }
    }
}

fn get_theme() -> Theme {
    loop {
        let mut theme = String::new();
        print!("{}", "Enter theme (light, dark, both): ".blue());

        // Flush the buffer
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut theme)
            .expect("Failed to read line");

        let theme = match Theme::from_input(&theme) {
            Ok(theme) => theme,
            Err(e) => {
                eprintln!("{}\n", e.red());
                continue;
            }
        };

        return theme;
    }
}

fn markdown_to_html(theme: &Theme) {
    let output_dir = create_output_dir();

    let output = match theme {
        Theme::Light => output_dir.join("output_light.html"),
        Theme::Dark => output_dir.join("output_dark.html"),
        Theme::Both => panic!("Invalid theme"),
    };

    let markdown_css = match theme {
        Theme::Light => "github-markdown-light.css",
        Theme::Dark => "github-markdown-dark.css",
        Theme::Both => panic!("Invalid theme"),
    };
    let markdown_css = format!("static/markdown-css/{}", markdown_css);

    let current_dir = env::current_dir().expect("Could not get current directory");
    let templates_dir = current_dir.join("templates");
    let template = templates_dir.join("github-markdown-template.html");

    run_pandoc("VPS-Setup.md", &output, &markdown_css, &template);
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

fn run_pandoc(input: &str, output: &PathBuf, css: &str, template: &PathBuf) {
    let status = Command::new("pandoc")
        .arg(input)
        .arg("-o")
        .arg(output)
        .arg("--standalone")
        .arg("--embed-resources")
        .arg("--template")
        .arg(template)
        .arg("--css")
        .arg(css)
        .status()
        .expect("Could not execute pandoc");

    if status.success() {
        println!("Conversion successful.");
    } else {
        eprintln!("Conversion failed.");
    }
}
