// read parquet file
use parquet::file::reader::{FileReader, SerializedFileReader};
use parquet::file::reader::FileReader as _;
use parquet::record::RowAccessor;
use std::fs::File;
use std::io::{BufReader, Cursor};

fn main() -> anyhow::Result<()> {
    let file = File::open("meters.parquet")?;
    // let reader = BufReader::new(file);
    let parquet_reader = SerializedFileReader::new(file)?;

    let schema = parquet_reader.metadata().file_metadata().schema();
    println!("{:?}", schema);

    for i in 0..parquet_reader.metadata().num_row_groups() {
        let row_group = parquet_reader.get_row_group(i)?;
        for j in 0..row_group.num_columns() {
            let mut page_iter = row_group.get_column_page_reader(j)?;
            while let Some(page) = page_iter.next() {
                let page = page?;
                let mut page_values = page.buffer();
                // let page_values = match page.buffer() {
                //     Ok(buf) => buf,
                //     Err(e) => {
                //         eprintln!("Buffer error: {}", e);
                //         continue;
                //     }
                // };

                // 直接处理值，跳过定义级别和重复级别
                if j == 0 {
                    let values = page_values.to_byte_array_slice()?;
                    for value in values {
                        println!("String value: {:?}", value);
                    }
                } else if j == 1 {
                    let values = page_values.to_i64_slice()?;
                    for value in values {
                        println!("i64 value: {}", value);
                    }
                } else if j == 2 {
                    let values = page_values.to_f64_slice()?;
                    for value in values {
                        println!("f64 value: {}", value);
                    }
                }
            }
        }
    }
    Ok(())
}