use crate::application::error::ApplicationError;
use crate::application::ports::object_storage::ObjectStorage;
use async_trait::async_trait;
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
