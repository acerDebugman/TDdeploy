// use std::{collections::HashMap, env, fmt::format, path::Path, sync::Arc, time::Duration };
// use anyhow::Context;
// use arrow::{array::{BinaryBuilder, Float32Builder, Int32Builder, Int64Builder, LargeBinaryBuilder, RecordBatch, StringBuilder, TimestampMillisecondBuilder, TimestampNanosecondBuilder}, datatypes::{DataType, Schema}, ipc::writer::StreamWriter};
// use serde_json::json;
// use taos::*;
// use rand::Rng;
// use assert_cmd::Command;
// use taosx_core::{build_ipc, core_metrics::{get_metrics, insert_metrics, load_metrics, CoreMetrics, TaskMetrics}, plugins::transform::TableOptions, runners::{kafka::KAFKA_ID, set_tcp_keepalive}, sink::ipc_metric::IpcMetrics, utils::port_pool::PortPool, Parser, TaskNotifySender};
// use taosx_ipc::ack::AckReaderBuilder;
// use tokio::task::JoinSet;
// use tokio_util::sync::CancellationToken;


// /// # description_cn
// /// 测试发生 0x0388 Database not exists 错误时,数据是否按照 abnormal 处理策略进行处理
// /// 1. 配置abnormal处理策略为 archive 等参数,并开启 sink 端
// /// 2. 发送模拟数据
// /// 3. 确认数据写入
// /// 4. drop database
// /// 5. 确认数据进行 archive
// /// ```shell
// /// BACKUP_DSN=tmq+ws://192.168.2.139:6041/log cargo nextest run test_backup_with_taos --nocapture --retries 0
// /// ```
// /// 
// /// # jira
// /// Close https://jira.taosdata.com:18080/browse/TD-36123
// /// # example
// /// ```shell
// /// cargo nextest run test_td36123_with_taos --nocapture --retries 0
// /// ```
// // #[tokio::test]
// #[tokio::test(flavor = "multi_thread")]
// pub async fn test_td5820_with_taos() -> anyhow::Result<()> {
//     const DEFAULT_DB: &str = "ts5820";
//     let task_id = Some(999);
//     let to_dsn = env::var("TO_DSN")
//         // .unwrap_or(format!("taos+ws://127.0.0.1:6030/{}", DEFAULT_DB))
//         // .unwrap_or(format!("taos+ws://172.18.0.2:6041/{}", DEFAULT_DB))
//         .unwrap_or(format!("taos://172.18.0.2:6030"))
//         .into_dsn()?;
//     // let taosx_cmd = env::var("TAOSX_CMD").unwrap_or("taosx".to_string());
//     let archive_dir = env::var("ARCHIVE_DIR")
//         .ok()
//         .map(|p| Path::new(&p).to_path_buf())
//         // .unwrap_or_else(|| tempfile::TempDir::new().unwrap().keep());
//         .unwrap_or_else(|| Path::new(&format!("/var/lib/taos/taosx/tasks/{}/archived", task_id.unwrap())).to_path_buf());
//     let cache_dir = env::var("CACHE_DIR")
//         .ok()
//         .map(|p| Path::new(&p).to_path_buf())
//         // .unwrap_or_else(|| tempfile::TempDir::new().unwrap().keep());
//         .unwrap_or_else(|| Path::new(&format!("/var/lib/taos/taosx/tasks/{}/cache", task_id.unwrap())).to_path_buf());

//     // 1. create databases 
//     println!("====== create databases =====");
//     let pool = TaosBuilder::from_dsn(&to_dsn)?.pool()?;
//     let taos = pool.get().await?;

//     taos.exec_many([
//         format!("DROP DATABASE IF EXISTS `{DEFAULT_DB}`"),
//         format!("CREATE DATABASE `{DEFAULT_DB}`"),
//         format!("USE `{DEFAULT_DB}`"),
//     ])
//     .await?;

//     // start ipc server
//     let mut parser = gen_parser();
//     if let Some(ref mut parser) = parser {
//         unsafe {
//             let global = parser.global() as  *const TableOptions as *mut TableOptions;
//             (*global).process_on_abnormal.archive.location = archive_dir.to_string_lossy().to_string();
//             (*global).process_on_abnormal.cache.location = cache_dir.to_string_lossy().to_string();

//             // (*global).process_on_abnormal.archive.organize_params(task_id.unwrap(), "".into(), false)?;
//             // (*global).process_on_abnormal.cache.organize_params(task_id.unwrap(), "".into(), true)?;
//         }
//     }
//     println!("====== going to send data =====");
//     let _metrics = init_task_metrics(task_id.unwrap()).await;

