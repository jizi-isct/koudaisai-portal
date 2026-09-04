use crate::application::error::{DeleteError, InsertError, UpdateError};
use crate::application::ports::events26_api::{Events26Api, UpdateIconError, UpdateMenuError};
use anyhow::{Context, anyhow};
use async_trait::async_trait;
use events26_api::apis::admin_api::{
    CreateProjectParams, DeleteProjectIconParams, DeleteProjectMenuParams, DeleteProjectParams,
    UpdateProjectDescriptionParams, UpdateProjectMenuParams, UpdateProjectParams, create_project,
    delete_project, delete_project_icon, delete_project_menu, update_project,
    update_project_description, update_project_menu,
};
use events26_api::apis::configuration::Configuration;
use events26_api::apis::{Error, ResponseContent, urlencode};
use events26_api::models::{GetProjectDetails200ResponseMenu, Project, ProjectDescription};
use reqwest::header::{HeaderMap, HeaderValue};
use tracing::warn;

const CF_ACCESS_CLIENT_ID: &str = "CF-Access-Client-Id";
const CF_ACCESS_CLIENT_SECRET: &str = "CF-Access-Client-Secret";

/// OpenAPI 仕様から生成した `events26_api` クライアントを用いた [`Events26Api`] の実装。
///
/// `/admin/v1` 配下は Cloudflare Access で保護されているため、サービストークンを
/// 既定ヘッダとして全リクエストに付与する。生成クライアントの [`Configuration`] は
/// 任意ヘッダを持てないので、`reqwest::Client` 側の `default_headers` に載せている。
pub struct Events26ApiClient {
    configuration: Configuration,
}

impl Events26ApiClient {
    /// ベース URL と Cloudflare Access のサービストークンから構築する。
    ///
    /// トークン未設定でも構築は成功させ、警告だけ出す(S3 / SES / Discord と同じ扱い)。
    /// ローカル開発では未設定が普通で、ここで落とすと関係ない機能まで起動しなくなるため。
    /// 実際の呼び出しは Access に 403 で弾かれ `InternalError` になる。
    pub fn new(
        base_url: impl Into<String>,
        client_id: &str,
        client_secret: &str,
    ) -> anyhow::Result<Self> {
        let mut headers = HeaderMap::new();
        if client_id.is_empty() || client_secret.is_empty() {
            warn!(
                "events26 API の Cloudflare Access サービストークンが未設定です。\
                 /admin 配下の呼び出しは失敗します"
            );
        } else {
            headers.insert(
                CF_ACCESS_CLIENT_ID,
                HeaderValue::from_str(client_id).context("invalid CF-Access-Client-Id")?,
            );
            let mut secret =
                HeaderValue::from_str(client_secret).context("invalid CF-Access-Client-Secret")?;
            secret.set_sensitive(true);
            headers.insert(CF_ACCESS_CLIENT_SECRET, secret);
        }

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .context("failed to build reqwest client for events26 API")?;

        Ok(Self {
            configuration: Configuration {
                base_path: base_url.into().trim_end_matches('/').to_string(),
                client,
                ..Configuration::default()
            },
        })
    }

    /// 設定(接続先 + 機密情報)から構築する。
    pub fn from_config(
        cfg: &crate::config::Events26,
        secrets: &crate::config::Secrets,
    ) -> anyhow::Result<Self> {
        Self::new(
            cfg.base_url.clone(),
            secrets.plans_info_api_client_id.as_ref(),
            secrets.plans_info_api_client_secret.as_ref(),
        )
    }
}

/// 生成クライアントのエラーから HTTP ステータスを取り出す。
/// 通信・デシリアライズ失敗など、レスポンスに至らなかった場合は `None`。
fn status_code<T>(error: &Error<T>) -> Option<u16> {
    match error {
        Error::ResponseError(ResponseContent { status, .. }) => Some(status.as_u16()),
        _ => None,
    }
}

/// 生成クライアントのエラーからレスポンス本文を取り出す。
fn response_body<T>(error: &Error<T>) -> Option<&str> {
    match error {
        Error::ResponseError(ResponseContent { content, .. }) => Some(content),
        _ => None,
    }
}

/// ステータスと本文を残したうえで `anyhow::Error` に落とす。
/// 生成エラー型は `Display` しか持たず本文が消えるため、ここで組み立てる。
fn internal_error<T>(operation: &str, error: Error<T>) -> anyhow::Error {
    match error {
        Error::ResponseError(ResponseContent {
            status, content, ..
        }) => anyhow!("events26 API {operation} failed with status {status}: {content}"),
        other => anyhow!("events26 API {operation} failed: {other}"),
    }
}

#[async_trait]
impl Events26Api for Events26ApiClient {
    async fn create_project(&self, project: &Project) -> Result<Project, InsertError> {
        create_project(
            &self.configuration,
            CreateProjectParams {
                project: project.clone(),
            },
        )
        .await
        .map_err(|e| match status_code(&e) {
            Some(409) => InsertError::Conflict,
            _ => InsertError::InternalError(internal_error("create_project", e)),
        })
    }

    async fn update_project(
        &self,
        project_id: &str,
        project: &Project,
    ) -> Result<Project, UpdateError> {
        update_project(
            &self.configuration,
            UpdateProjectParams {
                project_id: project_id.to_string(),
                project: project.clone(),
            },
        )
        .await
        .map_err(|e| match status_code(&e) {
            Some(404) => UpdateError::NotFound,
            _ => UpdateError::InternalError(internal_error("update_project", e)),
        })
    }

