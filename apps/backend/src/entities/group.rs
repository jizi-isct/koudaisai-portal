use crate::entities::group::plan::booth::BoothRead;
use crate::entities::group::plan::general::GeneralRead;
use crate::entities::group::plan::labo::LaboRead;
use crate::entities::group::plan::stage::StageRead;
use crate::entities::group::plan::{
    PlanCreate, PlanRead, PlanTypeCreate, PlanTypeRead, PlanTypeUpdate, PlanUpdate,
};
use crate::entities::group::press::{PressCreate, PressRead, PressUpdate};
use crate::sea_orm_entities;
use crate::util::IntoActiveValue;
use chrono::{DateTime, Utc};
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, DbConn, DbErr, EntityTrait, TransactionTrait};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod plan;
pub mod press;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GroupTypeCreate {
    TypePlan(PlanCreate),
    TypePress(PressCreate),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GroupTypeRead {
    TypePlan(PlanRead),
    TypePress(PressRead),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GroupTypeUpdate {
    TypePlan(PlanUpdate),
    TypePress(PressUpdate),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GroupCreate {
    name: String,
    #[serde(flatten)]
    r#type: GroupTypeCreate,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GroupRead {
    id: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    name: String,
    #[serde(flatten)]
    r#type: GroupTypeRead,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GroupUpdate {
    #[serde(default)]
    name: Option<String>,
    #[serde(flatten, default)]
    r#type: Option<GroupTypeUpdate>,
}

impl GroupCreate {
    pub async fn insert(self, db_conn: &DbConn, id: String) -> Result<(), DbErr> {
        let transaction = db_conn.begin().await?;

        let mut group_type;
        match self.r#type {
            GroupTypeCreate::TypePlan(plan) => {
                let mut plan_type;
                match plan.r#type {
                    PlanTypeCreate::TypeBooth(booth) => {
                        // 責任者アカウントを生成
                        let representative1_id = Uuid::new_v4();
                        let representative2_id = Uuid::new_v4();
                        let representative3_id = Uuid::new_v4();
                        booth
                            .representative1
                            .insert(&transaction, representative1_id.clone(), id.clone())
                            .await?;
                        booth
                            .representative2
                            .insert(&transaction, representative2_id.clone(), id.clone())
                            .await?;
                        booth
                            .representative3
                            .insert(&transaction, representative3_id.clone(), id.clone())
                            .await?;

                        // 模擬店情報を生成
                        sea_orm_entities::group_plan_booth::ActiveModel {
                            id: Default::default(),
                            representative1: Set(representative1_id),
                            representative2: Set(representative2_id),
                            representative3: Set(representative3_id),
                        }
                        .insert(&transaction)
                        .await?;

                        plan_type = sea_orm_entities::sea_orm_active_enums::PlanType::Booth;
                    }
                    PlanTypeCreate::TypeGeneral(general) => {
                        // 責任者アカウントを生成
                        let representative1_id = Uuid::new_v4();
                        let representative2_id = Uuid::new_v4();
                        let representative3_id = Uuid::new_v4();
                        general
                            .representative1
                            .insert(&transaction, representative1_id.clone(), id.clone())
                            .await?;
                        general
                            .representative2
                            .insert(&transaction, representative2_id.clone(), id.clone())
                            .await?;
                        general
                            .representative3
                            .insert(&transaction, representative3_id.clone(), id.clone())
                            .await?;

                        // 一般企画情報を生成
                        sea_orm_entities::group_plan_general::ActiveModel {
                            id: Default::default(),
                            representative1: Set(representative1_id),
                            representative2: Set(representative2_id),
                            representative3: Set(representative3_id),
                        }
                        .insert(&transaction)
                        .await?;

                        plan_type = sea_orm_entities::sea_orm_active_enums::PlanType::General;
                    }
                    PlanTypeCreate::TypeStage(stage) => {
                        // 責任者アカウントを生成
                        let representative1_id = Uuid::new_v4();
                        let representative2_id = Uuid::new_v4();
                        let representative3_id = Uuid::new_v4();
                        stage
                            .representative1
                            .insert(&transaction, representative1_id.clone(), id.clone())
                            .await?;
                        stage
                            .representative2
                            .insert(&transaction, representative2_id.clone(), id.clone())
                            .await?;
                        stage
                            .representative3
                            .insert(&transaction, representative3_id.clone(), id.clone())
                            .await?;

                        // ステージ企画情報を生成
                        sea_orm_entities::group_plan_stage::ActiveModel {
                            id: Default::default(),
                            representative1: Set(representative1_id),
                            representative2: Set(representative2_id),
                            representative3: Set(representative3_id),
                        }
                        .insert(&transaction)
                        .await?;

                        plan_type = sea_orm_entities::sea_orm_active_enums::PlanType::Stage;
                    }
                    PlanTypeCreate::TypeLabo(labo) => {
                        // 責任者アカウントを生成
                        let representative_id = Uuid::new_v4();
                        labo.representative
                            .insert(&transaction, representative_id.clone(), id.clone())
                            .await?;

                        // 研究室企画情報を生成
                        sea_orm_entities::group_plan_labo::ActiveModel {
                            id: Default::default(),
                            representative: Set(representative_id),
                        }
                        .insert(&transaction)
                        .await?;

                        plan_type = sea_orm_entities::sea_orm_active_enums::PlanType::Labo;
                    }
                }
                // 企画情報を生成
                sea_orm_entities::group_plan::ActiveModel {
                    id: Set(id.clone()),
                    r#type: Set(plan_type),
                }
                .insert(&transaction)
                .await?;

                group_type = sea_orm_entities::sea_orm_active_enums::GroupType::Plan;
            }
            GroupTypeCreate::TypePress(press) => {
                let representative_id = Uuid::new_v4();
                press
                    .representative
                    .insert(&transaction, representative_id.clone(), id.clone())
                    .await?;
                sea_orm_entities::group_press::ActiveModel {
                    id: Default::default(),
                    representative: Set(representative_id),
                }
                .insert(&transaction)
                .await?;
                group_type = sea_orm_entities::sea_orm_active_enums::GroupType::Press;
            }
        };

        // 団体情報を生成
        sea_orm_entities::group::ActiveModel {
            id: Set(id.clone()),
            created_at: Set(Some(Utc::now().into())),
            updated_at: Set(Some(Utc::now().into())),
            exhibitor_name: Set(self.name),
            r#type: Set(group_type),
        }
        .insert(&transaction)
        .await?;

        transaction.commit().await?;

        Ok(())
    }
}

impl GroupRead {
    pub async fn find_by_id(db_conn: &DbConn, id: String) -> Result<Option<GroupRead>, DbErr> {
        let model_group = match sea_orm_entities::group::Entity::find_by_id(id.clone())
            .one(db_conn)
            .await
        {
            Ok(Some(model)) => model,
            Ok(None) => return Ok(None),
            Err(e) => return Err(e),
        };

        match model_group.r#type {
            sea_orm_entities::sea_orm_active_enums::GroupType::Plan => {
                let model_plan = match sea_orm_entities::group_plan::Entity::find_by_id(id.clone())
                    .one(db_conn)
                    .await
                {
                    Ok(Some(model)) => model,
                    Ok(None) => return Ok(None),
                    Err(e) => return Err(e),
                };

                match model_plan.r#type {
                    sea_orm_entities::sea_orm_active_enums::PlanType::Booth => {
                        // 模擬店企画の情報を取得
                        let model_booth =
                            match sea_orm_entities::group_plan_booth::Entity::find_by_id(id)
                                .one(db_conn)
                                .await
                            {
                                Ok(Some(model)) => model,
                                Ok(None) => return Ok(None),
                                Err(e) => return Err(e),
                            };

                        // 模擬店企画の情報を返す
                        Ok(Some(GroupRead {
                            id: model_group.id,
                            created_at: model_group.created_at.unwrap().to_utc(),
                            updated_at: model_group.updated_at.unwrap().to_utc(),
                            name: model_group.exhibitor_name,
                            r#type: GroupTypeRead::TypePlan(PlanRead {
                                r#type: PlanTypeRead::TypeBooth(BoothRead {
                                    representative1: model_booth.representative1,
                                    representative2: model_booth.representative2,
                                    representative3: model_booth.representative3,
                                }),
                            }),
                        }))
                    }
                    sea_orm_entities::sea_orm_active_enums::PlanType::General => {
                        // 一般企画の情報を取得
                        let model_general =
                            match sea_orm_entities::group_plan_general::Entity::find_by_id(id)
                                .one(db_conn)
                                .await
                            {
                                Ok(Some(model)) => model,
                                Ok(None) => return Ok(None),
                                Err(e) => return Err(e),
                            };

                        // 一般企画の情報を返す
                        Ok(Some(GroupRead {
                            id: model_group.id,
                            created_at: model_group.created_at.unwrap().to_utc(),
                            updated_at: model_group.updated_at.unwrap().to_utc(),
                            name: model_group.exhibitor_name,
                            r#type: GroupTypeRead::TypePlan(PlanRead {
                                r#type: PlanTypeRead::TypeGeneral(GeneralRead {
                                    representative1: model_general.representative1,
                                    representative2: model_general.representative2,
                                    representative3: model_general.representative3,
                                }),
                            }),
                        }))
                    }
                    sea_orm_entities::sea_orm_active_enums::PlanType::Stage => {
                        // ステージ企画の情報を取得
                        let model_stage =
                            match sea_orm_entities::group_plan_stage::Entity::find_by_id(id)
                                .one(db_conn)
                                .await
                            {
                                Ok(Some(model)) => model,
                                Ok(None) => return Ok(None),
                                Err(e) => return Err(e),
                            };

                        // ステージ企画の情報を返す
                        Ok(Some(GroupRead {
                            id: model_group.id,
                            created_at: model_group.created_at.unwrap().to_utc(),
                            updated_at: model_group.updated_at.unwrap().to_utc(),
                            name: model_group.exhibitor_name,
                            r#type: GroupTypeRead::TypePlan(PlanRead {
                                r#type: PlanTypeRead::TypeStage(StageRead {
                                    representative1: model_stage.representative1,
                                    representative2: model_stage.representative2,
                                    representative3: model_stage.representative3,
                                }),
                            }),
                        }))
                    }
                    sea_orm_entities::sea_orm_active_enums::PlanType::Labo => {
                        // 研究室企画の情報を取得
                        let model_labo =
                            match sea_orm_entities::group_plan_labo::Entity::find_by_id(id)
                                .one(db_conn)
                                .await
                            {
                                Ok(Some(model)) => model,
                                Ok(None) => return Ok(None),
                                Err(e) => return Err(e),
                            };

                        // 研究室企画の情報を返す
                        Ok(Some(GroupRead {
                            id: model_group.id,
                            created_at: model_group.created_at.unwrap().to_utc(),
                            updated_at: model_group.updated_at.unwrap().to_utc(),
                            name: model_group.exhibitor_name,
                            r#type: GroupTypeRead::TypePlan(PlanRead {
                                r#type: PlanTypeRead::TypeLabo(LaboRead {
                                    representative: model_labo.representative,
                                }),
                            }),
                        }))
                    }
                }
            }
            sea_orm_entities::sea_orm_active_enums::GroupType::Press => {
                // 学内取材団体の情報を取得
                let model_press =
                    match sea_orm_entities::group_press::Entity::find_by_id(id.clone())
                        .one(db_conn)
                        .await
                    {
                        Ok(Some(model)) => model,
                        Ok(None) => return Ok(None),
                        Err(e) => return Err(e),
                    };

                // 学内取材団体の情報を返す
                Ok(Some(GroupRead {
                    id: model_group.id,
                    created_at: model_group.created_at.unwrap().to_utc(),
                    updated_at: model_group.updated_at.unwrap().to_utc(),
                    name: model_group.exhibitor_name,
                    r#type: GroupTypeRead::TypePress(PressRead {
                        representative: model_press.representative,
                    }),
                }))
            }
        }
    }
}

