// TODO(sqlx移行): 削除された entities 層への参照。application層の型へ再配線する。
// use crate::entities::group_id::GroupId;
// use crate::entities::plan_details::{PlanDetailsCreate, PlanDetailsRead};
use crate::middlewares::CurrentUser;
use crate::routes_legacy::AppState;
use crate::util::AppResponse;
use axum::extract::{ConnectInfo, Path, State};
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Extension, Json, Router};
use http::{HeaderMap, Method, StatusCode};
use std::net::SocketAddr;
use std::sync::Arc;
use tracing::instrument;

#[instrument(name = "init /api/v2/groups/{id}/plan-details")]
pub fn init_router() -> Router<Arc<AppState>> {
    Router::new().route("/", get(get_plan_details).put(put_plan_details))
}

// TODO(sqlx移行): シグネチャが削除された entities 型 (GroupId / PlanDetailsRead) を参照し、
// 本体で state.db_conn 経由の UserRead::from_claims を呼んでいたためスタブ化。
// application層へ再配線したら元のシグネチャと本体を復元する。
//
// 元のシグネチャ:
// async fn get_plan_details(
//     ConnectInfo(_addr): ConnectInfo<SocketAddr>,
//     State(state): State<Arc<AppState>>,
//     Extension(current_user): Extension<CurrentUser>,
//     Path(id): Path<GroupId>,
// ) -> AppResponse
async fn get_plan_details() -> axum::http::StatusCode {
    // 権限チェック: ユーザーのみ
    // let user = match current_user {
    //     CurrentUser::User(claims) => {
    //         // TODO(sqlx移行): DBアクセス。application層へ再配線。
    //         crate::entities::user::UserRead::from_claims(claims, &state.db_conn).await?
    //     }
    //     _ => return Ok((StatusCode::FORBIDDEN, ().into_response())),
    // };

    // 団体IDを文字列として取得
    // let group_id = match id {
    //     GroupId::String(id) => id,
    //     GroupId::Us => user.group_id.clone(),
    // };

    // ユーザーが団体に属していない場合は拒否
    // if group_id != user.group_id {
    //     return Ok((StatusCode::FORBIDDEN, ().into_response()));
    // }

    // 上流の plans API に委譲（存在しない場合は 404）
    // let url = format!("https://api2025.jizi.jp/v1/plans/{}/details", group_id);
    // let resp = state
    //     .http_client
    //     .request(Method::GET, url)
    //     .header("Cache-Control", "no-cache")
    //     .send()
    //     .await;

    // let resp = match resp {
    //     Ok(r) => r,
    //     Err(e) => {
    //         tracing::warn!(error = %e, "failed to call upstream plans api");
    //         return Ok((StatusCode::INTERNAL_SERVER_ERROR, ().into_response()));
    //     }
    // };

    // if resp.status() == StatusCode::NOT_FOUND {
    //     return Ok((StatusCode::NOT_FOUND, ().into_response()));
    // }

    // if !resp.status().is_success() {
    //     let status = resp.status();
    //     let body_text = resp.text().await.unwrap_or_default();
    //     tracing::error!(status = %status, body = %body_text, "upstream returned error");
    //     return Ok((StatusCode::INTERNAL_SERVER_ERROR, ().into_response()));
    // }

    // let body = resp.json::<PlanDetailsRead>().await;
    // match body {
    //     Ok(details) => Ok((StatusCode::OK, Json(details).into_response())),
    //     Err(e) => {
    //         tracing::error!(error = %e, "failed to parse upstream response");
    //         Ok((StatusCode::INTERNAL_SERVER_ERROR, ().into_response()))
    //     }
    // }
    axum::http::StatusCode::NOT_IMPLEMENTED
}

#[instrument(name = "PUT /api/v2/groups/{id}/plan-details")]
// TODO(sqlx移行): シグネチャが削除された entities 型 (GroupId / PlanDetailsCreate) を参照し、
// 本体で state.db_conn 経由の UserRead::from_claims を呼んでいたためスタブ化。
// application層へ再配線したら元のシグネチャと本体を復元する。
//
// 元のシグネチャ:
// async fn put_plan_details(
//     ConnectInfo(_addr): ConnectInfo<SocketAddr>,
//     State(state): State<Arc<AppState>>,
//     Extension(current_user): Extension<CurrentUser>,
//     Path(id): Path<GroupId>,
//     Json(payload): Json<PlanDetailsCreate>,
// ) -> AppResponse
async fn put_plan_details() -> axum::http::StatusCode {
    // 権限チェック: ユーザーのみ
    // let user = match current_user {
    //     CurrentUser::User(claims) => {
    //         // TODO(sqlx移行): DBアクセス。application層へ再配線。
    //         crate::entities::user::UserRead::from_claims(claims, &state.db_conn).await?
    //     }
    //     _ => return Ok((StatusCode::FORBIDDEN, ().into_response())),
    // };

    // 団体IDを文字列として取得
    // let group_id = match id {
    //     GroupId::String(id) => id,
    //     GroupId::Us => user.group_id.clone(),
    // };

    // ユーザーが団体に属していない場合は拒否
    // if group_id != user.group_id {
    //     return Ok((StatusCode::FORBIDDEN, ().into_response()));
    // }

    // let url = format!(
    //     "https://api2025.jizi.jp/v1/admin/plans/{}/details",
    //     group_id
    // );
    // let mut req = state.http_client.request(Method::PUT, url).json(&payload);
    // let mut headers = HeaderMap::new();
    // headers.insert(
    //     "CF-Access-Client-Id",
    //     state.secrets.plans_info_api_client_id.as_ref().parse()?,
    // );
    // headers.insert(
    //     "CF-Access-Client-Secret",
    //     state
    //         .secrets
    //         .plans_info_api_client_secret
    //         .as_ref()
    //         .parse()?,
    // );
    // req = req.headers(headers);
    // let resp = req.send().await;

    // let resp = match resp {
    //     Ok(r) => r,
    //     Err(e) => {
    //         tracing::error!(error = %e, "failed to call upstream plans api");
    //         return Ok((StatusCode::INTERNAL_SERVER_ERROR, ().into_response()));
    //     }
    // };

    // if resp.status() == StatusCode::NOT_FOUND {
    //     return Ok((StatusCode::NOT_FOUND, ().into_response()));
    // }

    // if !resp.status().is_success() {
    //     let status = resp.status();
    //     let body_text = resp.text().await.unwrap_or_default();
    //     tracing::warn!(status = %status, body = %body_text, "upstream returned error");
    //     return Ok((StatusCode::INTERNAL_SERVER_ERROR, ().into_response()));
    // }

    // Ok((StatusCode::OK, ().into_response()))
    axum::http::StatusCode::NOT_IMPLEMENTED
}
