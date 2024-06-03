#![deny(warnings)]
use warp::Filter;

#[tokio::main]
async fn main() {
    // Match any request and return hello world!
    let welcome = warp::path::end().map(|| "Welcome to Hippo Quiz Master");

    let quizes = warp::path("quizes").map(|| "Here are quizes");
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
