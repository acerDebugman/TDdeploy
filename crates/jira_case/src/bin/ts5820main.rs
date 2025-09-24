use jira_case::{gen_data::td_blob_data};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    td_blob_data().await?;
    Ok(())
}
