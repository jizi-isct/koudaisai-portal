use serde::Serialize;
use crate::application::ports::clock::Clock;
use crate::domain::group::GroupType;
use crate::domain::group_id::GroupId;
use crate::domain::user_id::UserId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Membership {
    user_id: UserId,
    group_id: GroupId,
}

impl Membership {
    pub fn new(group_id: GroupId, user_id: UserId, clock: &dyn Clock) -> Self {
        Self { user_id, group_id }
    }

    pub fn from_group_type<C: Clock>(group_id: GroupId, group_type: &GroupType, clock: &C) -> Vec<Self> {
        match group_type {
            GroupType::Press { representative } => {
                vec![
                    Membership::new(group_id, *representative, clock)
                ]
            }
            GroupType::GeneralProject { representative1, representative2, representative3 } => {
                vec![
                    Membership::new(group_id, *representative1, clock),
                    Membership::new(group_id, *representative2, clock),
                    Membership::new(group_id, *representative3, clock)
                ]
            }
            GroupType::BoothProject { representative1, representative2, representative3 } => {
                vec![
                    Membership::new(group_id, *representative1, clock),
                    Membership::new(group_id, *representative2, clock),
                    Membership::new(group_id, *representative3, clock)
                ]
            }
            GroupType::LabProject { representative } => {
                vec![
                    Membership::new(group_id, *representative, clock)
                ]
            }
            GroupType::StageProject { representative1, representative2, representative3 } => {
                vec![
                    Membership::new(group_id, *representative1, clock),
                    Membership::new(group_id, *representative2, clock),
                    Membership::new(group_id, *representative3, clock)
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::user_id::UserId;
    use crate::domain::group_id::GroupId;
    use crate::domain::group::GroupType;
    use crate::application::ports::clock::Clock;
    use chrono::{DateTime, Utc};
    use uuid::Uuid;

    struct MockClock;
    impl Clock for MockClock {
        fn now(&self) -> DateTime<Utc> {
            Utc::now()
        }
    }

    fn setup_ids() -> (GroupId, UserId) {
        let group_id = GroupId::new('G', 1).unwrap();
        let user_id = UserId::new(Uuid::new_v4());
        (group_id, user_id)
    }

    #[test]
    fn test_new_success() {
        let (group_id, user_id) = setup_ids();
        let clock = MockClock;
        let membership = Membership::new(group_id, user_id, &clock);

        assert_eq!(membership.group_id(), group_id);
        assert_eq!(membership.user_id(), user_id);
    }

    #[test]
    fn test_from_group_type_press() {
        let (group_id, representative) = setup_ids();
        let group_type = GroupType::Press { representative };
        let clock = MockClock;

        let memberships = Membership::from_group_type(group_id, &group_type, &clock);

        assert_eq!(memberships.len(), 1);
        assert_eq!(memberships[0].group_id(), group_id);
        assert_eq!(memberships[0].user_id(), representative);
    }

    #[test]
    fn test_from_group_type_general_project() {
        let group_id = GroupId::new('G', 1).unwrap();
        let rep1 = UserId::new(Uuid::new_v4());
        let rep2 = UserId::new(Uuid::new_v4());
        let rep3 = UserId::new(Uuid::new_v4());
        let group_type = GroupType::GeneralProject {
            representative1: rep1,
            representative2: rep2,
            representative3: rep3,
        };
        let clock = MockClock;

        let memberships = Membership::from_group_type(group_id, &group_type, &clock);

        assert_eq!(memberships.len(), 3);
        assert_eq!(memberships[0].user_id(), rep1);
        assert_eq!(memberships[1].user_id(), rep2);
        assert_eq!(memberships[2].user_id(), rep3);
        for m in &memberships {
            assert_eq!(m.group_id(), group_id);
        }
    }

    #[test]
    fn test_from_group_type_booth_project() {
        let group_id = GroupId::new('B', 1).unwrap();
        let rep1 = UserId::new(Uuid::new_v4());
        let rep2 = UserId::new(Uuid::new_v4());
        let rep3 = UserId::new(Uuid::new_v4());
        let group_type = GroupType::BoothProject {
            representative1: rep1,
            representative2: rep2,
            representative3: rep3,
        };
        let clock = MockClock;

        let memberships = Membership::from_group_type(group_id, &group_type, &clock);

        assert_eq!(memberships.len(), 3);
        assert_eq!(memberships[0].user_id(), rep1);
        assert_eq!(memberships[1].user_id(), rep2);
        assert_eq!(memberships[2].user_id(), rep3);
    }

    #[test]
    fn test_from_group_type_lab_project() {
        let (group_id, representative) = setup_ids();
        let group_type = GroupType::LabProject { representative };
        let clock = MockClock;

        let memberships = Membership::from_group_type(group_id, &group_type, &clock);

        assert_eq!(memberships.len(), 1);
        assert_eq!(memberships[0].user_id(), representative);
    }

    #[test]
    fn test_from_group_type_stage_project() {
        let group_id = GroupId::new('S', 1).unwrap();
        let rep1 = UserId::new(Uuid::new_v4());
        let rep2 = UserId::new(Uuid::new_v4());
        let rep3 = UserId::new(Uuid::new_v4());
        let group_type = GroupType::StageProject {
            representative1: rep1,
            representative2: rep2,
            representative3: rep3,
        };
        let clock = MockClock;

        let memberships = Membership::from_group_type(group_id, &group_type, &clock);

        assert_eq!(memberships.len(), 3);
        assert_eq!(memberships[0].user_id(), rep1);
        assert_eq!(memberships[1].user_id(), rep2);
        assert_eq!(memberships[2].user_id(), rep3);
    }
}

