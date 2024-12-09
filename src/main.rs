use std::io;
use std::process::Command;

fn main() {
    loop {
        let mut theme = String::new();
        println!("Enter theme (light, dark, both): ");
        io::stdin()
            .read_line(&mut theme)
            .expect("Failed to read line");

        let theme = match Theme::from_input(&theme) {
            Ok(theme) => theme,
            Err(e) => {
                eprintln!("{}", e);
                continue;
            }
        };

        break;
    }

    markdown_to_html();
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

fn markdown_to_html() {
    let status = Command::new("pandoc")
        .arg("VPS-Setup.md")
        .arg("-o")
        .arg("output_dark.html")
        .arg("--standalone")
        .arg("--embed-resources")
        .arg("--template")
        .arg("github-markdown-template.html")
        .arg("--css")
        .arg("github-markdown-dark.css")
        .status()
        .expect("Could not execute pandoc");

    if status.success() {
        println!("Conversion successful.");
    } else {
        eprintln!("Conversion failed.");
    }
}
