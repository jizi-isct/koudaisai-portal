use crate::entities::exhibitors_root::Model;
use crate::entities::{
    exhibitors_category_booth, exhibitors_category_general, exhibitors_category_labo,
    exhibitors_category_stage, exhibitors_root, sea_orm_active_enums, users,
};
use crate::middlewares::CurrentUser;
use crate::routes::AppState;
use crate::util::sha::stretch_with_salt;
use crate::util::AppError;
use axum::extract::{ConnectInfo, Path, State};
use axum::response::{IntoResponse, Response};
use axum::routing::{post, put};
use axum::{Extension, Json, Router};
use http::StatusCode;
use sea_orm::{ActiveModelTrait, ActiveValue, EntityTrait, QueryFilter, TransactionTrait};
use sea_orm::{ColumnTrait, EntityOrSelect};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::sync::Arc;
use tracing::{instrument, warn};
use uuid::Uuid;

#[instrument(name = "init /api/v1/exhibitors")]
pub fn init_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", post(post_exhibitors).get(get_exhibitors))
        .route("/{id}", put(put_exhibitors_id).get(get_exhibitors_id))
}

#[derive(Deserialize, Debug)]
struct PostExhibitorsPayload {
    id: String,
    exhibitor_name: String,
    #[serde(rename = "type")]
    r#type: ExhibitionType,
    representatives: (
        RepresentativeWrite,
        RepresentativeWrite,
        RepresentativeWrite,
    ),
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
enum ExhibitionType {
    Booth,
    General,
    Stage,
    Labo,
}
impl Into<sea_orm_active_enums::ExhibitionType> for ExhibitionType {
    fn into(self) -> sea_orm_active_enums::ExhibitionType {
        match self {
            Self::Booth => sea_orm_active_enums::ExhibitionType::Booth,
            Self::General => sea_orm_active_enums::ExhibitionType::General,
            Self::Stage => sea_orm_active_enums::ExhibitionType::Stage,
            Self::Labo => sea_orm_active_enums::ExhibitionType::Labo,
        }
    }
}
impl From<sea_orm_active_enums::ExhibitionType> for ExhibitionType {
    fn from(value: sea_orm_active_enums::ExhibitionType) -> Self {
        match value {
            sea_orm_active_enums::ExhibitionType::Booth => Self::Booth,
            sea_orm_active_enums::ExhibitionType::General => Self::General,
            sea_orm_active_enums::ExhibitionType::Stage => Self::Stage,
            sea_orm_active_enums::ExhibitionType::Labo => Self::Labo,
        }
    }
}
#[derive(Deserialize, Debug)]
struct RepresentativeWrite {
    first_name: String,
    last_name: String,
    m_address: String,
}

fn new_user_model(
    representative: &RepresentativeWrite,
    exhibition_id: String,
    uuid: Uuid,
) -> users::ActiveModel {
    users::ActiveModel {
        id: ActiveValue::Set(uuid),
        created_at: ActiveValue::NotSet,
        updated_at: ActiveValue::NotSet,
        first_name: ActiveValue::Set(representative.first_name.clone()),
        last_name: ActiveValue::Set(representative.last_name.clone()),
        m_address: ActiveValue::Set(representative.m_address.clone()),
        password_hash: ActiveValue::NotSet,
        password_salt: ActiveValue::NotSet,
        exhibition_id: ActiveValue::Set(exhibition_id),
    }
}
type PostExhibitorsResponse = (String, String, String);
#[instrument(name = "POST /api/v1/exhibitors", skip(state))]
#[axum::debug_handler]
async fn post_exhibitors(
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<Arc<AppState>>,
    Extension(current_user): Extension<CurrentUser>,
    Json(payload): Json<PostExhibitorsPayload>,
) -> Result<(StatusCode, Response), AppError> {
    match current_user {
        CurrentUser::Admin(_) => {}
        _ => return Ok((StatusCode::FORBIDDEN, "Access forbidden.".into_response())),
    };

    // conflict check
    if let Some(_) = exhibitors_root::Entity::find_by_id(payload.id.clone())
        .one(&state.db_conn)
        .await?
    {
        return Ok((StatusCode::CONFLICT, "Conflict.".into_response()));
    }
    if let Some(_) = users::Entity::find()
        .filter(users::Column::MAddress.eq(payload.representatives.0.m_address.clone()))
        .one(&state.db_conn)
        .await?
    {
        return Ok((StatusCode::CONFLICT, "Conflict.".into_response()));
    }
    // generate uuids
    let uuids = (Uuid::new_v4(), Uuid::new_v4(), Uuid::new_v4());

    // transaction
    let txn = state.db_conn.begin().await?;

    // exhibitors_root
    exhibitors_root::ActiveModel {
        id: ActiveValue::Set(payload.id.clone()),
        created_at: ActiveValue::NotSet,
        updated_at: ActiveValue::NotSet,
        exhibitor_name: ActiveValue::Set(payload.exhibitor_name.clone()),
        r#type: ActiveValue::Set(payload.r#type.clone().into()),
        exhibition_name: ActiveValue::NotSet,
        icon_id: ActiveValue::NotSet,
        description: ActiveValue::NotSet,
        representative1: ActiveValue::Set(Some(uuids.0.clone())),
        representative2: ActiveValue::Set(Some(uuids.1.clone())),
        representative3: ActiveValue::Set(Some(uuids.2.clone())),
    }
    .insert(&txn)
    .await?;

    //exhibitors_category_...
    match payload.r#type {
        ExhibitionType::Booth => {
            exhibitors_category_booth::ActiveModel {
                id: ActiveValue::Set(payload.id.clone()),
                location: ActiveValue::NotSet,
                starting_time_day1: ActiveValue::NotSet,
                ending_time_day1: ActiveValue::NotSet,
                starting_time_day2: ActiveValue::NotSet,
                ending_time_day2: ActiveValue::NotSet,
            }
            .insert(&txn)
            .await?;
        }
        ExhibitionType::General => {
            exhibitors_category_general::ActiveModel {
                id: ActiveValue::Set(payload.id.clone()),
                location: ActiveValue::NotSet,
                starting_time_day1: ActiveValue::NotSet,
                ending_time_day1: ActiveValue::NotSet,
                starting_time_day2: ActiveValue::NotSet,
                ending_time_day2: ActiveValue::NotSet,
            }
            .insert(&txn)
            .await?;
        }
        ExhibitionType::Stage => {
            exhibitors_category_stage::ActiveModel {
                id: ActiveValue::Set(payload.id.clone()),
                r#type: Default::default(),
            }
            .insert(&txn)
            .await?;
        }
        ExhibitionType::Labo => {
            exhibitors_category_labo::ActiveModel {
                id: ActiveValue::Set(payload.id.clone()),
                location: ActiveValue::NotSet,
                starting_time_day1: ActiveValue::NotSet,
                ending_time_day1: ActiveValue::NotSet,
                starting_time_day2: ActiveValue::NotSet,
                ending_time_day2: ActiveValue::NotSet,
            }
            .insert(&txn)
            .await?;
        }
    }

