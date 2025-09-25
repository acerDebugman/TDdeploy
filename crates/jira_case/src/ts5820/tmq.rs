use std::time::Duration;

use chrono::{DateTime, Local};
use taos::*;


pub async fn test_tmq() -> anyhow::Result<()> {
    let _ = tokio::spawn(async move {
        let _ = subscribe("taos://localhost:6030/?group.id=test").await;
    }); 

    producer("taos://localhost:6030", 10000000).await?;
    Ok(())
}


// tmq producer, 持续向 td 里写数据即可
pub async fn producer(dsn: &str, limit: usize) -> anyhow::Result<()> {
    // let dsn = "taos://192.168.2.131:6030";
    //let dsn = "taos://127.0.0.1:6030";

    let pool = TaosBuilder::from_dsn(dsn)?.pool()?;

    let taos = pool.get().await?;

    let db = "ts5820";
    taos.exec_many([
        format!("USE `{db}`"),
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
        let item = format!("\\x{}", "255044462D312E330D0A".repeat(10));
        println!("item len: {}", item.len());
        let inserted = taos.exec_many([
            &format!("INSERT INTO `t1` using `meters` (`groupid`,`location`) tags(1,\"BJ\") (`ts`,`id`,`voltage`,`v_blob`) values(now,1,11,'{}')", item),
        ]).await?;

        println!("inserted: {:?}", inserted);
        tokio::time::sleep(Duration::from_secs(1)).await;
    }

    Ok(()) 
}

// Query options 2, use deserialization with serde.
#[derive(Debug, serde::Deserialize)]
#[allow(dead_code)]
struct Record {
    // deserialize timestamp to chrono::DateTime<Local>
    ts: DateTime<Local>,
    // float to f32
    current: Option<f32>,
    // int to i32
    voltage: Option<i32>,
    phase: Option<f32>,
}

async fn prepare(taos: Taos) -> anyhow::Result<()> {
    let inserted = taos.exec_many([
        // create child table
        "CREATE TABLE `d0` USING `meters` TAGS(0, 'Los Angles')",
        // insert into child table
        "INSERT INTO `d0` values(now - 10s, 10, 116, 0.32)",
        // insert with NULL values
        "INSERT INTO `d0` values(now - 8s, NULL, NULL, NULL)",
        // insert and automatically create table with tags if not exists
        "INSERT INTO `d1` USING `meters` TAGS(1, 'San Francisco') values(now - 9s, 10.1, 119, 0.33)",
        // insert many records in a single sql
        "INSERT INTO `d1` values (now-8s, 10, 120, 0.33) (now - 6s, 10, 119, 0.34) (now - 4s, 11.2, 118, 0.322)",
    ]).await?;
    assert_eq!(inserted, 6);
    Ok(())
}


pub async fn subscribe(dsn: &str, group_id: &str) -> anyhow::Result<()> {
    // std::env::set_var("RUST_LOG", "debug");
    pretty_env_logger::init();
    // let dsn = "taos://localhost:6030";
    let builder = TaosBuilder::from_dsn(dsn)?;

    let taos = builder.build().await?;
    let db = "ts5820";

    // prepare database
    taos.exec_many([
        "DROP TOPIC IF EXISTS tmq_meters".to_string(),
        format!("DROP DATABASE IF EXISTS `{db}`"),
        format!("CREATE DATABASE `{db}`"),
        format!("USE `{db}`"),
        // create super table
        // "CREATE TABLE `meters` (`ts` TIMESTAMP, `current` FLOAT, `voltage` INT, `phase` FLOAT) TAGS (`groupid` INT, `location` BINARY(16))".to_string(),
        "create stable meters(ts timestamp, id int, voltage int, v_blob blob) tags(groupid int, location varchar(24));".to_string(),
        // create topic for subscription
        format!("CREATE TOPIC tmq_meters with META AS DATABASE {db}")
    ])
    .await?;

    // let task = tokio::spawn(prepare(taos));

    tokio::time::sleep(Duration::from_secs(1)).await;

    // subscribe
    // let tmq = TmqBuilder::from_dsn("taos://localhost:6030/?group.id=test")?;
    let tmq = TmqBuilder::from_dsn(format!("{}?group.id={}", dsn, group_id))?;

    let mut consumer = tmq.build().await?;
    consumer.subscribe(["tmq_meters"]).await?;

    {
        let mut stream = consumer.stream();

        while let Some((offset, message)) = stream.try_next().await? {
            // get information from offset

            // the topic
            let topic = offset.topic();
            // the vgroup id, like partition id in kafka.
            let vgroup_id = offset.vgroup_id();
            println!("* in vgroup id {vgroup_id} of topic {topic}\n");

            if let Some(data) = message.into_data() {
                while let Some(block) = data.fetch_raw_block().await? {
                    // one block for one table, get table name if needed
                    let name = block.table_name();
                    let records: Vec<Record> = block.deserialize().try_collect()?;
                    println!(
                        "** table: {}, got {} records: {:#?}\n",
                        name.unwrap(),
                        records.len(),
                        records
                    );
                }
            }
            consumer.commit(offset).await?;
        }
    }

    consumer.unsubscribe().await;

    // task.await??;

    Ok(())
}

