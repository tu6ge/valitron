//! Run with
//!
//! ```not_rust
//! cargo run --example axum --features="full"
//!
//! curl '127.0.0.1:3000?title='
//! -> Input validation error: [[title], msg:[title is required,title should be starts with `hi`,]]
//!
//! curl '127.0.0.1:3000?title=hihihi'
//! -> <h1>Hello, hihihi!</h1>
//! ```

use std::net::SocketAddr;

use axum::{
    extract::{rejection::FormRejection, Form},
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use valitron::{
    available::{Required, StartWith},
    register::ValidatorError,
    RuleExt, Validatable, Validator,
};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(handler));

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BlogInput {
    pub title: String,
}

async fn handler(Form(input): Form<BlogInput>) -> Result<Html<String>, ServerError> {
    input.validate(
        Validator::new()
            .rule("title", Required.and(StartWith("hi")))
            .map(String::from)
            .message([
                ("title.required", "title is required"),
                ("title.start_with", "title should be starts with `hi`"),
            ]),
    )?;

    Ok(Html(format!("<h1>Hello, {}!</h1>", input.title)))
}

#[derive(Debug, Error)]
pub enum ServerError {
    #[error(transparent)]
    ValidationError(#[from] ValidatorError<String>),

    #[error(transparent)]
    AxumFormRejection(#[from] FormRejection),
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        match self {
            ServerError::ValidationError(msg) => {
                let mut result = String::new();
                for (name, msg_vec) in msg.iter() {
                    result.push_str(&format!("[{}]", name.as_str()));
                    result.push_str(", msg:[");

                    for msg in msg_vec.iter() {
                        result.push_str(msg.as_str());
                        result.push(',');
                    }
                    result.push(']');
                }
                let message = format!("Input validation error: [{}]", result);
                (StatusCode::BAD_REQUEST, message)
            }
            ServerError::AxumFormRejection(_) => (StatusCode::BAD_REQUEST, self.to_string()),
        }
        .into_response()
    }
}
