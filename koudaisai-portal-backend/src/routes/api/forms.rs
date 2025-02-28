use crate::entities::prelude::{ExhibitorsRoot, Forms, Users};
use crate::entities::{exhibitors_root, form_responses, forms, users};
use crate::forms::responses::{Answer, FormResponse};
use crate::forms::{AccessControl, Form, Info, Item};
use crate::middlewares::UserId;
use crate::routes::AppState;
use axum::extract::{ConnectInfo, Path, State};
use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::{Extension, Json, Router};
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveEnum, ActiveModelBehavior, ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait,
    QueryFilter,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use std::iter::Map;
use std::net::SocketAddr;
use std::sync::Arc;
use tracing::{info, instrument, warn};
use uuid::Uuid;

#[instrument(name = "init /api/v1/forms")]
pub fn init_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("", get(get_forms).post(post_forms))
        .route("/{form_id}/responses", post(post_response))
}

#[instrument(name = "GET /api/v1/forms", skip(state))]
async fn get_forms(
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<Arc<AppState>>,
    Extension(user_id): Extension<UserId>,
) -> Result<String, StatusCode> {
    let forms = match user_id {
        UserId::Admin(_) => Forms::find().all(&state.db_conn).await,
        UserId::User(uuid) => {
            let user = Users::find_by_id(uuid).one(&state.db_conn).await;
            let user = match user {
                Ok(user) => user,
                Err(err) => {
                    warn!("internal server error occurred while finding user: {}", err);
                    return Err(StatusCode::INTERNAL_SERVER_ERROR);
                }
            };
            let user = match user {
                Some(user) => user,
                None => {
                    warn!("internal server error occurred: user doesn't exist");
                    return Err(StatusCode::INTERNAL_SERVER_ERROR);
                }
            };
            let exhibition_id = user.exhibition_id;
            let exhibitor = ExhibitorsRoot::find_by_id(exhibition_id)
                .one(&state.db_conn)
                .await;
            let exhibitor = match exhibitor {
                Ok(exhibitor) => exhibitor,
                Err(err) => {
                    warn!(
                        "internal server error occurred while finding exhibitor: {}",
                        err
                    );
                    return Err(StatusCode::INTERNAL_SERVER_ERROR);
                }
            };
            let exhibitor = match exhibitor {
                Some(exhibitor) => exhibitor,
                None => {
                    warn!("internal server error occurred: exhibitor doesn't exist");
                    return Err(StatusCode::INTERNAL_SERVER_ERROR);
                }
            };
            let exhibitor_type = exhibitor.r#type.into_value().to_string();
            Forms::find()
                .filter(forms::Column::AccessControlRoles.contains(exhibitor_type))
                .all(&state.db_conn)
                .await
        }
        UserId::None => {
            Forms::find()
                .filter(forms::Column::AccessControlRoles.contains("none"))
                .all(&state.db_conn)
                .await
        }
    };

    let form_models = match forms {
        Ok(forms) => forms,
        Err(err) => {
            warn!(
                "internal server error occurred while finding forms: {}",
                err
            );
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let mut forms = vec![];
    for form_model in form_models {
        let form = match Form::from_model(&form_model) {
            Ok(form) => form,
            Err(err) => {
                warn!(
                    "internal server error occurred while parsing form: {:?}",
                    err
                );
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
        };
        forms.push(form);
    }
    let json = match serde_json::to_string(&forms) {
        Ok(json) => json,
        Err(err) => {
            warn!(
                "internal server error occurred while serializing the result: {:?}",
                err
            );
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    Ok(json)
}

#[derive(Serialize, Deserialize, Debug)]
struct NewForm {
    info: Info,
    items: Vec<Item>,
    access_control: AccessControl,
}

#[instrument(name = "POST /api/v1/forms", skip(state))]
async fn post_forms(
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<Arc<AppState>>,
    Extension(user_id): Extension<UserId>,
    Json(new_form): Json<NewForm>,
) -> Result<(StatusCode, Json<Form>), StatusCode> {
    if let UserId::Admin(uuid) = user_id {
        let model = forms::ActiveModel {
            form_id: ActiveValue::NotSet,
            created_at: ActiveValue::NotSet,
            updated_at: ActiveValue::NotSet,
            info: Set(json!(new_form.info)),
            items: Set(json!(new_form.items)),
            access_control_roles: Set(new_form.access_control.roles),
        };
        match model.insert(&state.db_conn).await {
            Ok(model) => {
                let form = match Form::from_model(&model) {
                    Ok(form) => form,
                    Err(err) => {
                        warn!("Internal server error while parce form: {}", err);
                        return Err(StatusCode::INTERNAL_SERVER_ERROR);
                    }
                };
                info!("new form was added by {}", uuid);
                Ok((StatusCode::ACCEPTED, Json::from(form)))
            }
            Err(err) => {
                warn!("Internal server error while inserting new form: {}", err);
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    } else {
        Err(StatusCode::FORBIDDEN)
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct NewResponse {
    answers: HashMap<String, Answer>,
}

#[instrument(name = "POST /api/v1/forms/{form_id}/responses", skip(state))]
async fn post_response(
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<Arc<AppState>>,
    Extension(user_id): Extension<UserId>,
    Path(form_id): Path<Uuid>,
    Json(new_response): Json<NewResponse>,
) -> Result<(StatusCode, Json<FormResponse>), StatusCode> {
    if let UserId::User(uuid) = user_id {
        // 回答権があるかどうか確認
        let user = match users::Entity::find_by_id(uuid).one(&state.db_conn).await {
            Ok(Some(model)) => model,
            Ok(None) => {
                return Err(StatusCode::NOT_FOUND);
            }
            Err(err) => {
                warn!(
                    "internal server error occurred while selecting user: {:?}",
                    err
                );
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
        };
        let exhibitor = match exhibitors_root::Entity::find_by_id(user.exhibition_id)
            .one(&state.db_conn)
            .await
        {
            Ok(Some(model)) => model,
            Ok(None) => {
                return Err(StatusCode::NOT_FOUND);
            }
            Err(err) => {
                warn!(
                    "internal server error occurred while selecting exhibitor: {:?}",
                    err
                );
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
        };
        let form = match forms::Entity::find_by_id(form_id).one(&state.db_conn).await {
            Ok(Some(model)) => model,
            Ok(None) => {
                return Err(StatusCode::NOT_FOUND);
            }
            Err(err) => {
                warn!(
                    "internal server error occurred while selecting form: {:?}",
                    err
                );
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
        };
        if !form
            .access_control_roles
            .contains(&exhibitor.r#type.into_value().to_string())
        {
            return Err(StatusCode::FORBIDDEN);
        }

        let response = form_responses::ActiveModel {
            response_id: ActiveValue::NotSet,
            created_at: ActiveValue::NotSet,
            updated_at: ActiveValue::NotSet,
            form_id: Set(form_id),
            respondent_id: Set(uuid),
            answers: Set(json!(new_response.answers)),
        };
        match response.insert(&state.db_conn).await {
            Ok(model) => {
                let response = match FormResponse::from_model(&model) {
                    Ok(response) => response,
                    Err(err) => {
                        warn!("Internal server error while converting response: {}", err);
                        return Err(StatusCode::INTERNAL_SERVER_ERROR);
                    }
                };
                info!("new form was added by {}", uuid);
                Ok((StatusCode::ACCEPTED, Json::from(response)))
            }
            Err(err) => {
                warn!("Internal server error while inserting new form: {}", err);
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    } else {
        Err(StatusCode::FORBIDDEN)
    }
}
