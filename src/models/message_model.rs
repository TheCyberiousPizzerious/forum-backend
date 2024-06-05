use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct MessageMessage {
    pub message: String,
}

#[derive(Serialize, Deserialize)]
pub struct ErrorMessage {
    pub error: String,
}

#[derive(Serialize, Deserialize)]
pub struct RegisterSending {
    pub username: String,
    pub admin: bool,
    pub user_id: Uuid,
}

#[derive(Serialize, Deserialize)]
pub struct SendWhatever<T> {
    pub field: T,
}

/// traits for the different structs defined in message_models.rs
pub trait MessageTraits {
    /// function that takes a &str argument and returns an instance of a message struct
    /// ```no_run
    /// let message = MessageMessage::new_from("something that is in a message");
    /// ```
    fn new_from(str: String) -> Self;
}

impl MessageTraits for ErrorMessage {
    fn new_from(str: String) -> Self {
        Self {
            error: str,
        }
    }
}

impl MessageTraits for MessageMessage {
    fn new_from(str: String) -> Self {
        Self {
            message: str,
        }
    }
}

pub trait WhatevTraits<T> {
    fn new_from(whatev: T) -> Self;
}

impl<T> WhatevTraits<T> for SendWhatever<T> {
    fn new_from(whatev: T) -> Self {
        Self {
            field: whatev
        }
    }
}