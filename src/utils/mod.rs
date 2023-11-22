pub mod github;
pub mod groups;
pub mod inlines;
pub mod keyboard;
pub mod message;
pub mod resources;

pub fn cargo_like_log(title: &str, message: &str) {
    println!("{}{}{}{} {} {}", 
        " ".repeat(12 - title.len()),
        "\x1b[1;32m",
        title,
        "\x1b[0m",
        message,
        " ".repeat(8)
    );
}