//     let cancel = CancellationToken::new();
//     let with_agent = None;
//     let (tx, rx) = flume::bounded(10);
//     let notify: TaskNotifySender = tx.into();

//     let port_pool = PortPool::default();
//     let ipc_port = port_pool
//         .get()
//         .await
//         .ok_or_else(|| anyhow::format_err!("No available port for Kafka connection"))?;
//     let server_addr = format!("127.0.0.1:{}", ipc_port);
//     let (mut ipc, _) = build_ipc(
//         Some(&server_addr),
//         parser,
//         &to_dsn,
//         Some(KAFKA_ID),
//         None,
//         None,
//         &cancel,
//         with_agent,
//         None,
//         task_id,
//         notify.clone(),
//         None,
//     )
//     .await?;
//     println!("====== after build_ipc =====");

//     let mut join_set = match send_data(
//         &server_addr,
//         notify.clone(),
//         cancel.clone(),
//     ).await
//     {
//         Ok(set) => set,
//         Err(err) => {
//             cancel.cancel();
//             let _ = ipc.send(());
//             let _ = ipc.close().await;
//             anyhow::bail!("test error: {:#}", err);
//         }
//     };

//     println!("====== after send data =====");
//     let cancel_clone = cancel.clone();
//     let fut = tokio::spawn(async move {
//         tokio::select! {
//             status = async {
//                 while let Some(res) = join_set.join_next().await {
//                     match res {
//                         Ok(Ok(status)) => {
//                             println!("send data exit with status: {:?}", status);
//                             if status.is_timeout() {
//                                 return Ok(status);
//                             }
//                         }
//                         Ok(Err(err)) => {
//                             println!("send data exit with error: {:#}", err);
//                             Err(err).context("send data runners error")?;
//                         }
//                         Err(err) => {
//                             println!("send data exit with error: {:#}", err);
//                             anyhow::bail!("send data exit with error: {:#}", err);
//                         }
//                     }
//                 }
//                 Ok(ExitStatus::Finished)
//             } => {
//                 match status {
//                     Ok(status) => {
//                         println!("status ok");
//                         cancel.cancel();
//                         if status.is_timeout() {
//                             // wait for completion
//                             tokio::time::sleep(Duration::from_millis(100)).await;
//                             join_set.abort_all();
//                             // stop the connector
//                             println!("Kafka task timeout");
//                             ipc.close().await?;
//                             return Ok(());
//                         }
//                         tokio::time::sleep(Duration::from_millis(100)).await;
//                         join_set.abort_all();
//                         match ipc.try_recv_error() {
//                             Ok(res) => {
//                                 tracing::error!("IPC Error: {res}");
//                                 anyhow::bail!("Kafka worker exit with IPC error: {res}");
//                             }
//                             Err(_) => {
//                                 tracing::info!("Kafka worker done successfully");
//                             }
//                         }
//                     }
//                     Err(err) => {
//                         cancel.cancel();
//                         join_set.abort_all();
//                         let _ = ipc.send(());
//                         anyhow::bail!("Kafka exit with error: {:#}", err);
//                     }
//                 }
//             },
//             err = ipc.recv_error() => {
//                 println!("have received worker thread panicked message, terminate child process");
//                 cancel.cancel();
//                 join_set.abort_all();
//                 if let Some(err) = err {
//                     let _ = ipc.send(());
//                     let _ = ipc.close().await;
//                     anyhow::bail!("Kafka writer error: {err:#}");
//                 }
//             },
//             _ = cancel.cancelled() => {
//                 println!("test task cancelled");
//                 join_set.abort_all();
//             }
//         }

//         // send an empty tuple
//         let _ = ipc.send(());
//         // stop the connector
//         println!("Kafka task Done");
//         ipc.close().await?;
//         // wait for completion
//         tokio::time::sleep(Duration::from_millis(100)).await;
//         Ok(())
//     });

//     tokio::select! {
//         rs = fut => {
//             let _ = rs?;
//         },
//         _ = tokio::signal::ctrl_c() => {
//             cancel_clone.cancel();
//         }
//     }

//     println!("going to sleep");
//     tokio::time::sleep(Duration::from_millis(5000)).await;

//     Ok(())
// }

