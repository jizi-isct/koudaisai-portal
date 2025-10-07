use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
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
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Answer {
    pub item_id: Uuid,
    #[serde(flatten)]
    pub answer: Answers,
}

/// 回答の種類
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Answers {
    #[serde(rename = "answer_text")]
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
