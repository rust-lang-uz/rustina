use teloxide::{
    payloads::SendMessageSetters,
    prelude::*,
    types::{InlineKeyboardMarkup, ParseMode},
};

use crate::utils::keyboard::Keyboard;

static TEXT: &str = "⚠️ <b>Bu komanda faqat shaxsiy chat uchun!</b>";

pub fn keyboard() -> InlineKeyboardMarkup {
    let mut keyboard: Keyboard = Keyboard::new();
    keyboard.url("Shaxsiy Chat", "https://t.me/rustinabot")
}

pub async fn is_private(bot: &Bot, msg: &Message) -> ResponseResult<()> {
    bot.send_message(msg.chat.id, TEXT)
        .parse_mode(ParseMode::Html)
        .reply_markup(keyboard())
        .await?;

    Ok(())
}
