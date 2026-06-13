// TODO(sqlx移行): application層へ再配線するまでコメントアウト
// use crate::entities;
// use crate::entities::form::{FormCreate, FormRead, FormUpdate};
// use crate::entities::target_specifier::TargetSpecifier;
// use crate::entities::user::UserRead;
use crate::middlewares::CurrentUser;
use crate::routes::AppState;
use crate::util::AppResponse;
use axum::extract::{ConnectInfo, Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Extension, Json, Router};
use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::Arc;
use tracing::instrument;
use uuid::Uuid;

#[instrument(name = "init /api/v2/forms")]
pub fn init_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(get_forms).post(post_forms))
        .route(
            "/{form_id}",
            get(get_form).patch(patch_form).delete(delete_form),
        )
}

#[instrument(name = "GET /api/v2/forms", skip(state))]
async fn get_forms(
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<Arc<AppState>>,
    Extension(current_user): Extension<CurrentUser>,
) -> AppResponse {
    // TODO(sqlx移行): DBアクセスをapplication層へ再配線
    // let forms = match current_user {
    //     CurrentUser::Admin(_) => FormRead::find_all(&state.db_conn).await?,
    //     CurrentUser::User(claims) => {
    //         let user_read = UserRead::from(claims, &state.db_conn).await?;
    //         user_read.get_forms(&state.db_conn).await?
    //     }
    //     CurrentUser::None => {
    //         let forms_ = FormRead::find_all(&state.db_conn).await?;
    //         let mut forms = vec![];
    //         for form in forms_ {
    //             for target in &form.targets {
    //                 if target.does_user_match(None, &state.db_conn).await? {
    //                     forms.push(form);
    //                     break;
    //                 }
    //             }
    //         }
    //         forms
    //     }
    // };
    //
    // Ok((StatusCode::OK, Json(forms).into_response()))
    todo!("sqlx移行: application層へ再配線")
}

// TODO(sqlx移行): シグネチャが削除済みの FormCreate を参照するためスタブ化。application層へ再配線する。
// #[instrument(name = "POST /api/v2/forms", skip(state))]
// async fn post_forms(
//     ConnectInfo(_addr): ConnectInfo<SocketAddr>,
//     State(state): State<Arc<AppState>>,
//     Extension(current_user): Extension<CurrentUser>,
//     Json(form): Json<FormCreate>,
// ) -> AppResponse {
//     if let CurrentUser::Admin(claims) = current_user {
//         form.insert(
//             &state.db_conn,
//             Some(Uuid::from_str(claims.subject().as_str())?),
//         )
//         .await?;
//         Ok((StatusCode::NO_CONTENT, ().into_response()))
//     } else {
//         Ok((StatusCode::FORBIDDEN, "Access forbidden.".into_response()))
//     }
// }
async fn post_forms() -> axum::http::StatusCode {
    axum::http::StatusCode::NOT_IMPLEMENTED
}

#[instrument(name = "GET /api/v2/forms/{form_id}", skip(state))]
async fn get_form(
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<Arc<AppState>>,
    Extension(current_user): Extension<CurrentUser>,
    Path(form_id): Path<Uuid>,
) -> AppResponse {
    // TODO(sqlx移行): DBアクセスをapplication層へ再配線
    // let form = match current_user {
    //     CurrentUser::Admin(_) => FormRead::find_by_id(form_id, &state.db_conn).await?,
    //     CurrentUser::User(claims) => {
    //         let user = UserRead::from(claims, &state.db_conn).await?;
    //         let form = FormRead::find_by_id(form_id, &state.db_conn).await?;
    //         match form {
    //             Some(form) => {
    //                 let mut res = None;
    //                 for target in &form.targets {
    //                     if target.does_user_match(Some(&user), &state.db_conn).await? {
    //                         res = Some(form);
    //                         break;
    //                     }
    //                 }
    //                 res
    //             }
    //             None => None,
    //         }
    //     }
    //     CurrentUser::None => {
    //         let form = FormRead::find_by_id(form_id, &state.db_conn).await?;
    //         match form {
    //             Some(form) => {
    //                 if form
    //                     .targets
    //                     .iter()
    //                     .any(|target| matches!(target, TargetSpecifier::UserNologin))
    //                 {
    //                     Some(form)
    //                 } else {
    //                     None
    //                 }
    //             }
    //             None => None,
    //         }
    //     }
    // };
    //
    // match form {
    //     Some(form) => Ok((StatusCode::OK, Json(form).into_response())),
    //     None => Ok((StatusCode::NOT_FOUND, "form not found.".into_response())),
    // }
    todo!("sqlx移行: application層へ再配線")
}

// TODO(sqlx移行): シグネチャが削除済みの FormUpdate を参照するためスタブ化。application層へ再配線する。
// #[instrument(name = "PATCH /api/v2/forms/{form_id}", skip(state))]
// async fn patch_form(
//     ConnectInfo(_addr): ConnectInfo<SocketAddr>,
//     State(state): State<Arc<AppState>>,
//     Extension(current_user): Extension<CurrentUser>,
//     Path(form_id): Path<Uuid>,
//     Json(form): Json<FormUpdate>,
// ) -> AppResponse {
//     if let CurrentUser::Admin(claims) = current_user {
//         form.update(
//             &state.db_conn,
//             form_id,
//             Some(Uuid::from_str(claims.subject().as_str())?),
//         )
//         .await?;
//         Ok((StatusCode::NO_CONTENT, ().into_response()))
//     } else {
//         Ok((StatusCode::FORBIDDEN, "Access forbidden.".into_response()))
//     }
// }
async fn patch_form() -> axum::http::StatusCode {
    axum::http::StatusCode::NOT_IMPLEMENTED
}

#[instrument(name = "POST /api/v2/forms/delete", skip(state))]
async fn delete_form(
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<Arc<AppState>>,
    Extension(current_user): Extension<CurrentUser>,
    Path(form_id): Path<Uuid>,
) -> AppResponse {
    // TODO(sqlx移行): DBアクセスをapplication層へ再配線
    // if let CurrentUser::Admin(_) = current_user {
    //     let rows_affected = entities::form::delete_form_by_id(form_id, &state.db_conn).await?;
    //     if rows_affected == 0 {
    //         Ok((StatusCode::NOT_FOUND, "form not found.".into_response()))
    //     } else {
    //         Ok((StatusCode::ACCEPTED, "Accepted.".into_response()))
    //     }
    // } else {
    //     Ok((StatusCode::FORBIDDEN, "Access forbidden.".into_response()))
    // }
    todo!("sqlx移行: application層へ再配線")
}
