use crate::application::error::ApplicationError;
use std::time::Duration;

/// オブジェクトストレージ(S3 等)へのゲートウェイ。
/// ファイル本体は永続化集約ではなく外部ストレージに置かれるため、
/// リポジトリではなくポート(email と同種の外部サービス抽象)として定義する。
#[async_trait::async_trait]
pub trait ObjectStorage {
    /// 指定キーへアップロードするためのプレサインド URL(PUT)を発行する。
    async fn presigned_upload_url(
        &self,
        key: &str,
        expires_in: Duration,
    ) -> Result<String, ApplicationError>;

    /// 指定キーをダウンロードするためのプレサインド URL(GET)を発行する。
    /// `file_name` はレスポンスの Content-Disposition に用いる。
    async fn presigned_download_url(
        &self,
        key: &str,
        file_name: &str,
        expires_in: Duration,
    ) -> Result<String, ApplicationError>;

    /// 指定キーのオブジェクト本体(バイト列)をサーバーサイドで取得する。
    /// Discord への添付など、URL ではなく実体が必要な場合に用いる。
    async fn get_object(&self, key: &str) -> Result<Vec<u8>, ApplicationError>;
}
