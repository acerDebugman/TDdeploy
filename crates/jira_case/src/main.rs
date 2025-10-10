use jira_case::ts7433;


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // jira_case::offen::mysql::mysql_main().await?;
    ts7433::ts7433_breakpoint_main()?;
    Ok(())
}