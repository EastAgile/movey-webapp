//! This module contains useful components for building Authentication
//! rituals. These are typically pieces that are used across projects,
//! but not necessarily a full framework.

use serde::{Deserialize, Serialize};

pub mod password;
pub use password::make_random_password;

pub mod token_generator;
pub use token_generator::OneTimeUseTokenGenerator;

/// A smaller, serialize-able instance of an Account
/// that can be used to avoid a database hit.
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub is_admin: bool,
    pub is_anonymous: bool,
}

impl Default for User {
    /// A default user is anonymous.
    fn default() -> Self {
        User {
            id: 0,
            name: String::new(),
            is_admin: false,
            is_anonymous: true,
        }
    }
}
