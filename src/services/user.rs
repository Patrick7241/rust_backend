use crate::common::token;
use crate::db::mysql;
use crate::db::schema::users::dsl::*; // 👈 引入 users、username、password
use diesel::prelude::*;
use anyhow::Result;

pub fn login(username_input: &str, password_input: &str) -> Result<String> {
    // 从连接池获取连接
    let mut conn = mysql::get_conn()?;

    // 查询数据库
    let user_opt: Option<(String, String)> = users
        .filter(username.eq(username_input))
        .select((username, password))
        .first::<(String, String)>(&mut conn)
        .optional()?; // 不存在返回 None

    // 释放连接（更新统计计数）
    mysql::release_conn();

    // 校验用户
    match user_opt {
        Some((db_username, db_password)) if db_password == password_input => {
            let token = token::generate(&db_username, 30)?;
            Ok(token)
        }
        _ => Err(anyhow::anyhow!("用户名或密码错误")),
    }
}