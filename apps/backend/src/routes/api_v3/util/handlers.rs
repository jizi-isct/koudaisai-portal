use super::dto::MetaInfo;
use axum::extract::Query;
use serde::Deserialize;
use utoipa::IntoParams;
use utoipa_axum_auto_into_response::http_response;

/// メタ情報取得のクエリパラメーター。
#[derive(Deserialize, IntoParams)]
pub struct MetaQuery {
    /// メタ情報を取得する対象 URL。
    url: String,
}

#[http_response]
pub enum GetMetaResponse {
    #[response(status = OK, description = "Returns the page title and description")]
    Ok(MetaInfo),
    #[response(status = FORBIDDEN, description = "Forbidden")]
    Forbidden,
    #[response(status = INTERNAL_SERVER_ERROR, description = "Internal server error")]
    InternalServerError,
}

#[utoipa::path(
    get,
    description = "Fetch OpenGraph/HTML metadata (title, description) for a URL.",
    params(MetaQuery),
    path = "/meta",
    responses(GetMetaResponse),
    tag = super::super::UTIL_TAG
)]
pub async fn get_meta(Query(query): Query<MetaQuery>) -> GetMetaResponse {
    todo!()
}
