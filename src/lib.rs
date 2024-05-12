pub mod errors {
    pub use anyhow::{Context, Result};
    pub use axum::{http::StatusCode, response::IntoResponse as IR, Json};
    pub type Rejection = (StatusCode, Json<String>);
}
