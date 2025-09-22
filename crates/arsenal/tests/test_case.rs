// #[tokio::test]
// async fn db_not_exist_abnormal() -> anyhow::Result<()> {
//     let config = r#"{
//         "global": {
//             "database_not_exist": "archive"
//         },
//         "mutate": [],
//         "model": []
//     }"#;
//     let parser = serde_json::from_str::<Parser>(config)?;
//     let batch = arrow::array::record_batch!(
//         ("a", Int32, [1, 2, 3]),
//         ("b", Float64, [Some(4.0), None, Some(5.0)])
//     )?;
//     let (archive_tx, rx) = flume::unbounded();
//     handle_flat_abnormal(
//         ProcessOnAbnormalEnum::DatabaseNotExist(
//             &parser.global().process_on_abnormal.database_not_exist,
//         ),
//         &batch,
//         archive_tx.clone(),
//     )?;
//     let rs = rx.recv();
//     dbg!(&rs);
//     assert!(rs.is_ok());

//     let config = r#"{
//         "global": {
//             "database_not_exist": "break"
//         },
//         "mutate": [],
//         "model": []
//     }"#;
//     let parser = serde_json::from_str::<Parser>(config)?;
//     let rs = handle_flat_abnormal(
//         ProcessOnAbnormalEnum::DatabaseNotExist(
//             &parser.global().process_on_abnormal.database_not_exist,
//         ),
//         &batch,
//         archive_tx.clone(),
//     );
//     assert!(rs.is_err());
//     Ok(())
// }

// #[tokio::test]
// async fn db_conn_abnormal() -> anyhow::Result<()> {
//     let config = r#"{
//         "global": {
//             "database_connection_error": "cache"
//         },
//         "mutate": [],
//         "model": []
//     }"#;
//     let parser = serde_json::from_str::<Parser>(config)?;
//     let batch = arrow::array::record_batch!(
//         ("a", Int32, [1, 2, 3]),
//         ("b", Float64, [Some(4.0), None, Some(5.0)])
//     )?;
//     let (cache_tx, rx) = flume::unbounded();
//     handle_flat_abnormal(
//         ProcessOnAbnormalEnum::DatabaseConnectionError(
//             &parser
//                 .global()
//                 .process_on_abnormal
//                 .database_connection_error,
//         ),
//         &batch,
//         cache_tx.clone(),
//     )?;
//     let rs = rx.recv();
//     dbg!(&rs);
//     assert!(rs.is_ok());

//     let config = r#"{
//         "global": {
//             "database_connection_error": "break"
//         },
//         "mutate": [],
//         "model": []
//     }"#;
//     let parser = serde_json::from_str::<Parser>(config)?;
//     let rs = handle_flat_abnormal(
//         ProcessOnAbnormalEnum::DatabaseConnectionError(
//             &parser
//                 .global()
//                 .process_on_abnormal
//                 .database_connection_error,
//         ),
//         &batch,
//         cache_tx.clone(),
//     );
//     dbg!(&rs);
//     assert!(rs.is_err());
//     Ok(())
// }


// #[tokio::test]
// pub async fn test_handle_sql_too_long_with_taos_0() -> anyhow::Result<()> {
//     // let _ = tracing_subscriber::fmt::fmt().with_level(true).try_init();
//     tracing_subscriber::fmt::fmt()
//         .with_max_level(tracing::Level::TRACE)
//         .init();

//     // prepare
//     let host = std::env::var("HOST").unwrap_or("127.0.0.1".to_string());
//     let to = format!("taos://{host}/");

//     let pool = {
//         let builder = TaosBuilder::from_dsn(to.into_dsn()?)?;
//         let mut pool_config = builder.default_pool_config();
//         pool_config.timeouts.wait = Some(Duration::from_secs(15));
//         builder.with_pool_config(pool_config)?
//     };
//     let taos = pool.get().await?;

//     let db = "test_sql_too_long";
//     taos.exec_many([
//         format!("drop database if exists `{db}`"),
//         format!("create database `{db}`"),
//         format!("use {db}"),
//     ])
//     .await?;
//     let mut taos = Some(taos);

//     let task_id: i64 = 999;
//     let target_precision= taos::Precision::Millisecond;

//     let stable = String::from("taosx_task_t1");
//     let metrics = Arc::new(CoreMetrics::IPC(IpcMetrics::new(
//         stable, task_id, None,
//     )));
//     insert_metrics(task_id, metrics.clone()).await;

