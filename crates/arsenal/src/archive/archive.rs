use arrow::array::RecordBatch;
use flume::Receiver;

use crate::{archive::{handling_strategy::{archive::Archive, cache::Cache}, parser::Parser}, utils::files::{delete_oldest_parquet_file, write_to_parquet_file}};



#[derive(Debug)]
pub enum ArchiveType {
    Cache,
    Archive,
}

pub struct ArchiveConsumer {
    task_id: i64,
    cache: Cache,
    archive: Archive,
    // cache_appender: RollingFileAppender,
    // archive_appender: RollingFileAppender,
}

impl ArchiveConsumer {
    // pub fn new(task_id: i64, parser: Option<Parser>) -> Self {
    pub fn new(task_id: i64, cache: Cache, archive: Archive) -> Self {
        // Self { task_id, parser }
        Self {
            task_id,
            cache_cfg: cache,
            archive_cfg: archive,
        }
    }

    pub async fn consume(
        &mut self,
        receiver: Receiver<(ArchiveType, RecordBatch)>,
    ) -> anyhow::Result<()> {
        // get configurations
        // let (cache, archive) = match self.parser.clone() {
        //     Some(parser) => (
        //         parser.global().process_on_abnormal.cache.clone(),
        //         parser.global().process_on_abnormal.archive.clone(),
        //     ),
        //     None => (Cache::default(), Archive::default()),
        // };
        // tracing::debug!(
        //     "start the 'cache & archive' thread, task id: {}, cache: {:?}, archive: {:?}",
        //     self.task_id,
        //     cache,
        //     archive
        // );



        // println!("xxxzgc ******* 2 in ArchiveCosnumer::consume: {:?}", archive);
        // // get metrics
        // let metrics = get_metrics_arc_from_i64(Some(self.task_id)).await;
        // let metrics = metrics.ipc();
        // receive data and write to files
        while let Ok((archive_type, batch)) = receiver.recv_async().await {
            println!("xxxzgc ******* 2 in ArchiveCosnumer::consume: {:?}", archive_type);
            match archive_type {
                ArchiveType::Cache => {
                    match write_to_parquet_file(self.task_id, &cache.location, 0, 0, "", &batch) {
                        Ok(_) => {
                            println!("cache records success, {} rows", batch.num_rows());
                        }
                        Err(e) => match cache.on_fail.handle(format!("{e:#}")) {
                            Ok(_) => {}
                            Err(e) => return Err(e),
                        },
                    }
                }
                ArchiveType::Archive => {
                    match write_to_parquet_file(
                        self.task_id,
                        &archive.location,
                        archive.keep_days_value,
                        archive.max_size_value,
                        archive.max_size_unit.as_str(),
                        &batch,
                    ) {
                        Ok(_) => {
                            // metrics.add_archived_rows(batch.num_rows() as u64);
                            println!("archive records success, {} rows", batch.num_rows());
                        }
                        Err(e) => match archive.on_fail.handle(format!("{e:#}")) {
                            Ok(retry) => {
                                if retry {
                                    if let Err(e) =
                                        delete_oldest_parquet_file(self.task_id, &archive.location)
                                    {
                                        println!("rotate archive file failed, err: {e:#}");
                                    }
                                    if let Err(e) = write_to_parquet_file(
                                        self.task_id,
                                        &archive.location,
                                        archive.keep_days_value,
                                        archive.max_size_value,
                                        archive.max_size_unit.as_str(),
                                        &batch,
                                    ) {
                                        println!(
                                            "retry archive records failed, {} rows, err: {e:#}",
                                            batch.num_rows()
                                        );
                                    }
                                }
                            }
                            Err(e) => return Err(e),
                        },
                    }
                }
            }
        }
        Ok(())
    }
}