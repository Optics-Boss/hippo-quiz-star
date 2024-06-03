#![deny(warnings)]
use serde::{Deserialize, Serialize};
use warp::Filter;
use std::convert::Infallible;

#[derive(Deserialize, Serialize)]
pub struct Quiz {
    id: u64,
    title: String,
}

#[tokio::main]
async fn main() {
    // Match any request and return hello world!
    let welcome = warp::path::end().map(|| "Welcome to Hippo Quiz Master");

    let quizes = warp::path!("quizes")
        .and(warp::get())
        .and_then(list_quizes);


    let quiz = warp::path("quiz")
        .and(warp::path::param())
        .map(|quiz: String| format!("Quiz number:{}!", quiz));


   let routes = warp::get().and(
        welcome
        .or(quizes)
        .or(quiz),
    );

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

pub async fn list_quizes() -> Result<impl warp::Reply, Infallible> {
    let quiz = Quiz {
        id: 1,
        title: "Test".to_string()
    };

    let quiz2 = Quiz {
        id: 2,
        title: "This is Quiz 2".to_string()
    };

   Ok(warp::reply::json(&[quiz, quiz2]))
}
