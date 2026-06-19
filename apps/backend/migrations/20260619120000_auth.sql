-- 認証刷新: ワンタイムトークン(activation/reset) + サーバーサイド回転セッション。
-- 旧来のステートレス refresh JWT の denylist は廃止する。
--
-- 方針は init.sql に準拠: STRICT・日時は unix ミリ秒(INTEGER)・ID/列挙は TEXT・
-- 代数的データ型のバリアントは CHECK で構造的整合性のみ担保する。

-- 旧 denylist を撤去(回転セッションへ置換)。
drop table revoked_refresh_tokens;

-- one_time_tokens -------------------------------------------------------
-- 単一使用トークン。平文は "{id}.{secret}" でユーザーへ配送し，DB には
-- token_hash(= HMAC-SHA256(pepper, secret) の hex)のみ保存する。
create table one_time_tokens
(
    id          text primary key not null,                          -- uuid
    user_id     text             not null references users (id) on delete cascade,
    purpose     text             not null check (purpose in ('activation', 'password_reset')),
    token_hash  text             not null,
    created_at  integer          not null,                          -- unix ms
    expires_at  integer          not null,
    consumed_at integer,                                            -- 未消費は null
    m_address   text                                                -- activation の m_address 束縛(nullable)
) strict;

create index idx_one_time_tokens_user_purpose on one_time_tokens (user_id, purpose);

-- sessions (ファミリ) ----------------------------------------------------
-- リフレッシュトークンのファミリ。絶対期限は発行時に固定し，回転で延びない。
create table sessions
(
    id                  text primary key not null,                  -- uuid (session_id)
    user_id             text             not null references users (id) on delete cascade,
    created_at          integer          not null,                  -- unix ms
    absolute_expires_at integer          not null,
    state               text             not null check (state in ('active', 'revoked')),
    revoked_at          integer,
    revocation_reason   text check (revocation_reason in
        ('logout', 'logout_all', 'reuse_detected', 'password_changed',
         'deactivated', 'admin_revoke', 'evicted_lru', 'expired')),

    -- SessionState(代数的データ型)のバリアントごとの列構成
    check (
        (state = 'active' and revoked_at is null and revocation_reason is null)
        or (state = 'revoked' and revoked_at is not null and revocation_reason is not null)
    )
) strict;

create index idx_sessions_user_id on sessions (user_id);

-- session_tokens (世代) -------------------------------------------------
-- 各世代が独自の secret_hash とアイドル期限を持つ。回転で旧世代を rotated にする。
create table session_tokens
(
    id              text primary key not null,                      -- uuid (token_id)
    session_id      text             not null references sessions (id) on delete cascade,
    generation      integer          not null,
    secret_hash     text             not null,
    issued_at       integer          not null,                      -- unix ms
    idle_expires_at integer          not null,
    status          text             not null check (status in ('issued', 'rotated', 'revoked')),

    unique (session_id, generation)
) strict;

create index idx_session_tokens_session_id on session_tokens (session_id);
