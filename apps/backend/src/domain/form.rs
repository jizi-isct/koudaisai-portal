use crate::application::ports::clock::Clock;
use crate::domain::error::{FactoryError, InvalidTransitionError};
use crate::domain::form_id::FormId;
use crate::domain::target_specifier::TargetSpecifier;
use chrono::{DateTime, Utc};
use std::mem::discriminant;
use uuid::Uuid;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FormType {
    TypeExternal { form_url: String },
}

#[derive(Clone)]
pub struct Form {
    id: FormId,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    created_by: Uuid,
    updated_by: Uuid,
    targets: Vec<TargetSpecifier>,
    name: String,
    summary: String,
    due_date: DateTime<Utc>,
    r#type: FormType,
}

impl Form {
    pub fn register<C: Clock>(
        created_by: Uuid,
        targets: Vec<TargetSpecifier>,
        name: String,
        summary: String,
        due_date: DateTime<Utc>,
        r#type: FormType,
        clock: &C,
    ) -> Result<Self, FactoryError> {
        if name.trim().is_empty() {
            return Err(FactoryError::InvalidInput("Form name is empty".to_string()));
        }

        if summary.trim().is_empty() {
            return Err(FactoryError::InvalidInput("Summary is empty".to_string()));
        }

        let now = clock.now();

        Ok(Self {
            id: FormId::new(Uuid::new_v4()),
            created_at: now,
            updated_at: now,
            created_by,
            updated_by: created_by,
            targets,
            name,
            summary,
            due_date,
            r#type,
        })
    }

    pub fn restore(
        id: FormId,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
        created_by: Uuid,
        updated_by: Uuid,
        targets: Vec<TargetSpecifier>,
        name: String,
        summary: String,
        due_date: DateTime<Utc>,
        r#type: FormType,
    ) -> Self {
        Self {
            id,
            created_at,
            updated_at,
            created_by,
            updated_by,
            targets,
            name,
            summary,
            due_date,
            r#type,
        }
    }

    pub fn id(&self) -> FormId {
        self.id
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }

    pub fn created_by(&self) -> Uuid {
        self.created_by
    }

    pub fn updated_by(&self) -> Uuid {
        self.updated_by
    }

    pub fn targets(&self) -> &Vec<TargetSpecifier> {
        &self.targets
    }

    pub fn set_targets<C: Clock>(
        &mut self,
        targets: Vec<TargetSpecifier>,
        updated_by: Option<Uuid>,
        clock: &C,
    ) -> () {
        self.targets = targets;

        if let Some(user_id) = updated_by {
            self.updated_by = user_id;
        }

        self.updated_at = clock.now();

        ()
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn rename<C: Clock>(
        &mut self,
        name: String,
        updated_by: Uuid,
        clock: &C,
    ) -> Result<(), FactoryError> {
        if name.trim().is_empty() {
            return Err(FactoryError::InvalidInput("Form name is empty".to_string()));
        }

        self.name = name;
        self.updated_by = updated_by;
        self.updated_at = clock.now();

        Ok(())
    }

    pub fn summary(&self) -> &str {
        &self.summary
    }

    pub fn change_summary<C: Clock>(
        &mut self,
        summary: String,
        updated_by: Uuid,
        clock: &C,
    ) -> Result<(), FactoryError> {
        if summary.trim().is_empty() {
            return Err(FactoryError::InvalidInput("Summary is empty".to_string()));
        }

        self.summary = summary;
        self.updated_by = updated_by;
        self.updated_at = clock.now();

        Ok(())
    }

    pub fn due_date(&self) -> DateTime<Utc> {
        self.due_date
    }

    pub fn change_due_date<C: Clock>(
        &mut self,
        due_date: DateTime<Utc>,
        updated_by: Uuid,
        clock: &C,
    ) -> Result<(), FactoryError> {
        self.due_date = due_date;
        self.updated_by = updated_by;
        self.updated_at = clock.now();

        Ok(())
    }

    pub fn r#type(&self) -> &FormType {
        &self.r#type
    }

