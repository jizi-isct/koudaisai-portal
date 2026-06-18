use crate::application::error::ApplicationError;
use crate::application::ports::discord::{Discord, DiscordMessage};
use anyhow::anyhow;
use async_trait::async_trait;
use serenity::builder::{CreateAttachment, CreateEmbed, ExecuteWebhook};
use serenity::http::Http;
use serenity::model::webhook::Webhook;

/// serenity の webhook を用いた [`Discord`] の実装。
///
/// `webhook_url` に `thread_id` クエリが含まれる場合は、そのスレッドへ投稿する。
pub struct WebhookDiscord {
    webhook_url: String,
    thread_id: Option<u64>,
}

impl WebhookDiscord {
    pub fn new(webhook_url: impl Into<String>) -> Self {
        let webhook_url = webhook_url.into();
        let thread_id = reqwest::Url::parse(&webhook_url).ok().and_then(|url| {
            url.query_pairs()
                .find(|(key, _)| key == "thread_id")
                .and_then(|(_, value)| value.parse::<u64>().ok())
        });
        Self {
            webhook_url,
            thread_id,
        }
    }
}

#[async_trait]
impl Discord for WebhookDiscord {
    async fn send(&self, message: &DiscordMessage) -> Result<(), ApplicationError> {
        // webhook の実行にボットトークンは不要(トークンは webhook URL から取得される)。
        let http = Http::new("");
        let webhook = Webhook::from_url(&http, &self.webhook_url)
            .await
            .map_err(|e| ApplicationError::InternalError(anyhow!(e.to_string())))?;

        let embeds = message
            .embeds
            .iter()
            .map(|embed| {
                let mut builder = CreateEmbed::new();
                if let Some(title) = &embed.title {
                    builder = builder.title(title.as_str());
                }
                if let Some(description) = &embed.description {
                    builder = builder.description(description.as_str());
                }
                if let Some(color) = embed.color {
                    builder = builder.color(color);
                }
                for field in &embed.fields {
                    builder = builder.field(field.name.as_str(), field.value.as_str(), field.inline);
                }
                builder
            })
            .collect::<Vec<_>>();

        let mut builder = ExecuteWebhook::new().embeds(embeds);
        if let Some(username) = &message.username {
            builder = builder.username(username.as_str());
        }
        for attachment in &message.attachments {
            builder = builder.add_file(CreateAttachment::bytes(
                attachment.bytes.clone(),
                attachment.file_name.as_str(),
            ));
        }
        if let Some(thread_id) = self.thread_id {
            builder = builder.in_thread(thread_id);
        }

        webhook
            .execute(&http, false, builder)
            .await
            .map_err(|e| ApplicationError::InternalError(anyhow!(e.to_string())))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_parses_thread_id_from_url_query() {
        let discord =
            WebhookDiscord::new("https://discord.com/api/webhooks/1/token?thread_id=42");
        assert_eq!(discord.thread_id, Some(42));
    }

    #[test]
    fn new_without_thread_id_query_is_none() {
        let discord = WebhookDiscord::new("https://discord.com/api/webhooks/1/token");
        assert_eq!(discord.thread_id, None);
    }
}
