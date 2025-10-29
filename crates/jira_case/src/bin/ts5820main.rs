use jira_case::offen::kafka::kafka_main;


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // td_blob_data_big().await?;
    // test_tmq().await?;
    kafka_main().await?;
    // subscribe::subscribe().await?;
    // test_poll_with_sleep().await?;
    //loop_data().await?;
    Ok(())
}
