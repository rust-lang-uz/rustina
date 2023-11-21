use teloxide::{payloads::*, prelude::*, requests::JsonRequest, types::*};

trait RequestTopicFriendly {
    type Err: std::error::Error + Send;
    type SendMessageTF: Request<Payload = SendMessage, Err = Self::Err>;

    /// For Telegram documentation see [`SendMessage`].
    fn send_message_tf<C, T>(&self, chat_id: C, text: T) -> Self::SendMessageTF
    where
        C: Into<Recipient>,
        T: Into<String>;
}

impl RequestTopicFriendly for Bot {
    type Err = teloxide::errors::RequestError;
    type SendMessageTF = JsonRequest<teloxide::payloads::SendMessage>;

    fn send_message_tf<C, T>(&self, chat_id: C, text: T) -> Self::SendMessageTF
    where
        C: Into<Recipient>,
        T: Into<String>,
    {
        Self::SendMessageTF::new(
            self.clone(),
            teloxide::payloads::SendMessage::new(chat_id, text),
        )
    }
}
