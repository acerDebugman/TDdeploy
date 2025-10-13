use std::{fs::OpenOptions, sync::Arc};

use arrow::array::{RecordBatch, StringArray, TimestampNanosecondArray};
use arrow_schema::{DataType, Field, Schema};
use chrono::Utc;
use parquet::{arrow::ArrowWriter, basic::{Compression, ZstdLevel}, file::properties::WriterProperties};


pub fn td38264_main() -> anyhow::Result<()> {
    // let batch = arrow::array::record_batch!(
    //     (
    //         "test1",
    //         LargeBinary,
    //         [b"1234567890", &b"1234567890".repeat(1024)]
    //     ),
    //     ("test2", Binary, [b"1234567890", &b"12345".repeat(10)]),
    //     ("name", Utf8, ["r1", "r2"])
    // )?;
    let batch = arrow::array::record_batch!(
        (
            "test1",
            Utf8,
            ["1234567890", "1234567890"]
        ),
        ("name", Utf8, ["r1", "r2"])
    )?;

    let err_vec = vec!["z".repeat(1024 * 1024); batch.num_rows()];
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