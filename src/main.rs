use clap::{Parser, ValueEnum};
use colored::Colorize;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::{env, fs};
use thiserror::Error;

/// Supported output formats
#[derive(Debug, Copy, Clone, ValueEnum)]
enum OutputFormat {
    Html,
    Pdf,
    Both,
}

/// Command line arguments
#[derive(Parser, Debug)]
#[command(
    name = "markdown-converter",
    about = "Convert Markdown files to HTML and PDF using Pandoc",
    version
)]
struct Args {
    /// Input markdown file (relative to input directory)
    #[arg(short, long)]
    input: String,

    /// Output format
    #[arg(short, long, value_enum, default_value_t = OutputFormat::Html)]
    format: OutputFormat,

    /// Custom output filename (without extension)
    #[arg(short, long)]
    output: Option<String>,
}

#[derive(Error, Debug)]
enum ConverterError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Pandoc execution failed: {0}")]
    PandocExecution(String),

    #[error("Failed to get current directory")]
    CurrentDir,
}

type Result<T> = std::result::Result<T, ConverterError>;

/// Represents project directories
struct ProjectPaths {
    output_dir: PathBuf,
    templates_dir: PathBuf,
    input_dir: PathBuf,
}

/// Main conversion function
fn main() -> Result<()> {
    let args = Args::parse();

    // Set up project directories
    let paths = setup_project_directories()?;

    // Determine input and output paths
    let input_file = paths.input_dir.join(&args.input);
    let output_filename = args.output.unwrap_or_else(|| {
        Path::new(&args.input)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("output")
            .to_string()
    });

    let template = paths.templates_dir.join("github-markdown-template.html");

    // Process based on selected format
    match args.format {
        OutputFormat::Html => {
            let html_output = paths.output_dir.join(format!("{}.html", output_filename));
            convert_to_html(&input_file, &html_output, &template)?;
            println!(
                "{} HTML file saved to: {}",
                "Success:".green(),
                html_output.display()
            );
        }
        OutputFormat::Pdf => {
            let pdf_output = paths.output_dir.join(format!("{}.pdf", output_filename));
            convert_to_pdf(&input_file, &pdf_output, &template)?;
            println!(
                "{} PDF file saved to: {}",
                "Success:".green(),
                pdf_output.display()
            );
        }
        OutputFormat::Both => {
            let html_output = paths.output_dir.join(format!("{}.html", output_filename));
            let pdf_output = paths.output_dir.join(format!("{}.pdf", output_filename));

            convert_to_html(&input_file, &html_output, &template)?;
            println!(
                "{} HTML file saved to: {}",
                "Success:".green(),
                html_output.display()
            );

            convert_to_pdf(&input_file, &pdf_output, &template)?;
            println!(
                "{} PDF file saved to: {}",
                "Success:".green(),
                pdf_output.display()
            );
        }
    }

    Ok(())
}

/// Set up project directories
fn setup_project_directories() -> Result<ProjectPaths> {
    let current_dir = env::current_dir().map_err(|_| ConverterError::CurrentDir)?;

    let output_dir = current_dir.join("output");
    let templates_dir = current_dir.join("templates");
    let input_dir = current_dir.join("input");

    // Create output directory if it doesn't exist
    if !output_dir.exists() {
        fs::create_dir(&output_dir)?;
        println!("{} Created output directory", "Info:".blue());
    }

    // Validate that required directories exist
    if !templates_dir.exists() {
        println!(
            "{} Templates directory does not exist: {}",
            "Warning:".yellow(),
            templates_dir.display()
        );
    }

    if !input_dir.exists() {
        println!(
            "{} Input directory does not exist: {}",
            "Warning:".yellow(),
            input_dir.display()
        );
    }

    Ok(ProjectPaths {
        output_dir,
        templates_dir,
        input_dir,
    })
}

/// Convert markdown to HTML using pandoc
fn convert_to_html(input: &Path, output: &Path, template: &Path) -> Result<()> {
    println!(
        "{} Converting {} to HTML...",
        "Info:".blue(),
        input.display()
    );

    let status = Command::new("pandoc")
        .arg(input)
        .arg("-o")
        .arg(output)
        .arg("--standalone")
        .arg("--embed-resources")
        .arg("--template")
        .arg(template)
        .status()?;

    if !status.success() {
        return Err(ConverterError::PandocExecution(format!(
            "Pandoc failed with exit code: {}",
            status
        )));
    }

    Ok(())
}

/// Convert markdown to PDF using pandoc
fn convert_to_pdf(input: &Path, output: &Path, template: &Path) -> Result<()> {
    println!(
        "{} Converting {} to PDF...",
        "Info:".blue(),
        input.display()
    );

    // Erstelle temporäres CSS für PDF-Export
    let pdf_css = create_temp_pdf_css()?;

    let status = ProcessCommand::new("pandoc")
        .arg(input)
        .arg("-o")
        .arg(output)
        .arg("--pdf-engine=weasyprint") // Bessere Engine für Webinhalte
        .arg("--standalone")
        .arg("--template")
        .arg(template)
        .arg("--css")
        .arg(&pdf_css)
        .arg("--dpi=300") // Höhere Auflösung für bessere Qualität
        .arg("--wrap=none") // Verhindern von unerwünschten Zeilenumbrüchen
        .status()?;

    if !status.success() {
        return Err(ConverterError::PandocExecution(format!(
            "Pandoc failed with exit code: {}",
            status
        )));
    }

    // Temporäre Datei aufräumen
    fs::remove_file(pdf_css).ok();

    Ok(())
}

fn create_temp_pdf_css() -> Result<PathBuf> {
    let temp_dir = env::temp_dir();
    let css_path = temp_dir.join("pdf_styles.css");

    let css_content = r#"
    @page {
        margin: 0.7cm;
        size: A4;
    }
    
    body {
        margin: 0;
        background-color: #ffffff;
        color: #24292e;
        -webkit-print-color-adjust: exact;
        print-color-adjust: exact;
    }
    
    /* Dark Mode für PDF - aktivieren Sie dies für Dark Mode PDF */
    /*
    body {
        background-color: #0d1117 !important;
        color: #c9d1d9 !important;
    }
    
    pre, code, div.sourceCode {
        background-color: #161b22 !important;
        border-color: #30363d !important;
    }
    */
    
    .theme-switch {
        display: none !important;
    }
    
    /* Verbesserte Codeblock-Darstellung für PDF */
    pre, div.sourceCode {
        page-break-inside: avoid;
        white-space: pre-wrap;
        overflow-wrap: break-word;
        border-radius: 4px;
        padding: 1em;
    }
    "#;

    fs::write(&css_path, css_content)?;
    Ok(css_path)
}
