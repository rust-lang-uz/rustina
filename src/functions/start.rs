use crate::utils::kbmng::Keyboard;
use teloxide::{
    payloads::SendMessageSetters,
    prelude::*,
    types::{InlineKeyboardMarkup, ParseMode},
};

static TEXT: &str = r#"
<b>Assalomu alaykum, hurmatli Rustacean!</b>

Sizni ko'rib turganimdan bag'oyatda xursandman. Men O'zbek Rust jamiyati tomonidan yaratilgan bot hisoblanib, O'zbek Rust jamiyati foydalanuvchilari uchun foydali resurslarni yetkazish, saqlash va ularni saralash uchun xizmat qilaman.
"#;

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
