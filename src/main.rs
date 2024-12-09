use colored::*;
use std::io;
use std::io::Write;
use std::process::Command;

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
    let output = match theme {
        Theme::Light => "output_light.html",
        Theme::Dark => "output_dark.html",
        Theme::Both => panic!("Invalid theme"),
    };

    let output = format!("output/{}", output);

    Command::new("mkdir")
        .arg("output")
        .status()
        .expect("Could not create output directory");

    let markdown_css = match theme {
        Theme::Light => "github-markdown-light.css",
        Theme::Dark => "github-markdown-dark.css",
        Theme::Both => panic!("Invalid theme"),
    };
    let markdown_css = format!("statis/markdown-css/{}", markdown_css);

    run_pandoc("VPS-Setup.md", &output, &markdown_css);
}

fn run_pandoc(input: &str, output: &str, css: &str) {
    let status = Command::new("pandoc")
        .arg(input)
        .arg("-o")
        .arg(output)
        .arg("--standalone")
        .arg("--embed-resources")
        .arg("--template")
        .arg("templates/github-markdown-template.html")
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
