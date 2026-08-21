-- 管理者が変更するグローバル設定。
-- singleton = 1 の CHECK 制約により、物理的に 1 行だけ保持する。
create table settings
(
    singleton                 integer primary key not null check (singleton = 1),
    show_occasions_on_portal  integer          not null check (show_occasions_on_portal in (0, 1))
) strict;

insert into settings (singleton, show_occasions_on_portal) values (1, 0);
