mod offen;
use jira_case::offen;



#[tokio::main]
async fn main() -> anyhow::Result<()> {
    offen::mysql::mysql_main().await?;
    Ok(())
}