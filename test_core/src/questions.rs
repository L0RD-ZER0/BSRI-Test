use super::answers::AnswerDomain;
use crate::json::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum QuestionCategory {
    Masculinity,
    Femininity,
    Filler,
}

#[derive(Serialize, Deserialize, Debug)]
/// A struct representing a question in memory.
///
/// # Fields
///
/// * `statement` - The statement of the question.
/// * `category` - The category of the question.
/// * `answer_type` - The answer type of the question.
///
/// # Examples
///
/// ```
/// use test_core::questions::Question;
///
/// let q = Question::new(
///    "statement".to_string(),
///   test_core::questions::QuestionCategory::Masculinity,
///   test_core::answers::AnswerDomain::Range(1, 2),
/// );
///
/// println!("Debug :: {:?}", q);
/// // returns Debug :: Question { statement: "statement", category: Masculinity, answer_type: Range(1, 2) }
///
/// ```
///
/// ```
/// use test_core::questions::Question;
///
/// let q = Question::new(
///   "statement".to_string(),
///   test_core::questions::QuestionCategory::Masculinity,
///   test_core::answers::AnswerDomain::Range(1, 2),
/// );
///
/// println!("Serialized :: {}", q.to_json());
/// // returns Serialized :: {
/// //   "statement": "statement",
/// //   "category": "masculinity",
/// //   "answer_type": { "type": "range", "value": [1, 2] }
/// // }
///
/// ```
///
/// ```
/// use test_core::questions::Question;
///
/// let q = Question::from_json(r#"{
///    "statement": "statement",
///    "category": "masculinity",
///    "answer_type": { "type": "range", "value": [1, 2] }
/// }"#);
///
/// println!("Deserialized :: {:?}", q);
/// // returns Debug :: Question { statement: "statement", category: Masculinity, answer_type: Range(1, 2) }
/// ```
pub struct Question {
    pub statement: String,
    pub category: QuestionCategory,
    pub answer_type: AnswerDomain,
}

impl Question {
    pub fn new(statement: String, category: QuestionCategory, answer_type: AnswerDomain) -> Self {
        Self {
            statement,
            category,
            answer_type,
        }
    }

    pub fn from_json(json: &str) -> Self {
        json::from_str(json).unwrap()
    }

    pub fn from_json_slice(json: &[u8]) -> Self {
        json::from_slice(json).unwrap()
    }

    pub fn from_json_value(json: json::Value) -> Self {
        json::from_value(json).unwrap()
    }

    pub fn to_json(&self) -> String {
        json::to_str(self).unwrap()
    }
}

#[cfg(test)]
mod tests {

    use super::Question;
    use crate::answers::AnswerDomain;
    use crate::questions::QuestionCategory;

    #[test]
    fn constructor() {
        let q = Question::new(
            "statement".to_string(),
            QuestionCategory::Masculinity,
            AnswerDomain::Range(1, 2),
        );

        assert_eq!(q.statement, "statement");
        assert!(matches!(q.category, QuestionCategory::Masculinity));
        assert!(matches!(q.answer_type, AnswerDomain::Range(1, 2)));
    }

    #[test]
    fn serialization() {
        let q = Question::new(
            "statement".to_string(),
            QuestionCategory::Masculinity,
            AnswerDomain::Range(1, 2),
        );

        let json = q.to_json();

        let serialized = concat!(
            "{",
            "\n  \"statement\": \"statement\",",
            "\n  \"category\": \"masculinity\",",
            "\n  \"answer_type\": {",
            "\n    \"type\": \"range\",",
            "\n    \"value\": [",
            "\n      1,",
            "\n      2",
            "\n    ]",
            "\n  }",
            "\n}"
        )
        .to_owned();

        // Isn't necessary as asserts aren't compiled in release mode.
        // It is here for reference.
        //
        // #[cfg(not(debug_assertions))]
        // let serialized = concat!("{",
        //         "\"statement\":\"statement\",",
        //         "\"category\":\"masculinity\",",
        //         "\"answer_type\":{",
        //             "\"type\":\"range\",",
        //             "\"value\":[",
        //                 "1,",
        //                 "2",
        //             "]",
        //         "}",
        //     "}").to_owned();

        assert_eq!(json, serialized);
    }

    #[test]
    fn deserialization() {
        let q = Question::from_json(
            r#"{
            "statement": "statement",
            "category": "masculinity",
            "answer_type": { "type": "range", "value": [1, 2] }
        }"#,
        );

        assert_eq!(q.statement, "statement");
        assert!(matches!(q.category, QuestionCategory::Masculinity));
        assert!(matches!(q.answer_type, AnswerDomain::Range(1, 2)));
    }
}
