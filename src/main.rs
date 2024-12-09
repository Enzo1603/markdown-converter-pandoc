use std::process::Command;

fn main() {
    // let status = Command::new("mkdir")
    //     .arg("neues_verzeichnis")
    //     .status()
    //     .expect("Konnte mkdir nicht ausführen");

    // if status.success() {
    //     println!("Verzeichnis wurde erfolgreich erstellt.");
    // } else {
    //     eprintln!("Fehler beim Erstellen des Verzeichnisses.");
    // }

    markdown_to_html();
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
