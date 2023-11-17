use crate::utils::{group_manager::Groups, keyboard_manager::Keyboard};
use teloxide::{
    payloads::SendMessageSetters,
    prelude::*,
    types::{InlineKeyboardMarkup, ParseMode},
};

static TEXT: &str = "<b>Telegramdagi Rust Hamjamiyatlari yoki Guruhlari:</b>\nAgar o'zingizni guruhingizni qo'shmoqchi bo'lsangiz, bizni <a href='https://github.com/rust-lang-uz/rustina/blob/main/communities.json'>community.json</a> ni yangilang!";

pub async fn command(bot: &Bot, msg: &Message) -> ResponseResult<()> {
    let groups: Groups = Groups::new();

    bot.send_message(msg.chat.id, TEXT)
        .parse_mode(ParseMode::Html)
        .reply_markup(keyboard(&groups, 1))
        .await?;

    Ok(())
}

pub async fn callback(bot: &Bot, q: &CallbackQuery, args: &Vec<&str>) -> ResponseResult<()> {
    let groups: Groups = Groups::new();
    
    // Parse page
    let page: i32 = match args[1].parse() {
        Ok(page) => page,
        Err(_) => 1,
    };

    if args.is_empty() {
        if let Some(Message { id, chat, .. }) = q.message.clone() {
            bot.edit_message_text(chat.id, id, TEXT)
                .parse_mode(ParseMode::Html)
                .reply_markup(keyboard(&groups, page))
                .await?;
        } else if let Some(id) = q.inline_message_id.clone() {
            bot.edit_message_text_inline(id, "Oopsie, something went wrong...")
                .await?;
        }
    }

    Ok(())
}

pub fn keyboard(groups: &Groups, page: i32) -> InlineKeyboardMarkup {
    let mut keyboard = Keyboard::new();

    for group in groups.get_groups(page, 5) {
        keyboard.text(
            &group.name,
            &format!(
                "detail_{}_{}",
                page,
                group.telegram.clone().replace("@", "")
            ),
        );
        keyboard.row();
    }

    if !groups.get_groups(page + 1, 5).is_empty() {
        keyboard.text(&"Keyingi ➡️", &format!("group_{}", page + 1));
    }

    if page > 1 {
        keyboard.text(&"⬅️ Oldingi", &format!("group_{}", page - 1));
    }

    keyboard.get()
}
