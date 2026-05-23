# Contributing
私たちはこのリポジトリに対する貢献を歓迎します！
We welcome contributions to this repository!

## 開発環境を構築する

このリポジトリでは Nix flakes を使って開発に必要なツールを揃えます．`flake.nix` で管理している主なツールは Rust，Node.js，cargo-watch，git です．

### 事前に必要なもの

- [Nix](https://nixos.org/download/)
- [direnv](https://direnv.net/)
- Docker Desktop など，Docker daemon を起動できる環境

Docker と Docker Compose は開発用 DB・Keycloak の起動に使います．Nix の dev shell には Docker daemon は含まれないため，`npx nx docker-up backend` を実行する前に Docker Desktop などを起動してください．

### セットアップ手順

1. **direnv で Nix の開発環境を有効化**
    ```shell
    direnv allow
    ```

    以降はリポジトリディレクトリに入ると自動で Nix の開発環境が有効になります．
2. **依存関係をインストール**
    リポジトリルートで
    ```shell
    npm ci
    ```
    を実行する
3. **jwt署名用の鍵を生成**
    ```shell
    cd apps/backend/debug
    ./init-keys.sh
    ```
4. **コンフィグを正しい位置に配置**
    `npx nx dev backend` または `npx nx dev` で開発環境を起動する場合は，`apps/backend/debug/default-config.toml` が自動で読み込まれます．

    手動で backend を起動する場合は，以下のようにコンフィグを配置してください．
    ```shell
    cd apps/backend/debug
    cp default-config.toml <配置先>
    ```
   `<配置先>` はOSによって異なります．
   - macOS: `~/Library/Application Support/rs.koudaisai-portal/`
   - Linux: `~/.config/koudaisai-portal/`
   - Windows: `C:\Users\<ユーザー名>\AppData\Roaming\rs.koudaisai-portal\`

   または，環境変数 `KOUDAISAI_PORTAL_CONFIG_PATH` にコンフィグファイルのパスを指定できます．
   ```shell
   export KOUDAISAI_PORTAL_CONFIG_PATH=/path/to/config.toml
   ```
    
5. **開発環境用データベース・Keycloakを起動**
    ```shell
    npx nx docker-up backend
    ```
6. **開発環境を起動**
    ```shell
    npx nx dev
    ```

    次の URL で各アプリケーションにアクセスできます．
    - portal: `http://portal.koudaisai.localhost`
    - admin: `http://admin.koudaisai.localhost`
    - join: `http://join.koudaisai.localhost`
    - backend API: `http://api.koudaisai.localhost`

    Caddy が 80 番ポートを利用できない環境では，管理者権限での実行が必要になる場合があります．

## 貢献の方法 / How to Contribute
1. Issue を立てるか，既存の Issue を確認して，取り組みたいものを見つける．
2. リポジトリをフォークし，ローカルにクローン( チームメンバーはスキップしてください． )
3. 新しいブランチを作成し，変更を加える．
4. 変更をコミットし，プッシュ
5. プルリクエストを作成し，変更内容を説明する．
6. メンテナーがレビューし承認・マージ

### ブランチ命名規則
ブランチは以下の命名規則に基づいて命名してください．
```
<prefix>/#<issue番号>-<変更内容を示す短いタイトル>
```

- prefix:
  - 新規機能を追加する場合: `feature`
  - バグを修正する場合: `fix`
  - 急を要する修正: `hotfix`
  - リファクタリング: `refactor`
  - コードの軽微な改善: `chore`
  - テスト関連: `test`
  - etc...
- 例
  - `feature/#1-add-login-page`
  - `fix/#2-fix-bug-in-login-page`
  - `hotfix/#3-fix-critical-bug`
