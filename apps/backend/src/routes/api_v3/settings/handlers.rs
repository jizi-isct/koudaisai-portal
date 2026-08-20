use super::super::V3State;
use super::dto::{SettingsRead, ShowOccasionsOnPortalRead, ShowOccasionsOnPortalUpdate};
use crate::application::error::ApplicationOperationError;
use crate::domain::actor_ctx::ActorContext;
use axum::Json;
use axum::extract::State;
use utoipa_axum_auto_into_response::http_response;

#[http_response]
pub enum GetSettingsResponse {
    #[response(status = OK, description = "Global settings")]
    Ok(SettingsRead),
    #[response(status = FORBIDDEN, description = "Forbidden")]
    Forbidden,
    #[response(status = INTERNAL_SERVER_ERROR, description = "Internal server error")]
    InternalServerError,
}

#[utoipa::path(
    get,
    description = "Get all global settings. Requires the settings:read admin permission.",
    path = "/",
    responses(GetSettingsResponse),
    tag = super::super::SETTINGS_TAG
)]
pub async fn get_settings(State(st): State<V3State>, actor: ActorContext) -> GetSettingsResponse {
    match st.app.settings().get_all(&actor).await {
        Ok(settings) => GetSettingsResponse::Ok(SettingsRead::from(settings)),
        Err(ApplicationOperationError::Unauthorized) => GetSettingsResponse::Forbidden,
        Err(_) => GetSettingsResponse::InternalServerError,
    }
}

#[http_response]
pub enum GetShowOccasionsOnPortalResponse {
    #[response(status = OK, description = "Whether occasion locations are visible to groups")]
    Ok(ShowOccasionsOnPortalRead),
    #[response(status = FORBIDDEN, description = "Forbidden")]
    Forbidden,
    #[response(status = INTERNAL_SERVER_ERROR, description = "Internal server error")]
    InternalServerError,
}

#[utoipa::path(
    get,
    description = "Get whether occasion locations are visible to groups.",
    path = "/show-occasions-on-portal",
    responses(GetShowOccasionsOnPortalResponse),
    tag = super::super::SETTINGS_TAG
)]
pub async fn get_show_occasions_on_portal(
    State(st): State<V3State>,
    actor: ActorContext,
) -> GetShowOccasionsOnPortalResponse {
    match st.app.settings().get_show_occasions_on_portal(&actor).await {
        Ok(value) => GetShowOccasionsOnPortalResponse::Ok(value.into()),
        Err(ApplicationOperationError::Unauthorized) => GetShowOccasionsOnPortalResponse::Forbidden,
        Err(_) => GetShowOccasionsOnPortalResponse::InternalServerError,
    }
}

#[http_response]
pub enum PatchShowOccasionsOnPortalResponse {
    #[response(status = OK, description = "Occasion location visibility updated")]
    Ok(ShowOccasionsOnPortalRead),
    #[response(status = FORBIDDEN, description = "Forbidden")]
    Forbidden,
    #[response(status = INTERNAL_SERVER_ERROR, description = "Internal server error")]
    InternalServerError,
}

#[utoipa::path(
    patch,
    description = "Change whether occasion locations are visible to groups. Requires the settings:write admin permission.",
    path = "/show-occasions-on-portal",
    responses(PatchShowOccasionsOnPortalResponse),
    request_body = ShowOccasionsOnPortalUpdate,
    tag = super::super::SETTINGS_TAG
)]
pub async fn patch_show_occasions_on_portal(
    State(st): State<V3State>,
    actor: ActorContext,
    Json(body): Json<ShowOccasionsOnPortalUpdate>,
) -> PatchShowOccasionsOnPortalResponse {
    match st
        .app
        .settings()
        .change_show_occasions_on_portal(&actor, body.show_occasions_on_portal)
        .await
    {
        Ok(settings) => {
            PatchShowOccasionsOnPortalResponse::Ok(settings.show_occasions_on_portal().into())
        }
        Err(ApplicationOperationError::Unauthorized) => {
            PatchShowOccasionsOnPortalResponse::Forbidden
        }
        Err(_) => PatchShowOccasionsOnPortalResponse::InternalServerError,
    }
}
