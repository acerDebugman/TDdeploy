use std::sync::Arc;

use arsenal::taoz::{Header, ZCodec, ZMessage};
use taos::*;
use tokio::io::AsyncWriteExt;


#[tokio::main]
pub async fn main() -> anyhow::Result<()> {


    Ok(())
}

async fn write() -> anyhow::Result<()> {
    let taos = TaosBuilder::from_dsn("taos:///")?.build().await?;
    pretty_env_logger::formatted_builder().filter_level(log::LevelFilter::Debug);
    taos.exec_many([
        "drop topic if exists abc1",
        "create topic abc1 with meta as database abc1",
        "use abc1",
    ])
    .await?;

    // let writer = std::fs::File::create("abc1.test.z")?;
    let writer = tokio::fs::File::create("abc1.test.bin").await?;

    let writer = async_compression::tokio::write::ZstdEncoder::new(writer);
    let mut writer = ZCodec::new(writer);
    // let writer =
    let db = "abc1";
    writer
        .write_head_async(&Header::new("1.6.0", "3.3.0.0", db.to_string()))
        .await?;

    let mut tmq = TmqBuilder::from_dsn("taos:///?group.id=c")?.build().await?;
    tmq.subscribe([db]).await?;
    let writer = Arc::new(tokio::sync::Mutex::new(writer));

    let rows = tmq
        .stream_with_timeout(Timeout::from_millis(500))
        .map_err(anyhow::Error::from)
        .map_ok(|(offset, message)| async {
            let mut rows = 0;
            let mut writer = writer.lock().await;
            match message {
                MessageSet::Meta(meta) => {
                    // dbg!(meta.as_json_meta().await?);
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
                        // dbg!(len);
                        // tracing::info!("");
                        tracing::info!(
                            "table {} rows: {}",
                            block.table_name().unwrap(),
                            block.nrows()
                        );
                    }
                    writer.finish_data_async().await.unwrap();
                }
                _ => unreachable!(),
            }
            writer.flush().await.unwrap();
            tmq.commit(offset).await?;
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

async fn read() -> anyhow::Result<()> {
    let taos = TaosBuilder::from_dsn("taos:///")?.build().await?;
    taos.exec_many([
        "drop database if exists abc3",
        "create database if not exists abc3",
        "use abc3",
    ])
    .await?;

    let reader = tokio::fs::File::open("abc1.test.bin").await?;
    let reader = tokio::io::BufReader::new(reader);

    let reader = async_compression::tokio::bufread::ZstdDecoder::new(reader);

    let mut reader = ZCodec::new(reader);

    let header = reader.header_async().await?;
    dbg!(header);

    // let mut rows = AtomicU64::new(0);
    let mut rows = 0;

    loop {
        let res = reader.read_message_async().await;
        match res {
            Ok(message) => match message {
                ZMessage::Meta(meta) => taos.write_raw_meta(&meta).await?,
                ZMessage::Data(data) => {
                    // dbg!(&data);
                    for raw in data {
                        rows += raw.nrows();
                        taos.write_raw_block(&raw).await?;
                    }
                    println!("rows: {}", rows);
                    // taos.write_raw_data(data[0]).await?
                }
                ZMessage::Raw(_raw_type, raw) => taos.write_raw_meta(&raw).await?,
            },
            Err(err) => {
                dbg!(&err);
                if err.kind() == std::io::ErrorKind::UnexpectedEof {
                    break;
                }
                dbg!(&err);
                break;
            }
        }
    }
    println!("total {} rows", rows);
    Ok(())
}