use crate::entities::group::plan::booth::BoothRead;
use crate::entities::group::plan::general::GeneralRead;
use crate::entities::group::plan::labo::LaboRead;
use crate::entities::group::plan::stage::StageRead;
use crate::entities::group::plan::{
    PlanCreate, PlanRead, PlanTypeCreate, PlanTypeRead, PlanTypeUpdate, PlanUpdate,
};
use crate::entities::group::press::{PressCreate, PressRead, PressUpdate};
use crate::sea_orm_entities;
use crate::util::sha::stretch_with_salt;
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
    pub name: String,
    #[serde(flatten)]
    pub r#type: GroupTypeCreate,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GroupRead {
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub name: String,
    #[serde(flatten)]
    pub r#type: GroupTypeRead,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GroupUpdate {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(flatten, default)]
    pub r#type: Option<GroupTypeUpdate>,
}

impl GroupCreate {
    /// 新規団体をデータベースに挿入します。
    /// # Arguments
    /// * `db_conn` - データベース接続
    /// * `activation_salt` - ユーザーのactivation tokenを生成するためのソルト
    /// * `stretch_cost` - ユーザーのactivation tokenを生成するためのストレッチコスト
    /// * `id` - 団体のID
    /// # Returns
    /// * `Result<Vec<String>, DbErr>` - 団体の挿入により生成されたユーザーのactivation tokenのリストまたはdbエラー
    pub async fn insert(
        self,
        db_conn: &DbConn,
        activation_salt: &str,
        stretch_cost: u8,
        id: String,
    ) -> Result<Vec<String>, DbErr> {
        let transaction = db_conn.begin().await?;
        let mut generated_users_m_addresses = vec![];

        let mut group_type;
        match self.r#type {
            GroupTypeCreate::TypePlan(plan) => {
                let mut plan_type;
                match plan.r#type {
                    PlanTypeCreate::TypeBooth(booth) => {
                        generated_users_m_addresses.push(booth.representative1.m_address.clone());
                        generated_users_m_addresses.push(booth.representative2.m_address.clone());
                        generated_users_m_addresses.push(booth.representative3.m_address.clone());
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
                            id: Set(id.clone()),
                            representative1: Set(representative1_id),
                            representative2: Set(representative2_id),
                            representative3: Set(representative3_id),
                        }
                        .insert(&transaction)
                        .await?;

                        plan_type = sea_orm_entities::sea_orm_active_enums::PlanType::Booth;
                    }
                    PlanTypeCreate::TypeGeneral(general) => {
                        generated_users_m_addresses.push(general.representative1.m_address.clone());
                        generated_users_m_addresses.push(general.representative2.m_address.clone());
                        generated_users_m_addresses.push(general.representative3.m_address.clone());
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
                            id: Set(id.clone()),
                            representative1: Set(representative1_id),
                            representative2: Set(representative2_id),
                            representative3: Set(representative3_id),
                        }
                        .insert(&transaction)
                        .await?;

                        plan_type = sea_orm_entities::sea_orm_active_enums::PlanType::General;
                    }
                    PlanTypeCreate::TypeStage(stage) => {
                        generated_users_m_addresses.push(stage.representative1.m_address.clone());
                        generated_users_m_addresses.push(stage.representative2.m_address.clone());
                        generated_users_m_addresses.push(stage.representative3.m_address.clone());
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
                            id: Set(id.clone()),
                            representative1: Set(representative1_id),
                            representative2: Set(representative2_id),
                            representative3: Set(representative3_id),
                        }
                        .insert(&transaction)
                        .await?;

                        plan_type = sea_orm_entities::sea_orm_active_enums::PlanType::Stage;
                    }
                    PlanTypeCreate::TypeLabo(labo) => {
                        generated_users_m_addresses.push(labo.representative.m_address.clone());
                        // 責任者アカウントを生成
                        let representative_id = Uuid::new_v4();
                        labo.representative
                            .insert(&transaction, representative_id.clone(), id.clone())
                            .await?;

                        // 研究室企画情報を生成
                        sea_orm_entities::group_plan_labo::ActiveModel {
                            id: Set(id.clone()),
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
                generated_users_m_addresses.push(press.representative.m_address.clone());
                let representative_id = Uuid::new_v4();
                press
                    .representative
                    .insert(&transaction, representative_id.clone(), id.clone())
                    .await?;
                sea_orm_entities::group_press::ActiveModel {
                    id: Set(id.clone()),
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

        // ユーザーのactivation tokenを生成
        let mut activation_tokens = vec![];
        for m_address in generated_users_m_addresses {
            activation_tokens.push(stretch_with_salt(
                &m_address,
                activation_salt,
                2_i32.pow(stretch_cost as u32),
            ))
        }

        Ok(activation_tokens)
    }
}

impl GroupRead {
    pub async fn get_all(db_conn: &DbConn) -> Result<Vec<Self>, DbErr> {
        let groups = crate::sea_orm_entities::group::Entity::find()
            .all(db_conn)
            .await?;
        let mut result = Vec::new();

        for group in groups {
            if let Some(group_read) = Self::find_by_id(db_conn, group.id.clone()).await? {
                result.push(group_read);
            }
        }

        Ok(result)
    }

    pub async fn find_by_id<S: Into<String>>(
        db_conn: &DbConn,
        id: S,
    ) -> Result<Option<GroupRead>, DbErr> {
        let id = id.into();
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

pub async fn delete_group(db_conn: &DbConn, id: String) -> Result<(), DbErr> {
    // 団体情報の削除
    sea_orm_entities::group::Entity::delete_by_id(id.clone())
        .exec(db_conn)
        .await?;

    Ok(())
}