// pub fn gen_parser() -> Option<Parser> {
//     let config = r#"
//     {
//         "global": {
//                 "cache": {
//                         "max_size": "1GB",
//                         "location": "",
//                         "on_fail": "skip"
//                 },
//                 "archive": {
//                         "keep_days": "30d",
//                         "max_size": "1GB",
//                         "location": "",
//                         "on_fail": "rotate"
//                 },
//                 "database_connection_error": "cache",
//                 "database_not_exist": "break",
//                 "table_not_exist": "retry",
//                 "primary_timestamp_overflow": "archive",
//                 "primary_timestamp_null": "archive",
//                 "primary_key_null": "archive",
//                 "table_name_length_overflow": "archive",
//                 "table_name_contains_illegal_char": {
//                         "replace_to": ""
//                 },
//                 "variable_not_exist_in_table_name_template": {
//                         "replace_to": ""
//                 },
//                 "field_name_not_found": "add_field",
//                 "field_name_length_overflow": "archive",
//                 "field_length_extend": true,
//                 "field_length_overflow": "archive",
//                 "ingesting_error": "archive",
//                 "connection_timeout_in_second": "30s"
//         },
//         "parse": {
//                 "id": {
//                         "as": "int"
//                 },
//                 "voltage": {
//                         "as": "int"
//                 },
//                 "v_blob": {
//                         "as": "blob"
//                 },
//                 "groupid": {
//                         "as": "int"
//                 },
//                 "location": {
//                         "as": "nchar(50)"
//                 },
//                 "time": {
//                         "as": "nchar(50)"
//                 }
//         },
//         "model": {
//                 "name": "t${groupid}",
//                 "using": "meters",
//                 "tags": ["groupid", "location"],
//                 "columns": ["ts", "id", "voltage", "v_blob"]
//         },
//         "mutate": [{
//                 "map": {
//                         "ts": {
//                                 "cast": "time",
//                                 "as": "TIMESTAMP(ms)"
//                         },
//                         "id": {
//                                 "cast": "id",
//                                 "as": "INT"
//                         },
//                         "voltage": {
//                                 "cast": "voltage",
//                                 "as": "INT"
//                         },
//                         "v_blob": {
//                                 "cast": "v_blob",
//                                 "as": "blob"
//                         },
//                         "groupid": {
//                                 "cast": "groupid",
//                                 "as": "INT"
//                         },
//                         "location": {
//                                 "cast": "location",
//                                 "as": "VARCHAR"
//                         }
//                 }
//         }]
//     }
//     "#;
//     serde_json::from_str::<Parser>(config).ok()
// }

// pub async fn send_data(
//     server_addr: &str,
//     _notify: TaskNotifySender,
//     cancel: CancellationToken,
// ) -> anyhow::Result<JoinSet<Result<ExitStatus, anyhow::Error>>> {
//     let mut join_set: JoinSet<Result<ExitStatus, anyhow::Error>> = tokio::task::JoinSet::new();
//     let stream = std::net::TcpStream::connect(server_addr)?;
//     set_tcp_keepalive(&stream)?; 
//     stream.set_read_timeout(None)?; 

//     let ack_stream = stream.try_clone()?;
//     set_tcp_keepalive(&ack_stream)?;
//     ack_stream.set_read_timeout(None)?;

//     let schema = build_schema();
//     let mut writer = StreamWriter::try_new(stream, &schema)?;

//     join_set.spawn(async move {
//         let fut = async {
//             loop {
//                 let batch = match gen_rand_batch() {
//                     Ok(v) => v,
//                     Err(e) => {
//                         println!("gen rand batch error: {e:?}");
//                         return Err::<ExitStatus, anyhow::Error>(e.into());
//                     },
//                 };
//                 println!("batch: {:?}", batch);
//                 if let Err(e) = writer.write(&batch) {
//                     println!("writer write batch error: {e:?}");
//                     return Err::<ExitStatus, anyhow::Error>(e.into());
//                 }

//                 tokio::time::sleep(std::time::Duration::from_secs(1)).await;
//             }
//         };
        
//         tokio::select! {
//             rs = fut => {
//                 if let Err(e) = rs {
//                     println!("fut error: {e:?}");
//                     return Err(e);
//                 }
//             },
//             _ = cancel.cancelled() => {
//                 println!("gen batch cancelled");
//                 return Ok(ExitStatus::Aborted);
//             }
//         }
//         Ok(ExitStatus::Finished)
//     });

//     let ack_reader = AckReaderBuilder::new(taosx_ipc::prelude::AckType::Lush).open(ack_stream);
//     join_set.spawn(async move {
//         for ack in ack_reader {
//             if !ack.success() {
//                 println!("ack.code = {:?}, ack.message = {:?}, ack.context = {:?}, ack found error", ack.code(), ack.message(), ack.context());
//                 if let Some(message) = ack.message() {
//                     //todo: send error out 
//                     anyhow::bail!("IPC server writer error: {message:#}");
//                 } else {
//                     anyhow::bail!("IPC server writer error with code: {}", ack.code());
//                 }
//             } 
//             // TODO: add assert
//             println!("recv ack msg: {:?}", ack);
//         }
//         println!("ACK reader finished");
//         Ok(ExitStatus::Finished)
//     });

