# openapi-generator テンプレート上書き

`nx generate-rust-client backend` が使う openapi-generator (rust generator) の
テンプレート上書き。ここに置いたファイルだけが差し替わり、残りは generator 内蔵の
テンプレートにフォールバックする。

## 上書きしている理由

生成されたモデルをポータルの OpenAPI 仕様(utoipa)にそのまま載せるため。
これが無いと `events26_api::models::*` は `utoipa::ToSchema` を実装せず、
外部クレートの型なので orphan rule により後付けもできない。結果、
`/api/v3/events26` のリクエストボディを自由形式オブジェクトとしてしか
公開できなくなる。

## 内蔵テンプレートからの差分

いずれも openapi-generator 7.24.0 の `rust/` テンプレートのコピーに以下を加えたもの。
generator を上げたときは、内蔵テンプレートを取り直して同じ変更を当て直すこと。

    unzip -j <openapi-generator-cli.jar> 'rust/model.mustache' 'rust/Cargo.mustache'

- `model.mustache`: 全 `#[derive(...)]` に `utoipa::ToSchema` を追加
- `model.mustache`: プロパティ由来の enum に `#[schema(as = {{classname}}{{{enumName}}})]` を追加。
  生成される内側の enum は `Type` / `Tag` のように名前が重複し、utoipa は型名を
  そのままスキーマ名にするため、これが無いと components 上で互いに上書きし合う
  (例: `FoodStallProject.type` が `enum: ["stage"]` として公開されてしまう)。
- `Cargo.mustache`: 依存に `utoipa = "^5.5"` を追加
