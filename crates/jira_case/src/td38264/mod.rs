use std::{fs::OpenOptions, sync::Arc};

use arrow::array::{RecordBatch, StringArray, StringBuilder, TimestampNanosecondArray};
use arrow_schema::{DataType, Field, Schema};
use chrono::Utc;
use parquet::{arrow::ArrowWriter, basic::{Compression, ZstdLevel}, file::properties::WriterProperties};


pub fn td38264_main() -> anyhow::Result<()> {

    for _ in 0..1024 {
        pressing_test_parquet()?;
    }

    Ok(())
}

pub fn pressing_test_parquet() -> anyhow::Result<()> {
    // let batch = arrow::array::record_batch!(
    //     (
    //         "test1",
    //         LargeBinary,
    //         [b"1234567890", &b"1234567890".repeat(1024)]
    //     ),
    //     ("test2", Binary, [b"1234567890", &b"12345".repeat(10)]),
    //     ("name", Utf8, ["r1", "r2"])
    // )?;
    const ROW_CNT: usize = 3;
    // let test1_vec = vec!["1234567890"; ROW_CNT];
    // let name_vec = vec!["r1"; ROW_CNT];

    let flat_columns = vec![
        arrow::datatypes::Field::new("test1", DataType::Utf8, true),
        arrow::datatypes::Field::new("name", DataType::Utf8, true),
    ];

    let schema = Schema::new(flat_columns);
    let mut test_builder = StringBuilder::new();
    for _ in 0..ROW_CNT {
        test_builder.append_value("1");
    }
    let mut name_builder = StringBuilder::new();
    if ROW_CNT > 13 {
        for _ in 0..3 {
            name_builder.append_option(None::<&str>);
        }
        for _ in 0..10 {
            name_builder.append_option(Some("r1"));
        }
        for _ in 0..(ROW_CNT - 13) {
            name_builder.append_option(Some("r2"));
        }
    } else {
        for _ in 0..ROW_CNT {
            name_builder.append_option(Some("r1"));
        }
    }

    let batch = RecordBatch::try_new(
        Arc::new(schema),
        vec![
            Arc::new(test_builder.finish()),
            Arc::new(name_builder.finish()),
        ],
    )?;

    let err_msg = include!("err_insert.txt");
    let err_vec = vec![err_msg.to_string(); batch.num_rows()];
    // let err_vec = vec![err_msg.repeat(1024 * 1024); batch.num_rows()];
    let err_timestamp_vec = vec![Utc::now().timestamp_nanos_opt().unwrap(); batch.num_rows()];
    let batch = build_archive_batch(&batch, err_vec, err_timestamp_vec)?.ok_or_else(|| anyhow::anyhow!("build archive batch failed"))?;
    recordbatch_to_parquet(&batch, "/tmp/td38264.parquet")?;

    Ok(())
}


pub fn recordbatch_to_parquet(batch: &RecordBatch, parquet_path: &str) -> anyhow::Result<()> {
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&parquet_path)?;
    let schema = batch.schema();
    let props = WriterProperties::builder()
        .set_compression(Compression::ZSTD(ZstdLevel::default()))
        .build();
    let mut writer = ArrowWriter::try_new(file, schema, Some(props))?;
    writer.write(&batch)?;
    writer.close()?;
    Ok(())
}



pub fn build_archive_batch(
    batch: &RecordBatch,
    err_vec: Vec<String>,
    err_timestamp_vec: Vec<i64>,
) -> anyhow::Result<Option<RecordBatch>> {
    if batch.num_rows() == 0 {
        return Ok(None);
    }

    // get fields and columns
    let mut fields_vec = batch.schema().fields().to_vec();
    let mut columns_vec = batch.columns().to_vec();

    // add new fields and columns to record
    let new_field_1 = Field::new("_taosx_error_", DataType::Utf8, false);
    let new_field_2 = Field::new(
        "_taosx_error_timestamp_",
        DataType::Timestamp(arrow_schema::TimeUnit::Nanosecond, None),
        false,
    );
    let new_column_1 = Arc::new(StringArray::from(err_vec));
    let new_column_2 = Arc::new(TimestampNanosecondArray::from(err_timestamp_vec));

    fields_vec.push(Arc::new(new_field_1));
    fields_vec.push(Arc::new(new_field_2));
    columns_vec.push(new_column_1);
    columns_vec.push(new_column_2);

    // create a new RecordBatch with the additional column
    let new_schema = Arc::new(Schema::new(fields_vec));
    let new_batch = RecordBatch::try_new(new_schema, columns_vec)?;
    Ok(Some(new_batch))
}


// let schema = records.batches[0].schema();
// let batch = concat_batches(&schema, records.batches.iter())?;
// if let Err(e) =
//     process_archive(&err, &batch, archive_tx.clone())
//         .await
// {
//     tracing::error!("archive error: {e:#}");
// }


// use itertools::Itertools;

// use super::Parser;
// use crate::plugins::transform::{MessageArrowRecords, MessageTableMeta, TableOptions};
// use std::sync::Arc;

// #[tokio::test]
// async fn test_sql_insert_part() {
//     let parser = r#"{
//         "parse": {
//             "value": {"json": ""}
//         },
//         "model": {
//             "name": "t_${DEV_ID}",
//             "using": "deva",
//             "tags": [ "dev_id" ],
//             "columns": [ "_ts", "_val0", "_val1" ]
//         },
//         "mutate": [{
//             "map": {
//                 "_ts": {
//                     "cast": "_ts",
//                     "as": "TIMESTAMP(ms)"
//                 },
//                 "_val0": {
//                     "cast": "_val0",
//                     "as": "INT"
//                 },
//                 "_val1": {
//                     "cast": "_val1",
//                     "as": "INT"
//                 },
//                 "dev_id": {
//                     "cast": "DEV_ID",
//                     "as": "VARCHAR"
//                 }
//             }
//         }]

//     }"#;
//     let parser: Parser = serde_json::from_str(parser).unwrap();

//     let raw_data = arrow::array::record_batch!(
//         ("topic", Utf8, ["test", "test", "test"]),
//         (
//             "value",
//             Utf8,
//             [
//                 r#"{"_ts": "2024-12-02T18:00:00+08:00", "_val0": 12, "DEV_ID": "2212"}"#,
//                 r#"{"_ts": "2024-12-02T18:00:00+08:00", "_val1": 13, "DEV_ID": "2213"}"#,
//                 r#"{"_ts": "2024-12-02T18:00:01+08:00", "DEV_ID": "2212"}"#
//             ]
//         )
//     )
//     .unwrap();
//     let (tx, _rx) = flume::bounded(10);

//     let records = parser
//         .parse_message_from_records(&raw_data, false, tx.clone())
//         .unwrap();
//     dbg!("zgc:", &records);
        

//     if let super::Message::Records(records) = records {
//         let groups: std::collections::HashMap<Option<String>, Vec<&MessageArrowRecords>> = records
//             .iter()
//             .into_group_map_by(|m| m.stable_name().map(|s| s.to_string()));
//         dbg!("zgc2:", &groups);
//         println!("--sql_insert_part--");
//         for record in &records {
//             let sql = record.sql_insert_part(taos::Precision::Millisecond, true, true, None);
//             dbg!(&sql);
//         }
//         println!("--sql_insert_part_skip_null--");
//         for record in &records {
//             let sql = record.sql_insert_part_skip_null(taos::Precision::Millisecond);
//             dbg!(&sql);
//         }
//     }
// }
