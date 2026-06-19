use crate::application::error::ApplicationError;
use crate::application::ports::meta_fetcher::{MetaFetcher, PageMeta};
use async_trait::async_trait;
use scraper::{Html, Selector};

/// reqwest で HTML を取得し、`scraper` で OpenGraph / `<title>` / meta description を
/// 抽出する [`MetaFetcher`] 実装。
pub struct ReqwestMetaFetcher {
    client: reqwest::Client,
}

impl ReqwestMetaFetcher {
    pub fn new(client: reqwest::Client) -> Self {
        Self { client }
    }

    /// 既定の reqwest クライアントで構築する。
    pub fn from_config() -> Self {
        Self::new(reqwest::Client::new())
    }
}

impl Default for ReqwestMetaFetcher {
    fn default() -> Self {
        Self::from_config()
    }
}

/// `<meta property="og:xxx" content="...">` / `<meta name="xxx" content="...">` の content を引く。
fn meta_content(doc: &Html, attr: &str, value: &str) -> Option<String> {
    let selector = Selector::parse(&format!("meta[{attr}=\"{value}\"]")).ok()?;
    doc.select(&selector)
        .find_map(|el| el.value().attr("content"))
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}

#[async_trait]
impl MetaFetcher for ReqwestMetaFetcher {
    async fn fetch(&self, url: &str) -> Result<PageMeta, ApplicationError> {
        let resp = self
            .client
            .get(url)
            .send()
            .await
            .map_err(|e| ApplicationError::InternalError(anyhow::anyhow!(e.to_string())))?;
        let body = resp
            .text()
            .await
            .map_err(|e| ApplicationError::InternalError(anyhow::anyhow!(e.to_string())))?;

        // Html::parse は Send でない内部表現を持つため、`.await` をまたがないよう
        // 抽出をブロック内で完結させてから返す。
        let (title, description) = {
            let doc = Html::parse_document(&body);
            let title = meta_content(&doc, "property", "og:title").or_else(|| {
                let sel = Selector::parse("title").ok()?;
                doc.select(&sel)
                    .next()
                    .map(|el| el.text().collect::<String>().trim().to_string())
                    .filter(|s| !s.is_empty())
            });
            let description = meta_content(&doc, "property", "og:description")
                .or_else(|| meta_content(&doc, "name", "description"));
            (title, description)
        };

        Ok(PageMeta { title, description })
    }
}
