use anyhow::Ok;
use taos::*;


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let db = "test";
    let topic = "tmq_meters";
    let group_id = 1;

    let dsn = "ws://localhost:6041";
    let taos = TaosBuilder::from_dsn(dsn)?.build().await?;

    taos.exec_many([
        &format!("drop topic if exists {topic}"),
        &format!("drop database if exists {db}"),
        &format!("create database {db}"),
        // &format!("create topic {topic} as database {db}"),
        &format!("use {db}"),

    ]).await?;
    taos.exec_many([
        // create super table
        "CREATE TABLE `meters` (`ts` TIMESTAMP, `current` FLOAT, `voltage` INT, `phase` FLOAT) \
         TAGS (`groupid` INT, `location` BINARY(16))",
        // create child table
        "CREATE TABLE `d0` USING `meters` TAGS(0, 'Los Angles')",
        // create topic
        &format!("create topic {topic} as stable meters"),
        "INSERT INTO `d0` values(now - 10s, 10, 116, 0.32)",
        "INSERT INTO `d1` USING `meters` TAGS(1, 'San Francisco') values(now - 9s, 10.1, 119, 0.33)",
    ]).await?;

    let dsn = format!("ws://localhost:6041?group.id={group_id}&auto.offset.reset=earliest");
    let mut consumer = TmqBuilder::from_dsn(dsn)?.build().await?;

    let topics = consumer.list_topics().await?;
    println!("Topics: {topics:?}");

    let assignments = consumer.assignments().await;
    println!("Assignments: {assignments:?}");

    consumer.subscribe(["tmq_meters"]).await?;

    for _ in 0..3 {
        if let Some((offset, msg)) = consumer.recv_timeout(taos::Timeout::Never).await? {
            println!("Offset: {offset:?}, msg: {msg:?}");

            let vgroup_id = offset.vgroup_id();

            let committed = consumer.committed(topic, vgroup_id).await?;
            println!("Committed: {committed:?}");

            let position = consumer.position(topic, vgroup_id).await?;
            println!("Position: {position:?}");

            consumer.commit(offset).await?;
        }
    }

    consumer.unsubscribe().await;

    Ok(())
}
