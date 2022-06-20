use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Message {
    Chat(String),
    Reload,
}

pub enum MessageError {
    Error,
}

impl Message {
    pub fn from_serialized(source: String) -> Result<Self, MessageError> {
        match serde_json::from_str(source.as_str()) {
            Ok(thas) => Ok(thas),
            _ => Err(MessageError::Error),
        }
    }

    pub fn to_serialized(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn serialieze_chat_message() {
        let m = crate::Message::Chat("message content".to_string());
        let serialised = m.to_serialized();

        assert_eq!(
            m,
            crate::Message::from_serialized(serialised)
                .unwrap_or(crate::Message::Chat("".to_string()))
        );
    }

    #[test]
    fn serialieze_chat_message2() {
        let m = crate::Message::Chat("next message".to_string());
        let serialised = m.to_serialized();

        assert_eq!(
            m,
            crate::Message::from_serialized(serialised)
                .unwrap_or(crate::Message::Chat("".to_string()))
        );
    }
}
