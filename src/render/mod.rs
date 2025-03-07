use crossterm::{
    execute,
    style::{Color, Print, SetForegroundColor},
};
use std::io::{stdout, Write};

pub fn display_content(headlines: Vec<String>, paragraphs: Vec<String>, links: Vec<(String, String)>) {
    let mut stdout = stdout();

    // Print Headlines in Green
    execute!(stdout, SetForegroundColor(Color::Green)).unwrap();
    println!("\n🟢 Headlines:");
    for h1 in headlines {
        execute!(stdout, Print(format!("🔹 {}\n", h1))).unwrap();
    }

    // Print Paragraphs in White
    execute!(stdout, SetForegroundColor(Color::White)).unwrap();
    println!("\n🟡 Paragraphs:");
    for p in paragraphs {
        execute!(stdout, Print(format!("{}\n", p))).unwrap();
    }

    // Print Links in Blue
    execute!(stdout, SetForegroundColor(Color::Blue)).unwrap();
    println!("\n🔵 Links:");
    for (text, url) in links {
        execute!(stdout, Print(format!("🔗 {} -> {}\n", text, url))).unwrap();
    }

    // Reset color
    execute!(stdout, SetForegroundColor(Color::Reset)).unwrap();
}
