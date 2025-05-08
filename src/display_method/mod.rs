//! ## `DisplayMethod`
//!
//! This module contains the `DisplayMethod` trait and its implementations for various display methods.

mod comma_separated;
mod common;
mod double_joined;
mod formatter;
mod joined;
mod separated;

pub use comma_separated::CommaSeparated;
pub use double_joined::DoubleJoined;
pub use joined::Joined;
pub use separated::Separated;

/// Common trait for display methods.
pub trait DisplayMethod {}