    //users
    new_user_model(
        &payload.representatives.0,
        payload.id.clone(),
        uuids.0.clone(),
    )
    .insert(&txn)
    .await?;
    new_user_model(
        &payload.representatives.1,
        payload.id.clone(),
        uuids.1.clone(),
    )
    .insert(&txn)
    .await?;
    new_user_model(
        &payload.representatives.2,
        payload.id.clone(),
        uuids.2.clone(),
    )
    .insert(&txn)
    .await?;

    //commit
    txn.commit().await?;

    //generate activation tokens
    let activation_tokens: PostExhibitorsResponse = (
        stretch_with_salt(
            payload.representatives.0.m_address.as_str(),
            state.web.auth.activation_salt.as_str(),
            2_i32.pow(state.web.auth.stretch_cost as u32),
        )
        .await,
        stretch_with_salt(
            payload.representatives.1.m_address.as_str(),
            state.web.auth.activation_salt.as_str(),
            2_i32.pow(state.web.auth.stretch_cost as u32),
        )
        .await,
        stretch_with_salt(
            payload.representatives.2.m_address.as_str(),
            state.web.auth.activation_salt.as_str(),
            2_i32.pow(state.web.auth.stretch_cost as u32),
        )
        .await,
    );

    Ok((StatusCode::CREATED, Json(activation_tokens).into_response()))
}

#[derive(Serialize, Debug)]
struct GetExhibitorsResponseElement {
    id: String,
    created_at: String,
    updated_at: String,
    exhibitor_name: String,
    exhibition_name: Option<String>,
    icon_id: Option<String>,
    description: Option<String>,
    r#type: ExhibitionType,
    representatives: (Uuid, Uuid, Uuid),
}

impl From<exhibitors_root::Model> for GetExhibitorsResponseElement {
    fn from(value: Model) -> Self {
        Self {
            id: value.id,
            created_at: value.created_at.unwrap().to_string(),
            updated_at: value.updated_at.unwrap().to_string(),
            exhibitor_name: value.exhibitor_name,
            exhibition_name: value.exhibition_name,
            icon_id: value.icon_id,
            description: value.description,
            r#type: ExhibitionType::from(value.r#type),
            representatives: (
                value.representative1.unwrap(),
                value.representative2.unwrap(),
                value.representative3.unwrap(),
            ),
        }
    }
}