//     Ok(join_set)
// }

// pub fn build_schema() -> Schema {
//     let mut metadata = HashMap::new();
//     metadata.insert(String::from("version"), String::from("1.0"));
//     metadata.insert(String::from("stream"), String::from("flat"));
//     metadata.insert(String::from("ack"), String::from("lush"));
//     let flat_columns = vec![
//         arrow::datatypes::Field::new("id", DataType::Int32, false),
//         arrow::datatypes::Field::new("voltage", DataType::Float32, false),
//         arrow::datatypes::Field::new("v_blob", DataType::LargeBinary, false),
//         arrow::datatypes::Field::new("groupid", DataType::Int32, true),
//         arrow::datatypes::Field::new("location", DataType::Utf8, false),
//         arrow::datatypes::Field::new(
//             "time",
//             DataType::Timestamp(arrow::datatypes::TimeUnit::Millisecond, None),
//             false,
//         ),
//     ];

//     Schema::new(flat_columns).with_metadata(metadata)
// }

// pub fn gen_rand_batch() -> anyhow::Result<RecordBatch> {
//     let schema = build_schema();

//     let mut rng = rand::thread_rng();
//     let rand = rng.r#gen::<i32>();

//     let mut timestamp = TimestampMillisecondBuilder::new();
//     timestamp.append_value(chrono::Utc::now().timestamp_millis());
//     let mut id = Int32Builder::new();
//     id.append_value(1);
//     let mut voltage = Float32Builder::new();
//     voltage.append_value(33.3);
//     let mut v_blob = LargeBinaryBuilder::new();
//     v_blob.append_value(b"98f463");
//     // v_blob.append_value(123);
//     let mut group_id = Int32Builder::new();
//     group_id.append_value(123);
//     let mut location = StringBuilder::new();
//     location.append_value("BJ");

//     let batch = RecordBatch::try_new(
//         Arc::new(schema),
//         vec![
//             Arc::new(id.finish()),
//             Arc::new(voltage.finish()),
//             Arc::new(v_blob.finish()),
//             Arc::new(group_id.finish()),
//             Arc::new(location.finish()),
//             Arc::new(timestamp.finish()),
//         ],
//     )?;

//     Ok(batch)
// }

// #[derive(Debug)]
// pub enum ExitStatus {
//     /// Nothing to consume
//     None,
//     /// Finished
//     Finished,
//     /// Timeout to poll next message
//     Timeout,
//     /// Cancelled by upstream or other consumers.
//     Aborted,
// }

// impl ExitStatus {
//     pub fn is_timeout(&self) -> bool {
//         matches!(self, Self::Timeout)
//     }
// }

// pub async fn init_task_metrics(task_id: i64) -> Option<Arc<CoreMetrics>> {
//     let stable = String::from("taosx_task_") + "kafka";
//     let metrics = Arc::new(CoreMetrics::IPC(IpcMetrics::new(
//         stable, task_id, None,
//     )));
//     insert_metrics(task_id, metrics.clone()).await;
//     Some(metrics)

//     // let metrics = try_get_metrics::<IpcMetrics>(task_id).await;
//     // if let Some(metrics) = metrics {
//     //     tracing::info!("reset metrics for task {}", task_id);
//     //     metrics.ipc().reset();
//     //     Some(metrics)
//     // } else {
//     //     tracing::info!("create new metrics for task {}", task_id);
//     //     let stable = String::from("taosx_task_") + "kafka";
//     //     let metrics = Arc::new(CoreMetrics::IPC(IpcMetrics::new(
//     //         stable, task_id, None,
//     //     )));
//     //     insert_metrics(task_id, metrics.clone()).await;
//     //     Some(metrics)
//     // }
// }

// // pub async fn try_get_metrics<T: TaskMetrics>(task_id: i64) -> Option<Arc<CoreMetrics>> {
// //     if let Some(metrics) = get_metrics(task_id).await {
// //         Some(metrics)
// //     } else {
// //         println!("load metrics for task {}", task_id);
// //         if let Some(metrics) = load_metrics::<T>(task_id.to_string().as_str()) {
// //             let metrics = Arc::new(metrics.into());
// //             insert_metrics(task_id, metrics.clone()).await;
// //             Some(metrics)
// //         } else {
// //             tracing::debug!("no metrics found for task {}", task_id);
// //             None
// //         }
// //     }
// // }