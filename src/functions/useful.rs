use crate::utils::{keyboard::Keyboard, resources::Resources};
use teloxide::{
    payloads::SendMessageSetters,
    prelude::*,
    types::{InlineKeyboardMarkup, ParseMode},
};

static TEXT: &str = "<b>Rustga oid foydali materiallar:</b>\n\
Agar o'zingizdan material qo'shmoqchi bo'lsangiz, bizni \
<a href='https://github.com/rust-lang-uz/rustina/blob/main/source.json'>\
source.json</a> ni yangilang!";

pub async fn command(bot: &Bot, msg: &Message, resources: &Resources) -> ResponseResult<()> {
    let categories = resources.get_keys();

    bot.send_message(msg.chat.id, TEXT)
        .parse_mode(ParseMode::Html)
        .reply_markup(keyboard_list(1, categories))
        .disable_web_page_preview(true)
        .await?;

    Ok(())
}

pub fn keyboard_list(page: u32, categories: Vec<String>) -> InlineKeyboardMarkup {
    let mut keyboard = Keyboard::new();

    for category in categories {
        keyboard.text(
            &format!(
                "{}{}",
                &category[0..1].to_uppercase(),
                &category[1..].replace("_", " ")
            ),
            &format!("useful_{}_{}", page, category),
        );
        keyboard.row();
    }

    keyboard.get()
}
