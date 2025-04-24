use std::env;
use std::fs::{self, OpenOptions};
use std::io::{self, Write};

fn save_note(title: &str, content: &str) -> io::Result<()> {
    let filename = format!("{}.md", title.replace(" ", "_"));
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(filename)?;
    writeln!(file, "# {}\n\n{}", title, content)?;
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: inkdrop-cli <title> <content>");
        return;
    }
    let title = &args[1];
    let content = &args[2];
    match save_note(title, content) {
        Ok(_) => println!("Note saved successfully."),
        Err(e) => eprintln!("Error saving note: {}", e),
    }
}
