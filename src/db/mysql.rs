use diesel::{
    mysql::MysqlConnection,
    r2d2::{ConnectionManager, Pool, PooledConnection},
    prelude::*
};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use dotenvy::dotenv;
use once_cell::sync::OnceCell;
use std::{env, sync::atomic::{AtomicUsize, Ordering}};
use std::time::Duration;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

// 类型别名
pub type DbPool = Pool<ConnectionManager<MysqlConnection>>;

// 全局连接池
static DB_POOL: OnceCell<DbPool> = OnceCell::new();

// 使用中的连接计数
static ACTIVE_CONN: AtomicUsize = AtomicUsize::new(0);

/// 初始化连接池 + 自动运行迁移
pub fn init_db() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<MysqlConnection>::new(database_url.clone());

    let pool = Pool::builder()
        .max_size(20) // 最大连接数，可调
        .min_idle(Some(5)) // 最少空闲连接
        .connection_timeout(Duration::from_secs(5)) // 取连接超时时间
        .build(manager)
        .expect("Failed to create db pool");

    // 先运行 migrations（用单个普通连接）
    {
        let mut conn = MysqlConnection::establish(&database_url)
            .expect("Error connecting for migrations");
        run_migrations(&mut conn);
    }

    DB_POOL.set(pool).expect("DB pool already initialized");
}

/// 执行 migrations
fn run_migrations(conn: &mut MysqlConnection) {
    println!("Running pending migrations...");
    conn.run_pending_migrations(MIGRATIONS)
        .expect("Failed to run migrations");
    println!("✅ Migrations completed");
}

/// 获取全局连接池
pub fn get_pool() -> &'static DbPool {
    DB_POOL.get().expect("Database pool is not initialized")
}

/// 从连接池获取一个连接（带统计的）
pub fn get_conn() -> Result<PooledConnection<ConnectionManager<MysqlConnection>>, diesel::r2d2::PoolError> {
    ACTIVE_CONN.fetch_add(1, Ordering::SeqCst); // 当前连接+1
    let conn = get_pool().get();
    if conn.is_err() {
        ACTIVE_CONN.fetch_sub(1, Ordering::SeqCst); // 失败则减回去
    }
    conn
}

/// 释放连接（统计用）
pub fn release_conn() {
    ACTIVE_CONN.fetch_sub(1, Ordering::SeqCst);
}

/// 打印连接池状态
pub fn print_pool_status() {
    let pool = get_pool();
    let max_size = pool.max_size();
    let active = ACTIVE_CONN.load(Ordering::SeqCst);
    let idle = pool.state().idle_connections;
    println!(
        "[DB STATUS] max_size: {}, active: {}, idle: {}",
        max_size, active, idle
    );
}