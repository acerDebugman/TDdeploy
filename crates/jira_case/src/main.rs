

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    jira_case::offen::mysql::mysql_main().await?;
    Ok(())
}