
pub fn ts7443_breakpoint_main() -> anyhow::Result<()> {
    // let path = breakpoints_db_dir(task_id);
    let path = "/home/algo/tmp/east_airplane/breakpoints";
    let rs = breakpoints_get_all(path)?;
    println!("{:?}", rs);
    Ok(())
}


pub fn breakpoints_get_all(sled_path: &str) -> anyhow::Result<Vec<(String, String)>> {
    let path = std::path::Path::new(sled_path);
    // if path not exist, return None to avoid create db file
    if !path.exists() {
        return Ok(vec![]);
    }
    let db =
        sled::open(path).map_err(|err| anyhow::anyhow!("sled open db file failed: {:?}", err))?;
    let mut result = vec![];
    for item in db.iter() {
        let (key, value) = item?;
        result.push((
            String::from_utf8(key.to_vec())?,
            String::from_utf8(value.to_vec())?,
        ));
    }
    Ok(result)
}
