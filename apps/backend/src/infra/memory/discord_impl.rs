use crate::application::error::ApplicationError;
use crate::application::ports::discord::{Discord, DiscordMessage};
use anyhow::anyhow;
use async_trait::async_trait;
use std::sync::{Arc, RwLock};

/// テスト用のインメモリ [`Discord`] 実装。
/// 外部送信は行わず、送信されたメッセージを記録するだけ。
pub struct MemoryDiscord {
    sent_messages: Arc<RwLock<Vec<DiscordMessage>>>,
}

impl Default for MemoryDiscord {
    fn default() -> Self {
        Self::new()
    }
}

impl MemoryDiscord {
    pub fn new() -> Self {
        Self {
            sent_messages: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// これまでに送信されたメッセージの一覧を返す。
    pub fn sent_messages(&self) -> Vec<DiscordMessage> {
        self.sent_messages.read().unwrap().clone()
    }
}

#[async_trait]
impl Discord for MemoryDiscord {
    async fn send(&self, message: &DiscordMessage) -> Result<(), ApplicationError> {
        let mut sent_messages = self
            .sent_messages
            .write()
            .map_err(|e| ApplicationError::InternalError(anyhow!(e.to_string())))?;
        sent_messages.push(message.clone());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::ports::discord::DiscordEmbed;

    #[tokio::test]
    async fn send_records_the_message() {
        let discord = MemoryDiscord::new();
        let message = DiscordMessage {
            username: Some("tester".to_string()),
            embeds: vec![DiscordEmbed {
                title: Some("hello".to_string()),
                ..Default::default()
            }],
            attachments: vec![],
        };

        discord.send(&message).await.expect("send should succeed");

        let sent = discord.sent_messages();
        assert_eq!(sent.len(), 1);
        assert_eq!(sent[0].username.as_deref(), Some("tester"));
        assert_eq!(sent[0].embeds.len(), 1);
    }
}
