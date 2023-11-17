pub mod about;
pub mod groups;
pub mod help;
pub mod inline;
pub mod latest;
pub mod rules;
pub mod start;
pub mod version;

pub use inline::inline;

use crate::utils::github::GitHub;
use crate::Command;
use std::error::Error;
use teloxide::prelude::*;

pub async fn commands(
    bot: Bot,
    _me: teloxide::types::Me,
    msg: Message,
    cmd: Command,
    github: GitHub,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let _ = match cmd {
        Command::Start => crate::functions::start::command(&bot, &msg).await,
        Command::Help => crate::functions::help::command(&bot, &msg, &cmd).await,
        Command::Rules => crate::functions::rules::command(&bot, &msg).await,
        Command::About => crate::functions::about::command(&bot, &msg).await,
        Command::Groups => crate::functions::groups::command(&bot, &msg).await,
        Command::Latest => crate::functions::latest::command(&bot, github, &msg).await,
        Command::Version => crate::functions::version::command(&bot, github, &msg).await,
    };

    Ok(())
}

pub async fn callback(
    bot: Bot,
    q: CallbackQuery,
    github: GitHub,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut args: Vec<&str> = Vec::new();

    if let Some(data) = q.data.clone() {
        if data.contains('_') {
            args = data.split('_').collect();
        } else {
            args.push(&data);
        }

        let _ = match args.remove(0) {
            "group" => crate::functions::groups::callback_list(&bot, &q, &args).await,
            "detail" => crate::functions::groups::callback_detail(&bot, &q, &args).await,
            "version" => crate::functions::version::callback_list(&bot, &q, &args, github).await,
            "changelog" => crate::functions::version::callback_detail(&bot, &q, &args, github).await,
            _ => Ok(()),
        };
    }

    Ok(())
}
