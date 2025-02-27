pub mod question;
pub mod responses;

use chrono::{DateTime, Utc};
use question::Question;
use sea_orm::{DatabaseConnection, DbErr, EntityTrait};
use serde::de::{MapAccess, Visitor};
use serde::ser::SerializeMap;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::Formatter;
use tracing::warn;
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
#[derive(Serialize, Deserialize)]
pub struct Info {
    pub title: String,
    pub document_title: String,
    pub description: String,
}

/// フォームの単一の項目
/// * `item_id`: アイテムのID
/// * `created_at`: 作成日時
/// * `updated_at`: 更新日時
/// * `title`: 回答者に表示される項目のタイトル
/// * `description`: 回答者に表示される項目の説明
/// * `item`: アイテムの種類とより細かいプロパティ
pub struct Item {
    pub item_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub title: String,
    pub description: String,
    pub item: Items,
}

/// アイテムの種類
#[derive(Serialize, Deserialize)]
pub enum Items {
    Question(ItemQuestion),
    PageBreak(ItemPageBreak),
    Text(ItemText),
}

/// 一つの質問を含む項目
/// * `question`: 表示される質問
#[derive(Serialize, Deserialize)]
pub struct ItemQuestion {
    question: Question,
}

/// 改ページ
#[derive(Serialize, Deserialize)]
pub struct ItemPageBreak {}

/// テキスト
#[derive(Serialize, Deserialize)]
pub struct ItemText {}

/// フォームのアクセス制限
/// * `roles`: アクセス可能なロール
#[derive(Serialize, Deserialize)]
pub struct AccessControl {
    pub roles: Vec<String>,
}

impl Serialize for Item {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(6))?;
        map.serialize_entry("item_id", &self.item_id)?;
        map.serialize_entry("created_at", &self.created_at)?;
        map.serialize_entry("updated_at", &self.updated_at)?;
        map.serialize_entry("title", &self.title)?;
        map.serialize_entry("description", &self.description)?;
        match &self.item {
            Items::Question(item) => {
                map.serialize_entry("item_question", &item)?;
            }
            Items::PageBreak(item) => {
                map.serialize_entry("item_page_break", &item)?;
            }
            Items::Text(item) => {
                map.serialize_entry("item_text", &item)?;
            }
        }
        map.end()
    }
}

impl<'de> Deserialize<'de> for Item {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(ItemVisitor)
    }
}

struct ItemVisitor;

impl<'de> Visitor<'de> for ItemVisitor {
    type Value = Item;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("Item")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut item_id = None;
        let mut created_at = None;
        let mut updated_at = None;
        let mut title = None;
        let mut description = None;
        let mut item = None;
        while let Some(key) = map.next_key()? {
            match key {
                "item_id" => {
                    if item_id.is_some() {
                        return Err(de::Error::duplicate_field("item_id"));
                    }
                    item_id = Some(map.next_value()?)
                }
                "created_at" => {
                    if created_at.is_some() {
                        return Err(de::Error::duplicate_field("created_at"));
                    }
                    created_at = Some(map.next_value()?)
                }
                "updated_at" => {
                    if updated_at.is_some() {
                        return Err(de::Error::duplicate_field("updated_at"));
                    }
                    updated_at = Some(map.next_value()?)
                }
                "title" => {
                    if title.is_some() {
                        return Err(de::Error::duplicate_field("title"));
                    }
                    title = Some(map.next_value()?)
                }
                "description" => {
                    if description.is_some() {
                        return Err(de::Error::duplicate_field("description"));
                    }
                    description = Some(map.next_value()?)
                }
                "item_question" => {
                    if item.is_some() {
                        return Err(de::Error::duplicate_field("item"));
                    }
                    item = Some(Items::Question(map.next_value()?));
                }
                "item_page_break" => {
                    if item.is_some() {
                        return Err(de::Error::duplicate_field("item"));
                    }
                    item = Some(Items::PageBreak(map.next_value()?));
                }
                "item_text" => {
                    if item.is_some() {
                        return Err(de::Error::duplicate_field("item"));
                    }
                    item = Some(Items::Text(map.next_value()?));
                }
                a => {
                    return Err(de::Error::unknown_field(
                        a,
                        &[
                            "item_id",
                            "created_at",
                            "updated_at",
                            "title",
                            "description",
                            "item_question",
                            "item_page_break",
                            "item_text",
                        ],
                    ))
                }
            }
        }
        let item_id = item_id.ok_or_else(|| de::Error::missing_field("item_id"))?;
        let created_at = created_at.ok_or_else(|| de::Error::missing_field("created_at"))?;
        let updated_at = updated_at.ok_or_else(|| de::Error::missing_field("updated_at"))?;
        let title = title.ok_or_else(|| de::Error::missing_field("title"))?;
        let description = description.ok_or_else(|| de::Error::missing_field("description"))?;
        let item = item.ok_or_else(|| de::Error::missing_field("item"))?;
        Ok(Item {
            item_id,
            created_at,
            updated_at,
            title,
            description,
            item,
        })
    }
}

#[derive(Debug)]
pub enum FormFromModelError {
    SerdeError(serde_json::error::Error),
    DbError(DbErr),
    NotFound,
}
impl Form {
    pub async fn from_model(
        db_conn: &DatabaseConnection,
        model: &crate::entities::forms::Model,
    ) -> Result<Self, FormFromModelError> {
        let form_id = model.form_id;
        let created_at = model.created_at.unwrap().into();
        let updated_at = model.updated_at.unwrap().into();
        let info =
            serde_json::from_value(model.info.clone()).map_err(FormFromModelError::SerdeError)?;
        let item_ids = &model.items as &Vec<Uuid>;
        let mut items = vec![];
        let access_control = AccessControl {
            roles: (&model.access_control_roles).clone(),
        };

        for item_id in item_ids {
            let item = crate::entities::prelude::FormItems::find_by_id(item_id.clone())
                .one(db_conn)
                .await;
            let item = match item {
                Ok(Some(item)) => item,
                Ok(None) => {
                    warn!("internal server error occurred while finding form item: no form item");
                    return Err(FormFromModelError::NotFound);
                }
                Err(err) => {
                    warn!(
                        "internal server error occurred while finding form item: {}",
                        err
                    );
                    return Err(FormFromModelError::DbError(err));
                }
            };
            items.push(Item::from_model(&item).map_err(FormFromModelError::SerdeError)?);
        }
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

impl Item {
    pub fn from_model(
        model: &crate::entities::form_items::Model,
    ) -> Result<Self, serde_json::error::Error> {
        let item_id = model.item_id;
        let created_at = model.created_at.unwrap().into();
        let updated_at = model.updated_at.unwrap().into();
        let title = model.title.clone();
        let description = model.description.clone();
        let item = serde_json::from_value::<Items>(model.item.clone())?;

        Ok(Self {
            item_id,
            created_at,
            updated_at,
            title,
            description,
            item,
        })
    }
}
