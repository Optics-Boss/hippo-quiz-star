pub mod models {
    use serde::{Deserialize, Serialize};

    #[derive(Deserialize, Serialize, Debug)]
    pub struct Quiz {
       pub title: String,
    }

    pub fn build_quiz(title: String) -> Quiz {
        Quiz {
            title
        }
    }

    #[derive(Deserialize, Serialize, Debug)]
    pub struct Question {
        image: String,
        statement: String,
        right_answer: String,
        wrong_answer_1: String,
        wrong_answer_2: String,
        wrong_answer_3: String,
    }

    pub fn build_question (
          image: String,
          statement: String,
          right_answer: String,
          wrong_answer_1: String,
          wrong_answer_2: String,
          wrong_answer_3: String,
        ) -> Question {
        Question {
          image,
          statement,
          right_answer,
          wrong_answer_1,
          wrong_answer_2,
          wrong_answer_3,
        }
    }
}
