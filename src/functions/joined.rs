use teloxide::{prelude::*, types::*};

static TEXT: &str = "<b>Salom bo'lajak Rustacean!</b>\n\n\
Sizlarni bu guruhda ko'rib turganimizdan mamnunmiz. Bu guruh Rust dasturlash tiliga qaratilgan hisoblanib, \
bu yerda ushbu til haqida gaplashish, savollar berish yoki o'z fikrlaringiz bilan bo'lishishingiz mumkin. \
Hamda, agar siz ushbu dasturlash tilida butunlay yangi bo'lsangiz, /roadmap yordamida kerakli boshlang'ich \
maslahatlar, va hamda /useful yordamoda foydali resurslar olishingiz mumkin.
";

pub async fn trigger(bot: &Bot, msg: &Message) -> ResponseResult<()> {
    let message = bot
        .send_message(msg.chat.id, TEXT)
        .parse_mode(ParseMode::Html);

    let message: Message = if msg.thread_id.is_some() {
        message
            .message_thread_id(msg.thread_id.unwrap())
            .send()
            .await?
    } else {
        message.send().await?
    };

    let thread_bot = bot.clone();
    tokio::spawn(async move {
        tokio::time::sleep(tokio::time::Duration::from_secs(60 * 5)).await;
        match thread_bot.delete_message(message.chat.id, message.id).await {
            Ok(_) => {}
            Err(_) => {}
        };
    });

    Ok(())
}