impl GroupUpdate {
    pub async fn update(self, db_conn: &DbConn, id: String) -> Result<(), DbErr> {
        let transaction = db_conn.begin().await?;

        // 団体情報の更新
        sea_orm_entities::group::ActiveModel {
            id: Set(id.clone()),
            updated_at: Set(Some(Utc::now().into())),
            exhibitor_name: Set(self.name.unwrap_or_default()),
            ..Default::default()
        }
        .update(&transaction)
        .await?;

        match self.r#type {
            Some(GroupTypeUpdate::TypePlan(plan)) => {
                match plan.r#type {
                    PlanTypeUpdate::TypeBooth(booth) => {
                        // 模擬店企画の更新
                        sea_orm_entities::group_plan_booth::ActiveModel {
                            id: Set(id.clone()),
                            representative1: booth.representative1.into_active_value(),
                            representative2: booth.representative2.into_active_value(),
                            representative3: booth.representative3.into_active_value(),
                        }
                        .update(&transaction)
                        .await?;
                    }
                    PlanTypeUpdate::TypeGeneral(general) => {
                        // 一般企画の更新
                        sea_orm_entities::group_plan_general::ActiveModel {
                            id: Set(id.clone()),
                            representative1: general.representative1.into_active_value(),
                            representative2: general.representative2.into_active_value(),
                            representative3: general.representative3.into_active_value(),
                        }
                        .update(&transaction)
                        .await?;
                    }
                    PlanTypeUpdate::TypeStage(stage) => {
                        // ステージ企画の更新
                        sea_orm_entities::group_plan_stage::ActiveModel {
                            id: Set(id.clone()),
                            representative1: stage.representative1.into_active_value(),
                            representative2: stage.representative2.into_active_value(),
                            representative3: stage.representative3.into_active_value(),
                        }
                        .update(&transaction)
                        .await?;
                    }
                    PlanTypeUpdate::TypeLabo(labo) => {
                        // 研究室企画の更新
                        sea_orm_entities::group_plan_labo::ActiveModel {
                            id: Set(id.clone()),
                            representative: labo.representative.into_active_value(),
                        }
                        .update(&transaction)
                        .await?;
                    }
                }
            }
            Some(GroupTypeUpdate::TypePress(press)) => {
                // 学内取材団体の更新
                sea_orm_entities::group_press::ActiveModel {
                    id: Set(id.clone()),
                    representative: press.representative.into_active_value(),
                }
                .update(&transaction)
                .await?;
            }
            None => {}
        }
        transaction.commit().await
    }
}
