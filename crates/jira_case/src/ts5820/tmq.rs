use std::{time::Duration};
// use taosx_core::taoz::Header;
// use taosx_core::taoz::ZCodec;
use std::sync::Arc;
use taos::tokio::io::AsyncWriteExt;
use chrono::{DateTime, Local};
use taos::*;
use tokio::sync::{mpsc, oneshot, watch};
use tokio_util::sync::CancellationToken;

use crate::taosz::{Header, ZCodec};

pub async fn test_taos_core_tmq2local() -> anyhow::Result<()> {

    // let dsn = "taos://192.168.2.131:6030";
    let cancel = CancellationToken::new();
    // tmq_to_local(Some("999"), "", "", cancel).await?;

    Ok(())
}

pub async fn test_tmq() -> anyhow::Result<()> {
    let db = "ts5820";
    let addr = "localhost:6030";
    // let addr = "172.18.0.2:6030";
    let _ = tokio::spawn(async move {
        let _ = subscribe(&format!("taos://{}", addr), db, "test").await;
    });

    // let _ = tokio::spawn(async move {
    //     match tmq2local(db, addr).await {
    //         Ok(_) => {
    //             println!("tmq2local success");
    //         }
    //         Err(e) => {
    //             println!("tmq2local error: {:?}", e);
    //         }
    //     }
    // });

    tokio::time::sleep(Duration::from_secs(5)).await;
    producer(&format!("taos://{}", addr), db, 10000000).await?;

    Ok(())
}


