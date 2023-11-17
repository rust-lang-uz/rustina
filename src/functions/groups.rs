use crate::utils::kbmng::Keyboard;
use teloxide::{
    payloads::SendMessageSetters,
    prelude::*,
    types::{InlineKeyboardMarkup, ParseMode},
};

static TEXT: &str = "<b>Telegramdagi Rust Hamjamiyatlari yoki Guruhlari:</b>\nAgar o'zingizni guruhingizni qo'shmoqchi bo'lsangiz, bizni <a href='https://github.com/rust-lang-uz/rustina/blob/main/communities.json'>community.json</a> ni yangilang!";

pub fn keyboard() -> InlineKeyboardMarkup {
    let mut keyboard = Keyboard::new();
    keyboard.url("Jamiyat", "https://t.me/rustlanguz");
    keyboard.url("Web Sahifa", "https://rust-lang.uz")
}

pub async fn command(bot: &Bot, msg: &Message) -> ResponseResult<()> {
    bot.send_message(msg.chat.id, TEXT)
        .parse_mode(ParseMode::Html)
        .reply_markup(keyboard())
        .await?;

    Ok(())
}

pub async fn callback(bot: &Bot, q: &CallbackQuery, args: &Vec<&str>) -> ResponseResult<()> {
    if args.is_empty() {
        if let Some(Message { id, chat, .. }) = q.message.clone() {
            bot.edit_message_text(chat.id, id, TEXT)
                .parse_mode(ParseMode::Html)
                .reply_markup(keyboard())
                .await?;
        } else if let Some(id) = q.inline_message_id.clone() {
            bot.edit_message_text_inline(id, "Oopsie, something went wrong...")
                .await?;
        }
    }

    Ok(())
}
