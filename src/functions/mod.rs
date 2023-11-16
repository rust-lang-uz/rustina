pub mod about;
pub mod help;
pub mod inline;
pub mod rules;
pub mod start;

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
    };

    Ok(())
}
