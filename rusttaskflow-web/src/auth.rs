use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use rusttaskflow_core::{Result, TaskFlowError};
use serde::{Deserialize, Serialize};
use std::env;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct AuthUser {
    pub user_id: Uuid,
    pub username: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // User ID
    pub username: String,
    pub email: String,
    pub exp: usize, // Expiration time
    pub iat: usize, // Issued at
}

pub struct AuthService {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
}

impl AuthService {
    pub fn new() -> Self {
        let secret = env::var("JWT_SECRET")
            .unwrap_or_else(|_| "your-secret-key-change-this-in-production".to_string());
        
        Self {
            encoding_key: EncodingKey::from_secret(secret.as_ref()),
            decoding_key: DecodingKey::from_secret(secret.as_ref()),
        }
    }

    pub fn generate_token(&self, user_id: Uuid, username: &str, email: &str) -> Result<String> {
        let now = chrono::Utc::now();
        let exp = (now + chrono::Duration::hours(24)).timestamp() as usize;
        let iat = now.timestamp() as usize;

        let claims = Claims {
            sub: user_id.to_string(),
            username: username.to_string(),
            email: email.to_string(),
            exp,
            iat,
        };

        encode(&Header::default(), &claims, &self.encoding_key)
            .map_err(|e| TaskFlowError::Authentication {
                message: format!("Failed to generate token: {}", e),
            })
    }

    pub fn verify_token(&self, token: &str) -> Result<Claims> {
        decode::<Claims>(token, &self.decoding_key, &Validation::default())
            .map(|data| data.claims)
            .map_err(|e| TaskFlowError::Authentication {
                message: format!("Invalid token: {}", e),
            })
    }

    pub fn hash_password(password: &str) -> Result<String> {
        bcrypt::hash(password, bcrypt::DEFAULT_COST)
            .map_err(|e| TaskFlowError::Authentication {
                message: format!("Failed to hash password: {}", e),
            })
    }

    pub fn verify_password(password: &str, hash: &str) -> Result<bool> {
        bcrypt::verify(password, hash)
            .map_err(|e| TaskFlowError::Authentication {
                message: format!("Failed to verify password: {}", e),
            })
    }
}

impl Default for AuthService {
    fn default() -> Self {
        Self::new()
    }
}