use chrono::{DateTime, Utc};
use serde::de::{MapAccess, Visitor};
use serde::ser::SerializeMap;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use std::collections::HashMap;
use std::fmt::Formatter;
use uuid::Uuid;

/// フォームの回答
/// * `response_id`: 回答のID
/// * `created_at`: 作成日時
/// * `updated_at`: 更新日時
/// * `form_id`: フォームのID
/// * `respondent_id`: 回答者のID
/// * `answers`: 質問に対する回答(question_idをキーとする)
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
/// * `question_id`: 質問の回答
/// * `answer`: 回答の種類と詳細な情報
#[derive(Debug, Clone)]
pub struct Answer {
    pub question_id: Uuid,
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
        map.serialize_entry("question_id", &self.question_id)?;
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
        let mut question_id = None;
        let mut answer = None;
        while let Some(key) = map.next_key()? {
            match key {
                "question_id" => {
                    if question_id.is_some() {
                        return Err(de::Error::duplicate_field("question_id"));
                    }
                    question_id = Some(map.next_value()?);
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
                        &["question_id", "answer_text"],
                    ))
                }
            }
        }
        let question_id = question_id.ok_or_else(|| de::Error::missing_field("question_id"))?;
        let answer = answer.ok_or_else(|| de::Error::missing_field("answer"))?;
        Ok(Answer {
            question_id,
            answer,
        })
    }
}

impl FormResponse {
    pub fn from_model(
        model: &crate::entities::form_responses::Model,
    ) -> Result<Self, serde_json::error::Error> {
        let response_id = model.response_id;
        let created_at = model.created_at.unwrap().into();
        let updated_at = model.updated_at.unwrap().into();
        let form_id = model.form_id;
        let respondent_id = model.respondent_id.to_string();
        let answers1 = serde_json::from_value::<Vec<Answer>>(model.answers.clone())?;
        let mut answers = HashMap::new();
        for answer in answers1 {
            answers.insert(answer.question_id, answer);
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
