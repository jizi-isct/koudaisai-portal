use crate::middlewares::CurrentUser;
use crate::routes::AppState;
use crate::util::AppResponse;
use axum::extract::{ConnectInfo, Query};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Extension, Json, Router};
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::sync::Arc;
use tracing::instrument;

#[instrument(name = "init /api/v2/util")]
pub fn init_router() -> Router<Arc<AppState>> {
    Router::new().route("/meta", get(get_meta))
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Params {
    url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct MetaInfo {
    title: Option<String>,
    description: Option<String>,
}

#[instrument(name = "GET /api/v2/util/meta")]
async fn get_meta(
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    Extension(current_user): Extension<CurrentUser>,
    Query(params): Query<Params>,
) -> AppResponse {
    if !matches!(current_user, CurrentUser::Admin(_)) {
        return Ok((StatusCode::FORBIDDEN, "Forbidden".into_response()));
    }

    let client = reqwest::Client::new();
    let res = client
        .get(&params.url)
        .header("User-Agent", "Mozilla/5.0 (compatible; MyBot/1.0)")
        .send()
        .await;

    let html = match res {
        Ok(r) => match r.text().await {
            Ok(t) => t,
            Err(_) => {
                return Ok((
                    StatusCode::OK,
                    Json(MetaInfo {
                        title: None,
                        description: None,
                    })
                    .into_response(),
                ))
            }
        },
        Err(_) => {
            return Ok((
                StatusCode::OK,
                Json(MetaInfo {
                    title: None,
                    description: None,
                })
                .into_response(),
            ))
        }
    };

    let document = Html::parse_document(&html);

    let title = document
        .select(&Selector::parse("title").unwrap())
        .next()
        .map(|el| el.text().collect::<String>());

    let meta_description = Selector::parse("meta[name=\"description\"]").unwrap();
    let og_description = Selector::parse("meta[property=\"og:description\"]").unwrap();

    let description = document
        .select(&meta_description)
        .next()
        .and_then(|el| el.value().attr("content"))
        .or_else(|| {
            document
                .select(&og_description)
                .next()
                .and_then(|el| el.value().attr("content"))
        })
        .map(|s| s.to_string());

    Ok((
        StatusCode::OK,
        Json(MetaInfo { title, description }).into_response(),
    ))
}
