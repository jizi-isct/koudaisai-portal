-- 工大祭ポータル 初期スキーマ (SQLite)
--
-- 方針:
--   * データベースにはデータ「構造」のみを置き，ドメイン知識はバックエンドのコードで表現する．
--   * ただし CHECK 制約は代数的データ型(タグ付きユニオン)の表現として用いる．
--     すなわち「どの列がどのバリアントで non-null か」という構造的整合性のみを担保し，
--     業務ルールそのものは記述しない．
--   * SQLite は CREATE DOMAIN / CREATE TYPE / 配列型を持たないため，
--     - ID・列挙はすべて TEXT
--     - 列挙の取りうる値は CHECK (... IN (...)) で表現
--     - 日時は UNIX エポックからのミリ秒(INTEGER)
--     - 配列(targets)は JSON 配列文字列の TEXT
--   * 全テーブルを STRICT 宣言し，宣言した型(TEXT/INTEGER)を実際に強制する．
--   * 外部キーを効かせるには接続ごとに PRAGMA foreign_keys = ON が必要(infra 側で設定)．

-- users -----------------------------------------------------------------

create table users
(
    id                  text primary key not null,                  -- uuid
    created_at          integer          not null,                  -- unix ms
    updated_at          integer          not null,
    name                text             not null,
    m_address           text             not null unique,
    status              text             not null check (status in ('registered', 'active', 'deactivated')),
    password_phc        text,
    password_changed_at integer,
    deactivated_at      integer,
    deactivation_reason text,

    -- UserStatus(代数的データ型)のバリアントごとの列構成
    check (
        (
            status = 'registered'
                and password_phc is null
                and password_changed_at is null
                and deactivated_at is null
                and deactivation_reason is null
            )
            or (
            status = 'active'
                and password_phc is not null
                and password_changed_at is not null
                and deactivated_at is null
                and deactivation_reason is null
            )
            or (
            status = 'deactivated'
                and password_phc is not null
                and password_changed_at is not null
                and deactivated_at is not null
                and deactivation_reason is not null
            )
        )
) strict;

-- groups ----------------------------------------------------------------
-- GroupType は団体種別を表す単純な列挙型。所属と役職は memberships で表現する。

create table groups
(
    id         text primary key not null,                            -- GroupId 文字列表現 "X-NNN"
    created_at integer          not null,                            -- unix ms
    updated_at integer          not null,
    name       text             not null,
    type       text             not null check (type in ('press', 'general_project', 'booth_project', 'lab_project', 'stage_project'))
) strict;

-- memberships -----------------------------------------------------------
-- ユーザーのグループへの所属と役職(Role)。
--   * 同一グループ内では各役職スロットを高々1人が担うため (group_id, role) を主キーとする。
--     これにより「各役職は最大1人(<=1)」という構造的整合性のみを担保する。
--   * 研究室企画で責任者(representative)と実施担当者(operator)を兼任する場合は，
--     同一ユーザーが role 違いの2行を持つ(= 同一 (user_id, group_id) が複数行になりうる)。
--   * 「どの役職がどの団体種別で有効か」「責任者が互いに別人か」「充足しているか」等の
--     業務ルールはバックエンド(Membership::validate_set)で担保する。

create table memberships
(
    group_id   text    not null references groups (id) on delete cascade,
    user_id    text    not null references users (id) on delete cascade,
    role       text    not null check (role in ('representative', 'operator', 'first_responsible', 'second_responsible', 'third_responsible')),
    created_at integer not null,                                     -- unix ms

    primary key (group_id, role)
) strict;

-- ユーザーの所属グループ逆引き(ActorContext 構築など)用の索引
create index idx_memberships_user_id on memberships (user_id);

-- approval_requests -----------------------------------------------------

