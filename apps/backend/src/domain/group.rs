use crate::application::ports::clock::Clock;
use crate::domain::error::FactoryError;
use crate::domain::group_id::GroupId;
use chrono::{DateTime, Utc};

/// 団体の種類を表す列挙型です．
/// 各団体の種類の詳細については委員内ドキュメント(JIZI Wikiなど)を参照してください．
/// メンバーの所属とロールは `Membership` で表現し，どのロール構成が妥当かは
/// `Membership::validate_set` が検証します．
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GroupType {
    /// 学内取材団体．一人の代表者(Representative)を持つ．
    Press,
    /// 一般企画団体．第一〜第三責任者(First/Second/ThirdResponsible)計３名を持つ．
    GeneralProject,
    /// 模擬店企画団体．第一〜第三責任者計３名を持つ．
    BoothProject,
    /// 研究室企画団体．企画責任者(Representative)と企画実施担当者(Operator)を持つ．兼任可能．
    LabProject,
    /// ステージ企画団体．第一〜第三責任者計３名を持つ．
    StageProject,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Group {
    id: GroupId,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    name: String,
    r#type: GroupType,
}

impl Group {
    pub fn register<C: Clock>(
        id: GroupId,
        name: String,
        r#type: GroupType,
        clock: &C,
    ) -> Result<Self, FactoryError> {
        if name.trim().is_empty() {
            return Err(FactoryError::InvalidInput("Name is empty".to_string()));
        }

        Ok(Self {
            id,
            created_at: clock.now(),
            updated_at: clock.now(),
            name,
            r#type,
        })
    }

    pub fn restore(
        id: GroupId,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
        name: String,
        r#type: GroupType,
    ) -> Result<Self, FactoryError> {
        if name.trim().is_empty() {
            return Err(FactoryError::InvalidInput("Name is empty".to_string()));
        }

        Ok(Self {
            id,
            created_at,
            updated_at,
            name,
            r#type,
        })
    }

    pub fn id(&self) -> GroupId {
        self.id
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn rename<C: Clock>(&mut self, name: String, clock: &C) -> Result<(), FactoryError> {
        if name.trim().is_empty() {
            return Err(FactoryError::InvalidInput("Name is empty".to_string()));
        }
        self.name = name;
        self.updated_at = clock.now();
        Ok(())
    }

    pub fn r#type(&self) -> &GroupType {
        &self.r#type
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    struct MockClock {
        now: DateTime<Utc>,
    }

    impl Clock for MockClock {
        fn now(&self) -> DateTime<Utc> {
            self.now
        }
    }

    fn group_id() -> GroupId {
        GroupId::new('A', 1).unwrap()
    }

    #[test]
    fn test_register_success() {
        let now = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
        let clock = MockClock { now };

        let group = Group::register(
            group_id(),
            "Test Group".to_string(),
            GroupType::Press,
            &clock,
        )
        .unwrap();

        assert_eq!(group.id(), group_id());
        assert_eq!(group.name(), "Test Group");
        assert_eq!(group.r#type(), &GroupType::Press);
        assert_eq!(group.created_at(), now);
        assert_eq!(group.updated_at(), now);
    }

    #[test]
    fn test_register_empty_name_fails() {
        let clock = MockClock { now: Utc::now() };

        let result = Group::register(
            group_id(),
            "   ".to_string(),
            GroupType::GeneralProject,
            &clock,
        );

        assert!(matches!(result, Err(FactoryError::InvalidInput(_))));
    }

    #[test]
    fn test_restore_success() {
        let created_at = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
        let updated_at = Utc.with_ymd_and_hms(2024, 1, 1, 1, 0, 0).unwrap();
        let name = "Restored Group".to_string();
        let group_type = GroupType::Press;

        let group =
            Group::restore(group_id(), created_at, updated_at, name.clone(), group_type).unwrap();

        assert_eq!(group.id(), group_id());
        assert_eq!(group.name(), name);
        assert_eq!(group.r#type(), &group_type);
        assert_eq!(group.created_at(), created_at);
        assert_eq!(group.updated_at(), updated_at);
    }

    #[test]
    fn test_rename_success() {
        let initial = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
        let clock1 = MockClock { now: initial };
        let mut group = Group::register(
            group_id(),
            "Old Name".to_string(),
            GroupType::LabProject,
            &clock1,
        )
        .unwrap();

        let updated = Utc.with_ymd_and_hms(2024, 1, 2, 0, 0, 0).unwrap();
        let clock2 = MockClock { now: updated };
        group.rename("New Name".to_string(), &clock2).unwrap();

        assert_eq!(group.name(), "New Name");
        assert_eq!(group.updated_at(), updated);
    }

    #[test]
    fn test_rename_whitespace_fails() {
        let clock = MockClock { now: Utc::now() };
        let mut group = Group::register(
            group_id(),
            "Valid Name".to_string(),
            GroupType::StageProject,
            &clock,
        )
        .unwrap();

        let result = group.rename("   ".to_string(), &clock);

        assert!(matches!(result, Err(FactoryError::InvalidInput(_))));
        assert_eq!(group.name(), "Valid Name");
    }
}
