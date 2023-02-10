//import the Filter trait from warp
use std::str::FromStr;
use std::io::{Error, ErrorKind};
use serde::Serialize;
use warp::{Filter, Reply, Rejection};

#[derive(Debug, Serialize)]
struct Question {
    id: QuestionId,
    title: String,
    content: String,
    tags: Option<Vec<String>>,
}
#[derive(Debug, Serialize)]
struct QuestionId(String);

impl Question {
    fn new(id: QuestionId, title: String, content: String, tags: Option<Vec<String>>) -> Self {
        Question {
            id,
            title,
            content,
            tags,
        }
    }
}

impl FromStr for QuestionId {
    type Err = std::io::Error;

    fn from_str(id: &str) -> Result<Self, Self::Err> {
        match id.is_empty() {
            false => Ok(QuestionId(id.to_string())),
            true => Err(Error::new(ErrorKind::InvalidInput, "No id provided")),
        }
    }
}


async fn get_questions() -> Result <impl warp::Reply, warp::Rejection> {
    let question = Question::new(
        QuestionId::from_str("1").expect("No id provided"),
        "First Question".to_string(),
        "Content of question".to_string(),
        Some(vec!("faq".to_string())),
    );

    Ok(warp::reply::json(&question
    ))

}

#[tokio::main]
async fn main() {
    // create a path Filter
    // let hello = warp::path("hello").map(|| format!("Hello, World!"));
    // let hello = warp::get()
    //     .map(|| format!("Hello, World!"));

    // // start the server and pass the route filter to it
    // warp::serve(hello)
    //     .run(([127, 0, 0, 1], 3030))
    //     .await;
    let get_items = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and_then(get_questions);

    let routes = get_items;

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;

}

/*
Every time an HTTP request comes in, the framework processes it in a few steps:

1 Check the request path inside the HTTP request.

2 Check the HTTP method (for example, GET, PUT, or POST).

3 Forward the request to a route handler that is responsible for the path and type.

4 Before forwarding the request to the route handler, the request can be passed
through middleware, which checks things like authentication headers or adds
further information to the request that is handed down to the route handler.
*/

/*
Create a mental or physical list of checkpoints you go through to see if and how your
framework is doing the following:
 How does it parse the incoming PATH and HTTP method?
 Can I parse JSON requests directly from the HTTP body?
 How can I parse uniform resource identifier (URI) parameters from the request?
 How can I add middleware such as authentication or logging?
 How do I pass objects like a database connection to the route handlers?
 How do I have to return an HTTP response?
 Does it have a built-in session or cookie handling?
*/