create table approval_requests
(
    id               text primary key not null,                      -- uuid
    issued_at        integer          not null,                      -- unix ms
    issued_by        text             not null references users (id), -- UserId
    type             text             not null check (type in ('edit_exhibition_info')),
    status           text             not null check (status in ('pending', 'approved', 'rejected', 'closed')),
    description      text,
    icon_key         text,
    issue_reason     text             not null,
    approved_by      text,                                            -- admin_id (uuid)
    approved_at      integer,                                         -- unix ms
    approval_reason  text,
    rejected_by      text,                                            -- admin_id (uuid)
    rejected_at      integer,                                         -- unix ms
    rejection_reason text,
    closed_at        integer,                                         -- unix ms

    -- ApprovalRequestStatus(代数的データ型)のバリアントごとの列構成
    check (
        (
            status = 'pending'
                and approved_by is null
                and approved_at is null
                and approval_reason is null
                and rejected_by is null
                and rejected_at is null
                and rejection_reason is null
                and closed_at is null
            )
            or (
            status = 'approved'
                and approved_by is not null
                and approved_at is not null
                and rejected_by is null
                and rejected_at is null
                and rejection_reason is null
                and closed_at is null
            )
            or (
            status = 'rejected'
                and approved_by is null
                and approved_at is null
                and approval_reason is null
                and rejected_by is not null
                and rejected_at is not null
                and closed_at is null
            )
            or (
            status = 'closed'
                and approved_by is null
                and approved_at is null
                and approval_reason is null
                and rejected_by is null
                and rejected_at is null
                and rejection_reason is null
                and closed_at is not null
            )
        )
) strict;

-- document_categories ---------------------------------------------------

create table document_categories
(
    id         text primary key not null,                            -- uuid
    created_at integer          not null,                            -- unix ms
    updated_at integer          not null,
    title      text             not null,
    emoji      text
) strict;

-- documents -------------------------------------------------------------

create table documents
(
    id         text primary key not null,                            -- uuid
    created_at integer          not null,                            -- unix ms
    updated_at integer          not null,
    created_by text             not null,                            -- admin_id (uuid)
    updated_by text             not null,                            -- admin_id (uuid)
    title      text             not null,
    category   text references document_categories (id),             -- uuid, nullable
    targets    text             not null check (json_valid(targets)), -- JSON array of TargetSpecifier
    format     text             not null check (format in ('markdown', 'pdf', 'misc')),
    content    text,
    file_key   text,
    file_name  text,

    -- DocumentFormat(代数的データ型)のバリアントごとの列構成
    check (
        (
            format = 'markdown'
                and content is not null
                and file_key is null
                and file_name is null
            )
            or (
            format in ('pdf', 'misc')
                and content is null
                and file_key is not null
                and file_name is not null
            )
        )
) strict;

-- forms -----------------------------------------------------------------

create table forms
(
    id         text primary key not null,                            -- uuid
    created_at integer          not null,                            -- unix ms
    updated_at integer          not null,
    created_by text             not null,                            -- admin_id (uuid)
    updated_by text             not null,                            -- admin_id (uuid)
    targets    text             not null check (json_valid(targets)), -- JSON array of TargetSpecifier
    name       text             not null,
    summary    text             not null,
    due_date   integer          not null,                            -- unix ms．ドメイン Form.due_date は必須
    type       text             not null check (type in ('external')),
    form_url   text,

    -- FormType(代数的データ型)のバリアントごとの列構成
    check (
        type = 'external'
            and form_url is not null
        )
) strict;

-- notifications ---------------------------------------------------------

create table notifications
(
    id                  text primary key not null,                   -- uuid
    created_at          integer          not null,                   -- unix ms
    updated_at          integer          not null,
    created_by          text,                                        -- admin_id (uuid)．ドメインは Option<...> のため nullable
    updated_by          text,                                        -- admin_id (uuid)
    targets             text             not null check (json_valid(targets)),
    type                text             not null check (type in ('markdown', 'approval_request')),
    approval_request_id text references approval_requests (id),
    title               text,
    content             text,

    -- NotificationType(代数的データ型)のバリアントごとの列構成
    check (
        (
            type = 'markdown'
                and approval_request_id is null
                and title is not null
                and content is not null
            )
            or (
            type = 'approval_request'
                and approval_request_id is not null
                and title is null
                and content is null
            )
        )
) strict;

create table read_notifications
(
    user_id         text not null references users (id) on delete cascade,
    notification_id text not null references notifications (id) on delete cascade,
    primary key (user_id, notification_id)
) strict;

create table revoked_refresh_tokens
(
    refresh_token text primary key not null,
    exp           integer          not null
) strict;
