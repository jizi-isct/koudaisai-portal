use crate::entities::prelude::{ExhibitorsRoot, Forms, Users};
use crate::entities::{exhibitors_root, form_responses, forms, users};
use crate::forms::responses::{Answer, FormResponse};
use crate::forms::{AccessControl, Form, Info, Item};
use crate::middlewares::CurrentUser;
use crate::routes::AppState;
use crate::util::AppResponse;
use axum::extract::{ConnectInfo, Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, post, put};
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
use tracing::{info, instrument, trace, warn};
use uuid::Uuid;

#[instrument(name = "init /api/v1/forms")]
pub fn init_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(get_forms).post(post_forms))
        .route(
            "/{form_id}",
            get(get_form).put(put_form).delete(delete_form),
        )
        .route("/{form_id}/responses", post(post_response))
}

#[instrument(name = "GET /api/v1/forms", skip(state))]
async fn get_forms(
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<Arc<AppState>>,
    Extension(current_user): Extension<CurrentUser>,
) -> AppResponse {
    trace!("hello");
    let form_models = match current_user {
        CurrentUser::Admin(_) => Forms::find().all(&state.db_conn).await?,
        CurrentUser::User(claims) => {
            trace!("finding user");
            let user = Users::find_by_id(claims.sub).one(&state.db_conn).await?;
            let user = match user {
                Some(user) => user,
                None => {
                    warn!("internal server error occurred: user doesn't exist");
                    return Ok((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "User not found".into_response(),
                    ));
                }
            };
            trace!("finding exhibitor");
            let exhibition_id = user.exhibition_id;
            let exhibitor = ExhibitorsRoot::find_by_id(exhibition_id)
                .one(&state.db_conn)
                .await?;
            let exhibitor = match exhibitor {
                Some(exhibitor) => exhibitor,
                None => {
                    warn!("internal server error occurred: exhibitor doesn't exist");
                    return Ok((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "exhibitor not found".into_response(),
                    ));
                }
            };
            let exhibitor_type = exhibitor.r#type.into_value().to_string();
            trace!("finding forms");
            let models = Forms::find().all(&state.db_conn).await?;
            let mut filtered: Vec<forms::Model> = vec![];
            for model in models {
                if model.access_control_roles.contains(&exhibitor_type) {
                    filtered.push(model)
                }
            }
            filtered
        }
        CurrentUser::None => {
            let models = Forms::find().all(&state.db_conn).await?;
            let mut filtered: Vec<forms::Model> = vec![];
            for model in models {
                if model.access_control_roles.contains(&"none".to_string()) {
                    filtered.push(model)
                }
            }
            filtered
        }
    };

    let mut forms = vec![];
    for form_model in form_models {
        forms.push(Form::from_model(&form_model)?);
    }
    let json = serde_json::to_string(&forms)?;

    Ok((StatusCode::OK, json.into_response()))
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
) -> AppResponse {
    if let CurrentUser::Admin(_) = current_user {
        let model = forms::ActiveModel {
            form_id: Set(Uuid::new_v4()),
            created_at: NotSet,
            updated_at: NotSet,
            info: Set(json!(new_form.info)),
            items: Set(json!(new_form.items)),
            access_control_roles: Set(new_form.access_control.roles),
        };
        let model = model.insert(&state.db_conn).await?;
        let form = Form::from_model(&model)?;
        info!("new form  added successfully");
        Ok((StatusCode::ACCEPTED, Json::from(form).into_response()))
    } else {
        Ok((StatusCode::FORBIDDEN, "Access forbidden.".into_response()))
    }
}

#[instrument(name = "GET /api/v1/forms/{form_id}", skip(state))]
async fn get_form(
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<Arc<AppState>>,
    Extension(current_user): Extension<CurrentUser>,
    Path(form_id): Path<Uuid>,
) -> AppResponse {
    trace!("hello");
    let form_model = match current_user {
        CurrentUser::Admin(_) => Forms::find_by_id(form_id).one(&state.db_conn).await?,
        CurrentUser::User(claims) => {
            trace!("finding user");
            let user = Users::find_by_id(claims.sub).one(&state.db_conn).await?;
            let user = match user {
                Some(user) => user,
                None => {
                    warn!("internal server error occurred: user doesn't exist");
                    return Ok((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "User not found".into_response(),
                    ));
                }
            };
            trace!("finding exhibitor");
            let exhibition_id = user.exhibition_id;
            let exhibitor = ExhibitorsRoot::find_by_id(exhibition_id)
                .one(&state.db_conn)
                .await?;
            let exhibitor = match exhibitor {
                Some(exhibitor) => exhibitor,
                None => {
                    warn!("internal server error occurred: exhibitor doesn't exist");
                    return Ok((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "exhibitor not found".into_response(),
                    ));
                }
            };
            let exhibitor_type = exhibitor.r#type.into_value().to_string();
            trace!("finding forms");
            let model = Forms::find_by_id(form_id).one(&state.db_conn).await?;
            if model == None
                || !model
                    .clone()
                    .unwrap()
                    .access_control_roles
                    .contains(&exhibitor_type)
            {
                None
            } else {
                model
            }
        }
        CurrentUser::None => {
            let model = Forms::find_by_id(form_id).one(&state.db_conn).await?;
            if model == None
                || !model
                    .clone()
                    .unwrap()
                    .access_control_roles
                    .contains(&"none".to_string())
            {
                None
            } else {
                model
            }
        }
    };

    if form_model == None {
        return Ok((StatusCode::NOT_FOUND, "form not found.".into_response()));
    }

    let form = Form::from_model(&form_model.unwrap())?;

    Ok((StatusCode::OK, Json(form).into_response()))
}

