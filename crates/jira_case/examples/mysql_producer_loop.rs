use chrono::NaiveDateTime;
use sqlx::{MySql, MySqlPool, Pool, mysql::MySqlPoolOptions};
use std::env;
use sqlx::types::time::OffsetDateTime;

// 1. 连接串里暂时不写“database”，先连到 mysql 系统库
// const DEFAULT_URL: &str = "mysql://root:123456@localhost:3306/mysql";
const DEFAULT_URL: &str = "mysql://root:taosdata@192.168.2.131:3306/mysql";

// 目标库名
const DB_NAME: &str = "testdb";

#[derive(Debug, sqlx::FromRow)]
struct User {
    id: i64,
    username: String,
    email: String,
    created_at: OffsetDateTime,
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // 如果 .env 存在就读取
    // dotenvy::dotenv()?;
    unsafe { env::set_var("DATABASE_URL", DEFAULT_URL); }

    // 2. 先连到系统库，创建业务库
    let root_pool = MySqlPoolOptions::new()
        .max_connections(1)
        .connect(&env::var("DATABASE_URL").unwrap_or_else(|_| DEFAULT_URL.to_string()))
        .await?;

    create_database_if_not_exists(&root_pool, DB_NAME).await?;
    root_pool.close().await;

    // 3. 再连到业务库，创建表并写入数据
    let app_url = format!(
        "mysql://{}:{}@{}/{}",
        env::var("DB_USER").unwrap_or_else(|_| "root".to_string()),
        env::var("DB_PASS").unwrap_or_else(|_| "taosdata".to_string()),
        env::var("DB_ADDR").unwrap_or_else(|_| "192.168.2.131:3306".to_string()),
        DB_NAME
    );
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&app_url)
        .await?;

    create_table_if_not_exists(&pool).await?;

    // 4. 插入示例
    let uid = insert_user(&pool, "rustacean", "rust@crates.io").await?;
    println!("新用户 ID = {}", uid);

    // 5. 查出来验货
    // let user = sqlx::query_as!(
    //     User,
    //     "SELECT id, username, email, created_at FROM users WHERE id = ?",
    //     uid
    // )
    // .fetch_one(&pool)
    // .await?;
    let user = sqlx::query_as::<_, User>(
        &format!("SELECT id, username, email, created_at FROM users WHERE id = {}", uid)
    )
    .fetch_one(&pool)
    .await?;
    println!("刚写入的用户: {:?}", user);

    pool.close().await;
    Ok(())
}

/// 建库
async fn create_database_if_not_exists(pool: &Pool<MySql>, db: &str) -> Result<(), sqlx::Error> {
    let sql = format!("CREATE DATABASE IF NOT EXISTS `{}` CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;", db);
    sqlx::query(&sql).execute(pool).await?;
    println!("数据库 `{}` 已确保存在", db);
    Ok(())
}

/// 建表
async fn create_table_if_not_exists(pool: &MySqlPool) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id          BIGINT AUTO_INCREMENT PRIMARY KEY,
            username    VARCHAR(32) NOT NULL,
            email       VARCHAR(64) NOT NULL,
            created_at  TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        ) ENGINE = InnoDB
          DEFAULT CHARSET = utf8mb4
          COLLATE = utf8mb4_unicode_ci;
        "#,
    )
    .execute(pool)
    .await?;
    println!("表 `users` 已确保存在");
    Ok(())
}

/// 插入用户，返回 last_insert_id
async fn insert_user(pool: &MySqlPool, name: &str, mail: &str) -> Result<i64, sqlx::Error> {
    // let rec = sqlx::query!(
    //     r#"
    //     INSERT INTO users (username, email)
    //     VALUES (?, ?)
    //     ON DUPLICATE KEY UPDATE
    //         email = VALUES(email),
    //         created_at = NOW()
    //     "#,
    //     name,
    //     mail
    // )
    // .execute(pool)
    // .await?;
    let rec = sqlx::query(
        "INSERT INTO users (username, email) VALUES (?, ?)",
    )
    .bind(name)
    .bind(mail)
    .execute(pool)
    .await?;
    Ok(rec.last_insert_id() as i64)
}