#[instrument(name = "GET /api/v1/exhibitors", skip(state))]
async fn get_exhibitors(
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<Arc<AppState>>,
    Extension(current_user): Extension<CurrentUser>,
) -> Result<(StatusCode, Response), AppError> {
    match current_user {
        CurrentUser::Admin(_) => {}
        _ => return Ok((StatusCode::FORBIDDEN, "Access forbidden.".into_response())),
    };

    let models = exhibitors_root::Entity.select().all(&state.db_conn).await?;
    let mut exhibitors: Vec<GetExhibitorsResponseElement> = vec![];
    for model in models {
        exhibitors.push(model.into())
    }

    Ok((StatusCode::OK, Json(exhibitors).into_response()))
}

type GetExhibitorsIdResponse = GetExhibitorsResponseElement;
#[instrument(name = "GET /api/v1/exhibitors/{id}", skip(state))]
async fn get_exhibitors_id(
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<Arc<AppState>>,
    Extension(current_user): Extension<CurrentUser>,
    Path(id): Path<String>,
) -> Result<(StatusCode, Response), AppError> {
    //permission check
    match current_user {
        CurrentUser::Admin(_) => {}
        CurrentUser::User(claims) => {
            // 属しているかどうか確認
            let model = users::Entity::find_by_id(claims.sub)
                .one(&state.db_conn)
                .await?;
            if model == None {
                return Ok((StatusCode::NOT_FOUND, "Subject not found.".into_response()));
            }
            if model.unwrap().exhibition_id != id {
                // FORBIDDEN等にすると参加団体の存在が無駄に露呈してしまう
                return Ok((StatusCode::NOT_FOUND, "Not found.".into_response()));
            }
        }
        CurrentUser::None => {
            return Ok((StatusCode::FORBIDDEN, "Access forbidden.".into_response()));
        }
    }

    //select
    let model = exhibitors_root::Entity::find_by_id(id)
        .one(&state.db_conn)
        .await?;
    if model == None {
        return Ok((StatusCode::NOT_FOUND, "Not found.".into_response()));
    }
    let response: GetExhibitorsIdResponse = model.unwrap().into();
    Ok((StatusCode::OK, Json(response).into_response()))
}

#[derive(Deserialize, Debug)]
struct PutExhibitorsIdPayload {
    exhibition_name: Option<String>,
    icon_id: Option<String>,
    description: Option<String>,
}

#[instrument(name = "PUT /api/v1/exhibitors/{id}", skip(state))]
async fn put_exhibitors_id(
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<Arc<AppState>>,
    Extension(current_user): Extension<CurrentUser>,
    Path(id): Path<String>,
    Json(payload): Json<PutExhibitorsIdPayload>,
) -> Result<(StatusCode, Response), AppError> {
    //permission che
    match current_user {
        CurrentUser::Admin(_) => {}
        CurrentUser::User(claims) => {
            // 属しているかどうか確認
            let model = users::Entity::find_by_id(claims.sub)
                .one(&state.db_conn)
                .await?;
            if model == None {
                return Ok((StatusCode::NOT_FOUND, "Subject not found.".into_response()));
            }
            if model.unwrap().exhibition_id != id {
                // FORBIDDEN等にすると参加団体の存在が無駄に露呈してしまう
                return Ok((StatusCode::NOT_FOUND, "Not found.".into_response()));
            }
        }
        CurrentUser::None => {
            return Ok((StatusCode::FORBIDDEN, "Access forbidden.".into_response()));
        }
    }

    // update
    let exhibition_name = match payload.exhibition_name {
        Some(it) => ActiveValue::Set(Some(it)),
        None => ActiveValue::NotSet,
    };
    let icon_id = match payload.icon_id {
        Some(it) => ActiveValue::Set(Some(it)),
        None => ActiveValue::NotSet,
    };
    let description = match payload.description {
        Some(it) => ActiveValue::Set(Some(it)),
        None => ActiveValue::NotSet,
    };
    exhibitors_root::ActiveModel {
        id: ActiveValue::Set(id),
        created_at: ActiveValue::NotSet,
        updated_at: ActiveValue::NotSet,
        exhibitor_name: ActiveValue::NotSet,
        r#type: ActiveValue::NotSet,
        exhibition_name,
        icon_id,
        description,
        representative1: ActiveValue::NotSet,
        representative2: ActiveValue::NotSet,
        representative3: ActiveValue::NotSet,
    }
    .update(&state.db_conn)
    .await?;

    Ok((StatusCode::CREATED, "Created.".into_response()))
}
