//! Run with
//!
//! ```not_rust
//! cargo run --example actix --features="full"
//!
//! curl -X POST -H "Content-Type: application/json" -d '{"username":""}' '127.0.0.1:3000'
//! -> Input validation error: [[username], msg:[username is required,]]
//!
//! curl -X POST -H "Content-Type: application/json" -d '{"username":"foo"}' '127.0.0.1:3000'
//! -> Welcome, foo!
//! ```

use actix_web::{
    body::BoxBody, http::StatusCode, web, App, HttpResponse, HttpServer, ResponseError, Result,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use valitron::{
    available::Required,
    register::{Validatable, ValidatorError},
    Validator,
};

#[derive(Deserialize, Serialize)]
struct Info {
    username: String,
}

/// extract `Info` using serde
async fn index(info: web::Json<Info>) -> Result<String, ServerError> {
    info.validate(
        Validator::new()
            .rule("username", Required)
            .message([("username.required", "username is required")]),
    )?;
    Ok(format!("Welcome {}!", info.username))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("listening on 127.0.0.1:3000");
    HttpServer::new(|| App::new().route("/", web::post().to(index)))
        .bind(("127.0.0.1", 3000))?
        .run()
        .await
}

#[derive(Debug, Error)]
pub enum ServerError {
    #[error(transparent)]
    ValidationError(#[from] ValidatorError),
    //
    // other ...
}

impl ResponseError for ServerError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        StatusCode::FORBIDDEN
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
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
                HttpResponse::with_body(self.status_code(), message).map_into_boxed_body()
            }
        }
    }
}
