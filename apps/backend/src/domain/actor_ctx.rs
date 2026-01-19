use crate::domain::group::GroupType;
use crate::domain::membership::Membership;
use crate::domain::user_id::UserId;

/// 認可に関するモデル
/// 認可に関するコンテキストを保持するモデルです。
/// ユーザー、管理者、未ログイン状態を表現します。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ActorContext {
    User {
        user_id: UserId,
        memberships: Vec<Membership>,
        group_type: GroupType,
    },
    Admin {
        user_id: UserId,
        claims: Vec<String>,
    },
    NoLogin
}