//     let cancel = CancellationToken::new();

//     let (archive_tx, archive_rx) = flume::bounded(0);
//     drop(archive_rx);

//     let mut qid = taoslog::utils::Span.get_qid().unwrap_or_else(Qid::init);
//     let mut max_lengths = HashMap::new();

//     let config = r#"{
//         "global": {
//             "database_connection_error": "break"
//         },
//         "mutate": [],
//         "model": []
//     }"#;
//     let parser = serde_json::from_str::<Parser>(config)?;

//     let batch = gen_record_batch(10)?;

//     let stable_meta = MessageTableMeta {
//         name: Arc::new("meters".into()),
//         using: None,
//         tags: None,
//     };
//     let opts = Arc::new(TableOptions::default());

//     let messages = vec![MessageArrowRecords {
//         table: stable_meta,
//         records: batch,
//         opts: opts,
//     }];

//     // prepare data
//     let success_n = handle_sql_too_long(
//         &pool,
//         &mut taos,
//         target_precision,
//         messages,
//         metrics.ipc(),
//         None,
//         &cancel,
//         &parser,
//         archive_tx.clone(),
//         &mut qid,
//         &mut max_lengths,
//     ).await?;

//     println!("success insert rows: {success_n}");

//     Ok(())
// }

// pub fn build_schema() -> Schema {
//     let mut metadata = HashMap::new();
//     metadata.insert(String::from("version"), String::from("1.0"));
//     metadata.insert(String::from("stream"), String::from("flat"));
//     metadata.insert(String::from("ack"), String::from("lush"));
//     let flat_columns = vec![
//         arrow::datatypes::Field::new(
//             "ts",
//             DataType::Timestamp(arrow::datatypes::TimeUnit::Millisecond, None),
//             false,
//         ),
//         arrow::datatypes::Field::new("id", DataType::Int32, false),
//         arrow::datatypes::Field::new("voltage", DataType::Float32, true),
//         arrow::datatypes::Field::new("v_blob", DataType::LargeBinary, true),
//         arrow::datatypes::Field::new("groupid", DataType::Int32, true),
//         arrow::datatypes::Field::new("location", DataType::Utf8, true),
//     ];

//     Schema::new(flat_columns).with_metadata(metadata)
// }

// pub fn gen_record_batch(n: usize) -> anyhow::Result<RecordBatch> {
//     let schema = build_schema();

//     let mut timestamp = TimestampMillisecondBuilder::new();
//     timestamp.append_value(chrono::Utc::now().timestamp_millis());
//     let mut id = Int32Builder::new();
//     id.append_value(1);
//     let mut voltage = Float32Builder::new();
//     voltage.append_value(3.3);
//     let mut v_blob = LargeBinaryBuilder::new();
//     v_blob.append_value(b"9".repeat(n));
//     // v_blob.append_value(123);
//     let mut group_id = Int32Builder::new();
//     group_id.append_value(3);
//     let mut location = StringBuilder::new();
//     location.append_value("AK");

//     let batch = RecordBatch::try_new(
//         Arc::new(schema),
//         vec![
//             Arc::new(timestamp.finish()),
//             Arc::new(id.finish()),
//             Arc::new(voltage.finish()),
//             Arc::new(v_blob.finish()),
//             Arc::new(group_id.finish()),
//             Arc::new(location.finish()),
//         ],
//     )?;

//     Ok(batch)
// }


//  #[tokio::test]
// pub async fn test_handle_sql_too_long_with_taos_1() -> anyhow::Result<()> {
//     // let _ = tracing_subscriber::fmt::fmt().with_level(true).try_init();
//     tracing_subscriber::fmt::fmt()
//         .with_max_level(tracing::Level::TRACE)
//         .init();

//     // prepare
//     let host = std::env::var("HOST").unwrap_or("127.0.0.1".to_string());
//     let to = format!("taos://{host}/");

//     let pool = {
//         let builder = TaosBuilder::from_dsn(to.into_dsn()?)?;
//         let mut pool_config = builder.default_pool_config();
//         pool_config.timeouts.wait = Some(Duration::from_secs(15));
//         builder.with_pool_config(pool_config)?
//     };
//     let taos = pool.get().await?;

