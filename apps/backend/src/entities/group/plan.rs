use crate::entities::group::plan::booth::{BoothCreate, BoothRead, BoothUpdate};
use crate::entities::group::plan::general::{GeneralCreate, GeneralRead, GeneralUpdate};
use crate::entities::group::plan::labo::{LaboCreate, LaboRead, LaboUpdate};
use crate::entities::group::plan::stage::{StageCreate, StageRead, StageUpdate};
use serde::{Deserialize, Serialize};

pub mod booth;
pub mod general;
pub mod labo;
pub mod stage;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlanTypeCreate {
    TypeBooth(BoothCreate),
    TypeGeneral(GeneralCreate),
    TypeStage(StageCreate),
    TypeLabo(LaboCreate),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlanTypeRead {
    TypeBooth(BoothRead),
    TypeGeneral(GeneralRead),
    TypeStage(StageRead),
    TypeLabo(LaboRead),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlanTypeUpdate {
    TypeBooth(BoothUpdate),
    TypeGeneral(GeneralUpdate),
    TypeStage(StageUpdate),
    TypeLabo(LaboUpdate),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlanCreate {
    #[serde(flatten)]
    pub r#type: PlanTypeCreate,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlanRead {
    #[serde(flatten)]
    pub r#type: PlanTypeRead,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlanUpdate {
    #[serde(flatten)]
    pub r#type: PlanTypeUpdate,
}
