use std::fs::{self, OpenOptions};
use std::fs::{DirEntry, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use anyhow::{anyhow, Context};
use arrow::array::RecordBatch;
use arrow_schema::ArrowError;
use chardetng::EncodingDetector;
use chrono::{Duration, Utc};
use encoding_rs::Encoding;
use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;
use parquet::arrow::ArrowWriter;
use parquet::basic::{Compression, ZstdLevel};
use parquet::file::properties::WriterProperties;
use taos::Itertools;

use crate::get_data_dir;

pub fn get_files_in_dir(dir: &str, ext: &str) -> Result<Vec<String>, anyhow::Error> {
    let path = Path::new(dir);
    if !path.is_dir() {
        return Err(anyhow!(format!("path {} is not dir", dir)));
    }

    let mut files = vec![];
    let mut stack = vec![path.to_path_buf()];

    while let Some(p) = stack.pop() {
        let dir_files = fs::read_dir(p)?;
        for entry in dir_files {
            let entry_path = entry?.path();
            if entry_path.is_dir() {
                stack.push(entry_path);
                continue;
            }
            if let Some(file) = entry_path
                .to_str()
                .filter(|f| ext.is_empty() || f.ends_with(ext))
            {
                files.push(file.to_owned());
            }
        }
    }

    Ok(files)
}

pub fn get_encode<T: AsRef<Path>>(file_path: T) -> anyhow::Result<&'static Encoding> {
    let file_path = file_path.as_ref();
    let mut file = File::open(file_path).map_err(|e| {
        anyhow::anyhow!(
            "failed to open file: {}, cause: {}",
            file_path.display(),
            e.to_string()
        )
    })?;

    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).map_err(|e| {
        anyhow::anyhow!(
            "failed to read file: {}, cause: {}",
            file_path.display(),
            e.to_string()
        )
    })?;

    get_encode_from_buffer(buffer.as_slice())
}

pub fn get_encode_from_buffer(buffer: &[u8]) -> anyhow::Result<&'static Encoding> {
    let mut detector = EncodingDetector::new();
    detector.feed(buffer, true);

    let encoding = detector.guess(None, true);

    Ok(encoding)
}

pub fn decompress_and_write_file(
    path: &std::path::PathBuf,
    data: &[u8],
) -> Result<(), std::io::Error> {
    use std::io::Write;
    let decode_buf = Vec::new();
    let mut decoder = flate2::write::GzDecoder::new(decode_buf);
    decoder.write_all(data)?;
    let writer = decoder.finish()?;
    let mut file = File::create(path)?;
    file.write_all(&writer)?;
    Ok(())
}

pub fn write_to_file(task_id: i64, filename: &String, record: &String) -> anyhow::Result<()> {
    let data_dir = get_data_dir();
    let path = data_dir.join("tasks").join(format!("{task_id}"));
    if !path.exists() {
        if let Err(err) = std::fs::create_dir_all(&path) {
            tracing::error!("failed to create dir {:?}: {}", path, err);
        }
    }
    let path = path.join(format!("{}.{}", filename, Utc::now().format("%Y%m%d")));
    let mut file = OpenOptions::new().create(true).append(true).open(path)?;
    writeln!(file, "{}", record)?;
    Ok(())
}

/// Write to parquet file
///
/// Write the record batch to the parquet file, and archive the file by date and size
pub fn write_to_parquet_file(
    task_id: i64,
    filename: &String,
    keep_days: usize,
    max_size: usize,
    max_size_unit: &str,
    record: &RecordBatch,
) -> anyhow::Result<()> {
    let data_dir = get_data_dir();
    let path = data_dir.join("tasks").join(format!("{task_id}"));
    if !path.exists() {
        if let Err(err) = std::fs::create_dir_all(&path) {
            tracing::error!("failed to create dir {:?}: {}", path, err);
        }
    }
    // delete old files
    if keep_days > 0 {
        delete_old_parquet_files_by_date(task_id, filename, keep_days)?;
    }
    if max_size > 0 {
        delete_old_parquet_files_by_size(task_id, filename, max_size, max_size_unit)?;
    }

    let path = path.join(format!("{}.{}", filename, Utc::now().format("%Y%m%d")));
    let file = OpenOptions::new().create(true).append(true).open(path)?;
    let schema = record.schema();
    let props = WriterProperties::builder()
        .set_compression(Compression::ZSTD(ZstdLevel::default()))
        .build();
    let mut writer =
        ArrowWriter::try_new(file, schema, Some(props)).context("init arrow writer error")?;
    writer.write(record)?;
    writer.close()?;
    Ok(())
}

