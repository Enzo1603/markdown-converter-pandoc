use std::process::Command;

fn main() {
    let status = Command::new("mkdir")
        .arg("neues_verzeichnis")
        .status()
        .expect("Konnte mkdir nicht ausführen");

    if status.success() {
        println!("Verzeichnis wurde erfolgreich erstellt.");
    } else {
        eprintln!("Fehler beim Erstellen des Verzeichnisses.");
    }
}
