use crate::application::error::ApplicationError;

/// Discord 埋め込み(embed)のフィールド。
#[derive(Debug, Clone)]
pub struct DiscordEmbedField {
    pub name: String,
    pub value: String,
    pub inline: bool,
}

/// Discord メッセージに付与する埋め込み(embed)。
#[derive(Debug, Clone, Default)]
pub struct DiscordEmbed {
    pub title: Option<String>,
    pub description: Option<String>,
    /// `0xRRGGBB` 形式の色。
    pub color: Option<u32>,
    pub fields: Vec<DiscordEmbedField>,
}

/// Discord メッセージに添付するファイル。
#[derive(Debug, Clone)]
pub struct DiscordAttachment {
    pub file_name: String,
    pub bytes: Vec<u8>,
}

/// Discord webhook へ送信する 1 メッセージ。
#[derive(Debug, Clone, Default)]
pub struct DiscordMessage {
    /// webhook 既定の表示名を上書きする送信者名。
    pub username: Option<String>,
    pub embeds: Vec<DiscordEmbed>,
    pub attachments: Vec<DiscordAttachment>,
}

/// Discord webhook へのゲートウェイ。
/// `email` / `object_storage` と同種の外部サービス抽象としてポート定義する。
/// メッセージの組み立て(承認申請の埋め込み等)は application 層が担い、
/// 本ポートは設定済み webhook への送信のみを責務とする。
#[async_trait::async_trait]
pub trait Discord {
    /// 設定済みの webhook へメッセージを送信する。
    async fn send(&self, message: &DiscordMessage) -> Result<(), ApplicationError>;
}
