use crate::application::error::{DeleteError, InsertError, UpdateError};
use crate::application::ports::events26_api::{Events26Api, UpdateIconError};
use anyhow::anyhow;
use async_trait::async_trait;
use events26_api::models::Project;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// 企画 ID → (content_type, 画像バイト列)。
type Icons = Arc<RwLock<HashMap<String, (String, Vec<u8>)>>>;

/// テスト用の [`Events26Api`]。呼び出しを記録するだけで外部へは出ない。
///
/// 企画情報の編集申請の承認は events26 への反映が伴うため、承認のテストには
/// この差し替えが要る。反映の失敗を再現できるよう、失敗を仕込めるようにしている。
pub struct MemoryEvents26Api {
    projects: Arc<RwLock<HashMap<String, Project>>>,
    descriptions: Arc<RwLock<HashMap<String, String>>>,
    icons: Icons,
    /// 真にすると、以降の書き込みがすべて `InternalError` になる。
    fails: Arc<RwLock<bool>>,
}

impl Default for MemoryEvents26Api {
    fn default() -> Self {
        Self::new()
    }
}

impl MemoryEvents26Api {
    pub fn new() -> Self {
        Self {
            projects: Arc::new(RwLock::new(HashMap::new())),
            descriptions: Arc::new(RwLock::new(HashMap::new())),
            icons: Arc::new(RwLock::new(HashMap::new())),
            fails: Arc::new(RwLock::new(false)),
        }
    }

    /// 以降の書き込みを失敗させる(反映に失敗したときの挙動を試すため)。
    pub fn fail_writes(&self) {
        *self.fails.write().unwrap() = true;
    }

    /// 反映された紹介文を取り出す。
    pub fn description(&self, project_id: &str) -> Option<String> {
        self.descriptions.read().unwrap().get(project_id).cloned()
    }

    /// 反映されたアイコンを (content_type, バイト列) で取り出す。
    pub fn icon(&self, project_id: &str) -> Option<(String, Vec<u8>)> {
        self.icons.read().unwrap().get(project_id).cloned()
    }

    fn guard(&self, operation: &str) -> Result<(), anyhow::Error> {
        if *self.fails.read().unwrap() {
            return Err(anyhow!("memory events26 api: {operation} failed"));
        }
        Ok(())
    }
}

#[async_trait]
impl Events26Api for MemoryEvents26Api {
    async fn create_project(&self, project: &Project) -> Result<Project, InsertError> {
        self.guard("create_project")?;
        let id = project_id(project);
        let mut projects = self.projects.write().unwrap();
        if projects.contains_key(&id) {
            return Err(InsertError::Conflict);
        }
        projects.insert(id, project.clone());
        Ok(project.clone())
    }

    async fn update_project(
        &self,
        project_id: &str,
        project: &Project,
    ) -> Result<Project, UpdateError> {
        self.guard("update_project")?;
        self.projects
            .write()
            .unwrap()
            .insert(project_id.to_string(), project.clone());
        Ok(project.clone())
    }

    async fn update_project_description(
        &self,
        project_id: &str,
        description: &str,
    ) -> Result<(), UpdateError> {
        self.guard("update_project_description")?;
        self.descriptions
            .write()
            .unwrap()
            .insert(project_id.to_string(), description.to_string());
        Ok(())
    }

    async fn delete_project(&self, project_id: &str) -> Result<(), DeleteError> {
        self.guard("delete_project")?;
        self.projects.write().unwrap().remove(project_id);
        Ok(())
    }

    async fn update_project_icon(
        &self,
        project_id: &str,
        content_type: &str,
        image: Vec<u8>,
    ) -> Result<(), UpdateIconError> {
        self.guard("update_project_icon")?;
        self.icons
            .write()
            .unwrap()
            .insert(project_id.to_string(), (content_type.to_string(), image));
        Ok(())
    }

    async fn delete_project_icon(&self, project_id: &str) -> Result<(), DeleteError> {
        self.guard("delete_project_icon")?;
        self.icons.write().unwrap().remove(project_id);
        Ok(())
    }
}

fn project_id(project: &Project) -> String {
    match project {
        Project::FoodStallProject(p) => p.id.clone(),
        Project::GeneralProject(p) => p.id.clone(),
        Project::LaboratoryProject(p) => p.id.clone(),
        Project::StageProject(p) => p.id.clone(),
    }
}
