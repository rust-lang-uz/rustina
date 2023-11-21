use teloxide::{payloads::SendMessageSetters, prelude::*, types::ParseMode};

pub async fn command(bot: &Bot, msg: &Message) -> ResponseResult<()> {
    println!("Command triggered: {:?}", msg);
    bot.send_message(msg.chat.id, view(msg))
        .parse_mode(ParseMode::Html)
        // .reply_markup(keyboard())
        .await?;

    Ok(())
}

pub fn view(msg: &Message) -> String {
    let mut message: String = String::new();

    message.push_str(&format!("<b>Chat ID:</b> {}", msg.chat.id));

    if msg.thread_id.is_some() {
        message.push_str(&format!("\n<b>Thread ID:</b> {}", msg.thread_id.unwrap()))
    }

    println!("Message: {:?}", message);

    message
}
