use chrono::{DateTime, Utc};
use std::iter::Map;
use uuid::Uuid;

/// フォームの回答
/// * `response_id`: 回答のID
/// * `created_at`: 作成日時
/// * `updated_at`: 更新日時
/// * `form_id`: フォームのID
/// * `respondent_id`: 回答者のID
/// * `answers`: 質問に対する回答(question_idをキーとする)
pub struct FormResponse {
    pub response_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub form_id: Uuid,
    pub respondent_id: String,
    pub answers: Map<String, Answer>,
}

/// 質問に対する回答
/// * `question_id`: 質問の回答
/// * `answer`: 回答の種類と詳細な情報
pub struct Answer {
    pub question_id: Uuid,
    pub answer: Answers,
}

/// 回答の種類
pub enum Answers {
    Text(AnswerText),
}

/// 質問に対する回答をテキストで表したもの
/// * `value`:  回答の値 \
///   質問の種類毎の回答の形式
///   * `Text`: ユーザーが入力したテキスト
pub struct AnswerText {
    pub value: String,
}
