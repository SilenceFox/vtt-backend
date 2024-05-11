use super::*;
use axum::{
    extract::State,
    http::{HeaderMap, HeaderName, StatusCode},
    response::IntoResponse,
    Json,
};
use serde_json::{json, Value};

#[derive(Debug, Serialize, Deserialize)]
pub enum Errors {
    InvalidUsername,
    InvalidRequest,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SendMessageRequest {
    message: String,
    username: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    message: String,
    error: Option<Errors>,
}

// Join the chat with axum
pub async fn join(State(chat): State<Arc<Mutex<Chat>>>, Json(req): Json<String>) -> Json<Response> {
    // Check username for invalid characters
    let user = req.trim();
    join_helper(user, &chat);

    let response = Response {
        message: String::from("You have joined"),
        error: None,
    };
    let json = Json(response);
    json
}

pub async fn send_message(
    chat: State<Arc<Mutex<Chat>>>,
    Json(req): Json<SendMessageRequest>,
) -> Json<Response> {
    // Destructure the data from the request
    let msg = req.message;
    let usr = req.username;
    // println!("{}: {}", usr, msg); // Debugging

    // Get MutexGuard for Chat
    let mut guard_chat = chat.lock().unwrap();

    // If user does not exists, add the new user
    if !guard_chat.check_user_exists(&usr) {
        info!(
            "Fallback activated, something is not right. User: {} not found",
            &usr
        );
        // HACK: The reason behind this is a fallback to avoid unwrap, ideally there would be no way
        // to a non existent user to send a message, thats why its a hack.
        guard_chat.user_join(&User::new_user(&usr));
    }

    // Gets an existing user from `Chat` and sends a message in his name
    let my_usr = guard_chat.get_your_user(&usr).unwrap().clone();
    guard_chat.send_msg(&my_usr, &msg);
    guard_chat.get_last_message(); // NOTE: Mostly debug until stabilized

    let response = Response {
        message: String::from(format!("{} sent a message", usr)),
        error: None,
    };
    Json(response)
}

pub async fn leave(
    State(chat): State<Arc<Mutex<Chat>>>,
    Json(req): Json<String>,
) -> Json<Response> {
    // Get the lock and parse the user
    let mut guard_chat = chat.lock().unwrap();
    let user = req.trim();
    let user_arc = guard_chat.get_your_user(&user).cloned();
    // now we validate if this user exists
    if let Some(user_arc) = user_arc {
        guard_chat.user_leave(&user_arc);
        Json(Response {
            message: String::from(format!("User: {} left", user)),
            error: None,
        })
    } else {
        error!("Non-existing user tried to leave the chat");
        Json(Response {
            message: String::from(format!("ERROR: User {} was not found", user)),
            error: Some(Errors::InvalidUsername),
        })
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct RollRequest {
    //later this can have a optional related skill
    // skill / Modifier: Option<String, i32>,
    title: Option<String>,
    description: Option<String>,
    dice: Option<String>,
    times: Option<i32>,
    range: Option<i32>,
    username: Option<String>,
}
pub async fn chat_roll(
    State(chat): State<Arc<Mutex<Chat>>>,
    Json(request): Json<RollRequest>,
) -> impl IntoResponse {
    // Deconstruct the request into usable variables
    let dice_kind = request
        .dice
        .map(|dice| {
            match dice.to_lowercase().trim() {
                "fate" => crate::dice::DiceKind::Fate,
                "faced" => crate::dice::DiceKind::Faced,
                _ => unimplemented!(), // TODO: Use a default
            }
        })
        .ok_or_else(|| error!("Failed to get dice kind"))
        .unwrap();

    let range = request.range.or(Some(20));
    let times = request.times.or(Some(1)); // should be per dice
    let title = request.title.or(Some("Roll".to_string()));
    let description = request.description.or(None);
    let user = request.username.or(Some("Anonymous".to_string()));
    let result = match dice_kind {
        crate::dice::DiceKind::Fate => {
            json!(crate::dice::Roll::new().roll(dice_kind, None, None))
        }
        crate::dice::DiceKind::Faced => {
            json!(crate::dice::Roll::new().roll(dice_kind, range, times))
        }
    };
    // This will assume you already have a existing user, will crash otherwise
    let user_arc = chat
        .lock()
        .unwrap()
        .get_your_user(&user.as_ref().unwrap())
        .unwrap()
        .clone();
    let output = format!("User: {} rolled: {}", &user.unwrap(), &result.to_string());
    chat.lock().unwrap().send_msg(&user_arc, &output);
    chat.lock().unwrap().get_last_message();
    Json(output)
}

pub async fn get_chat(
    State(chat): State<Arc<Mutex<Chat>>>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let guard_chat = chat.lock().unwrap();
    let history = guard_chat.get_history().clone();
    if history.is_empty() {
        error!("GET on chat resulted on error, chat is empty");
        Err((StatusCode::NOT_FOUND, Json("History is empty".to_string())))
    } else {
        info!("Chat request parsed successfully.");
        Ok((StatusCode::OK, Json(history)))
    }
}
