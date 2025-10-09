pub mod question;
pub mod responses;

use question::Question;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// フォーム
/// * `form_id`: フォームID
/// * `info`: フォームのタイトルと説明
/// * `items`: フォームのアイテムのリスト（質問、改ページ、テキストなど）
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Form {
    pub form_id: Uuid,
    pub info: Info,
    pub items: Vec<Item>,
}

/// フォームの一般情報
/// * `title`: 回答者に表示されるフォームのタイトル
/// * `document_title`: 編集者に表示されるフォームのタイトル
/// * `description`: フォームの説明
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Info {
    pub title: String,
    pub document_title: String,
    pub description: String,
}

/// フォームの単一の項目
/// * `item_id`: アイテムのID
/// * `title`: 回答者に表示される項目のタイトル
/// * `description`: 回答者に表示される項目の説明
/// * `item`: アイテムの種類とより細かいプロパティ
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Item {
    pub item_id: Uuid,
    pub title: String,
    pub description: String,
    #[serde(flatten)]
    pub item: Items,
}

/// アイテムの種類
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Items {
    #[serde(rename = "item_question")]
    Question(ItemQuestion),
    #[serde(rename = "item_page_break")]
    PageBreak(ItemPageBreak),
    #[serde(rename = "item_text")]
    Text(ItemText),
}

/// 一つの質問を含む項目
/// * `question`: 表示される質問
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ItemQuestion {
    question: Question,
}

/// 改ページ
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ItemPageBreak {}

/// テキスト
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ItemText {}
