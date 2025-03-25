# 工大祭ポータル

工大祭の参加団体向けポータルサイトです．

## dependencies

- rust
- [cargo-watch](https://crates.io/crates/cargo-watch/8.5.2)
  - dev環境に必要
- docker
- node, nx, npm

## ディレクトリ構成

- **apps**: アプリのソース
- **docs**: ドキュメント
- **libs**: ライブラリのソース

## 開発環境起動方法(backend)

1. dependenciesが全てインストールされているかを確認
2. `nx run backend:docker-up`を実行し，http://localhost:8080からkeycloakのclientを作成．
3. `.`で`nx dev`を実行

- 起動に失敗するのでコンフィグを以下のように書き換える．
   ```toml
   [logging]
   log_level = "Trace"
   json = false
   
   [web.server]
   host = "localhost"
   port = 8000
   base_url = "http://localhost:8000/admin"
   
   [web.auth]
   password_salt = "P3Z9Pya4ixZg73HH"
   activation_salt = "FNi7RleyOlsjWTLi"
   stretch_cost = 13
   jwt_secret_key_path = "./private-key.pem"
   jwt_public_key_path = "./public-key.pem"
   
   [web.auth.keycloak]
   id = "" #ここにkeycloakのclient idを入力
   secret = "" #ここにkeycloakのclient secretを入力
   issuer = "http://localhost:8080/realms/master"
   
   [web.static_files]
   web_path = "."
   admin_path = "."
   
   [db]
   address = "postgres://user:user@localhost:5432/koudaisai-portal"
   ```
  - configファイルの位置は以下の通り
    | OS | path |
    |-------------|--------------------------------------------------------------|
    | Linux(user) | ~/.config/koudaisai-portal/ |
    | Linux(root) | /etc/koudaisai-portal/ |
    | Windows | C:\Users\__username__\AppData\Roaming\koudaisai-portal |
    | macOS | /Users/__username__/Library/Preferences/rs.koudaisai-portal/ |

3. 再度`nx dev`
