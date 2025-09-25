use taos::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // let builder= TaosBuilder::from_dsn("taos://localhost:6030")?;
    let builder= TaosBuilder::from_dsn("taos://localhost:6041")?;
        
    let taos = builder.build().await?;

    let db = "test";

    taos.exec_many([
        format!("DROP DATABASE IF EXISTS `{db}`"),
        format!("CREATE DATABASE `{db}`"),
        format!("USE `{db}`"),
    ])
    .await?;

    let inserted = taos.exec_many([
        // create super table
        "CREATE TABLE `meters` (`ts` TIMESTAMP, `current` FLOAT, `voltage` INT, `phase` FLOAT) \
         TAGS (`groupid` INT, `location` BINARY(16))",
        // create child table
        "CREATE TABLE `d0` USING `meters` TAGS(0, 'Los Angles')",
    ]).await?;
    assert_eq!(inserted, 0);

    let bytes = include_bytes!("../../tests/test.txt");

    let mut block = RawBlock::parse_from_raw_block_v2(
        bytes.as_slice(),
        &[
            Field::new("ts", Ty::Timestamp, 8),
            Field::new("current", Ty::Float, 4),
            Field::new("voltage", Ty::Int, 4),
            Field::new("phase", Ty::Float, 4),
            Field::new("group_id", Ty::Int, 4),
            Field::new("location", Ty::VarChar, 16),
        ],
        &[8, 4, 4, 4, 4, 18],
        10,
        Precision::Millisecond,
    );
    // block.with_table_name("meters");
    block.with_table_name("d0");
    dbg!(&block);


    #[derive(Debug, serde::Deserialize)]
    #[allow(dead_code)]
    struct Record {
        ts: String,
        current: f32,
        voltage: i32,
        phase: f32,
        group_id: i32,
        location: String,
    }
    let rows: Vec<Record> = block.deserialize().try_collect().unwrap();
    dbg!(rows);

    taos.write_raw_block(&block).await?;

    Ok(())
}