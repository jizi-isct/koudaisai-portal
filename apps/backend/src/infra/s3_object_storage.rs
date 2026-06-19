use crate::application::error::ApplicationError;
use crate::application::ports::object_storage::ObjectStorage;
use async_trait::async_trait;
use aws_config::BehaviorVersion;
use aws_sdk_s3::config::Credentials;
use aws_sdk_s3::presigning::PresigningConfig;
use std::time::Duration;

/// aws-sdk-s3 を用いた [`ObjectStorage`] の実装。
pub struct S3ObjectStorage {
    client: aws_sdk_s3::Client,
    bucket: String,
}

impl S3ObjectStorage {
    pub fn new(client: aws_sdk_s3::Client, bucket: String) -> Self {
        Self { client, bucket }
    }

    /// S3 設定から aws-sdk-s3 クライアントを構築する(path-style・固定リージョン)。
    pub async fn from_config(cfg: &crate::config::S3) -> Self {
        let shared_cfg = aws_config::defaults(BehaviorVersion::latest())
            .credentials_provider(
                Credentials::builder()
                    .access_key_id(cfg.access_key_id.clone())
                    .secret_access_key(cfg.secret_access_key.clone())
                    .provider_name("default-provider")
                    .build(),
            )
            .endpoint_url(cfg.endpoint.clone())
            .region("ap-northeast-1")
            .load()
            .await;

        let s3_cfg = aws_sdk_s3::config::Builder::from(&shared_cfg)
            .force_path_style(true)
            .build();

        Self::new(aws_sdk_s3::Client::from_conf(s3_cfg), cfg.bucket.clone())
    }
}

#[async_trait]
impl ObjectStorage for S3ObjectStorage {
    async fn presigned_upload_url(
        &self,
        key: &str,
        expires_in: Duration,
    ) -> Result<String, ApplicationError> {
        let presigned = self
            .client
            .put_object()
            .bucket(&self.bucket)
            .key(key)
            .presigned(
                PresigningConfig::builder()
                    .expires_in(expires_in)
                    .build()
                    .map_err(|e| ApplicationError::InternalError(anyhow::anyhow!(e.to_string())))?,
            )
            .await
            .map_err(|e| ApplicationError::InternalError(anyhow::anyhow!(e.to_string())))?;

        Ok(presigned.uri().to_string())
    }

    async fn presigned_download_url(
        &self,
        key: &str,
        file_name: &str,
        expires_in: Duration,
    ) -> Result<String, ApplicationError> {
        let presigned = self
            .client
            .get_object()
            .bucket(&self.bucket)
            .key(key)
            .response_content_disposition(format!(r#"attachment; filename="{}""#, file_name))
            .presigned(
                PresigningConfig::builder()
                    .expires_in(expires_in)
                    .build()
                    .map_err(|e| ApplicationError::InternalError(anyhow::anyhow!(e.to_string())))?,
            )
            .await
            .map_err(|e| ApplicationError::InternalError(anyhow::anyhow!(e.to_string())))?;

        Ok(presigned.uri().to_string())
    }

    async fn get_object(&self, key: &str) -> Result<Vec<u8>, ApplicationError> {
        let output = self
            .client
            .get_object()
            .bucket(&self.bucket)
            .key(key)
            .send()
            .await
            .map_err(|e| ApplicationError::InternalError(anyhow::anyhow!(e.to_string())))?;

        let bytes = output
            .body
            .collect()
            .await
            .map_err(|e| ApplicationError::InternalError(anyhow::anyhow!(e.to_string())))?
            .into_bytes();

        Ok(bytes.to_vec())
    }
}
