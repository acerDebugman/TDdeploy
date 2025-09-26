pub mod tmq;
pub mod mysql;
pub mod read_parquet;
pub mod kafka;
pub mod subscribe;

use taos::*;

pub async fn loop_data() -> Result<(), anyhow::Error> {
    let db = "ts5820";
    let addr = "localhost:6030";

     loop_producer(&format!("taos://{}", addr), db, 10000000).await?;

    Ok(())
}

pub async fn loop_producer(dsn: &str, db: &str, limit: usize) -> anyhow::Result<()> {
    // let dsn = "taos://192.168.2.131:6030";
    //let dsn = "taos://127.0.0.1:6030";
    // let db = "ts5820";

    let pool = taos::TaosBuilder::from_dsn(dsn)?.pool()?;

    let taos = pool.get().await?;

    taos.exec_many([
        format!("DROP DATABASE IF EXISTS `{db}`"),
        format!("CREATE DATABASE `{db}`"),
        format!("USE `{db}`"),
        // "create stable meters(ts timestamp, id int, voltage int, v_blob varchar(1024)) tags(groupid int, location varchar(24));".to_string(),
        "create stable meters(ts timestamp, id int, voltage int, v_blob blob) tags(groupid int, location varchar(24));".to_string(),
    ])
    .await?;

    let mut cnt = 0;
    loop {
        cnt+=1;
        if cnt > limit {
            println!("produce end, reach limit: {}", limit);
            break;
        }

        // let item = format!("\\x{}", "255044462D312E330D0A".repeat(10));
        // 3M = 20 * 1000 * 150
        // let item = format!("\\x{}", "255044462D312E330D0A".repeat(1000 * 150));
        // 500k = 20 * 1000 * 25
        let item = format!("\\x{}", "255044462D312E330D0A".repeat(1000 * 25));
        // let item = format!("\\x{}", "255044462D312E330D0A");
        println!("item len: {}", item.len());
        let inserted = taos.exec_many([
            &format!("INSERT INTO `t1` using `meters` (`groupid`,`location`) tags(1,\"BJ\") (`ts`,`id`,`voltage`,`v_blob`) values(now,1,11,'{}')", item),
        ]).await?;

        println!("inserted: {:?}", inserted);
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }

    Ok(())
}