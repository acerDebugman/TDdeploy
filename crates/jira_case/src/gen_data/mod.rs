use chrono::{DateTime, Local};
use taos::*;


pub async fn td_blob_data_small() -> anyhow::Result<()> {
    let dsn = "taos://192.168.2.131:6030";
    //let dsn = "taos://127.0.0.1:6030";

    let pool = TaosBuilder::from_dsn(dsn)?.pool()?;

    let taos = pool.get().await?;

    let db = "ts5820";

    // prepare database
    taos.exec_many([
        format!("DROP DATABASE IF EXISTS `{db}`"),
        format!("CREATE DATABASE `{db}`"),
        format!("USE `{db}`"),
    ])
    .await?;

    let inserted = taos.exec_many([
        // create super table
        // "CREATE TABLE `meters` (`ts` TIMESTAMP, `current` FLOAT, `voltage` INT, `phase` FLOAT) TAGS (`groupid` INT, `location` BINARY(16))",
        "create stable meters(ts timestamp, id int, voltage int, v_blob blob) tags(groupid int, location varchar(24));",
        // create child table
        // "CREATE TABLE `d0` USING `meters` TAGS(0, 'Los Angles')",
        //r#"INSERT INTO `t1` using `meters` (`groupid`,`location`) tags(1,"BJ") (`ts`,`id`,`voltage`,`v_blob`) values(1751538339000,1,11,'0x255044462D312E330D0A')"#,
        r#"INSERT INTO `t1` using `meters` (`groupid`,`location`) tags(1,"BJ") (`ts`,`id`,`voltage`,`v_blob`) values(now,1,11,'0x255044462D312E330D0A')"#,
        // insert into child table
        // insert with NULL values
        // insert many records in a single sql
    ]).await?;

    println!("inserted: {:?}", inserted);

    // assert_eq!(inserted, 6);
    loop {
        let count: usize = taos
            .query_one("select count(*) from `meters`")
            .await?
            .unwrap_or_default();

        println!("count: {:?}", count); 

        if count >= inserted {
            break;
        } else {
            println!("waiting for data");
        }
    }

    let mut result = taos.query("select tbname, * from `meters`").await?;

    for field in result.fields() {
        println!("got field: {}", field.name());
    }

    // Query option 1, use rows stream.
    let mut rows = result.rows();
    let mut nrows = 0;
    while let Some(row) = rows.try_next().await? {
        for (col, (name, value)) in row.enumerate() {
            println!(
                "[{}] got value in col {} (named `{:>8}`): {}",
                nrows, col, name, value
            );
        }
        nrows += 1;
    }

    // Query options 2, use deserialization with serde.
    #[derive(Debug, serde::Deserialize)]
    #[allow(dead_code)]
    struct Record {
        tbname: String,
        // deserialize timestamp to chrono::DateTime<Local>
        ts: DateTime<Local>,
        // float to f32
        current: Option<f32>,
        // int to i32
        voltage: Option<i32>,
        phase: Option<f32>,
        groupid: i32,
        // binary/varchar to String
        location: String,
    }

    let records: Vec<Record> = taos
        .query("select tbname, * from `meters`")
        .await?
        .deserialize()
        .try_collect()
        .await?;

    dbg!(result.summary());
    assert_eq!(records.len(), 6);
    dbg!(records);
    Ok(())
}

pub async fn td_blob_data_big() -> anyhow::Result<()> {
    // let dsn = "taos://192.168.2.131:6030";
    // let dsn = "taos://172.18.0.2:6030";
    let dsn = "taos://127.0.0.1:6030";

    let pool = TaosBuilder::from_dsn(dsn)?.pool()?;

    let taos = pool.get().await?;

    let db = "ts5820";

    // prepare database
    taos.exec_many([
        format!("DROP DATABASE IF EXISTS `{db}`"),
        format!("CREATE DATABASE `{db}`"),
        format!("USE `{db}`"),
    ])
    .await?;

    // 6M = 20 * 1000 * 300
    //let item = format!("\\x{}", "255044462D312E330D0A".repeat(1000*300));
    // 2M = 20 * 1000 * 100
    //let item = format!("\\x{}", "255044462D312E330D0A".repeat(1000*100));
    let item = format!("{}", "255044462D312E330D0A".repeat(1000*100));

    // 1M = 20 * 1000 * 50
    //let item = format!("\\x{}", "255044462D312E330D0A".repeat(1000*50));
    //let item = format!("{}", "255044462D312E330D0A".repeat(1000*50));
   
    //let item = format!("\\x{}", "255044462D312E330D0A".repeat(1000*20));
    //let item = format!("{}", "255044462D312E330D0A".repeat(1000*20));
    println!("item len: {}", item.len());
    let inserted = taos.exec_many([
        // create super table
        // "CREATE TABLE `meters` (`ts` TIMESTAMP, `current` FLOAT, `voltage` INT, `phase` FLOAT) TAGS (`groupid` INT, `location` BINARY(16))",
        "create stable meters(ts timestamp, id int, voltage int, v_blob blob) tags(groupid int, location varchar(24));",
        // create child table
        // "CREATE TABLE `d0` USING `meters` TAGS(0, 'Los Angles')",
        //&format!("INSERT INTO `t1` using `meters` (`groupid`,`location`) tags(1,\"BJ\") (`ts`,`id`,`voltage`,`v_blob`) values(1751538339000,1,11,'{}')", item),
        &format!("INSERT INTO `t1` using `meters` (`groupid`,`location`) tags(1,\"BJ\") (`ts`,`id`,`voltage`,`v_blob`) values(now,1,11,'{}')", item),
        // insert into child table
        // insert with NULL values
        // insert many records in a single sql
    ]).await?;

    println!("inserted: {:?}", inserted);

    // assert_eq!(inserted, 6);
    loop {
        let count: usize = taos
            .query_one("select count(*) from `meters`")
            .await?
            .unwrap_or_default();

        println!("count: {:?}", count); 

        if count >= inserted {
            break;
        } else {
            println!("waiting for data");
        }
    }

    let mut result = taos.query("select tbname, * from `meters`").await?;

    for field in result.fields() {
        println!("got field: {}", field.name());
    }

    // Query option 1, use rows stream.
    let mut rows = result.rows();
    let mut nrows = 0;
    while let Some(row) = rows.try_next().await? {
        for (col, (name, value)) in row.enumerate() {
            println!(
                "[{}] got value in col {} (named `{:>8}`): {}",
                nrows, col, name, value
            );
        }
        nrows += 1;
    }

    // Query options 2, use deserialization with serde.
    #[derive(Debug, serde::Deserialize)]
    #[allow(dead_code)]
    struct Record {
        tbname: String,
        // deserialize timestamp to chrono::DateTime<Local>
        ts: DateTime<Local>,
        // float to f32
        current: Option<f32>,
        // int to i32
        voltage: Option<i32>,
        phase: Option<f32>,
        groupid: i32,
        // binary/varchar to String
        location: String,
    }

    let records: Vec<Record> = taos
        .query("select tbname, * from `meters`")
        .await?
        .deserialize()
        .try_collect()
        .await?;

    dbg!(result.summary());
    assert_eq!(records.len(), 6);
    dbg!(records);
    Ok(())
}


