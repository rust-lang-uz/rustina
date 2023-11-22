use teloxide::{payloads::*, prelude::*, requests::JsonRequest, types::*};

pub trait Rustina {
    type Err: std::error::Error + Send;
    type SendMessageTF: Request<Payload = SendMessage, Err = Self::Err>;

    /// For Telegram documentation see [`SendMessage`].
    fn send_message_tf<C, T>(&self, chat_id: C, text: T, message: &Message) -> Self::SendMessageTF
    where
        C: Into<Recipient>,
        T: Into<String>;
}

impl Rustina for Bot {
    type Err = teloxide::errors::RequestError;
    type SendMessageTF = JsonRequest<teloxide::payloads::SendMessage>;

    fn send_message_tf<C, T>(&self, chat_id: C, text: T, message: &Message) -> Self::SendMessageTF
    where
        C: Into<Recipient>,
        T: Into<String>,
    {
        match message.thread_id {
            Some(thread_id) => Self::SendMessageTF::new(
                self.clone(),
                teloxide::payloads::SendMessage::new(chat_id, text),
            )
            .message_thread_id(thread_id),
            None => Self::SendMessageTF::new(
                self.clone(),
                teloxide::payloads::SendMessage::new(chat_id, text),
            ),
        }
    }
}