/// Delete old parquet files by date
///
/// Delete the old parquet files in the directory until the oldest file is less than the keep days
pub fn delete_old_parquet_files_by_date(
    task_id: i64,
    filename: &String,
    keep_days: usize,
) -> anyhow::Result<()> {
    let data_dir = get_data_dir();
    let path = data_dir.join("tasks").join(format!("{task_id}"));
    let path = path.join(format!("{}.{}", filename, Utc::now().format("%Y%m%d")));
    let path = path.parent().context("get parent path error")?;
    if !path.exists() {
        if let Err(err) = std::fs::create_dir_all(path) {
            tracing::error!("failed to create dir {:?}: {}", path, err);
        }
    }

    let cutoff_date = Utc::now() - Duration::days(keep_days as i64);
    let files = read_parquet_dir_files(task_id, filename)?;

    for file in files {
        if let Some(file_name) = file.file_name().and_then(|s| s.to_str()) {
            let file_date = file_name
                .split('.')
                .next_back()
                .context("get file date error")?;
            if let Ok(file_date) = chrono::NaiveDate::parse_from_str(file_date, "%Y%m%d") {
                if file_date <= cutoff_date.naive_utc().date() {
                    tracing::info!("delete archived file: {:?}, since out of date", file);
                    fs::remove_file(file)?;
                }
            }
        }
    }
    Ok(())
}

/// Delete old parquet files by size
///
/// Delete the oldest parquet files in the directory until the total size of the files is less than the max size
pub fn delete_old_parquet_files_by_size(
    task_id: i64,
    filename: &String,
    max_size: usize,
    max_size_unit: &str,
) -> anyhow::Result<()> {
    let max_size = if max_size_unit.to_lowercase() == "gb" {
        max_size * 1024 * 1024 * 1024
    } else if max_size_unit.to_lowercase() == "mb" {
        max_size * 1024 * 1024
    } else if max_size_unit.to_lowercase() == "kb" {
        max_size * 1024
    } else {
        max_size
    };
    let mut total_file_size = 0;

    let entries = read_parquet_dir_entries(task_id, filename)?;
    let mut entries: Vec<(&DirEntry, u64)> = entries
        .iter()
        .map(|entry| -> anyhow::Result<_> {
            let file_size = entry.metadata().context("get entry metadata error")?.len();
            total_file_size += file_size;
            Ok((entry, file_size))
        })
        .try_collect()?;
    entries.sort_by_key(|(entry, _)| entry.file_name());

    for (entry, file_size) in entries {
        if total_file_size <= max_size as u64 {
            break;
        }
        let file_path = entry.path();
        tracing::info!("delete archived file: {:?}, since out of date", file_path);
        fs::remove_file(file_path)?;
        total_file_size -= file_size;
    }
    Ok(())
}

/// Delete oldest parquet file
///
/// Delete the oldest parquet file in the directory
pub fn delete_oldest_parquet_file(task_id: i64, filename: &String) -> anyhow::Result<()> {
    match read_parquet_dir_files(task_id, filename) {
        Ok(files) => {
            if files.is_empty() {
                return Ok(());
            }
            if let Some(file) = files.first() {
                tracing::info!("delete archived file: {:?}, since out of date", file);
                fs::remove_file(file)?;
            }
        }
        Err(err) => {
            return Err(err);
        }
    }
    Ok(())
}

/// Read parquet dir files
///
/// For archive files and cache files. If the path is not exists, create the paths recursively.
/// Only return the files in the level one of the path.
/// Sort the files by the file name, and filter the files by the filename.
pub fn read_parquet_dir_files(task_id: i64, filename: &String) -> anyhow::Result<Vec<PathBuf>> {
    if filename.is_empty() {
        return Ok(vec![]);
    }
    let mut entries = read_parquet_dir_entries(task_id, filename)?;
    entries.sort_by_key(|entry| entry.file_name());

    let files = entries
        .iter()
        .map(|entry| entry.path())
        .filter(|path| path.to_str().is_some_and(|s| s.contains(filename)))
        .collect_vec();
    Ok(files)
}

/// Read parquet dir entries
///
/// For archive files and cache files. If the path is not exists, create the paths recursively.
/// Only return the files in the level one of the path.
fn read_parquet_dir_entries(task_id: i64, filename: &String) -> anyhow::Result<Vec<DirEntry>> {
    let data_dir = get_data_dir();
    let path = data_dir.join("tasks").join(format!("{task_id}"));
    let path = path.join(format!("{}.{}", filename, Utc::now().format("%Y%m%d")));
    let path = path.parent().context("get parent path error")?;
    if !path.exists() {
        if let Err(err) = std::fs::create_dir_all(path) {
            tracing::error!("failed to create dir {:?}: {}", path, err);
        }
    }

    let entries = fs::read_dir(path)?
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().is_file())
        .collect_vec();
    Ok(entries)
}

