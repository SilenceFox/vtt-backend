use chrono::Utc;
use log::{error, info};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

/// This module is focused on the CHAT API endpoint.
/// TODO: GET chat messages
/// TODO: GET polling
/// TODO: POST new messages

#[derive(Debug)]
pub(crate) struct Chat {
    users: Vec<Arc<User>>,
    messages: Vec<Message>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct User {
    username: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Message {
    user: Arc<User>,
    time: String,
    message: String,
}
// macro_rules! msg {
//     ($user:expr, $message:expr) => {{
//         super::chat.send_message(&$user, $message.to_string())
//     }};
// }

impl Chat {
    pub fn new() -> Self {
        Self::new_chat(Vec::new(), Vec::new())
    }
    fn new_chat(users: Vec<Arc<User>>, messages: Vec<Message>) -> Self {
        Self { users, messages }
    }
    fn check_user_exists(&self, username: &str) -> bool {
        self.users
            .iter()
            .any(|user| user.username == username.trim())
    }
    fn user_join(&mut self, new_user: &Arc<User>) {
        self.users.push(new_user.clone())
    }
    fn user_leave(&mut self, removed_user: &Arc<User>) {
        // Checks if the given user is in the chat and removes him
        info!("{} has left the chat.", removed_user.username);
        self.users.retain(|x| x.username != removed_user.username)
    }
    fn add_to_history(&mut self, msg: Message) {
        self.messages.push(msg);
    }
    fn get_history(&self) {
        // TODO: Truncate the history to 50 messages
        for msg in &self.messages {
            println!("╠════════════════════════════╕");
            let formatted = format!(
                "║Time: {} \n║ |>{}: {}",
                msg.time,
                msg.user.username.to_uppercase(),
                msg.message,
            );
            println!("{formatted}")
        }
        println!("╚════════════════════════════╛");
    }
    fn send_message(&mut self, user: &Arc<User>, msg: String) {
        let message = Message::new_message(user.clone(), msg);
        self.add_to_history(message)
    }
    fn get_users(&self) {
        if self.users.is_empty() {
            error!("No users are currently in the chat");
        } else {
            info!("Users currently in the chat:");
            for user in self.users.iter() {
                info!("{}", user.username);
            }
            info!("End of list of users");
        }
        // Iterate over each user in the chat and print a log
    }
    pub(crate) fn get_your_user(&self, username: &str) -> Option<&Arc<User>> {
        self.users
            .iter()
            .find(|user| user.username == username.trim())
    }
    //
    // API specific methods
    //
}

impl User {
    /// Adds a new user
    pub fn new_user(username: String) -> Arc<Self> {
        let username = username.trim().to_string();
        info!("New user {} has joined", &username);
        Arc::new(Self { username })
    }
    /// Consumes the User struct and deletes it
    pub fn delete_user(self) {
        info!("The user {}, has been destroyed", self.username);
        ()
    }
}

impl Message {
    pub fn new_message(user: Arc<User>, message: String) -> Self {
        let time = Utc::now()
            .with_timezone(&chrono_tz::America::Sao_Paulo)
            .format("%d/%m [%H:%M:%S]")
            .to_string();
        let process_msg = message.trim().to_string();
        let message = Self {
            user,
            time,
            message: process_msg,
        };
        message
    }
}

pub(crate) fn handle_user_join(username: String, chat_state: &Arc<Mutex<Chat>>) {
    let mut chat = chat_state.lock().unwrap();
    if chat.check_user_exists(&username) {
        error!("User already exists in the chat");
    } else {
        let new_user = Arc::new(User::new_user(username));
        chat.user_join(&new_user);
    }
}

pub(crate) fn handle_user_leave(username: &Arc<User>, chat_state: &Arc<Mutex<Chat>>) {
    let mut chat = chat_state.lock().unwrap();
    match chat.check_user_exists(&username.username) {
        true => {
            info!("User: {} has left the chat.", &username.username);
            chat.user_leave(username)
        }
        false => error!("User: {} does not exist in the chat.", &username.username),
    }
}
pub(crate) fn handle_user_send_message(
    username: &Arc<User>,
    message: String,
    chat_state: &Arc<Mutex<Chat>>,
) {
    let mut chat = chat_state.lock().unwrap();
    if chat.check_user_exists(&username.username) {
        info!(
            "Message sent from: {}\nMessage: {}",
            &username.username, &message
        );
        chat.send_message(username, message)
    } else {
        chat.user_join(username);
        chat.send_message(username, message)
    }
}
