use crate::utils::kbmng::Keyboard;
use teloxide::{
    payloads::SendMessageSetters,
    prelude::*,
    types::{InlineKeyboardMarkup, ParseMode},
};

static TEXT: &str = r#"
<b>Hurmatli foydalanuvchi!</b>

Bizning botimiz aktiv tarzda shakllantirib boriladi. Buning ustida esa bir necha avtor va dasturchilar turadi, ushbu havolalar orqali bizning sinovchilarimizdan biriga aylaning va biz bilan botimiz, hamda guruhimiz ishlatish qulayligini oshiring.
"#;

pub fn keyboard() -> InlineKeyboardMarkup {
    let mut keyboard = Keyboard::new();
    keyboard.url(
        "Ochiq Havolalar",
        "https://github.com/rust-lang-uz/rustina",
    )
}

pub async fn command(bot: &Bot, msg: &Message) -> ResponseResult<()> {
    bot.send_message(msg.chat.id, TEXT)
        .parse_mode(ParseMode::Html)
        .reply_markup(keyboard())
        .await?;

    Ok(())
}
