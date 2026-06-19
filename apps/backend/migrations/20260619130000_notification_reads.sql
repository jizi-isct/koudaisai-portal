-- ユーザーごとの通知既読状態。`GET /api/v3/users/{id}/notifications` の is_read を担保する。
-- 方針は init.sql / auth.sql に準拠: STRICT・日時は unix ミリ秒(INTEGER)・ID は TEXT。
--
-- 行が存在すれば「既読」。未読は行なし。(user_id, notification_id) で一意。
create table notification_reads
(
    user_id         text    not null references users (id) on delete cascade,
    notification_id text    not null references notifications (id) on delete cascade,
    read_at         integer not null, -- unix ms
    primary key (user_id, notification_id)
) strict;

create index idx_notification_reads_user on notification_reads (user_id);
