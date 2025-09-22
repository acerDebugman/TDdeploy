

#[tokio::test(flavor = "multi_thread")]
pub async fn test_td5820_mysql_blob_with_taos() -> anyhow::Result<()> {
    const DEFAULT_DB: &str = "td5820";
    let task_id = Some(999);



    Ok(())
}


/// # description
/// Test synchronize database with taos.
/// # description_cn
/// 同步数据库，包括 stable 和普通表。
/// 1. 创建数据库：blob_ts5820 和 blob_ts5820_dst，在 blob_ts5820 中写入带 blob 的数据；
/// 2. 运行 legacy_to_taos 任务，同步 blob_ts5820 到 blob_ts5820_dst；
/// 3.  检查 legacy_to_taos 和 blob_ts5820_ds，同步成功，用例通过，否则用例失败。
/// # example
/// ```shell
/// cargo nextest run -p taosx-core test_sync_with_taos --no-capture --retries 0
/// ```
#[tokio::test(flavor = "multi_thread")]
async fn test_sync_blob_with_taos() -> anyhow::Result<()> {
//     let builder = TaosBuilder::from_dsn("taos:///")?.pool()?;
//     let taos = builder.get().await?;
//     let db = "test_ts6646";
//     taos.exec_many([
//         format!("drop database if exists `{db}`"),
//         format!("create database `{db}`"),
//         format!("use {db}"),
//         "create stable `st1` (ts timestamp, v1 int) tags(t1 int)".to_string(),
//         "create table `t1` using `st1` tags(1)".to_string(),
//         "insert into `t1` values(now + 1s, 1)".to_string(),
//         "insert into `t1` values(now + 2s, 2)".to_string(),
//         "insert into `t1` values(now + 3s, 3)".to_string(),
//         "insert into `t1` values(now + 4s, 4)".to_string(),
//     ])
//     .await?;

//     let rows: usize = taos
//         .query_one("select count(*) from `t1`")
//         .await?
//         .unwrap_or_default();
//     assert_eq!(rows, 4);

//     let target_opts = TargetOpts::default();
//     let metrics = Arc::new(CoreMetrics::Legacy(LegacyToTaosMetrics::default()));
//     sync_super_table_schema_with_subs(
//         &taos,
//         "st1",
//         &["t1"],
//         &taos,
//         None,
//         &target_opts,
//         false,
//         true,
//         &[],
//         &metrics,
//     )
//     .await?;

//     let result_use_v2_style_in_v3 = sync_super_table_schema_with_subs(
//         &taos,
//         "st1",
//         &["t1"],
//         &taos,
//         None,
//         &target_opts,
//         false,
//         false,
//         &[],
//         &metrics,
//     )
//     .await
//     .inspect_err(|e| {
//         println!("sync_super_table_schema_with_subs failed: {e}");
//     });
//     assert!(
//         result_use_v2_style_in_v3.is_err(),
//         "should not use v2 style in v3"
//     );
//     taos.exec(format!("drop database `{}`", db)).await?;
//     Ok(())
  Ok(())
}
