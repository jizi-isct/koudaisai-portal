use chrono::{DateTime, Utc};
use serde::de::{MapAccess, Visitor};
use serde::ser::SerializeMap;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use std::collections::HashMap;
use std::fmt::Formatter;
use std::str::FromStr;
use uuid::Uuid;

/// フォームの回答
/// * `response_id`: 回答のID
/// * `created_at`: 作成日時
/// * `updated_at`: 更新日時
/// * `form_id`: フォームのID
/// * `respondent_id`: 回答者のID
/// * `answers`: 質問に対する回答(item_idをキーとする)
#[derive(Serialize, Deserialize, Debug)]
pub struct FormResponse {
    pub response_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub form_id: Uuid,
    pub respondent_id: String,
    pub answers: HashMap<Uuid, Answer>,
}

/// 質問に対する回答
/// * `item_id`: 質問の回答
/// * `answer`: 回答の種類と詳細な情報
#[derive(Debug, Clone)]
pub struct Answer {
    pub item_id: Uuid,
    pub answer: Answers,
}

/// 回答の種類
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Answers {
    Text(AnswerText),
}

/// 質問に対する回答をテキストで表したもの
/// * `value`:  回答の値 \
///   質問の種類毎の回答の形式
///   * `Text`: ユーザーが入力したテキスト
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AnswerText {
    pub value: String,
}

impl Serialize for Answer {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(2))?;
        map.serialize_entry("item_id", &self.item_id)?;
        match &self.answer {
            Answers::Text(answer_text) => {
                map.serialize_entry("answer_text", &answer_text.value)?;
            }
        }
        map.end()
    }
}

impl<'de> Deserialize<'de> for Answer {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(AnswerVisitor)
    }
}

struct AnswerVisitor;

impl<'de> Visitor<'de> for AnswerVisitor {
    type Value = Answer;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("map")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut item_id = None;
        let mut answer = None;
        while let Some(key) = map.next_key()? {
            match key {
                "item_id" => {
                    if item_id.is_some() {
                        return Err(de::Error::duplicate_field("item_id"));
                    }
                    item_id = Some(map.next_value()?);
                }
                "answer_text" => {
                    if answer.is_some() {
                        return Err(de::Error::duplicate_field("answer"));
                    }
                    answer = Some(Answers::Text(map.next_value()?));
                }
                unknown => {
                    return Err(de::Error::unknown_field(
                        unknown,
                        &["item_id", "answer_text"],
                    ))
                }
            }
        }
        let item_id = item_id.ok_or_else(|| de::Error::missing_field("item_id"))?;
        let answer = answer.ok_or_else(|| de::Error::missing_field("answer"))?;
        Ok(Answer { item_id, answer })
    }
}

impl FormResponse {
    pub fn from_model(model: &crate::entities::form_responses::Model) -> anyhow::Result<Self> {
        let response_id = model.response_id;
        let created_at = model.created_at.unwrap().into();
        let updated_at = model.updated_at.unwrap().into();
        let form_id = model.form_id;
        let respondent_id = model.respondent_id.to_string();
        let answers1 = serde_json::from_value::<HashMap<String, Answer>>(model.answers.clone())?;
        let mut answers = HashMap::new();
        for (item_id, answer) in answers1 {
            answers.insert(Uuid::from_str(item_id.as_str())?, answer);
        }
        Ok(Self {
            response_id,
            created_at,
            updated_at,
            form_id,
            respondent_id,
            answers,
        })
    }
}
