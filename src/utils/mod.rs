pub mod github;
pub mod groups;
pub mod inlines;
pub mod keyboard;
pub mod message;
pub mod resources;

pub fn clog(title: &str, message: &str) {
    let title = if title.len() > 12 {
        title[..8].to_string() + "..."
    } else {
        title.to_string()
    };

    println!(
        "{}\x1b[1;32m{}\x1b[0m {} {}",
        " ".repeat(12 - title.len()),
        title,
        message,
        " ".repeat(8)
    );
}
