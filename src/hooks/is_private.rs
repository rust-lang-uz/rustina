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
    match bot.delete_message(msg.chat.id, msg.id).await {
        Ok(_) => {}
        Err(_) => {}
    };

    let message = bot
        .send_message(msg.chat.id, TEXT)
        .parse_mode(ParseMode::Html)
        .reply_markup(keyboard())
        .await?;

    let thread_bot = bot.clone();
    tokio::spawn(async move {
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        match thread_bot.delete_message(message.chat.id, message.id).await {
            Ok(_) => {}
            Err(_) => {}
        };
    });

    Ok(())
}
