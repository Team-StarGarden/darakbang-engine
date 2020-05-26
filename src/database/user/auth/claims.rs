use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TokenClaims {
    pub exp: usize,
    pub iat: usize,
    pub iss: String,
    pub sub: String,
}