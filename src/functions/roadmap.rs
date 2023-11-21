use crate::hooks;
use crate::utils::keyboard::Keyboard;
use teloxide::{
    payloads::SendMessageSetters,
    prelude::*,
    types::{InlineKeyboardMarkup, ParseMode},
};

static ROADMAP: &str = include_str!("../../data/roadmap.md");

pub async fn command(bot: &Bot, msg: &Message) -> ResponseResult<()> {
    if !msg.chat.is_private() {
        return {
            hooks::is_private(bot, msg).await.unwrap();
            Ok(())
        };
    }

    bot.send_message(msg.chat.id, ROADMAP)
        .parse_mode(ParseMode::Html)
        .reply_markup(keyboard())
        .await?;

    Ok(())
}

pub fn keyboard() -> InlineKeyboardMarkup {
    let mut keyboard = Keyboard::new();
    keyboard.url("Jamiyat", "https://t.me/rustlanguz");
    keyboard.url("Web Sahifa", "https://rust-lang.uz")
}
