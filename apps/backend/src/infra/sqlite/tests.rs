//! SQLite リポジトリのラウンドトリップ統合テスト。
//! インメモリ DB(単一接続)にマイグレーションを適用し，ドメインモデルの
//! 保存・復元・更新・削除が破綻なく行えること(ADT 列分解/復元，JSON targets，
//! トランザクション経路，CHECK 制約)を検証する。

use crate::application::ports::clock::Clock;
use crate::application::ports::repositories::approval_request_repo::ApprovalRequestRepo;
use crate::application::ports::repositories::document_category_repo::DocumentCategoryRepo;
use crate::application::ports::repositories::document_repo::DocumentRepo;
use crate::application::ports::repositories::form_repo::FormRepo;
use crate::application::ports::repositories::group_repo::GroupRepo;
use crate::application::ports::repositories::membership_repo::MembershipRepo;
use crate::application::ports::repositories::notification_repo::NotificationRepo;
use crate::application::ports::repositories::user_repo::UserRepo;
use crate::application::transaction::Transaction;
use crate::domain::admin_id::AdminId;
use crate::domain::approval_request::{
    ApprovalRequest, ApprovalRequestStatus, ApprovalRequestType,
};
use crate::domain::approval_request_id::ApprovalRequestId;
use crate::domain::document::{Document, DocumentFormat};
use crate::domain::document_category::DocumentCategory;
use crate::domain::email_address::EmailAddress;
use crate::domain::form::{Form, FormType};
use crate::domain::group::{Group, GroupType};
use crate::domain::group_id::GroupId;
use crate::domain::membership::Membership;
use crate::domain::notification::{Notification, NotificationType};
use crate::domain::notification_id::NotificationId;
use crate::domain::password_credentials::PasswordCredentials;
use crate::domain::target_specifier::TargetSpecifier;
use crate::domain::user::{User, UserStatus};
use crate::domain::user_id::UserId;
use crate::infra::sqlite::approval_request_repo_impl::SqliteApprovalRequestRepo;
use crate::infra::sqlite::document_category_repo_impl::SqliteDocumentCategoryRepo;
use crate::infra::sqlite::document_repo_impl::SqliteDocumentRepo;
use crate::infra::sqlite::form_repo_impl::SqliteFormRepo;
use crate::infra::sqlite::group_repo_impl::SqliteGroupRepo;
use crate::infra::sqlite::membership_repo_impl::SqliteMembershipRepo;
use crate::infra::sqlite::notification_repo_impl::SqliteNotificationRepo;
use crate::infra::sqlite::transaction_impl::SqliteTransaction;
use crate::infra::sqlite::user_repo_impl::SqliteUserRepo;
use chrono::{DateTime, TimeZone, Utc};
use sqlx::SqlitePool;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use std::str::FromStr;
use uuid::Uuid;

struct FixedClock {
    now: DateTime<Utc>,
}
impl Clock for FixedClock {
    fn now(&self) -> DateTime<Utc> {
        self.now
    }
}
impl Clock for &FixedClock {
    fn now(&self) -> DateTime<Utc> {
        self.now
    }
}

fn clock() -> FixedClock {
    FixedClock {
        now: Utc.with_ymd_and_hms(2026, 6, 12, 1, 2, 3).unwrap(),
    }
}

/// 単一接続のインメモリ DB プール。max_connections(1) で同一 DB を共有する。
async fn test_pool() -> SqlitePool {
    let opts = SqliteConnectOptions::from_str("sqlite::memory:")
        .unwrap()
        .foreign_keys(true);
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect_with(opts)
        .await
        .unwrap();
    sqlx::migrate!("./migrations").run(&pool).await.unwrap();
    pool
}

#[tokio::test]
async fn user_active_round_trip() {
    let pool = test_pool().await;
    let repo = SqliteUserRepo::new(pool);
    let c = clock();

    let id = UserId::new(Uuid::new_v4());
    let email = EmailAddress::new("active@example.com".to_string()).unwrap();
    let mut user = User::register(id, "Active User".to_string(), email.clone(), &c).unwrap();
    let creds = PasswordCredentials::new("argon2id$phc".to_string(), &c).unwrap();
    user.activate(creds, &c).unwrap();

    repo.insert(&user).await.unwrap();

    // 重複挿入は Conflict
    assert!(matches!(
        repo.insert(&user).await,
        Err(crate::application::error::InsertError::Conflict)
    ));

    let by_id = repo.find_by_id(id).await.unwrap().unwrap();
    assert_eq!(by_id, user);
    assert!(matches!(by_id.status(), UserStatus::Active { .. }));

    let by_addr = repo.find_by_m_address(&email).await.unwrap().unwrap();
    assert_eq!(by_addr.id(), id);

    repo.delete(id).await.unwrap();
    assert!(repo.find_by_id(id).await.unwrap().is_none());
}

