use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct TokenClaims {
    pub exp: usize,
    pub iat: usize,
    pub iss: String,
    pub sub: String,
}