#![allow(clippy::single_match)]

pub mod about;
pub mod groups;
pub mod help;
pub mod inline;
pub mod latest;
pub mod offtop;
pub mod rules;
pub mod start;
pub mod version;

pub use inline::inline;

use crate::utils::github::GitHub;
use crate::utils::group_manager::Groups;
use crate::Command;
use std::error::Error;
use teloxide::prelude::*;
use teloxide::types::*;

pub async fn commands(
    bot: Bot,
    me: Me,
    msg: Message,
    cmd: Command,
    github: GitHub,
    groups: Groups,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let _ = match cmd {
        Command::Start => crate::functions::start::command(&bot, &msg).await,
        Command::Help => crate::functions::help::command(&bot, &msg, &cmd).await,
        Command::Rules => crate::functions::rules::command(&bot, &msg).await,
        Command::About => crate::functions::about::command(&bot, &msg).await,
        Command::Groups => crate::functions::groups::command(&bot, &msg, &groups).await,
        Command::Latest => crate::functions::latest::command(&bot, github, &msg).await,
        Command::Version => crate::functions::version::command(&bot, github, &msg).await,
        Command::Off => crate::functions::offtop::command(&bot, &msg, &me).await,
    };

    Ok(())
}

pub async fn callback(
    bot: Bot,
    q: CallbackQuery,
    github: GitHub,
    groups: Groups,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut args: Vec<&str> = Vec::new();

    if let Some(data) = q.data.clone() {
        if data.contains('_') {
            args = data.split('_').collect();
        } else {
            args.push(&data);
        }

        let _ = match args.remove(0) {
            "group" => crate::functions::groups::callback_list(&bot, &q, &args, &groups).await,
            "detail" => crate::functions::groups::callback_detail(&bot, &q, &args).await,
            "version" => crate::functions::version::callback_list(&bot, &q, &args, github).await,
            "changelog" => {
                crate::functions::version::callback_detail(&bot, &q, &args, github).await
            }
            _ => Ok(()),
        };
    }

    Ok(())
}

pub async fn triggers(bot: Bot, msg: Message) -> Result<(), Box<dyn Error + Send + Sync>> {
    if let Some(user) = msg.from() {
        if let Some(username) = user.username.clone() {
            if username == "Channel_Bot" {
                // try to delete message and ignore error
                match bot.delete_message(msg.chat.id, msg.id).await {
                    Ok(_) => {}
                    Err(_) => {}
                }
            }
        }
    }

    Ok(())
}