#[tokio::test]
async fn group_lab_with_operator_via_transaction() {
    let pool = test_pool().await;
    let user_repo = SqliteUserRepo::new(pool.clone());
    let group_repo = SqliteGroupRepo::new(pool.clone());
    let membership_repo = SqliteMembershipRepo::new(pool.clone());
    let c = clock();

    // FK を満たすため代表者と実施担当者の user を先に作る
    let rep = UserId::new(Uuid::new_v4());
    let op = UserId::new(Uuid::new_v4());
    for (uid, addr) in [(rep, "rep@example.com"), (op, "op@example.com")] {
        let u = User::register(
            uid,
            "U".to_string(),
            EmailAddress::new(addr.to_string()).unwrap(),
            &c,
        )
        .unwrap();
        user_repo.insert(&u).await.unwrap();
    }

    let group_id = GroupId::new('L', 1).unwrap();
    let group_type = GroupType::LabProject {
        representative: rep,
        operator: op,
    };
    let group = Group::register(group_id, "Lab".to_string(), group_type.clone(), &c).unwrap();
    let memberships = Membership::from_group_type(group_id, group.r#type(), &c);

    // create_group 相当: 1 トランザクションで group + memberships を保存
    let mut tx = SqliteTransaction::new(pool.clone());
    tx.begin().await.unwrap();
    group_repo.insert_in(&mut tx, group).await.unwrap();
    for m in memberships {
        membership_repo.insert_in(&mut tx, m).await.unwrap();
    }
    tx.commit().await.unwrap();

    let restored = group_repo.find_by_id(group_id).await.unwrap().unwrap();
    assert_eq!(restored.r#type(), &group_type);

    let members = membership_repo.find_by_group_id(group_id).await.unwrap();
    assert_eq!(members.len(), 2);
}

#[tokio::test]
async fn group_insert_in_rolls_back_on_uncommitted_transaction() {
    let pool = test_pool().await;
    let user_repo = SqliteUserRepo::new(pool.clone());
    let group_repo = SqliteGroupRepo::new(pool.clone());
    let c = clock();

    // FK を満たす代表者を先にコミット済みで作る
    let rep = UserId::new(Uuid::new_v4());
    user_repo
        .insert(
            &User::register(
                rep,
                "U".to_string(),
                EmailAddress::new("rollback-rep@example.com".to_string()).unwrap(),
                &c,
            )
            .unwrap(),
        )
        .await
        .unwrap();

    let group_id = GroupId::new('P', 7).unwrap();
    let group = Group::register(
        group_id,
        "Press".to_string(),
        GroupType::Press {
            representative: rep,
        },
        &c,
    )
    .unwrap();

    // begin したが commit しない → ドロップでロールバックされる
    {
        let mut tx = SqliteTransaction::new(pool.clone());
        tx.begin().await.unwrap();
        group_repo.insert_in(&mut tx, group).await.unwrap();
        // tx をここで drop
    }

    assert!(group_repo.find_by_id(group_id).await.unwrap().is_none());
}

#[tokio::test]
async fn document_pdf_with_targets_round_trip() {
    let pool = test_pool().await;
    let repo = SqliteDocumentRepo::new(pool);
    let c = clock();

    let admin = AdminId::new(Uuid::new_v4());
    let targets = vec![
        TargetSpecifier::GroupTypePress,
        TargetSpecifier::UserId(UserId::new(Uuid::new_v4())),
        TargetSpecifier::UserNologin,
    ];
    let doc = Document::register(
        "資料".to_string(),
        None,
        DocumentFormat::Pdf {
            file_key: "k/1.pdf".to_string(),
            file_name: "1.pdf".to_string(),
        },
        targets.clone(),
        admin,
        &c,
    )
    .unwrap();
    let id = doc.id();

    repo.insert(&doc).await.unwrap();
    let restored = repo.find_by_id(id).await.unwrap().unwrap();
    assert_eq!(restored.targets(), targets.as_slice());
    assert!(matches!(restored.format(), DocumentFormat::Pdf { .. }));

    repo.delete(id).await.unwrap();
    assert!(matches!(
        repo.delete(id).await,
        Err(crate::application::error::DeleteError::NotFound)
    ));
}

#[tokio::test]
async fn form_round_trip() {
    let pool = test_pool().await;
    let repo = SqliteFormRepo::new(pool);
    let c = clock();

    let admin = Uuid::new_v4();
    let form = Form::register(
        admin,
        vec![TargetSpecifier::GroupTypeProjectBooth],
        "申請フォーム".to_string(),
        "概要".to_string(),
        c.now() + chrono::Duration::days(7),
        FormType::TypeExternal {
            form_url: "https://example.com/f".to_string(),
        },
        &c,
    )
    .unwrap();
    let id = form.id();

    repo.insert(form).await.unwrap();
    let restored = repo.find_by_id(id).await.unwrap().unwrap();
    assert_eq!(restored.name(), "申請フォーム");
    assert_eq!(
        restored.r#type(),
        &FormType::TypeExternal {
            form_url: "https://example.com/f".to_string()
        }
    );
}

/// FK(issued_by -> users)を満たすための発行ユーザーを作成して返す。
async fn seed_user(repo: &SqliteUserRepo, addr: &str, c: &FixedClock) -> UserId {
    let id = UserId::new(Uuid::new_v4());
    let user = User::register(
        id,
        "Issuer".to_string(),
        EmailAddress::new(addr.to_string()).unwrap(),
        c,
    )
    .unwrap();
    repo.insert(&user).await.unwrap();
    id
}

#[tokio::test]
async fn approval_request_lifecycle_and_find_by_user_ids() {
    let pool = test_pool().await;
    let user_repo = SqliteUserRepo::new(pool.clone());
    let repo = SqliteApprovalRequestRepo::new(pool);
    let c = clock();

    let issuer = seed_user(&user_repo, "issuer@example.com", &c).await;
    let other = seed_user(&user_repo, "other@example.com", &c).await;

    let id = ApprovalRequestId::generate();
    let mut request = ApprovalRequest::create(
        id,
        issuer,
        ApprovalRequestType::EditExhibitionInfo {
            description: Some("説明".to_string()),
            icon_key: None,
        },
        "申請理由".to_string(),
        &c,
    )
    .unwrap();

    repo.insert(&request).await.unwrap();
    let pending = repo.find_by_id(id).await.unwrap().unwrap();
    assert!(matches!(pending.status(), ApprovalRequestStatus::Pending));

    // pending -> approved を更新して復元
    let admin = AdminId::new(Uuid::new_v4());
    request
        .approve(admin, Some("承認".to_string()), &c)
        .unwrap();
    repo.update(&request).await.unwrap();

    let approved = repo.find_by_id(id).await.unwrap().unwrap();
    assert_eq!(approved, request);
    match approved.status() {
        ApprovalRequestStatus::Approved {
            approved_by,
            approval_reason,
            ..
        } => {
            assert_eq!(*approved_by, admin);
            assert_eq!(approval_reason.as_deref(), Some("承認"));
        }
        s => panic!("expected approved, got {:?}", s),
    }

    // find_by_user_ids は issued_by で絞り込む
    let by_issuer = repo.find_by_user_ids(&[issuer]).await.unwrap();
    assert_eq!(by_issuer.len(), 1);
    assert!(repo.find_by_user_ids(&[other]).await.unwrap().is_empty());
    assert!(repo.find_by_user_ids(&[]).await.unwrap().is_empty());

    repo.delete(id).await.unwrap();
    assert!(matches!(
        repo.delete(id).await,
        Err(crate::application::error::DeleteError::NotFound)
    ));
}

#[tokio::test]
async fn approval_request_rejected_and_closed_round_trip() {
    let pool = test_pool().await;
    let user_repo = SqliteUserRepo::new(pool.clone());
    let repo = SqliteApprovalRequestRepo::new(pool);
    let c = clock();
    let issuer = seed_user(&user_repo, "issuer2@example.com", &c).await;

    // rejected
    let rid = ApprovalRequestId::generate();
    let mut rejected = ApprovalRequest::create(
        rid,
        issuer,
        ApprovalRequestType::EditExhibitionInfo {
            description: None,
            icon_key: Some("icon".to_string()),
        },
        "理由".to_string(),
        &c,
    )
    .unwrap();
    rejected
        .reject(AdminId::new(Uuid::new_v4()), None, &c)
        .unwrap();
    repo.insert(&rejected).await.unwrap();
    assert_eq!(repo.find_by_id(rid).await.unwrap().unwrap(), rejected);

    // closed
    let cid = ApprovalRequestId::generate();
    let mut closed = ApprovalRequest::create(
        cid,
        issuer,
        ApprovalRequestType::EditExhibitionInfo {
            description: None,
            icon_key: None,
        },
        "理由".to_string(),
        &c,
    )
    .unwrap();
    closed.close(&c).unwrap();
    repo.insert(&closed).await.unwrap();
    let restored = repo.find_by_id(cid).await.unwrap().unwrap();
    assert!(matches!(
        restored.status(),
        ApprovalRequestStatus::Closed { .. }
    ));
}

#[tokio::test]
async fn notification_markdown_with_null_author_round_trip() {
    let pool = test_pool().await;
    let repo = SqliteNotificationRepo::new(pool);
    let c = clock();

    let id = NotificationId::generate();
    // システム通知: created_by = None
    let notification = Notification::create(
        id,
        vec![TargetSpecifier::UserNologin],
        NotificationType::markdown("題".to_string(), "本文".to_string()).unwrap(),
        None,
        &c,
    )
    .unwrap();

    repo.insert(&notification).await.unwrap();
    let restored = repo.find_by_id(id).await.unwrap().unwrap();
    assert_eq!(restored, notification);
    assert!(restored.created_by().is_none());

    repo.delete(id).await.unwrap();
    assert!(repo.find_by_id(id).await.unwrap().is_none());
}

#[tokio::test]
async fn notification_approval_request_variant_round_trip() {
    let pool = test_pool().await;
    let user_repo = SqliteUserRepo::new(pool.clone());
    let ar_repo = SqliteApprovalRequestRepo::new(pool.clone());
    let repo = SqliteNotificationRepo::new(pool);
    let c = clock();

    // approval_request_id は approval_requests への FK のため先に作る
    let issuer = seed_user(&user_repo, "issuer3@example.com", &c).await;
    let ar_id = ApprovalRequestId::generate();
    let ar = ApprovalRequest::create(
        ar_id,
        issuer,
        ApprovalRequestType::EditExhibitionInfo {
            description: None,
            icon_key: None,
        },
        "理由".to_string(),
        &c,
    )
    .unwrap();
    ar_repo.insert(&ar).await.unwrap();

    let id = NotificationId::generate();
    let notification = Notification::create(
        id,
        vec![TargetSpecifier::GroupTypePress],
        NotificationType::approval_request(ar_id),
        Some(AdminId::new(Uuid::new_v4())),
        &c,
    )
    .unwrap();
    repo.insert(&notification).await.unwrap();

    let restored = repo.find_by_id(id).await.unwrap().unwrap();
    assert_eq!(restored, notification);
    assert!(matches!(
        restored.notification_type(),
        NotificationType::ApprovalRequest { .. }
    ));
}

#[tokio::test]
async fn document_category_round_trip_and_update() {
    let pool = test_pool().await;
    let repo = SqliteDocumentCategoryRepo::new(pool);
    let c = clock();

    let mut category =
        DocumentCategory::register("カテゴリ".to_string(), Some("📁".to_string()), &c).unwrap();
    let id = category.id();

    repo.insert(&category).await.unwrap();
    assert_eq!(
        repo.find_by_id(id).await.unwrap().unwrap().title(),
        "カテゴリ"
    );

    category.change_title("新カテゴリ".to_string(), &c).unwrap();
    category.change_emoji(None, &c).unwrap();
    repo.update(&category).await.unwrap();

    let restored = repo.find_by_id(id).await.unwrap().unwrap();
    assert_eq!(restored.title(), "新カテゴリ");
    assert_eq!(restored.emoji(), None);

    // 存在しない id の更新は NotFound
    let mut ghost = DocumentCategory::register("g".to_string(), None, &c).unwrap();
    ghost.change_title("x".to_string(), &c).unwrap();
    assert!(matches!(
        repo.update(&ghost).await,
        Err(crate::application::error::UpdateError::NotFound)
    ));
}