    pub fn change_type<C: Clock>(
        &mut self,
        r#type: FormType,
        updated_by: Uuid,
        clock: &C,
    ) -> Result<(), InvalidTransitionError> {
        if discriminant(&r#type) != discriminant(&self.r#type) {
            return Err(InvalidTransitionError {});
        }

        self.r#type = r#type;
        self.updated_by = updated_by;
        self.updated_at = clock.now();

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::form_id::FormId;
    use crate::domain::target_specifier::TargetSpecifier;
    use uuid::Uuid;

    struct MockClock {
        now: DateTime<Utc>,
    }

    impl Clock for MockClock {
        fn now(&self) -> DateTime<Utc> {
            self.now
        }
    }

    fn setup_form(clock: &MockClock) -> Form {
        let created_by = Uuid::new_v4();
        let targets = vec![TargetSpecifier::GroupTypePress];
        let name = "Test Form".to_string();
        let summary = "Test Summary".to_string();
        let due_date = clock.now + chrono::Duration::days(7);
        let form_type = FormType::TypeExternal {
            form_url: "https://example.com/form".to_string(),
        };
        Form::register(
            created_by, targets, name, summary, due_date, form_type, clock,
        )
        .unwrap()
    }

    #[test]
    fn test_restore_success() {
        let id = FormId::new(Uuid::new_v4());
        let created_at = Utc::now();
        let updated_at = Utc::now();
        let created_by = Uuid::new_v4();
        let updated_by = Uuid::new_v4();
        let targets = vec![TargetSpecifier::GroupTypePress];
        let name = "Restored Form".to_string();
        let summary = "Restored Summary".to_string();
        let due_date = Utc::now() + chrono::Duration::days(7);
        let form_type = FormType::TypeExternal {
            form_url: "https://example.com/form".to_string(),
        };

        let form = Form::restore(
            id,
            created_at,
            updated_at,
            created_by,
            updated_by,
            targets.clone(),
            name.clone(),
            summary.clone(),
            due_date,
            form_type.clone(),
        );

        assert_eq!(form.id(), id);
        assert_eq!(form.created_at(), created_at);
        assert_eq!(form.updated_at(), updated_at);
        assert_eq!(form.created_by(), created_by);
        assert_eq!(form.updated_by(), updated_by);
        assert_eq!(form.targets(), &targets);
        assert_eq!(form.name(), name);
        assert_eq!(form.summary(), summary);
        assert_eq!(form.due_date(), due_date);
        assert_eq!(form.r#type(), &form_type);
    }

    #[test]
    fn test_set_targets_success() {
        let initial_time = Utc::now();
        let clock = MockClock { now: initial_time };
        let mut form = setup_form(&clock);

        let new_targets = vec![
            TargetSpecifier::GroupTypeProjectGeneral,
            TargetSpecifier::GroupTypeProjectBooth,
        ];
        let update_time = initial_time + chrono::Duration::seconds(10);
        let clock_updated = MockClock { now: update_time };

        form.set_targets(new_targets.clone(), None, &clock_updated);

        assert_eq!(form.targets(), &new_targets);
        assert_eq!(form.updated_at(), update_time);
        assert_eq!(form.created_at(), initial_time);
    }

    #[test]
    fn test_set_targets_with_updated_by() {
        let initial_time = Utc::now();
        let clock = MockClock { now: initial_time };
        let mut form = setup_form(&clock);
        let original_updated_by = form.updated_by();

        let new_targets = vec![TargetSpecifier::GroupTypeProjectGeneral];
        let new_updated_by = Uuid::new_v4();
        let update_time = initial_time + chrono::Duration::seconds(10);
        let clock_updated = MockClock { now: update_time };

        form.set_targets(new_targets.clone(), Some(new_updated_by), &clock_updated);

        assert_eq!(form.targets(), &new_targets);
        assert_eq!(form.updated_by(), new_updated_by);
        assert_ne!(form.updated_by(), original_updated_by);
        assert_eq!(form.updated_at(), update_time);
    }

    #[test]
    fn test_set_targets_without_updated_by() {
        let initial_time = Utc::now();
        let clock = MockClock { now: initial_time };
        let mut form = setup_form(&clock);
        let original_updated_by = form.updated_by();

        let new_targets = vec![TargetSpecifier::GroupTypeProjectGeneral];
        let update_time = initial_time + chrono::Duration::seconds(10);
        let clock_updated = MockClock { now: update_time };

        form.set_targets(new_targets.clone(), None, &clock_updated);

        assert_eq!(form.targets(), &new_targets);
        assert_eq!(form.updated_by(), original_updated_by);
        assert_eq!(form.updated_at(), update_time);
    }

    #[test]
    fn test_change_due_date_success() {
        let initial_time = Utc::now();
        let clock = MockClock { now: initial_time };
        let mut form = setup_form(&clock);

        let new_due_date = initial_time + chrono::Duration::days(14);
        let updated_by = Uuid::new_v4();
        let update_time = initial_time + chrono::Duration::seconds(10);
        let clock_updated = MockClock { now: update_time };

        let result = form.change_due_date(new_due_date, updated_by, &clock_updated);
        assert!(result.is_ok());
        assert_eq!(form.due_date(), new_due_date);
        assert_eq!(form.updated_by(), updated_by);
        assert_eq!(form.updated_at(), update_time);
        assert_eq!(form.created_at(), initial_time);
    }

    #[test]
    fn test_change_type_same_variant_success() {
        let initial_time = Utc::now();
        let clock = MockClock { now: initial_time };
        let mut form = setup_form(&clock);

        let new_type = FormType::TypeExternal {
            form_url: "https://example.com/new-form".to_string(),
        };
        let updated_by = Uuid::new_v4();
        let update_time = initial_time + chrono::Duration::seconds(10);
        let clock_updated = MockClock { now: update_time };

        let result = form.change_type(new_type.clone(), updated_by, &clock_updated);
        assert!(result.is_ok());
        assert_eq!(form.r#type(), &new_type);
        assert_eq!(form.updated_by(), updated_by);
        assert_eq!(form.updated_at(), update_time);
        assert_eq!(form.created_at(), initial_time);
    }
}