//     let db = "test_sql_too_long";
//     taos.exec_many([
//         format!("drop database if exists `{db}`"),
//         format!("create database `{db}`"),
//         format!("use {db}"),
//     ])
//     .await?;
//     let mut taos = Some(taos);

//     let task_id: i64 = 999;
//     let target_precision= taos::Precision::Millisecond;

//     let stable = String::from("taosx_task_t1");
//     let metrics = Arc::new(CoreMetrics::IPC(IpcMetrics::new(
//         stable, task_id, None,
//     )));
//     insert_metrics(task_id, metrics.clone()).await;

//     let cancel = CancellationToken::new();

//     let (archive_tx, archive_rx) = flume::bounded(0);
//     drop(archive_rx);

//     let mut qid = taoslog::utils::Span.get_qid().unwrap_or_else(Qid::init);
//     let mut max_lengths = HashMap::new();

//     let config = r#"{
//         "global": {
//             "database_connection_error": "break"
//         },
//         "mutate": [],
//         "model": []
//     }"#;
//     let parser = serde_json::from_str::<Parser>(config)?;

//     let batch = gen_record_batch(10)?;

//     let using = Some(Arc::new(STable::Name("meters".into())));
//     let tags = Some(Arc::new(
//         arrow::array::record_batch!(
//                 ("groupid", Int32, [3]),
//                 ("location", Utf8, ["AK"])
//             )?
//     ));
//     let stable_meta = MessageTableMeta {
//         name: Arc::new("t3".into()),
//         using,
//         tags,
//     };
//     let opts = Arc::new(TableOptions::default());

//     let messages = vec![MessageArrowRecords {
//         table: stable_meta,
//         records: batch,
//         opts: opts,
//     }];

//     // prepare data
//     let success_n = handle_sql_too_long(
//         &pool,
//         &mut taos,
//         target_precision,
//         messages,
//         metrics.ipc(),
//         None,
//         &cancel,
//         &parser,
//         archive_tx.clone(),
//         &mut qid,
//         &mut max_lengths,
//     ).await?;

//     println!("success insert rows: {success_n}");

//     Ok(())
// }

// pub fn build_schema() -> Schema {
//     let mut metadata = HashMap::new();
//     metadata.insert(String::from("version"), String::from("1.0"));
//     metadata.insert(String::from("stream"), String::from("flat"));
//     metadata.insert(String::from("ack"), String::from("lush"));
//     let flat_columns = vec![
//         arrow::datatypes::Field::new(
//             "ts",
//             DataType::Timestamp(arrow::datatypes::TimeUnit::Millisecond, None),
//             false,
//         ),
//         arrow::datatypes::Field::new("id", DataType::Int32, false),
//         arrow::datatypes::Field::new("voltage", DataType::Float32, true),
//         arrow::datatypes::Field::new("v_blob", DataType::LargeBinary, true),
//         // arrow::datatypes::Field::new("groupid", DataType::Int32, true),
//         // arrow::datatypes::Field::new("location", DataType::Utf8, true),
//     ];

//     Schema::new(flat_columns).with_metadata(metadata)
// }

// pub fn gen_record_batch(n: usize) -> anyhow::Result<RecordBatch> {
//     let schema = build_schema();

//     let mut timestamp = TimestampMillisecondBuilder::new();
//     timestamp.append_value(chrono::Utc::now().timestamp_millis());
//     timestamp.append_value(chrono::Utc::now().timestamp_millis() + 1);
//     let mut id = Int32Builder::new();
//     id.append_value(1);
//     id.append_value(2);
//     let mut voltage = Float32Builder::new();
//     voltage.append_value(3.3);
//     voltage.append_value(9.3);
//     let mut v_blob = LargeBinaryBuilder::new();
//     v_blob.append_value(b"9".repeat(n));
//     v_blob.append_value(b"8".repeat(n));
//     // v_blob.append_value(123);
//     // let mut group_id = Int32Builder::new();
//     // group_id.append_value(3);
//     // let mut location = StringBuilder::new();
//     // location.append_value("AK");

//     let batch = RecordBatch::try_new(
//         Arc::new(schema),
//         vec![
//             Arc::new(timestamp.finish()),
//             Arc::new(id.finish()),
//             Arc::new(voltage.finish()),
//             Arc::new(v_blob.finish()),
//             // Arc::new(group_id.finish()),
//             // Arc::new(location.finish()),
//         ],
//     )?;

//     Ok(batch)
// }
