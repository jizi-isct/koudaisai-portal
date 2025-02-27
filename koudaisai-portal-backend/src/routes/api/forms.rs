use crate::entities::forms;
use crate::entities::prelude::{ExhibitorsRoot, Forms, Users};
use crate::forms::Form;
use crate::middlewares::UserId;
use crate::routes::AppState;
use axum::extract::{ConnectInfo, State};
use axum::http::StatusCode;
use axum::routing::get;
use axum::{Extension, Router};
use axum_extra::{headers, TypedHeader};
use sea_orm::{ActiveEnum, ColumnTrait, EntityTrait, QueryFilter};
use std::net::SocketAddr;
use std::sync::Arc;
use tracing::{instrument, warn};

#[instrument(name = "init /api/v1/forms")]
pub fn init_router() -> Router<Arc<AppState>> {
    Router::new().route("", get(get_forms))
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
        let form = match Form::from_model(&state.db_conn, &form_model).await {
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
