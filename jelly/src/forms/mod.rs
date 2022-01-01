//! Implements a set of input types that can be used for Form handling. Mostly modeled after
//! Django's Form class.
//!
//! Ex:
//!
//! ```rust
//! use jelly::forms::{EmailField, PasswordField, Validation};
//! use serde::Deserialize;
//!
//! #[derive(Debug, Default, Deserialize)]
//! pub struct MyForm {
//!     pub email: EmailField,
//!     pub password: PasswordField
//! }
//!
//! impl Validation for MyForm {
//!     fn is_valid(&mut self) -> bool {
//!         self.email.is_valid() && self.password.is_valid()
//!     }
//! }
//! ```

mod booly;
pub use booly::BoolField;

mod date;
pub use date::DateField;

mod email;
pub use email::EmailField;

mod password;
pub use password::PasswordField;

mod slug;
pub use slug::SlugField;

mod text;
pub use text::TextField;

/// A trait that Forms can implement for validation. Each field type implements this trait, so you
/// can simply write your validation method as `field.is_valid()`.
pub trait Validation {
    /// Checks if the data held is valid. Should return a bool value.
    fn is_valid(&mut self) -> bool {
        false
    }
}
