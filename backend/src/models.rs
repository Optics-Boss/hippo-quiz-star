pub mod models {
    use serde::{Deserialize, Serialize};

    #[derive(Deserialize, Serialize)]
    pub struct Quiz {
       pub title: String,
    }

    pub fn build_quiz(title: String) -> Quiz {
        Quiz {
            title
        }
    }

    #[derive(Deserialize, Serialize)]
    pub struct Questions {
        quiz_title: String,
        statement: String,
        right_answer: String,
        wrong_answer_1: String,
        wrong_answer_2: String,
        wrong_answer_3: String,
    }
}
