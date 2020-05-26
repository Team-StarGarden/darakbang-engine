use thiserror::Error;

#[derive(Error, Debug)]
pub enum UserAuthError {
    #[error("User not found")]
    NotFound,
    #[error("Authentication error")]
    Authentication,
}