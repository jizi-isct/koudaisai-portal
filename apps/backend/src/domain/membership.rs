use crate::application::ports::clock::Clock;
use crate::domain::group::GroupType;
use crate::domain::group_id::GroupId;
use crate::domain::user_id::UserId;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Membership {
    user_id: UserId,
    group_id: GroupId,
}

impl Membership {
    pub fn new(group_id: GroupId, user_id: UserId, clock: &dyn Clock) -> Self {
        Self { user_id, group_id }
    }

    pub fn from_group_type<C: Clock>(
        group_id: GroupId,
        group_type: &GroupType,
        clock: &C,
    ) -> Vec<Self> {
        match group_type {
            GroupType::Press { representative } => {
                vec![Membership::new(group_id, *representative, clock)]
            }
            GroupType::GeneralProject {
                representative1,
                representative2,
                representative3,
            } => {
                vec![
                    Membership::new(group_id, *representative1, clock),
                    Membership::new(group_id, *representative2, clock),
                    Membership::new(group_id, *representative3, clock),
                ]
            }
            GroupType::BoothProject {
                representative1,
                representative2,
                representative3,
            } => {
                vec![
                    Membership::new(group_id, *representative1, clock),
                    Membership::new(group_id, *representative2, clock),
                    Membership::new(group_id, *representative3, clock),
                ]
            }
            GroupType::LabProject { representative } => {
                vec![Membership::new(group_id, *representative, clock)]
            }
            GroupType::StageProject {
                representative1,
                representative2,
                representative3,
            } => {
                vec![
                    Membership::new(group_id, *representative1, clock),
                    Membership::new(group_id, *representative2, clock),
                    Membership::new(group_id, *representative3, clock),
                ]
            }
        }
    }

    pub fn user_id(&self) -> UserId {
        self.user_id
    }

    pub fn group_id(&self) -> GroupId {
        self.group_id
    }
}

