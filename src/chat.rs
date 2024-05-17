use chrono::Utc;
use log::{error, info};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

/// This module is focused on the CHAT API endpoint.
/// DONE: GET chat messages
/// DONE: GET polling
/// DONE: POST new messages
/// DONE: POST Join chat

#[derive(Debug)]
pub(crate) struct Chat {
    users: Vec<Arc<User>>,
    messages: Vec<Message>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct User(String);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Message {
    user: Arc<User>,
    clock: Clock,
    message: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
/// With this struct anyone receiving messages can iterate over fields easily.
pub(crate) struct Clock {
    time: (i32,i32,i32), // Hour, minute, second
    date: (i32,i32,i32) // Day, Month, Year
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
        self.users.iter().any(|user| user.0 == username.trim())
    }

    fn user_join(&mut self, new_user: &Arc<User>) {
        self.users.push(new_user.clone())
    }

    fn user_leave(&mut self, removed_user: &Arc<User>) {
        // Checks if the given user is in the chat and removes him
        info!("{} has left the chat.", removed_user.0);
        self.users.retain(|x| x.0 != removed_user.0)
    }

    fn add_to_history(&mut self, msg: Message) {
        self.messages.push(msg);
    }

    fn get_history(&self) -> &Vec<Message> {
        // Get the entire history of messages in chronological order, first to last
        &self.messages
    }
    fn get_last_message(&self, format: bool) {
        if let Some(msg) = self.messages.last() {
            if format {
                let bar = "╰──────/MESSAGE-START/────────";

                let formatted = format!(
                    "│Time: {}:{} \n│ |>{}: \n{}\n{}",
                    &msg.clock.time.0,
                    &msg.clock.time.1,
                    msg.user.0.to_uppercase(),
                    bar,
                    msg.message,
                );
                println!("╭─────────────────────────────");
                println!("{formatted}");
                println!("╰───────/MESSAGE-END/─────────")
            }
        }
    }

    fn send_msg(&mut self, user: &Arc<User>, msg: &str) {
        let message = Message::new_message(user.clone(), msg);
        self.add_to_history(message)
    }

    fn get_users(&self) {
        if self.users.is_empty() {
            error!("No users are currently in the chat");
        } else {
            info!("Users currently in the chat:");
            for user in self.users.iter() {
                info!("{}", user.0);
            }
            info!("End of list of users");
        }
        // Iterate over each user in the chat and print a log
    }

    pub(crate) fn get_your_user(&self, username: &str) -> Option<&Arc<User>> {
        self.users.iter().find(|user| user.0 == username.trim())
    }
}

impl User {
    /// Adds a new user
    pub fn new_user(username: &str) -> Arc<Self> {
        let username = username.trim().to_string();
        info!("New user {} has joined", &username);
        Arc::new(Self(username))
    }

    /// Consumes the User struct and deletes it
    pub fn delete_user(self) {
        info!("The user {}, has been destroyed", self.0);
    }
    pub fn get_username(&self) -> &str {
        self.0.as_str()
    }
}

impl Message {
    pub fn new_message(user: Arc<User>, message: &str) -> Self {
        // When a new message is generated this will immediately mark the time.
        let time = Utc::now()
            .with_timezone(&chrono_tz::America::Sao_Paulo)
            .format("%H:%M:%S|%d/%m/%Y")
            .to_string();

        // the clock gets built
        // TODO: Remove unwraps()
        let parts: Vec<&str> = time.split('|').collect();
        let time_parts: Vec<i32> = parts[0].split(':')
            .map(|part| part.parse().unwrap())
            .collect();
        let date_parts: Vec<i32> = parts[1].split('/')
            .map(|part| part.parse().unwrap())
            .collect();

        // Compile the Clock into one struct and construct the final msg
        let clock = Clock {
            time: (time_parts[0], time_parts[1], time_parts[2]),
            date: (date_parts[0], date_parts[1], date_parts[2]),
        };

        // TODO: Filter out non-UTF8
        let process_msg = message.trim().to_string();

        Self {
            user,
            clock,
            message: process_msg,
        }
    }
}

pub(crate) fn join_helper(username: &str, chat_state: &Arc<Mutex<Chat>>) {
    let mut chat = chat_state.lock().unwrap();
    if chat.check_user_exists(username) {
        error!("User already exists in the chat");
    } else {
        let new_user = Arc::new(User::new_user(username));
        chat.user_join(&new_user);
    }
}

pub(crate) fn leave_helper(username: &Arc<User>, chat_state: &Arc<Mutex<Chat>>) {
    let mut chat = chat_state.lock().unwrap();

    match chat.check_user_exists(&username.0) {
        true => chat.user_leave(username),
        false => error!("User: {} does not exist in the chat.", &username.0),
    }
}
pub(crate) fn send_message_helper(
    username: &Arc<User>,
    message: &str,
    chat_state: &Arc<Mutex<Chat>>,
) {
    let mut chat = chat_state.lock().unwrap();

    if !chat.check_user_exists(&username.0) {
        info!("User does not exist in the chat. Creating new user...");
        chat.user_join(username);
        info!("User created: {}", &username.0);
    };

    info!("Message sent from: {}\nMessage: {}", &username.0, &message);

    chat.send_msg(username, message)
}

/// This module is focused on the API handlers
/// Export all API handlers and routes for the chat
pub mod api;