// tmq producer, 持续向 td 里写数据即可
pub async fn producer(dsn: &str, db: &str, limit: usize) -> anyhow::Result<()> {
    // let dsn = "taos://192.168.2.131:6030";
    //let dsn = "taos://127.0.0.1:6030";
    // let db = "ts5820";

    let pool = taos::TaosBuilder::from_dsn(dsn)?.pool()?;

    let taos = pool.get().await?;

    taos.exec_many([
        format!("USE `{db}`"),
        // "create stable meters(ts timestamp, id int, voltage int, v_blob blob) tags(groupid int, location varchar(24));".to_string(),
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
        // let item = format!("\\x{}", "255044462D312E330D0A".repeat(1000 * 25));
        let item = format!("\\x{}", "255044462D312E330D0A");
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

async fn prepare(taos: &taos::Taos) -> anyhow::Result<()> {
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


pub async fn subscribe(dsn: &str, db: &str, group_id: &str) -> anyhow::Result<()> {
    // std::env::set_var("RUST_LOG", "debug");
    pretty_env_logger::init();
    // let dsn = "taos://localhost:6030";
    let builder = taos::TaosBuilder::from_dsn(dsn)?;

    let taos = builder.build().await?;
    // let db = "ts5820";

    let topic = format!("tmq2_{db}");
    // prepare database
    taos.exec_many([
        format!("DROP TOPIC IF EXISTS {topic}"),
        format!("DROP DATABASE IF EXISTS `{db}`"),
        format!("CREATE DATABASE `{db}`"),
        format!("USE `{db}`"),
        // create super table
        // "CREATE TABLE `meters` (`ts` TIMESTAMP, `current` FLOAT, `voltage` INT, `phase` FLOAT) TAGS (`groupid` INT, `location` BINARY(16))".to_string(),
        // "create stable meters(ts timestamp, id int, voltage int, v_blob blob) tags(groupid int, location varchar(24));".to_string(),
        "create stable meters(ts timestamp, id int, voltage int, v_blob varchar(1024)) tags(groupid int, location varchar(24));".to_string(),
        // create topic for subscription
        format!("CREATE TOPIC {topic} with META AS DATABASE {db}")
    ])
    .await?;

    // let task = tokio::spawn(prepare(taos));

    tokio::time::sleep(Duration::from_secs(1)).await;

    // subscribe
    // let tmq = TmqBuilder::from_dsn("taos://localhost:6030/?group.id=test")?;
    let tmq = TmqBuilder::from_dsn(format!("{}?group.id={}&auto.offset.reset=earliest", dsn, group_id))?;

    let mut consumer = tmq.build().await?;
    consumer.subscribe([&topic]).await?;

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

    println!("done");

    Ok(())
}

async fn tmq2local(db: &str, addr: &str) -> anyhow::Result<()> {
    let taos = TaosBuilder::from_dsn(format!("taos://{addr}"))?.build().await?;
    // pretty_env_logger::formatted_builder().filter_level(log::LevelFilter::Debug);
    let topic = format!("tmq2_{db}");
    taos.exec_many([
        format!("drop topic if exists `{topic}`"),
        format!("DROP DATABASE IF EXISTS `{db}`"),
        format!("CREATE DATABASE `{db}`"),
        format!("USE `{db}`"),
        // "create stable meters(ts timestamp, id int, voltage int, v_blob blob) tags(groupid int, location varchar(24));".to_string(),
        "create stable meters(ts timestamp, id int, voltage int, v_blob varchar(1024)) tags(groupid int, location varchar(24));".to_string(),
        format!("create topic `{topic}` with meta as database {db}"),
        format!("use `{db}`"),
    ])
    .await?;

    let file_name = format!("{db}.test.z");
    // let writer = std::fs::File::create("abc1.test.z")?;
    let writer = tokio::fs::File::create(&file_name).await?;

    println!("db: {}, file_name: {}", db, file_name);
    
    let writer = async_compression::tokio::write::ZstdEncoder::new(writer);
    let mut writer = ZCodec::new(writer);
    // let writer =
    // let db = "abc1";
    writer
        .write_head_async(&Header::new("1.6.0", "3.3.0.0", db.to_string()))
        .await?;
    let mut consumer = TmqBuilder::from_dsn(format!("taos://{addr}?group.id={}&auto.offset.reset=earliest", "tmq_g2"))?.build().await?;
    consumer.subscribe([&topic]).await?;
    let writer = Arc::new(tokio::sync::Mutex::new(writer));
    
    let rows = consumer
        .stream_with_timeout(Timeout::from_millis(500))
        .map_err(anyhow::Error::from)
        .map_ok(|(offset, message)| async {
            let mut rows = 0;
            let mut writer = writer.lock().await;
            match message {
                MessageSet::Meta(meta) => {
                    dbg!("xxxzgc meta:", meta.as_json_meta().await?);
                    writer
                        .write_meta_async(&meta.as_raw_meta().await?)
                        .await
                        .unwrap();
                }
                MessageSet::Data(data) => {
                    writer.start_data_async().await.unwrap();
                    while let Some(block) = data.fetch_raw_block().await.unwrap() {
                        // dbg!(&block);
                        let _len = writer.write_data_async(&block).await.unwrap();
                        rows += block.nrows();
                        println!(
                            "table {} rows: {}",
                            block.table_name().unwrap(),
                            block.nrows());
                        // dbg!(len);
                        // tracing::info!("");
                        // tracing::info!(
                        //     "table {} rows: {}",
                        //     block.table_name().unwrap(),
                        //     block.nrows()
                        // );
                    }
                    writer.finish_data_async().await.unwrap();
                }
                _ => unreachable!(),
            }
            writer.flush().await.unwrap();
            consumer.commit(offset).await?;
            anyhow::Result::<usize>::Ok(rows)
        })
        .try_fold(0, |sum, n| async move { Ok(n.await? + sum) })
        .await?;
    let mut writer = writer.lock().await;
    writer.flush().await?;
    writer.shutdown().await?;

    // let mut bytes = Vec::with_capacity(10000);
    // bytes.resize(10000, 0xffu8);
    // writer.write_all(&bytes).await?;
    // writer.deref_mut().shutdown().await?;
    println!("backup {} rows in database {}", rows, db);

    Ok(())
}

// #[tokio::test]
pub async fn test_poll_with_sleep() -> anyhow::Result<()> {
    use taos_query::prelude::*;

    let taos = TaosBuilder::from_dsn("ws://localhost:6041")?
        .build()
        .await?;

    taos.exec_many([
        "drop topic if exists topic_1748512568",
        "drop database if exists test_1748512568",
        "create database test_1748512568 vgroups 10",
        "create topic topic_1748512568 as database test_1748512568",
        "use test_1748512568",
        "create table t0 (ts timestamp, c1 int, c2 float, c3 float)",
    ])
    .await?;

    let num = 3000;

    let (msg_tx, mut msg_rx) =
        mpsc::channel::<(MessageSet<Meta, Data>, oneshot::Sender<()>)>(100);

    let (cancel_tx, mut cancel_rx) = watch::channel(false);

    let cnt_handle: tokio::task::JoinHandle<anyhow::Result<()>> = tokio::spawn(async move {
        let mut cnt = 0;
        while let Some((mut msg, done_tx)) = msg_rx.recv().await {
            if let Some(data) = msg.data() {
                // while let Some(block) = data.fetch_block().await? {
                while let Some(block) = data.fetch_raw_block().await? {
                    cnt += block.nrows();
                }
            }
            let _ = done_tx.send(());
        }
        assert_eq!(cnt, num);
        Ok(())
    });

    let poll_handle: tokio::task::JoinHandle<anyhow::Result<()>> = tokio::spawn(async move {
        let tmq =
            TmqBuilder::from_dsn("ws://localhost:6041?group.id=10&auto.offset.reset=earliest")?;
        let mut consumer = tmq.build().await?;
        consumer.subscribe(["topic_1748512568"]).await?;

        let timeout = Timeout::Duration(Duration::from_secs(1));

        loop {
            tokio::select! {
                _ = cancel_rx.changed() => {
                    break;
                }
                res = consumer.recv_timeout(timeout) => {
                    if let Some((offset, message)) = res? {
                        let (done_tx, done_rx) = oneshot::channel();
                        msg_tx.send((message, done_tx)).await?;
                        let _ = done_rx.await;
                        consumer.commit(offset).await?;
                    }
                }
            }
        }

        consumer.unsubscribe().await;

        Ok(())
    });

    let mut sqls = Vec::with_capacity(100);

    let ts = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64;

    for i in 0..num {
        sqls.push(format!(
            "insert into t0 values ({}, {}, {}, {})",
            ts + i as i64,
            i,
            i as f32 * 1.1,
            i as f32 * 2.2
        ));

        if (i + 1) % 100 == 0 {
            taos.exec_many(&sqls).await?;
            sqls.clear();
            tokio::time::sleep(Duration::from_secs(2)).await;
        }
    }

    tokio::time::sleep(Duration::from_secs(20)).await;

    let _ = cancel_tx.send(true);

    poll_handle.await??;
    cnt_handle.await??;

    tokio::time::sleep(Duration::from_secs(3)).await;

    taos.exec_many([
        "drop topic topic_1748512568",
        "drop database test_1748512568",
    ])
    .await?;

    Ok(())
}