-- 既存の訂正申請受付を維持したまま、受付状態を切り替えられるようにする。
alter table settings
    add column accept_correction_requests integer not null default 1
        check (accept_correction_requests in (0, 1));
