use crate::AppState;

use super::*;
use anyhow::anyhow;
use axum::{
    extract::{Json, State},
    http::{HeaderMap, StatusCode},
};
use rusqlite::params;
use serde_json::Value;
use vtt_baxum::errors::*;

/// The handler for exporting the entire character sheet to JSON.
/// This has zero security, anyone can export your sheet.
pub async fn export_sheet(user: HeaderMap) -> Result<impl IR, Rejection> {
    if let Some(validated) = get_headers(&user, "user") {
        let owner = super::super::chat::User::new_user("Teste");
        let sheet = super::Sheet {
            name: Arc::from("Test"),
            owner,
            fatepoints: (3, 3),
        };
        Ok(Json(sheet))
    } else {
        error!("Failed to validate user");
        Err((
            StatusCode::NON_AUTHORITATIVE_INFORMATION,
            Json("You are not authorized to do that".to_string()),
        ))
    }
}

pub async fn import_sheet(Json(payload): Json<Sheet>) -> Result<impl IR, Rejection> {
    // Here we handle importing a sheet from a request
    // We presume that its coming from a previous export
    if payload.owner.get_username() == "Joao" {
        println!("User is: Joao");
        Ok(Json(payload.fatepoints))
    } else {
        println!("Woo");
        Err((
            StatusCode::NON_AUTHORITATIVE_INFORMATION,
            Json("Missing credentials".to_string()),
        ))
    }
}

pub async fn save_sheet_on_server(
    State(state): State<Arc<AppState>>,
    Json(sheet): Json<super::Sheet>,
) -> Result<impl IR, Rejection> {
    if sheet.owner.get_username().is_empty() {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json("Ownership rejected.".to_string()),
        ));
    }

    // Serialize sheet data to JSON
    let sheet_data = serde_json::to_string(&sheet).unwrap();
    let username = &sheet.owner.get_username();

    // Insert the data
    {
        let db = state.db.lock().await;
        db.execute(
            "INSERT INTO sheets (owner, sheet_data) VALUES (?1, ?2)",
            params![username, sheet_data],
        )
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(format!("Failed to save sheet: {}", e)),
            )
        })?;
    }

    Ok(Json("Successfully saved sheet").to_string())
}

// async fn db_get_sheet(State(db): State<Arc<AppState>>, owner: &str) -> Option<String> {
//     let db = db.db.lock().await;
//
//     // Prepare and execute a SELECT statement to retrieve the sheet_data for the given owner
//     let mut stmt = db
//         .prepare("SELECT sheet_data FROM sheets WHERE owner = ?1")
//         .unwrap();
//     let mut rows = stmt.query(params![owner]).unwrap();
//
//     // If a row is found, return the sheet_data as a String
//     if let Some(row) = rows.next().unwrap() {
//         let sheet_data: String = row.get(0).unwrap();
//         Some(sheet_data)
//     } else {
//         // If no row is found, return None
//         None
//     }
// }
fn db_remove_sheet() {
    todo!()
}

/// Will just crash if no headers match
fn get_headers(headers: &HeaderMap, header: &str) -> Option<String> {
    headers
        .get(header)
        .and_then(|value| value.to_str().ok())
        .map(|s| s.to_string())
}
