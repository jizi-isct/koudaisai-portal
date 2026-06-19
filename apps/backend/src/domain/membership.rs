use crate::application::ports::clock::Clock;
use crate::domain::error::FactoryError;
use crate::domain::group::GroupType;
use crate::domain::group_id::GroupId;
use crate::domain::user_id::UserId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// グループ内でのユーザーの役職．
/// どの役職が有効かは `GroupType` に依存する（Press/Lab は代表者、企画系は第一〜第三責任者など）．
/// その整合性は Group 集約が実行時に検証する．
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Role {
    /// 代表者（Press の代表者、Lab の企画責任者）
    Representative,
    /// 企画実施担当者（Lab）
    Operator,
    /// 第一責任者（企画系）
    FirstResponsible,
    /// 第二責任者（企画系）
    SecondResponsible,
    /// 第三責任者（企画系）
    ThirdResponsible,
}

/// ユーザーのグループへの所属．グループ内で1つの役職を持つ．
/// 同一グループ内では `(group_id, role)` が一意（各役職スロットに1人）．
/// Lab で代表者と実施担当者を兼任する場合、同一ユーザーが Representative と Operator の2つの Membership を持つ．
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Membership {
    user_id: UserId,
    group_id: GroupId,
    role: Role,
    created_at: DateTime<Utc>,
}

impl Membership {
    pub fn new(group_id: GroupId, user_id: UserId, role: Role, clock: &dyn Clock) -> Self {
        Self {
            user_id,
            group_id,
            role,
            created_at: clock.now(),
        }
    }

    /// 永続化層から復元する際に使用します（保存済みの `created_at` をそのまま渡す）．
    pub fn restore(
        group_id: GroupId,
        user_id: UserId,
        role: Role,
        created_at: DateTime<Utc>,
    ) -> Self {
        Self {
            user_id,
            group_id,
            role,
            created_at,
        }
    }

    pub fn user_id(&self) -> UserId {
        self.user_id
    }

    pub fn group_id(&self) -> GroupId {
        self.group_id
    }

    pub fn role(&self) -> Role {
        self.role
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    /// メンバーシップ集合が指定された団体種別に対して構造的に妥当かを検証します．
    /// 検証する不変条件:
    /// - すべての役職がその団体種別で許可されたものであること
    /// - 各役職を担うメンバーは最大1人（`<= 1`）であること（空席は許容．ユーザー削除等に対応）
    /// - 企画系の第一〜第三責任者は，存在する分について互いに別ユーザーであること（Lab の兼任は許容）
    ///
    /// 「全役職が埋まっているか（充足性）」はここでは検証しません（別途の問い合わせで扱う）．
    /// Group と Membership は独立しているため，この検証はメンバーシップを変更する
    /// アプリケーションサービスがコマンド実行時に明示的に呼び出して担保します．
    ///
    /// # Errors
    /// - `FactoryError::InvalidInput` - 許可されない役職・役職の重複・責任者の重複がある場合に発生します．
    pub fn validate_set(
        group_type: GroupType,
        memberships: &[Membership],
    ) -> Result<(), FactoryError> {
        let invalid = |msg: &str| Err(FactoryError::InvalidInput(msg.to_string()));

        // 団体種別ごとに許可される役職
        let permitted: &[Role] = match group_type {
            GroupType::Press => &[Role::Representative],
            GroupType::LabProject => &[Role::Representative, Role::Operator],
            GroupType::GeneralProject | GroupType::BoothProject | GroupType::StageProject => &[
                Role::FirstResponsible,
                Role::SecondResponsible,
                Role::ThirdResponsible,
            ],
        };

        // 1. すべての役職が許可されたものか
        if memberships.iter().any(|m| !permitted.contains(&m.role())) {
            return invalid("membership has a role that is not allowed for this group type");
        }

        // 2. 各役職は最大1人（<= 1）
        if permitted
            .iter()
            .any(|role| memberships.iter().filter(|m| m.role() == *role).count() > 1)
        {
            return invalid("a role can be held by at most one member");
        }

        // 3. 企画系は責任者同士が別ユーザーでなければならない（存在する分のみ・Lab の兼任は対象外）
        if matches!(
            group_type,
            GroupType::GeneralProject | GroupType::BoothProject | GroupType::StageProject
        ) {
            let users: Vec<_> = memberships.iter().map(|m| m.user_id()).collect();
            let has_duplicate = users
                .iter()
                .enumerate()
                .any(|(i, u)| users[i + 1..].contains(u));
            if has_duplicate {
                return invalid("responsibles must all be different");
            }
        }

        Ok(())
    }
}
