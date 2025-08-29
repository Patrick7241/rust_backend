use jsonwebtoken::{encode, decode, Header, Validation, Algorithm, EncodingKey, DecodingKey, TokenData};
use serde::{Serialize, Deserialize};
use chrono::{Utc, Duration};

/// JWT 负载结构（Claims）
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub username: String,   // 用户名
    pub user_id: String,   // 用户ID
    pub exp: usize,    // 过期时间（秒级时间戳）
    pub iat: usize,    // 签发时间（秒级时间戳）
}

// TODO change me
const SECRET_KEY: &[u8] = b"changeme";

/// 生成 JWT token
pub fn generate(username: &str, user_id: &str,expires_in_minutes: i64) -> Result<String, jsonwebtoken::errors::Error> {
    let now = Utc::now().timestamp() as usize;
    let expiration = (Utc::now() + Duration::minutes(expires_in_minutes)).timestamp() as usize;

    let claims = Claims {
        username: username.to_owned(),
        user_id: user_id.to_owned(),
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

    // 处理异常清空，签发时间不可能比现在时间大，防止攻击者仿造来自未来的token
    // 允许5秒的误差
    if data.claims.iat > now + 5{
        return Err(jsonwebtoken::errors::Error::from(jsonwebtoken::errors::ErrorKind::InvalidToken));
    }

    Ok(data)
}
