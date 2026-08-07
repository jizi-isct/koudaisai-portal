-- approval_requests に対象団体を持たせる。
--
-- 1 人が複数の団体に所属しうる(仕様)ため、申請者から対象団体を一意に決められない。
-- 承認時の企画情報API(events26)への反映先(企画番号 = 団体 ID)もこれで決まる。
--
-- SQLite は既存表への not null 列追加ができない(既定値が要る)ため nullable で足し、
-- 既存行は申請者の所属から埋める。以降の行はアプリ側が必ず設定する。

alter table approval_requests
    add column group_id text references groups (id);

-- 既存行の補完。所属が複数ある場合は最も古い所属を採る(発行時点の対象が残っていないため)。
update approval_requests
set group_id = (select m.group_id
                from memberships m
                where m.user_id = approval_requests.issued_by
                order by m.created_at
                limit 1)
where group_id is null;

-- 団体別の申請一覧(管理画面)で使う。
create index idx_approval_requests_group_id on approval_requests (group_id);
