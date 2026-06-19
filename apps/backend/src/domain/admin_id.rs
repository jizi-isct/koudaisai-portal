use std::fmt::Display;
use uuid::Uuid;

/// 管理者(JIZI)の識別子。
///
/// 参加団体ユーザーを表す [`crate::domain::user_id::UserId`] とは別物で，
/// 認証経路(Keycloak)も識別空間も異なる。DB 上の `admin_id` 列に対応する。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AdminId(Uuid);

impl AdminId {
    pub fn new(id: Uuid) -> Self {
        Self(id)
    }

    pub fn as_uuid(&self) -> Uuid {
        self.0
    }
}

impl Display for AdminId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<AdminId> for Uuid {
    fn from(admin_id: AdminId) -> Self {
        admin_id.0
    }
}