    async fn update_project_description(
        &self,
        project_id: &str,
        description: &str,
    ) -> Result<(), UpdateError> {
        update_project_description(
            &self.configuration,
            UpdateProjectDescriptionParams {
                project_id: project_id.to_string(),
                project_description: ProjectDescription {
                    description: description.to_string(),
                },
            },
        )
        .await
        .map(|_| ())
        .map_err(|e| match status_code(&e) {
            Some(404) => UpdateError::NotFound,
            _ => UpdateError::InternalError(internal_error("update_project_description", e)),
        })
    }

    async fn delete_project(&self, project_id: &str) -> Result<(), DeleteError> {
        delete_project(
            &self.configuration,
            DeleteProjectParams {
                project_id: project_id.to_string(),
            },
        )
        .await
        .map_err(|e| match status_code(&e) {
            Some(404) => DeleteError::NotFound,
            _ => DeleteError::InternalError(internal_error("delete_project", e)),
        })
    }

    /// 生成クライアントの `update_project_icon` はボディをファイルパスから読む形
    /// (`std::path::PathBuf`)で、`Content-Type` も付けない。ポータルは受け取った
    /// リクエストボディをメモリ上でそのまま中継したいので、ここだけ手で組み立てる。
    async fn update_project_icon(
        &self,
        project_id: &str,
        content_type: &str,
        image: Vec<u8>,
    ) -> Result<(), UpdateIconError> {
        let url = format!(
            "{}/admin/v1/projects/{}/icon",
            self.configuration.base_path,
            urlencode(project_id)
        );

        let response = self
            .configuration
            .client
            .put(&url)
            .header(reqwest::header::CONTENT_TYPE, content_type)
            .body(image)
            .send()
            .await
            .map_err(|e| anyhow!("events26 API update_project_icon failed: {e}"))?;

        let status = response.status();
        if status.is_success() {
            return Ok(());
        }

        // 本文は {"message": "..."} 形式。読めなくても握りつぶさずステータスは返す。
        let body = response.text().await.unwrap_or_default();
        match status.as_u16() {
            404 => Err(UpdateIconError::NotFound),
            // 400 空 / 413 サイズ超過 / 415 非対応形式 / 422 正方形でない。
            // いずれも送った画像が悪いので、理由をそのまま呼び出し側へ返す。
            400 | 413 | 415 | 422 => Err(UpdateIconError::InvalidImage(body)),
            _ => Err(UpdateIconError::InternalError(anyhow!(
                "events26 API update_project_icon failed with status {status}: {body}"
            ))),
        }
    }

    async fn delete_project_icon(&self, project_id: &str) -> Result<(), DeleteError> {
        delete_project_icon(
            &self.configuration,
            DeleteProjectIconParams {
                project_id: project_id.to_string(),
            },
        )
        .await
        .map_err(|e| match status_code(&e) {
            Some(404) => DeleteError::NotFound,
            _ => DeleteError::InternalError(internal_error("delete_project_icon", e)),
        })
    }

    async fn update_project_menu(
        &self,
        project_id: &str,
        menu: &GetProjectDetails200ResponseMenu,
    ) -> Result<(), UpdateMenuError> {
        update_project_menu(
            &self.configuration,
            UpdateProjectMenuParams {
                project_id: project_id.to_string(),
                get_project_details200_response_menu: menu.clone(),
            },
        )
        .await
        .map_err(|e| match status_code(&e) {
            Some(400) => {
                UpdateMenuError::InvalidMenu(response_body(&e).unwrap_or_default().to_string())
            }
            Some(404) => UpdateMenuError::NotFound,
            _ => UpdateMenuError::InternalError(internal_error("update_project_menu", e)),
        })
    }

    async fn delete_project_menu(&self, project_id: &str) -> Result<(), DeleteError> {
        delete_project_menu(
            &self.configuration,
            DeleteProjectMenuParams {
                project_id: project_id.to_string(),
            },
        )
        .await
        .map_err(|e| match status_code(&e) {
            Some(404) => DeleteError::NotFound,
            _ => DeleteError::InternalError(internal_error("delete_project_menu", e)),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{Events26, Secret, Secrets};

    #[test]
    fn new_tolerates_unset_service_token() {
        // 未設定でも起動できること(呼び出し時に Access が弾く)。
        assert!(Events26ApiClient::new("https://events26.koudaisai.jp", "", "").is_ok());
    }

    #[test]
    fn from_config_uses_configured_base_url_without_trailing_slash() {
        let cfg = Events26 {
            base_url: "https://events26-staging.koudaisai.jp/".to_string(),
        };
        let secrets = Secrets {
            plans_info_api_client_id: Secret::String("id".to_string()),
            plans_info_api_client_secret: Secret::String("secret".to_string()),
            ..Secrets::default()
        };
        let client = Events26ApiClient::from_config(&cfg, &secrets).unwrap();
        assert_eq!(
            client.configuration.base_path,
            "https://events26-staging.koudaisai.jp"
        );
    }

    #[test]
    fn default_base_url_points_at_production() {
        let cfg = Events26::default();
        assert_eq!(cfg.base_url, "https://events26.koudaisai.jp");
        assert_eq!(cfg.host().as_deref(), Some("events26.koudaisai.jp"));
    }
}
