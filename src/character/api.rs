use axum::{
    extract::{FromRequest, Json},
    http::{HeaderMap, StatusCode},
    response::IntoResponse as IR,
};
type Re<T, E> = Result<T, E>;

use super::*;

/// The handler for exporting the entire character sheet to JSON.
/// This has zero security, anyone can export your sheet.
pub async fn export_sheet(user: HeaderMap) -> Re<impl IR, impl IR> {
    if let Some(validated) = get_headers(&user, "user") {
        let owner = super::super::chat::User::new_user("Teste");
        let sheet = super::Sheet {
            name: Arc::from("Test"),
            owner,
            fatepoints: (3, 3),
        };
        Ok((StatusCode::OK, Json(sheet)))
    } else {
        error!("Failed to validate user");
        Err((
            StatusCode::NON_AUTHORITATIVE_INFORMATION,
            Json("You are not authorized to do that"),
        ))
    }
}

pub async fn import_sheet(Json(payload): Json<Sheet>) -> Re<impl IR, impl IR> {
    // Here we handle importing a sheet from a request
    // We presume that its coming from a previous export
    if payload.owner.get_username() == "Joao" {
        println!("User is: Joao");

        Ok((StatusCode::OK, Json(payload.fatepoints)))
    } else {
        println!("Woo");
        Err((
            StatusCode::NON_AUTHORITATIVE_INFORMATION,
            Json("Missing credentials".to_string()),
        ))
    }
}

/// Will just crash if no headers match
fn get_headers(headers: &HeaderMap, header: &str) -> Option<String> {
    headers
        .get(header)
        .and_then(|value| value.to_str().ok())
        .map(|s| s.to_string())
}