pub async fn td_blob_data_big_rawblock() -> anyhow::Result<()> {
    // let dsn = "taos://192.168.2.131:6030";
    // let dsn = "taos://172.18.0.2:6030";
    let dsn = "taos://127.0.0.1:6030";

    let pool = TaosBuilder::from_dsn(dsn)?.pool()?;

    let taos = pool.get().await?;

    let db = "ts5820";

    // prepare database
    taos.exec_many([
        format!("DROP DATABASE IF EXISTS `{db}`"),
        format!("CREATE DATABASE `{db}`"),
        format!("USE `{db}`"),
    ])
    .await?;

    // 6M = 20 * 1000 * 300
    //let item = format!("\\x{}", "255044462D312E330D0A".repeat(1000*300));
    // 2M = 20 * 1000 * 100
    //let item = format!("\\x{}", "255044462D312E330D0A".repeat(1000*100));
    let item = format!("{}", "255044462D312E330D0A".repeat(1000*100));

    // 1M = 20 * 1000 * 50
    //let item = format!("\\x{}", "255044462D312E330D0A".repeat(1000*50));
    //let item = format!("{}", "255044462D312E330D0A".repeat(1000*50));
   
    //let item = format!("\\x{}", "255044462D312E330D0A".repeat(1000*20));
    //let item = format!("{}", "255044462D312E330D0A".repeat(1000*20));
    println!("item len: {}", item.len());
    let inserted = taos.exec_many([
        // create super table
        // "CREATE TABLE `meters` (`ts` TIMESTAMP, `current` FLOAT, `voltage` INT, `phase` FLOAT) TAGS (`groupid` INT, `location` BINARY(16))",
        "create stable meters(ts timestamp, id int, voltage int, v_blob blob) tags(groupid int, location varchar(24));",
        // create child table
        // "CREATE TABLE `d0` USING `meters` TAGS(0, 'Los Angles')",
        //&format!("INSERT INTO `t1` using `meters` (`groupid`,`location`) tags(1,\"BJ\") (`ts`,`id`,`voltage`,`v_blob`) values(1751538339000,1,11,'{}')", item),
        &format!("INSERT INTO `t1` using `meters` (`groupid`,`location`) tags(1,\"BJ\") (`ts`,`id`,`voltage`,`v_blob`) values(now,1,11,'{}')", item),
        // insert into child table
        // insert with NULL values
        // insert many records in a single sql
    ]).await?;

    println!("inserted: {:?}", inserted);

    // assert_eq!(inserted, 6);
    loop {
        let count: usize = taos
            .query_one("select count(*) from `meters`")
            .await?
            .unwrap_or_default();

        println!("count: {:?}", count); 

        if count >= inserted {
            break;
        } else {
            println!("waiting for data");
        }
    }

    let mut result = taos.query("select tbname, * from `meters`").await?;

    for field in result.fields() {
        println!("got field: {}", field.name());
    }

    // Query option 1, use rows stream.
    let mut rows = result.rows();
    let mut nrows = 0;
    while let Some(row) = rows.try_next().await? {
        for (col, (name, value)) in row.enumerate() {
            println!(
                "[{}] got value in col {} (named `{:>8}`): {}",
                nrows, col, name, value
            );
        }
        nrows += 1;
    }

    // Query options 2, use deserialization with serde.
    #[derive(Debug, serde::Deserialize)]
    #[allow(dead_code)]
    struct Record {
        tbname: String,
        // deserialize timestamp to chrono::DateTime<Local>
        ts: DateTime<Local>,
        // float to f32
        current: Option<f32>,
        // int to i32
        voltage: Option<i32>,
        phase: Option<f32>,
        groupid: i32,
        // binary/varchar to String
        location: String,
    }

    let records: Vec<Record> = taos
        .query("select tbname, * from `meters`")
        .await?
        .deserialize()
        .try_collect()
        .await?;

    dbg!(result.summary());
    assert_eq!(records.len(), 6);
    dbg!(records);
    Ok(())
}