#[derive(Serialize, Deserialize, Debug)]
struct EditForm {
    info: Option<Info>,
    items: Option<Vec<Item>>,
    access_control: Option<AccessControl>,
}

#[instrument(name = "PUT /api/v1/forms/{form_id}", skip(state))]
async fn put_form(
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<Arc<AppState>>,
    Extension(current_user): Extension<CurrentUser>,
    Path(form_id): Path<Uuid>,
    Json(new_form): Json<EditForm>,
) -> AppResponse {
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

        let model = model.update(&state.db_conn).await?;
        let form = Form::from_model(&model)?;
        info!("form edited successfully");
        Ok((StatusCode::ACCEPTED, Json::from(form).into_response()))
    } else {
        Ok((StatusCode::FORBIDDEN, "Access forbidden.".into_response()))
    }
}

#[instrument(name = "POST /api/v1/forms/delete", skip(state))]
async fn delete_form(
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<Arc<AppState>>,
    Extension(current_user): Extension<CurrentUser>,
    Path(form_id): Path<Uuid>,
) -> AppResponse {
    if let CurrentUser::Admin(_) = current_user {
        match forms::Entity::find_by_id(form_id)
            .one(&state.db_conn)
            .await?
        {
            Some(_) => {}
            None => return Ok((StatusCode::NOT_FOUND, "form not found.".into_response())),
        };
        let res = forms::Entity::delete_by_id(form_id)
            .exec(&state.db_conn)
            .await?;
        if res.rows_affected == 0 {
            Ok((StatusCode::NOT_FOUND, "form not found.".into_response()))
        } else {
            Ok((StatusCode::ACCEPTED, "Accepted.".into_response()))
        }
    } else {
        Ok((StatusCode::FORBIDDEN, "Access forbidden.".into_response()))
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
) -> AppResponse {
    if let CurrentUser::User(claims) = current_user {
        // 回答権があるかどうか確認
        let user = match users::Entity::find_by_id(claims.sub)
            .one(&state.db_conn)
            .await?
        {
            Some(model) => model,
            None => {
                warn!("Subject not found.");
                return Ok((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "user not found.".into_response(),
                ));
            }
        };
        let exhibitor = match exhibitors_root::Entity::find_by_id(user.exhibition_id)
            .one(&state.db_conn)
            .await?
        {
            Some(model) => model,
            None => {
                warn!("Exhibitor not found.");
                return Ok((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "exhibitor not found.".into_response(),
                ));
            }
        };
        let form = match forms::Entity::find_by_id(form_id)
            .one(&state.db_conn)
            .await?
        {
            Some(model) => model,
            None => {
                return Ok((StatusCode::NOT_FOUND, "form not found.".into_response()));
            }
        };
        if !form
            .access_control_roles
            .contains(&exhibitor.r#type.into_value().to_string())
        {
            return Ok((StatusCode::FORBIDDEN, "Access forbidden.".into_response()));
        }

        let response = form_responses::ActiveModel {
            response_id: Set(Uuid::new_v4()),
            created_at: ActiveValue::NotSet,
            updated_at: ActiveValue::NotSet,
            form_id: Set(form_id),
            respondent_id: Set(claims.sub),
            answers: Set(json!(new_response.answers)),
        };
        let model = response.insert(&state.db_conn).await?;
        let response = FormResponse::from_model(&model)?;
        info!("new form was added by {}", claims.sub);
        Ok((StatusCode::ACCEPTED, Json::from(response).into_response()))
    } else {
        Ok((StatusCode::FORBIDDEN, "Access forbidden.".into_response()))
    }
}

#[instrument(name = "GET /api/v1/forms/{form_id}/responses", skip(state))]
async fn get_responses(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    State(state): State<Arc<AppState>>,
    Extension(current_user): Extension<CurrentUser>,
    Path(form_id): Path<Uuid>,
) -> AppResponse {
    let responses = match current_user {
        CurrentUser::User(claims) => form_responses::Entity::find()
            .filter(form_responses::Column::RespondentId.eq(claims.sub)),
        CurrentUser::Admin(_) => form_responses::Entity::find(),
        CurrentUser::None => {
            return Ok((StatusCode::FORBIDDEN, "Access forbidden.".into_response()));
        }
    }
    .all(&state.db_conn)
    .await?;
    let responses = responses
        .iter()
        .map(|response| FormResponse::from_model(response))
        .collect::<Result<Vec<FormResponse>, _>>()?;
    Ok((StatusCode::OK, Json::from(responses).into_response()))
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
