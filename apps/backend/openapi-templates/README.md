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
- `model.mustache`: プロパティ由来の enum の型名を `{{{enumName}}}` から
  `{{classname}}{{{enumName}}}` に変更。内蔵テンプレートのままだと `Type` / `Tag` の
  ように名前が重複し、utoipa は型名をそのままスキーマ名にするため components 上で
  互いに上書きし合う(例: `FoodStallProject.type` が `enum: ["stage"]` として
  公開されてしまう)。あわせて utoipa が既知の型として特別扱いする識別子との衝突も
  避けられる(`Time.date` の enum が `Date` という名前になり、日付型と解釈されて
  `{"type":"string","format":"date"}` として公開されていた)。
- `model.mustache`: プロパティ由来の enum の数値判定を `isInteger` から `isNumeric` に
  変更。`type: number` の enum(events26 の `Time.date` は `enum: [1, 2]`)が文字列
  enum として生成され、`"1"` / `"2"` と送ってしまっていたため。
  `serde_repr` は import ではなくパス指定で使う。ファイル先頭の `use` は
  `x-rust-has-integer-property-enum` で切られており `type: number` では入らない。
- `Cargo.mustache`: 依存に `utoipa = "^5.5"` を追加。数値 enum を変種名ではなく
  実際の数値としてスキーマに出すため `repr` フィーチャを有効にしている。
