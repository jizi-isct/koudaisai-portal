# dbの起動
`$ docker compose up -d`
# migration
## migrationの生成
`$ sea-orm-cli migrate generate <migration名>`
## dbのリセット
`$ sea-orm-cli migrate refresh -u postgres://postgres:postgres@localhost/koudaisai-portal`
## entityの生成
`$ sea-orm-cli generate entity -u postgres://postgres:postgres@localhost/koudaisai-portal ./src/entities`
# デバッグ用データ挿入クエリ
```postgresql
BEGIN;
INSERT INTO users (id, first_name, last_name, m_address, exhibition_id)
VALUES ('a9352a62-2377-49fc-85b3-fb22fcf50ac5',
        'Paul',
        'Johnson',
        'paul.j.3858@m.isct.ac.jp',
        'T-001'),
       ('5b43598c-df06-4123-9455-5ebd89d1d29b',
        'Thomas',
        'Bangalter',
        'thomas.b.4826@m.isct.ac.jp',
        'T-001'),
       ('5450b28a-b127-474a-86df-f5ba1d670b9f',
        'Guy-Manuel',
        'de Homem Christo',
        'guy-manuel.d.2371@m.isct.ac.jp',
        'T-001');

INSERT INTO exhibitors_root (id, exhibitor_name, type, representative1, representative2, representative3)
VALUES ('T-001',
        'test1',
        'GENERAL',
        'a9352a62-2377-49fc-85b3-fb22fcf50ac5',
        '5b43598c-df06-4123-9455-5ebd89d1d29b',
        '5450b28a-b127-474a-86df-f5ba1d670b9f');

INSERT INTO exhibitors_category_general
VALUES ('T-001');

COMMIT;
```