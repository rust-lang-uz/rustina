use super::start::keyboard;
use crate::Command;
use teloxide::{payloads::SendMessageSetters, prelude::*, types::ParseMode};

static TEXT: &[(&str, &str)] = &[
    ("docs", "reply qilingan odamga dok borligi haqida eslatish"),
    ("useful", "rust haqida foydali yoki kerakli ma'lumotlar"),
    ("latest", "eng oxirgi reliz haqida qisqacha ma'lumot"),
    ("version", "biron anniq reliz haqida to'liq ma'lumot"),
    (
        "off",
        "reply qilingan odamga offtop borligi haqida eslatish",
    ),
    ("group", "rust ga oid guruh va hamjamiyatlar"),
    ("help", "ushbu xabarni qayta ko'rsatish"),
    ("about", "ushbu botimizning rivojlantirish qismi"),
    ("rules", "qoidalarni aks ettirish"),
    (
        "which",
        "ushbu guruh va foydalanuvchi metrik ma'lumotlarini ko'rsatish",
    ),
];

pub async fn command(bot: &Bot, msg: &Message, _cmd: &Command) -> ResponseResult<()> {
    let mut text = String::new();

    text.push_str("<b>Mavjud komandalar ro'yxati:</b>\n\n");

    for cmd in TEXT {
        text.push('/');
        text.push_str(cmd.0);
        text.push_str(" - ");
        text.push_str(format!("<code>{text}</code>", text = cmd.1).as_str());
        text.push('\n');
    }

    bot.send_message(msg.chat.id, text)
        .parse_mode(ParseMode::Html)
        .reply_markup(keyboard())
        .await?;

    Ok(())
}
