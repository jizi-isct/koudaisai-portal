use crate::application::authz;
use crate::application::error::ApplicationError;
use crate::application::ports::object_storage::ObjectStorage;
use crate::domain::actor_ctx::ActorContext;
use std::time::Duration;
use uuid::Uuid;

/// presigned URL の有効期限(レガシー同様に 10 分)。
const PRESIGN_EXPIRY: Duration = Duration::from_secs(60 * 10);

/// アップロード用に発行したストレージキーと presigned URL の組。
pub struct UploadTicket {
    pub key: String,
    pub presigned_url: String,
}

pub struct FileApp<'a, OS: ObjectStorage> {
    object_storage: &'a OS,
}

impl<'a, OS: ObjectStorage> FileApp<'a, OS> {
    pub fn new(object_storage: &'a OS) -> Self {
        Self { object_storage }
    }

    /// 認可後、"{uuid}-{file_name}" のキーを生成しアップロード用 presigned URL を返す。
    pub async fn request_upload(
        &self,
        actor_ctx: &ActorContext,
        file_name: &str,
    ) -> Result<UploadTicket, ApplicationError> {
        // auth
        if !authz::can_upload_file(actor_ctx) {
            return Err(ApplicationError::Unauthorized);
        }

        let key = format!("{}-{}", Uuid::new_v4(), file_name);
        let presigned_url = self
            .object_storage
            .presigned_upload_url(&key, PRESIGN_EXPIRY)
            .await?;

        Ok(UploadTicket { key, presigned_url })
    }

    /// 認可後、指定キーのダウンロード用 presigned URL を返す。
    pub async fn request_download(
        &self,
        actor_ctx: &ActorContext,
        key: &str,
        file_name: &str,
    ) -> Result<String, ApplicationError> {
        // auth
        if !authz::can_download_file(actor_ctx) {
            return Err(ApplicationError::Unauthorized);
        }

        self.object_storage
            .presigned_download_url(key, file_name, PRESIGN_EXPIRY)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::user_id::UserId;

    /// テスト専用の ObjectStorage 実装(インフラ層に依存しない自己完結フェイク)。
    struct FakeObjectStorage;

    #[async_trait::async_trait]
    impl ObjectStorage for FakeObjectStorage {
        async fn presigned_upload_url(
            &self,
            key: &str,
            _expires_in: Duration,
        ) -> Result<String, ApplicationError> {
            Ok(format!("https://fake/{key}"))
        }

        async fn presigned_download_url(
            &self,
            key: &str,
            _file_name: &str,
            _expires_in: Duration,
        ) -> Result<String, ApplicationError> {
            Ok(format!("https://fake/{key}"))
        }
    }

    fn admin_ctx() -> ActorContext {
        ActorContext::Admin {
            name: "テストユーザー".to_string(),
            user_id: UserId::new(Uuid::new_v4()),
            claims: vec![],
        }
    }

    #[tokio::test]
    async fn request_upload_as_admin_returns_key_and_url() {
        let storage = FakeObjectStorage;
        let app = FileApp::new(&storage);

        let file_name = "example.pdf";
        let ticket = app
            .request_upload(&admin_ctx(), file_name)
            .await
            .expect("admin should be allowed to request upload");

        assert!(ticket.key.ends_with(file_name));
        assert!(!ticket.presigned_url.is_empty());
    }

    #[tokio::test]
    async fn request_upload_as_nologin_is_unauthorized() {
        let storage = FakeObjectStorage;
        let app = FileApp::new(&storage);

        let result = app
            .request_upload(&ActorContext::NoLogin, "example.pdf")
            .await;
        assert!(matches!(result, Err(ApplicationError::Unauthorized)));
    }

    #[tokio::test]
    async fn request_download_is_open_to_nologin() {
        let storage = FakeObjectStorage;
        let app = FileApp::new(&storage);

        // ダウンロードは無認証で許可される。
        let result = app
            .request_download(&ActorContext::NoLogin, "some-key", "example.pdf")
            .await;
        assert!(result.is_ok());
    }
}
