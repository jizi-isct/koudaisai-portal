use crate::entities::prelude::{ExhibitorsRoot, Forms, Users};
use crate::entities::{exhibitors_root, form_responses, forms, users};
use crate::forms::responses::{Answer, FormResponse};
use crate::forms::{AccessControl, Form, Info, Item};
use crate::middlewares::CurrentUser;
use crate::routes::AppState;
use axum::extract::{ConnectInfo, Path, State};
use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::{Extension, Json, Router};
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveEnum, ActiveModelBehavior, ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait,
    ModelTrait, NotSet, QueryFilter,
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
    Extension(current_user): Extension<CurrentUser>,
) -> Result<String, StatusCode> {
    let forms = match current_user {
        CurrentUser::Admin(_) => Forms::find().all(&state.db_conn).await,
        CurrentUser::User(claims) => {
            let user = Users::find_by_id(claims.sub).one(&state.db_conn).await;
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
        CurrentUser::None => {
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
    Extension(current_user): Extension<CurrentUser>,
    Json(new_form): Json<NewForm>,
) -> Result<(StatusCode, Json<Form>), StatusCode> {
    if let CurrentUser::Admin(_) = current_user {
        let model = forms::ActiveModel {
            form_id: NotSet,
            created_at: NotSet,
            updated_at: NotSet,
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
                info!("new form  added successfully");
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
struct EditForm {
    info: Option<Info>,
    items: Option<Vec<Item>>,
    access_control: Option<AccessControl>,
}

#[instrument(name = "POST /api/v1/forms/{form_id}", skip(state))]
async fn put_form(
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<Arc<AppState>>,
    Extension(current_user): Extension<CurrentUser>,
    Path(form_id): Path<Uuid>,
    Json(new_form): Json<EditForm>,
) -> Result<(StatusCode, Json<Form>), StatusCode> {
    if let CurrentUser::Admin(_) = current_user {
        let info = match new_form.info {
            Some(info) => Set(json!(info)),
            None => NotSet,
        };
        let items = match new_form.items {
            Some(items) => Set(json!(items)),
            None => NotSet,
        };
        let access_control_roles = match new_form.access_control {
            Some(access_control) => Set(access_control.roles),
            None => NotSet,
        };
        let model = forms::ActiveModel {
            form_id: Set(form_id),
            created_at: NotSet,
            updated_at: NotSet,
            info,
            items,
            access_control_roles,
        };

        match model.update(&state.db_conn).await {
            Ok(model) => {
                let form = match Form::from_model(&model) {
                    Ok(form) => form,
                    Err(err) => {
                        warn!(
                            "internal server error occurred while converting form: {:?}",
                            err
                        );
                        return Err(StatusCode::INTERNAL_SERVER_ERROR);
                    }
                };
                info!("form edited successfully");
                Ok((StatusCode::ACCEPTED, Json::from(form)))
            }
            Err(err) => {
                warn!(
                    "Internal server error occurred while updating form: {:?}",
                    err
                );
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    } else {
        Err(StatusCode::FORBIDDEN)
    }
}

#[instrument(name = "POST /api/v1/forms/delete", skip(state))]
async fn delete_form(
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<Arc<AppState>>,
    Extension(current_user): Extension<CurrentUser>,
    Path(form_id): Path<Uuid>,
) -> StatusCode {
    if let CurrentUser::Admin(_) = current_user {
        let form = match forms::Entity::find_by_id(form_id).one(&state.db_conn).await {
            Ok(Some(model)) => model,
            Ok(None) => return StatusCode::NOT_FOUND,
            Err(err) => {
                warn!(
                    "internal server error occurred while selecting form: {}",
                    err
                );
                return StatusCode::INTERNAL_SERVER_ERROR;
            }
        };
        match forms::Entity::delete_by_id(form_id)
            .exec(&state.db_conn)
            .await
        {
            Ok(res) => {
                if res.rows_affected == 0 {
                    StatusCode::NOT_FOUND
                } else {
                    StatusCode::ACCEPTED
                }
            }
            Err(err) => {
                warn!(
                    "internal server error occurred while deleting form: {}",
                    err
                );
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    } else {
        StatusCode::FORBIDDEN
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct ResponseInput {
    answers: HashMap<String, Answer>,
}

#[instrument(name = "POST /api/v1/forms/{form_id}/responses", skip(state))]
async fn post_response(
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<Arc<AppState>>,
    Extension(current_user): Extension<CurrentUser>,
    Path(form_id): Path<Uuid>,
    Json(new_response): Json<ResponseInput>,
) -> Result<(StatusCode, Json<FormResponse>), StatusCode> {
    if let CurrentUser::User(claims) = current_user {
        // 回答権があるかどうか確認
        let user = match users::Entity::find_by_id(claims.sub)
            .one(&state.db_conn)
            .await
        {
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
            respondent_id: Set(claims.sub),
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
                info!("new form was added by {}", claims.sub);
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

#[instrument(name = "GET /api/v1/forms/{form_id}/responses", skip(state))]
async fn get_responses(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    State(state): State<Arc<AppState>>,
    Extension(current_user): Extension<CurrentUser>,
    Path(form_id): Path<Uuid>,
) -> Result<(StatusCode, Json<Vec<FormResponse>>), StatusCode> {
    let responses = match current_user {
        CurrentUser::User(claims) => form_responses::Entity::find()
            .filter(form_responses::Column::RespondentId.eq(claims.sub)),
        CurrentUser::Admin(_) => form_responses::Entity::find(),
        CurrentUser::None => {
            return Err(StatusCode::UNAUTHORIZED);
        }
    }
    .all(&state.db_conn)
    .await;
    let responses = match responses {
        Ok(model) => model,
        Err(err) => {
            warn!(
                "internal server error occurred while selecting responses: {:?}",
                err
            );
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };
    let responses = match responses
        .iter()
        .map(|response| FormResponse::from_model(response))
        .collect::<Result<Vec<FormResponse>, _>>()
    {
        Ok(responses) => responses,
        Err(err) => {
            warn!(
                "internal server error occurred while converting response: {:?}",
                err
            );
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };
    Ok((StatusCode::OK, Json::from(responses)))
}

#[derive(Serialize, Deserialize, Debug)]
struct ResponseParams {
    form_id: Uuid,
    response_id: Uuid,
}
// #[instrument(name = "POST /api/v1/forms/{form_id}/responses/{response_id}", skip(state))]
// async fn put_response(
//     ConnectInfo(addr): ConnectInfo<SocketAddr>,
//     State(state): State<Arc<AppState>>,
//     Extension(user_id): Extension<CurrentUser>,
//     Path(params): Path<ResponseParams>,
//     Json(new_response): Json<ResponseInput>,
// ) {
//     form_responses::ActiveModel {
//         response_id: Set(params.response_id),
//         created_at: NotSet,
//         updated_at: NotSet,
//         form_id: Set(params.form_id),
//         respondent_id:,
//         answers: Default::default(),
//     }
//     form_responses::Entity::update()
// }
