use jsonwebtoken::{encode, decode, Header, Validation, Algorithm, EncodingKey, DecodingKey, TokenData};
use serde::{Serialize, Deserialize};
use chrono::{Utc, Duration};

/// JWT 负载结构（Claims）
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,   // 用户标识（比如用户名或用户ID）
    pub exp: usize,    // 过期时间（时间戳，秒）
}

// TODO change me
const SECRET_KEY: &[u8] = b"changeme";

/// 生成 JWT token
pub fn generate(sub: &str, expires_in_minutes: i64) -> Result<String, jsonwebtoken::errors::Error> {
    // 当前时间 + 有效期（分钟）
    let expiration = Utc::now()
        .checked_add_signed(Duration::minutes(expires_in_minutes))
        .expect("Invalid duration")
        .timestamp() as usize;

    let claims = Claims {
        sub: sub.to_owned(),
        exp: expiration,
    };

    // 生成 token，使用 HS256 算法
    encode(
        &Header::new(Algorithm::HS256),
        &claims,
        &EncodingKey::from_secret(SECRET_KEY),
    )
}

/// 验证并解析 JWT token
pub fn verify(token: &str) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(SECRET_KEY),
        &Validation::new(Algorithm::HS256),
    )
}