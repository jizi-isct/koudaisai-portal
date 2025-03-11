use serde::de::{MapAccess, Visitor};
use serde::ser::SerializeMap;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::Formatter;

/// フォームの質問
/// * `required` - 回答必須かどうか
/// * `question` - 質問の種類とより詳細なプロパティ
#[derive(Debug)]
pub struct Question {
    pub required: bool,
    pub question: Questions,
}

/// 質問の種類
#[derive(Debug)]
pub enum Questions {
    Text(QuestionText),
}

/// テキスト
/// * `paragraph` - trueの場合複数行にわたるテキスト。falseの場合一行の回答。
#[derive(Serialize, Deserialize, Debug)]
pub struct QuestionText {
    pub paragraph: bool,
}

impl Serialize for Question {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(2))?;
        map.serialize_entry("required", &self.required)?;
        match &self.question {
            Questions::Text(question_text) => {
                map.serialize_entry("question_text", &question_text)?;
            }
        }
        map.end()
    }
}

impl<'de> Deserialize<'de> for Question {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(QuestionVisitor)
    }
}

struct QuestionVisitor;

impl<'de> Visitor<'de> for QuestionVisitor {
    type Value = Question;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("map")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut required = None;
        let mut question = None;
        while let Some(key) = map.next_key::<String>()? {
            match key.as_str() {
                "required" => {
                    if required.is_some() {
                        return Err(de::Error::duplicate_field("required"));
                    }
                    required = Some(map.next_value()?);
                }
                "question_text" => {
                    if question.is_some() {
                        return Err(de::Error::duplicate_field("question"));
                    }
                    question = Some(Questions::Text(map.next_value()?));
                }
                unknown => {
                    return Err(de::Error::unknown_field(
                        unknown,
                        &["required", "question_text"],
                    ))
                }
            }
        }
        let required = required.ok_or_else(|| de::Error::missing_field("required"))?;
        let question = question.ok_or_else(|| de::Error::missing_field("question"))?;
        Ok(Question { required, question })
    }
}
