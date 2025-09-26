use jira_case::{gen_data::{td_blob_data_big, td_blob_data_small}, ts5820::{kafka::kafka_main, subscribe, tmq::test_tmq}};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // td_blob_data_big().await?;
    // test_tmq().await?;
    // kafka_main().await?;
    subscribe::subscribe().await?;
    Ok(())
}
