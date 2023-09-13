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

use async_trait::async_trait;
use axum::{
    extract::{rejection::FormRejection, Form, FromRequest},
    http::{Request, StatusCode},
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use thiserror::Error;
use valitron::{
    available::{Required, StartWith, Trim},
    RuleExt, Validator,
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

async fn handler(ValidatedForm(input): ValidatedForm<BlogInput>) -> Html<String> {
    Html(format!("<h1>Hello, {}!</h1>", input.title))
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedForm<T>(pub T);

#[async_trait]
impl<T, S, B> FromRequest<S, B> for ValidatedForm<T>
where
    T: Serialize + DeserializeOwned,
    S: Send + Sync,
    Form<T>: FromRequest<S, B, Rejection = FormRejection>,
    B: Send + 'static,
{
    type Rejection = ServerError;

    async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
        let Form(value) = Form::<T>::from_request(req, state).await?;
        let validate = Validator::new()
            .rule("title", Trim.and(Required).and(StartWith("hi")))
            .message([
                ("title.required", "title is required"),
                ("title.start_with", "title should be starts with `hi`"),
            ]);

        validate.validate(&value)?;

        Ok(ValidatedForm(value))
    }
}

#[derive(Debug, Error)]
pub enum ServerError {
    #[error(transparent)]
    ValidationError(#[from] valitron::register::ValidatorError),

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
