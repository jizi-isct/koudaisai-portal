use super::super::ErrorMessage;
use super::dto::MetaInfo;
use crate::application::error::ApplicationError;
use crate::domain::actor_ctx::ActorContext;
use axum::extract::{Query, State};
use serde::Deserialize;
use utoipa::IntoParams;
use utoipa_axum_auto_into_response::http_response;

/// メタ情報取得のクエリパラメーター。
#[derive(Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct MetaQuery {
    /// メタ情報を取得する対象 URL。
    url: String,
}

#[http_response]
pub enum GetMetaResponse {
    #[response(status = OK, description = "Returns the page title and description")]
    Ok(MetaInfo),
    #[response(status = FORBIDDEN, description = "Forbidden")]
    Forbidden(ErrorMessage),
    #[response(status = INTERNAL_SERVER_ERROR, description = "Internal server error")]
    InternalServerError(ErrorMessage),
}

#[utoipa::path(
    get,
    description = "Fetch OpenGraph/HTML metadata (title, description) for a URL.",
    params(MetaQuery),
    path = "/meta",
    responses(GetMetaResponse),
    tag = super::super::UTIL_TAG
)]
pub async fn get_meta(
    State(st): State<super::super::V3State>,
    actor: ActorContext,
    Query(query): Query<MetaQuery>,
) -> GetMetaResponse {
    match st.app.meta().get_meta(&actor, &query.url).await {
        Ok(pm) => GetMetaResponse::Ok(MetaInfo::from(pm)),
        Err(ApplicationError::Unauthorized) => {
            GetMetaResponse::Forbidden(ErrorMessage::forbidden())
        }
        Err(_) => GetMetaResponse::InternalServerError(ErrorMessage::internal_server_error()),
    }
}
