//! Core module - contains business logic and domain services.

pub mod date_utils;
pub mod event_service;

pub use date_utils::{get_date_suggestions, validate_date_input, validate_time_input};
pub use event_service::EventService;
