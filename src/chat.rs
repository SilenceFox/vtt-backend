use std::sync::Arc;

use chrono::Utc;
use log::info;

/// This module is focused on the CHAT API endpoint.
/// TODO: GET chat messages
/// TODO: GET polling
/// TODO: POST new messages

#[derive(Debug)]
pub(crate) struct Chat {
    users: Vec<User>,
    messages: Vec<Message>,
}

#[derive(Debug, Clone)]
pub(crate) struct User {
    username: String,
}

#[derive(Debug, Clone)]
pub(crate) struct Message {
    user: Arc<User>,
    time: String,
    message: String,
}

impl Chat {
    pub fn new() -> Self {
        Self::new_chat(Vec::new(), Vec::new())
    }
    fn new_chat(users: Vec<User>, messages: Vec<Message>) -> Self {
        Self { users, messages }
    }
    fn user_join(new_user: User) {
        todo!()
    }
    fn user_leave(removed_user: User) {
        todo!()
    }
    fn add_to_history(&mut self, msg: Message) {
        self.messages.push(msg);
    }
    pub fn get_history(&self) {
        // TODO: Truncate the history to 50 messages
        for msg in &self.messages {
            println!("");
            let formatted = format!(
                "=========\nTime: {} \n {}: {}\n=========",
                msg.time,
                msg.user.username.to_uppercase(),
                msg.message
            );
            println!("{formatted}")
        }
    }
    pub(crate) fn send_message(&mut self, user: Arc<User>, msg: String) {
        let message = Message::new_message(user, msg);
        self.add_to_history(message)
    }
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
