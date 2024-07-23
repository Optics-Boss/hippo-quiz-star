#![deny(warnings)]
use warp::Filter;
use std::convert::Infallible;
use std::fs;

mod models;
use crate::models::models::build_quiz;
use crate::models::models::build_question;

#[tokio::main]
async fn main() {
    let welcome = warp::path::end().map(|| "Welcome to Hippo Quiz Master");

    let quizes = warp::path!("quizes")
        .and(warp::get())
        .and_then(list_quizes);

    let quiz = warp::path("quiz")
        .and(warp::path::param())
        .and_then(|quiz: String| get_quizes(quiz));

    let static_files = warp::path("static")
        .and(warp::fs::dir("www/static"));

    let cors = warp::cors()
    .allow_any_origin();

   let routes = warp::get().and(
        welcome
        .or(quizes)
        .or(quiz)
        .or(static_files),
    ).with(cors);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

pub async fn list_quizes() -> Result<impl warp::Reply, Infallible> {
    let paths = fs::read_dir("./src/quizes").unwrap();
    let mut quizes = Vec::new();

    for path in paths {
       quizes.push(build_quiz(path.unwrap()
                 .path()
                 .display()
                 .to_string()
                 .strip_prefix("./src/quizes\\")
                 .unwrap()
                 .to_string()
                 ));
    }

    Ok(warp::reply::json(&[quizes]))
}

pub async fn get_image(image: String) -> String {
    format!("./quizes/images/{}", image)
}

pub async fn get_quizes(quiz: String) -> Result<impl warp::Reply, Infallible> {
    let path : String = "./src/quizes/".to_owned() + &quiz;
    let file_contents = fs::read_to_string(path).expect("Can't find file");
    let parts: Vec<&str> = file_contents.split("<question>").filter(|&question| !question.is_empty()).collect();

    let mut questions = Vec::new();
    for part in parts {
        if !part.replace("\r\n", "").is_empty() {
            let image = &part[part.find("<image>").unwrap_or(0)..part.find("</image>").unwrap_or(0)].strip_prefix("<image>").unwrap_or("");
            let statement = &part[part.find("<statement>").unwrap_or(0)..part.find("</statement>").unwrap_or(0)].strip_prefix("<statement>").unwrap_or("");
            let wrong_answer_1 = &part[part.find("<wrong_answer_1>").unwrap_or(0)..part.find("</wrong_answer_1>").unwrap_or(0)].strip_prefix("<wrong_answer_1>").unwrap_or("");
            let wrong_answer_2 = &part[part.find("<wrong_answer_2>").unwrap_or(0)..part.find("</wrong_answer_2>").unwrap_or(0)].strip_prefix("<wrong_answer_2>").unwrap_or("");
            let wrong_answer_3 = &part[part.find("<wrong_answer_3>").unwrap_or(0)..part.find("</wrong_answer_3>").unwrap_or(0)].strip_prefix("<wrong_answer_3>").unwrap_or("");
            let right_answer = &part[part.find("<right_answer>").unwrap_or(0)..part.find("</right_answer>").unwrap_or(0)].strip_prefix("<right_answer>").unwrap_or("");

            questions.push(
                build_question(
                    image.to_string(),
                    statement.to_string(),
                    wrong_answer_1.to_string(),
                    wrong_answer_2.to_string(),
                    wrong_answer_3.to_string(),
                    right_answer.to_string()
                )
            );
        }
    }

    println!("{:?}", questions);

    Ok(warp::reply::json(&[questions]))
}
