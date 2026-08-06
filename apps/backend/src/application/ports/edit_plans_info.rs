use crate::application::error::ApplicationError;


#[derive(Debug, Clone, PartialEq, Eq, Hash )]
pub struct PlaceId (pub String);

impl PlaceId {
    pub fn as_str(&self) -> &str{
        &self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PlanDateTime {
    pub day: u8,
    pub hour: u8,
    pub minute: u8
    
}

#[derive(Debug, Clone)]
pub struct TimeRange {
    pub start: PlanDateTime,
    pub end: PlanDateTime
}


#[derive(Debug, Clone)]
pub struct PlanOccasion {
    pub place: PlaceId,
    pub time_range: TimeRange 
}


#[derive(Debug, Clone)]
pub struct ProjectField {
    pub id: String,
    pub group_name: String,
    pub project_name: String,
    pub description: String,
    pub is_child_friendly: bool,
    pub is_recommended: bool,
    pub occasions: Vec<PlanOccasion>,
    pub tag: Option<Vec<String>>
}

#[async_trait::async_trait]
pub trait EditPlansInfo {
    async fn create(&self, field: &ProjectField) -> Result<(), ApplicationError>;
    async fn update(&self, field: &ProjectField) -> Result<(), ApplicationError>;
    async fn delete(&self, id: &String) -> Result<(), ApplicationError>;
}
