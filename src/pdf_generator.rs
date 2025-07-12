use std::process::Command;

pub fn generate_pdf(filename: &str) {
    let status = Command::new("typst").arg("compile").arg(filename).status();

    match status {
        Ok(status) => {
            if status.success() {
                println!("PDF generated successfully.");
                if let Err(e) = std::fs::remove_file(filename) {
                    eprintln!("Failed to delete typst file: {}", e);
                }
            } else {
                eprintln!("Typst command failed with status: {}", status);
            }
        }
        Err(e) => {
            eprintln!("Failed to execute typst command: {}", e);
        }
    }
}
