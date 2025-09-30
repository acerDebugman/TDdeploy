use sqlx::{mysql::MySqlPoolOptions, Executor};


pub async fn mysql_main() -> anyhow::Result<()> {
    let addr = "mysql:3306";
    let db = "test";
    loop_mysql(addr, db, 3).await?;
    Ok(())
}

async fn loop_mysql(addr: &str, db: &str, limit: usize) -> anyhow::Result<()> {
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&format!("mysql://root:taosdata@{addr}")).await?;

    let _ = pool.execute(format!("DROP DATABASE IF EXISTS {db}").as_str()).await?;
    let _ = pool.execute(format!("CREATE DATABASE IF NOT EXISTS {db}").as_str()).await?;
    let _ = pool.execute(format!("USE {db}").as_str()).await?;
    
    let mut pool_conn = pool.acquire().await?;
    let conn = pool_conn.as_mut();
    conn.execute(format!("use {db}").as_str()).await?;
    sqlx::raw_sql(r#"
        drop table if exists t0;
        CREATE TABLE `t0` (
            `id` int NOT NULL AUTO_INCREMENT,
            `voltage` int NOT NULL,
            `v_blob` blob NOT NULL,
            `groupid` int NOT NULL,
            `location` varchar(24) NOT NULL,
            `time` datetime DEFAULT CURRENT_TIMESTAMP,
            PRIMARY KEY (`id`)
        );
    "#).execute(conn)
        .await?;

    let mut idx = 0;
    loop {
        if idx >= limit {
            break;
        }
        idx += 1;

        let mut pool_conn = pool.acquire().await?;
        let conn = pool_conn.as_mut();
        conn.execute(format!("use {db}").as_str()).await?;
        
        // insert into t0(voltage, v_blob, groupid, location) values(123, 'zgc', 10, "bj");
        // insert into t0(voltage, v_blob, groupid, location) values(222, unhex('7a6763'), 10, "bj");
        sqlx::raw_sql(
            format!(
                r#"
                    insert into t0(voltage, v_blob, groupid, location) values(222, unhex('7a6763'), {}, "bj");
                "#, idx
            ).as_str()
        ).execute(conn)
            .await?;

        let rows = sqlx::query(
            r#"
            select id, voltage, v_blob, groupid, location from t0 order by id desc limit 1;
            "#,
        ).fetch_all(&pool)
        .await?;

        for row in rows {
            println!("result: {:?}", row);
        }
    }
    
    println!("loop mysql");
    Ok(())
}

pub async fn simple_query() -> anyhow::Result<()> {
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:password@localhost/test").await?;


    // Make a simple query to return the given parameter (use a question mark `?` instead of `$1` for MySQL/MariaDB)
    let row: (i64,) = sqlx::query_as("SELECT ?")
        .bind(150_i64)
        .fetch_one(&pool).await?;

    assert_eq!(row.0, 150);

    Ok(())
}