pub mod question;
pub mod responses;

use chrono::{DateTime, Utc};
use question::Question;
use serde::{Deserialize, Serialize, Serializer};
use uuid::Uuid;

/// フォーム
/// * `form_id`: フォームID
/// * `created_at`: 作成日時
/// * `updated_at`: 更新日時
/// * `info`: フォームのタイトルと説明
/// * `items`: フォームのアイテムのリスト（質問、改ページ、テキストなど）
/// * `access_control`: フォームのアクセス制限
#[derive(Serialize, Deserialize)]
pub struct Form {
    pub form_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub info: Info,
    pub items: Vec<Item>,
    pub access_control: AccessControl,
}

/// フォームの一般情報
/// * `title`: 回答者に表示されるフォームのタイトル
/// * `document_title`: 編集者に表示されるフォームのタイトル
/// * `description`: フォームの説明
#[derive(Serialize, Deserialize, Debug)]
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
#[derive(Serialize, Deserialize, Debug)]
pub struct Item {
    pub item_id: Uuid,
    pub title: String,
    pub description: String,
    #[serde(flatten)]
    pub item: Items,
}

/// アイテムの種類
#[derive(Serialize, Deserialize, Debug)]
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
#[derive(Serialize, Deserialize, Debug)]
pub struct ItemQuestion {
    question: Question,
}

/// 改ページ
#[derive(Serialize, Deserialize, Debug)]
pub struct ItemPageBreak {}

/// テキスト
#[derive(Serialize, Deserialize, Debug)]
pub struct ItemText {}

/// フォームのアクセス制限
/// * `roles`: アクセス可能なロール
#[derive(Serialize, Deserialize, Debug)]
pub struct AccessControl {
    pub roles: Vec<String>,
}

impl Form {
    pub fn from_model(
        model: &crate::sea_orm_entities::forms::Model,
    ) -> Result<Self, serde_json::error::Error> {
        let form_id = model.form_id;
        let created_at = model.created_at.unwrap().into();
        let updated_at = model.updated_at.unwrap().into();
        let info = serde_json::from_value(model.info.clone())?;
        let items = serde_json::from_value(model.items.clone())?;
        let access_control = AccessControl {
            roles: (&model.access_control_roles).clone(),
        };

        Ok(Form {
            form_id,
            created_at,
            updated_at,
            info,
            items,
            access_control,
        })
    }
}
