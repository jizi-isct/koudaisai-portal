mod plan_details;

// TODO(sqlx移行): 旧 entities / sea_orm 由来の import。application 層へ再配線後に置き換える。
// use crate::entities::group;
// use crate::entities::group::{GroupCreate, GroupRead, GroupUpdate};
use crate::middlewares::CurrentUser;
use crate::routes_legacy::AppState;
use crate::util::AppResponse;
use axum::extract::{ConnectInfo, Path, State};
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Extension, Json, Router};
use http::StatusCode;
// use sea_orm::DbErr;
use serde::Serialize;
use std::net::SocketAddr;
use std::sync::Arc;
use tracing::instrument;

#[instrument(name = "init /api/v2/groups")]
pub fn init_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(get_groups))
        .route(
            "/{id}",
            get(get_group)
                .put(put_group)
                .patch(patch_group)
                .delete(delete_group),
        )
        .nest("/{id}/plan-details", plan_details::init_router())
}

#[instrument(name = "GET /api/v2/groups", skip(state, current_user))]
async fn get_groups(
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<Arc<AppState>>,
    Extension(current_user): Extension<CurrentUser>,
) -> AppResponse {
    match current_user {
        CurrentUser::Admin(_) => {
            // 管理者はすべての団体を取得可能
            // TODO(sqlx移行): GroupRead::get_all を application 層へ再配線
            // let groups = GroupRead::get_all(&state.db_conn).await?;
            // Ok((StatusCode::OK, Json(groups).into_response()))
            todo!("sqlx移行: application層へ再配線")
        }
        _ => Ok((StatusCode::FORBIDDEN, ().into_response())),
    }
}

#[instrument(name = "GET /api/v2/groups/:id", skip(state, current_user))]
async fn get_group(
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<Arc<AppState>>,
    Extension(current_user): Extension<CurrentUser>,
    Path(id): Path<String>,
) -> AppResponse {
    match current_user {
        CurrentUser::Admin(_) => {
            // 管理者はどの団体も取得可能
            // TODO(sqlx移行): GroupRead::find_by_id を application 層へ再配線
            // let group = GroupRead::find_by_id(&state.db_conn, id).await?;
            // match group {
            //     Some(group) => Ok((StatusCode::OK, Json(group).into_response())),
            //     None => Ok((StatusCode::NOT_FOUND, ().into_response())),
            // }
            todo!("sqlx移行: application層へ再配線")
        }
        CurrentUser::User(_claims) => {
            // ユーザーは自分の所属する団体のみ取得可能
            // TODO(sqlx移行): UserRead::from_claims / GroupRead::find_by_id を application 層へ再配線
            // let user = crate::entities::user::UserRead::from_claims(claims, &state.db_conn).await?;
            // let group = GroupRead::find_by_id(&state.db_conn, id.clone()).await?;
            //
            // match group {
            //     Some(group) => {
            //         // ユーザーがこの団体に所属しているか確認
            //         if user.group_id == id {
            //             Ok((StatusCode::OK, Json(group).into_response()))
            //         } else {
            //             Ok((StatusCode::FORBIDDEN, ().into_response()))
            //         }
            //     }
            //     None => Ok((StatusCode::NOT_FOUND, ().into_response())),
            // }
            todo!("sqlx移行: application層へ再配線")
        }
        _ => Ok((StatusCode::FORBIDDEN, ().into_response())),
    }
}

#[derive(Serialize)]
struct PutGroupResponse {
    pub activation_tokens: Vec<String>,
}

// TODO(sqlx移行): シグネチャが削除済みの GroupCreate を参照していたため最小スタブに差し替え。
// application 層へ再配線後、元のシグネチャ・本体を復元する。
// #[instrument(name = "PUT /api/v2/groups/:id", skip(state, current_user))]
// async fn put_group(
//     ConnectInfo(_addr): ConnectInfo<SocketAddr>,
//     State(state): State<Arc<AppState>>,
//     Extension(current_user): Extension<CurrentUser>,
//     Path(id): Path<String>,
//     Json(group_create): Json<GroupCreate>,
// ) -> AppResponse {
//     match current_user {
//         CurrentUser::Admin(_) => {
//             // 管理者のみが団体を作成可能
//             let result = group_create
//                 .insert(
//                     &state.db_conn,
//                     state.web.auth.activation_salt.as_str(),
//                     state.web.auth.stretch_cost,
//                     id,
//                 )
//                 .await;
//             match result {
//                 Ok(activation_tokens) => Ok((
//                     StatusCode::CREATED,
//                     Json(PutGroupResponse { activation_tokens }).into_response(),
//                 )),
//                 Err(DbErr::RecordNotInserted) => Ok((StatusCode::CONFLICT, ().into_response())),
//                 Err(e) => {
//                     tracing::error!("Failed to create group: {}", e);
//                     Ok((StatusCode::INTERNAL_SERVER_ERROR, ().into_response()))
//                 }
//             }
//         }
//         _ => Ok((StatusCode::FORBIDDEN, ().into_response())),
//     }
// }
async fn put_group() -> axum::http::StatusCode {
    axum::http::StatusCode::NOT_IMPLEMENTED
}

// TODO(sqlx移行): シグネチャが削除済みの GroupUpdate を参照していたため最小スタブに差し替え。
// application 層へ再配線後、元のシグネチャ・本体を復元する。
// #[instrument(name = "PATCH /api/v2/groups/:id", skip(state, current_user))]
// async fn patch_group(
//     ConnectInfo(_addr): ConnectInfo<SocketAddr>,
//     State(state): State<Arc<AppState>>,
//     Extension(current_user): Extension<CurrentUser>,
//     Path(id): Path<String>,
//     Json(group_update): Json<GroupUpdate>,
// ) -> AppResponse {
//     match current_user {
//         CurrentUser::Admin(_) => {
//             // 管理者はどの団体も更新可能
//             group_update.update(&state.db_conn, id).await?;
//             Ok((StatusCode::NO_CONTENT, ().into_response()))
//         }
//         CurrentUser::User(claims) => {
//             // ユーザーは自分の所属する団体のみ更新可能
//             let user = crate::entities::user::UserRead::from_claims(claims, &state.db_conn).await?;
//
//             if user.group_id == id {
//                 group_update.update(&state.db_conn, id).await?;
//                 Ok((StatusCode::NO_CONTENT, ().into_response()))
//             } else {
//                 Ok((StatusCode::FORBIDDEN, ().into_response()))
//             }
//         }
//         _ => Ok((StatusCode::FORBIDDEN, ().into_response())),
//     }
// }
async fn patch_group() -> axum::http::StatusCode {
    axum::http::StatusCode::NOT_IMPLEMENTED
}

#[instrument(name = "DELETE /api/v2/groups/:id", skip(state, current_user))]
async fn delete_group(
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<Arc<AppState>>,
    Extension(current_user): Extension<CurrentUser>,
    Path(id): Path<String>,
) -> AppResponse {
    match current_user {
        CurrentUser::Admin(_) => {
            // 管理者のみが団体を削除可能
            // TODO(sqlx移行): group::delete_group を application 層へ再配線
            // group::delete_group(&state.db_conn, id).await?;
            // Ok((StatusCode::NO_CONTENT, ().into_response()))
            todo!("sqlx移行: application層へ再配線")
        }
        _ => Ok((StatusCode::FORBIDDEN, ().into_response())),
    }
}
