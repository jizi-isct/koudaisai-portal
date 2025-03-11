# 工大祭ポータル

工大祭の参加団体向けポータルサイトです．

## ディレクトリ構成

- **debug**: デバッグ
- **docs**: ドキュメント
- **koudaisai-portal-admin**: 工大祭ポータル管理サイトのソース
- **koudaisai-portal-backend**: バックエンドのソース
- **koudaisai-portal-web**: フロントエンドのソース

## 開発環境起動方法(backend)
1. `./debug`で`sudo docker compose up`を起動
    - `keycloak`と`postgres`が起動される
2. `./koudaisai-portal-backend`で`cargo run`
    - 起動に失敗するのでコンフィグを以下のように書き換える．
    ```toml
    [logging]
    log_level = "Trace"
    json = false
    
    [web.server]
    host = "localhost"
    port = 8000
    
    [web.auth]
    password_salt = "P3Z9Pya4ixZg73HH"
    activation_salt = "FNi7RleyOlsjWTLi"
    stretch_cost = 13
    jwt_secret_key_path = "./private-key.pem"
    jwt_public_key_path = "./public-key.pem"
    
    [web.auth.keycloak]
    id = "k-portal"
    secret = "QkKELXaHPwzukDVvMDUGPBLpJp1svbxY"
    issuer = "http://localhost:8080/realms/master"
    
    [web.static_files]
    web_path = "."
    admin_path = "."
    
    [db]
    address = "postgres://user:user@localhost:5432/koudaisai-portal"
    ```
    - configファイルの位置は以下の通り
        - | OS          | path                                                         |
                              |-------------|--------------------------------------------------------------|
          | Linux(user) | ~/.config/koudaisai-portal/                                  |
          | Linux(root) | /etc/koudaisai-portal/                                       |
          | Windows     | C:\Users\__username__\AppData\Roaming\koudaisai-portal       |
          | macOS       | /Users/__username__/Library/Preferences/rs.koudaisai-portal/ |

3. 再度`cargo run`
