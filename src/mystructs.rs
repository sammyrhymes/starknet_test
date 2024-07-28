use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum QuestionType {
    Oe,
    Sc,
    Mc,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuestionOption {
    pub answer: String,
    pub is_correct: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Question {
    pub question: String,
    pub options: Vec<QuestionOption>,
    #[serde(rename = "question_type")]
    pub question_type: QuestionType,
    pub points: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Answer {
    pub question_type: QuestionType,
    pub answer: Option<String>,
    pub options: Option<Vec<u8>>,
}