/// Read parquet file
///
/// Read the parquet file and return the record batches
// pub fn read_parquet_file(path: PathBuf) -> anyhow::Result<Vec<RecordBatch>> {
//     let mut batches = Vec::new();
//     // open the file
//     let file =
//         File::open(path.clone()).with_context(|| format!("Unable to open file '{:?}'", path))?;
//     let buffer = std::io::BufReader::new(file);
//     // the flag to identify the split point
//     let flag = b"PAR1PAR1";
//     // look for the flag and read the file in chunks
//     let mut content = Vec::new();
//     let mut windows = Vec::new();
//     for byte in buffer.bytes() {
//         match byte {
//             Ok(byte) => {
//                 content.push(byte);
//                 windows.push(byte);
//                 if windows.len() > flag.len() {
//                     windows.remove(0);
//                 }
//                 if windows == flag {
//                     // remove the extra bytes
//                     content.truncate(content.len() - 4);
//                     // transform to batches
//                     match transform_bytes_to_record(content.clone()) {
//                         Ok(mut vec) => batches.append(&mut vec),
//                         Err(err) => {
//                             anyhow::bail!("Error reading file: {err:#}");
//                         }
//                     }
//                     // begin new record
//                     content.clear();
//                     b"PAR1".iter().for_each(|x| content.push(*x));
//                 }
//             }
//             Err(err) => {
//                 anyhow::bail!("Error reading file: {err:#}");
//             }
//         }
//     }
//     // last record
//     match transform_bytes_to_record(content.clone()) {
//         Ok(mut vec) => batches.append(&mut vec),
//         Err(err) => {
//             anyhow::bail!("Error reading file: {err:#}");
//         }
//     }

//     Ok(batches)
// }

// /// Transform bytes to record
// ///
// /// the bytes are read from the parquet file, and transformed to record batches
// fn transform_bytes_to_record(bytes: Vec<u8>) -> Result<Vec<RecordBatch>, ArrowError> {
//     // build the parquet reader
//     let mut reader = ParquetRecordBatchReaderBuilder::try_new(bytes::Bytes::from(bytes))
//         .expect("Unable to create Parquet reader builder")
//         .build()
//         .expect("Unable to build Parquet reader");
//     // read all batches
//     reader
//         .next()
//         .into_iter()
//         .collect::<Result<Vec<RecordBatch>, ArrowError>>()
// }

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use arrow::array::{ArrayRef, Int32Array, StringArray};

    use super::*;

    #[test]
    fn test_get_file_encode() {
        let file_path = "./tests/opc/opcua-utf8.csv";
        let encode = get_encode(file_path).unwrap();
        assert_eq!(encode.name(), "UTF-8");

        let file_path = "./tests/opc/opcua-utf8bom.csv";
        let encode = get_encode(file_path).unwrap();
        assert_eq!(encode.name(), "UTF-8");

        let file_path = "./tests/opc/opcua-gbk.csv";
        let encode = get_encode(file_path).unwrap();
        assert_eq!(encode.name(), "GBK");
    }

    #[test]
    fn test_write_to_file() {
        let task_id = 1;
        let filename = "cache".to_string();
        let record = "hello world".to_string();
        write_to_file(task_id, &filename, &record).unwrap();
    }

    #[test]
    fn test_write_to_parquet_file() {
        let task_id = 1;
        let filename = "archive".to_string();
        let record = RecordBatch::try_from_iter([
            ("str1", Arc::new(StringArray::from(vec!["a"])) as ArrayRef),
            ("int1", Arc::new(Int32Array::from(vec![1])) as ArrayRef),
        ])
        .unwrap();
        write_to_parquet_file(task_id, &filename, 5, 2, "GB", &record).unwrap();
    }

    #[test]
    fn test_delete_old_parquet_files_by_date() {
        let task_id = 1;
        let filename = "archive/p1/p2/p3/p4/file".to_string();
        let keep_days = 5;
        let res = delete_old_parquet_files_by_date(task_id, &filename, keep_days);
        assert!(res.is_ok());
    }

    #[test]
    fn test_delete_old_parquet_files_by_size() {
        let task_id = 1;
        let filename = "archive/p1/p2/p3/p4/file".to_string();
        let max_size = 8;
        let max_size_unit = "GB";
        let res = delete_old_parquet_files_by_size(task_id, &filename, max_size, max_size_unit);
        assert!(res.is_ok());
    }

    #[test]
    fn test_delete_oldest_parquet_file() {
        let task_id = 1;
        let filename = "archive/p1/p2/p3/p4/file".to_string();
        let res = delete_oldest_parquet_file(task_id, &filename);
        assert!(res.is_ok());
    }

    #[ignore]
    #[test]
    fn test_read_parquet_dir_files() {
        let task_id = 1;
        let filename = "archive".to_string();
        let res = read_parquet_dir_files(task_id, &filename);
        dbg!(&res);
    }

    #[ignore]
    #[test]
    fn test_read_parquet_dir_entries() {
        let task_id = 1;
        let filename = "archive".to_string();
        let res = read_parquet_dir_entries(task_id, &filename);
        dbg!(&res);
    }

    // #[ignore]
    // #[test]
    // fn test_read_parquet_file() {
    //     let task_id = 7;
    //     let filename = "archived.20250226".to_string();

    //     let data_dir = get_data_dir();
    //     let path = data_dir
    //         .join("tasks")
    //         .join(format!("{task_id}"))
    //         .join(filename);

    //     let res = read_parquet_file(path);
    //     dbg!(&res);
    // }
}
