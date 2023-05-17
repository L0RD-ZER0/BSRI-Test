use crate::json::{
    Serialize,
    Deserialize,
};


#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", content = "value", rename_all = "snake_case")]
pub enum AnswerDomain {
    Range(i32, i32),
    MultiRange(i32, i32, i32),
    Choice(Vec<String>),
    MultiChoice(Vec<String>),
    YesNo,
    Any,
    None,
}


#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", content = "value", rename_all = "snake_case")]
pub enum Answer {
    Range(i32),
    MultiRange(i32, i32),
    Choice(String),
    MultiChoice(Vec<String>),
    YesNo(bool),
    Any(String),
    None,
}
