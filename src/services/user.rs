use crate::common::token;
use anyhow::Result;

pub fn login(username: &str, password: &str) -> Result<String> {
    // 模拟账号密码验证
    if username == "admin" && password == "123456" {
        // 生成 token（7 天有效期）
        let token = token::generate(username, 60 * 60 * 24 * 7)?;
        Ok(token)
    } else {
        Err(anyhow::anyhow!("用户名或密码错误"))
    }
}