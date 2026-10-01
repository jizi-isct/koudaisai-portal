# @koudaisai-portal/shared-events26

2026 年の企画詳細情報に使う UI コンポーネントです。API の取得と保存は呼び出し元が担当します。

- `MenuInfoForm`: `menu` で初期値を渡し、検証済みのメニューを `onSubmit` で受け取ります。`children` はフォーム末尾に表示され、保存ボタンなどを配置できます。`disabled` で保存中の入力を止められます。
- `AdditionalInfoEditor`: `initialMarkdown` で初期値を渡し、編集結果を `onChange` で受け取ります。解析エラーは `onError` に文字列で通知します。`readOnly` と `placeholder` を指定できます。

どちらもリポジトリ内では `@koudaisai-portal/shared-events26` から import します。モーダルの開閉、API 呼び出し、保存結果の表示は利用側で実装します。
