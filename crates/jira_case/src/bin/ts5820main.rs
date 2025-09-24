use jira_case::gen_data::{td_blob_data_big, td_blob_data_small};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    td_blob_data_big().await?;
    Ok(())
}
