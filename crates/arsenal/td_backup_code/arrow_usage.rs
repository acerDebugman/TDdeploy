

fn log_illegal_array(array: &ArrayRef, remove_idxs: Vec<u64>) -> anyhow::Result<()> {
    let idx_array = UInt64Array::from(remove_idxs);
    let removed_array = arrow::compute::take(array.as_ref(), &idx_array, None)?;
    let binary_array = arrow::compute::cast(removed_array.as_ref(), &DataType::Binary)?;
    let binary_array = binary_array.as_any().downcast_ref::<BinaryArray>().unwrap();
    for i in 0..binary_array.len() {
        if binary_array.is_null(i) {
            continue;
        }
        let illegal_row = String::from_utf8_lossy(binary_array.value(i));
        tracing::warn!("Json parse drop illegal data: {:?}", illegal_row);
    }
    Ok(())
}


