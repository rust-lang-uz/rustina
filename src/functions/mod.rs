pub mod help;
pub mod rules;
pub mod start;
pub mod about;

pub use teloxide::prelude::*;

use crate::Command;
use std::error::Error;

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
