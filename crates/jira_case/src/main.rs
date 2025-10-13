use jira_case::{td38264, ts7433};


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // jira_case::offen::mysql::mysql_main().await?;
    // ts7433::ts7433_breakpoint_main()?;
    td38264::td38264_main()?;
    Ok(())
}