use jsonwebtoken::{encode, decode, Header, Validation, Algorithm, EncodingKey, DecodingKey, TokenData};
use serde::{Serialize, Deserialize};
use chrono::{Utc, Duration};

/// JWT 负载结构（Claims）
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,   // 用户标识
    pub exp: usize,    // 过期时间（秒级时间戳）
    // 设置签发时间，防止短时间内被重放攻击，和签发时间间隔过长的token即使没有过期，也无效
    pub iat: usize,    // 签发时间（秒级时间戳）
}

// TODO change me
const SECRET_KEY: &[u8] = b"changeme";

/// 生成 JWT token
pub fn generate(sub: &str, expires_in_minutes: i64) -> Result<String, jsonwebtoken::errors::Error> {
    let now = Utc::now().timestamp() as usize;
    let expiration = (Utc::now() + Duration::minutes(expires_in_minutes)).timestamp() as usize;

    let claims = Claims {
        sub: sub.to_owned(),
        exp: expiration,
        iat: now,   // 签发时间
    };

    encode(
        &Header::new(Algorithm::HS256),
        &claims,
        &EncodingKey::from_secret(SECRET_KEY),
    )
}

/// 验证并解析 JWT token
pub fn verify(token: &str) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
    let data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(SECRET_KEY),
        &Validation::new(Algorithm::HS256),
    )?;

    let now = Utc::now().timestamp() as usize;
    if now.saturating_sub(data.claims.iat) > 10 {
        return Err(jsonwebtoken::errors::Error::from(
            jsonwebtoken::errors::ErrorKind::ExpiredSignature,
        ));
    }

    Ok(data)
}
