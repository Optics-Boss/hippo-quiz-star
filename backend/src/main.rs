#![deny(warnings)]
use warp::Filter;
use std::convert::Infallible;
use std::fs;

mod models;
use crate::models::models::Quiz;
// use crate::models::models::Question;
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

    let cors = warp::cors()
    .allow_any_origin();

   let routes = warp::get().and(
        welcome
        .or(quizes)
        .or(quiz),
    ).with(cors);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

pub async fn list_quizes() -> Result<impl warp::Reply, Infallible> {
    let paths = fs::read_dir("./src/quizes").unwrap();
    let mut quiz : Quiz = build_quiz("test".to_string()); 

    for path in paths {
       quiz = build_quiz(path.unwrap()
                 .path()
                 .display()
                 .to_string()
                 .strip_prefix("./src/quizes\\")
                 .unwrap()
                 .to_string()
                 );
    }

    Ok(warp::reply::json(&[quiz]))
}

pub async fn get_quizes(quiz: String) -> Result<impl warp::Reply, Infallible> {
    let path : String = "./src/quizes/".to_owned() + &quiz;
    let file_contents = fs::read_to_string(path).expect("Can't find file");

    let parts = file_contents.split("<question>");
    for part in parts {
        println!("{}", part)
    }

    let quiz : Quiz = build_quiz("test".to_string()); 
    build_question("test".to_string(), "test".to_string(), "test".to_string(), "test".to_string(), "test".to_string());

    Ok(warp::reply::json(&[quiz]))
}
