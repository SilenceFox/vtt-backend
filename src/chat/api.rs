use super::*;
use axum::{extract::State, http::StatusCode, Json};
use serde_json::json;
use vtt_baxum::errors::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct SendMessageRequest {
    message: String,
    username: String,
}

// Join the chat with axum
pub async fn join(State(chat): State<Arc<Mutex<Chat>>>, Json(req): Json<String>) -> impl IR {
    // Check username for invalid characters
    let user = req.trim();
    join_helper(user, &chat);
    Json("You have joined")
}

pub async fn send_message(
    chat: State<Arc<Mutex<Chat>>>,
    Json(req): Json<SendMessageRequest>,
) -> impl IR {
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

    Json(format!("{} sent a message", usr))
}

pub async fn leave(State(chat): State<Arc<Mutex<Chat>>>, Json(req): Json<String>) -> impl IR {
    // Get the lock and parse the user
    let mut guard_chat = chat.lock().unwrap();
    let user = req.trim();
    let user_arc = guard_chat.get_your_user(user).cloned();
    // now we validate if this user exists
    if let Some(user_arc) = user_arc {
        guard_chat.user_leave(&user_arc);
        Ok(Json(format!("User: {} left", user)))
    } else {
        error!("Non-existing user tried to leave the chat");

        Err((
            StatusCode::NOT_FOUND,
            Json(format!("ERROR: User {} was not found", user)),
        ))
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct RollRequest {
    // NOTE: later this can have a optional related skill
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
) -> impl IR {
    // Deconstruct the request into usable variables
    let dice_kind = get_dice(&request).unwrap();
    let mut guard_chat = chat.lock().unwrap();

    let range = request.range.or(Some(20));
    let times = request.times.or(Some(1)); // should be per dice
    let title = request.title.or(Some("Roll".to_string()));
    let description = request.description.or(None);
    let user = request.username.or(Some("Anonymous".to_string()));
    let result = if dice_kind == crate::dice::DiceKind::Fate {
        json!(crate::dice::Roll::new().roll(dice_kind, None, None))
    } else {
        // HACK: Dice should only be Fate or Faced, anything else is a bug.
        json!(crate::dice::Roll::new().roll(dice_kind, range, times))
    };

    // NOTE: This will assume you already have a existing user, will crash otherwise
    let user_arc = get_user_arc(&chat, &user.as_ref().unwrap());
    let output = format!("User: {} rolled: {}", &user.unwrap(), &result.to_string());
    guard_chat.send_msg(&user_arc, &output);
    guard_chat.get_last_message(); // NOTE: Mostly debug until stabilized
    Json(output)
}

/// If this function fails, the user will be defaulted to fate
fn get_user_arc(chat: &Arc<Mutex<Chat>>, username: &str) -> Arc<User> {
    chat.lock()
        .unwrap()
        .get_your_user(username)
        .unwrap()
        .clone()
}

/// Tries to get the DiceKind, if it fails, default to fate unless the request is invalid
fn get_dice(cx: &RollRequest) -> Result<crate::dice::DiceKind, String> {
    if let Some(dice_str) = &cx.dice {
        match dice_str.to_lowercase().trim() {
            "fate" => Ok(crate::dice::DiceKind::Fate),
            "faced" => Ok(crate::dice::DiceKind::Faced),
            _ => {
                error!("couldnt match the dice type, defaulting to fate");
                Err(format!("Provided {}, which is invalid", dice_str))
            }
        }
    } else {
        error!("Dice type not provided, defaulting to fate");
        Ok(crate::dice::DiceKind::Fate)
    }
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Messages {
    messages: Vec<Message>,
}

pub async fn get_chat(State(chat): State<Arc<Mutex<Chat>>>) -> Result<impl IR, Rejection> {
    let history = chat.lock().unwrap().get_history().clone();

    if history.is_empty() {
        error!("GET on chat resulted on error, chat is empty");

        Err((StatusCode::NOT_FOUND, Json("History is empty".to_string())))
    } else {
        info!("Chat request parsed successfully.");

        Ok(Json(Messages { messages: history }))
    }
}
