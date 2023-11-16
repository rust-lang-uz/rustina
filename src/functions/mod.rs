pub mod about;
pub mod help;
pub mod inline;
pub mod rules;
pub mod start;
pub mod groups;

pub use inline::inline;

use crate::Command;
use std::error::Error;
use teloxide::prelude::*;

pub async fn commands(
    bot: Bot,
    _me: teloxide::types::Me,
    msg: Message,
    cmd: Command,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let _ = match cmd {
        Command::Start => crate::functions::start::command(&bot, &msg).await,
        Command::Help => crate::functions::help::command(&bot, &msg, &cmd).await,
        Command::Rules => crate::functions::rules::command(&bot, &msg).await,
        Command::About => crate::functions::about::command(&bot, &msg).await,
        Command::Groups => crate::functions::groups::command(&bot, &msg).await,
    };

    Ok(())
}

pub async fn callback(bot: Bot, q: CallbackQuery) -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut args: Vec<&str> = Vec::new();

    if let Some(data) = q.data.clone() {
        if data.contains("_") {
            args = data.split("_").collect();
        } else {
            args.push(&data);
        }

        let _ = match args[0] {
            "group" => crate::functions::groups::callback(&bot, &q, &args).await,
            _ => Ok(())
        };
    }

    Ok(())
}
