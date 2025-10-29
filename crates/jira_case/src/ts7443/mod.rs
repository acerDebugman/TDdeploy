
/**
 * 多线程打开 期望报错:
 * in main: Err(sled open db file failed: Io(Custom { kind: Other, error: "could not acquire lock on \"/home/algo/tmp/east_airplane/breakpoints/db\": Os { code: 11, kind: WouldBlock, message: \"Resource temporarily unavailable\" }" }))
 */
pub async fn ts7443_breakpoint_main() -> anyhow::Result<()> {
    // let path = breakpoints_db_dir(task_id);
    let path = "/home/algo/tmp/east_airplane/breakpoints";
    let _ = tokio::spawn(async move {
        let rs = breakpoints_get_all(path).await;
        println!("in spawn: {:?}", rs);
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    });
    // let _ = jd.await?;
    tokio::time::sleep(std::time::Duration::from_secs(3)).await;

    let rs = breakpoints_get_all(path).await;
    println!("in main: {:?}", rs);
    Ok(())
}

pub async fn breakpoints_get_all(sled_path: &str) -> anyhow::Result<Vec<(String, String)>> {
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
    tokio::time::sleep(std::time::Duration::from_secs(10)).await;
    Ok(result)
}



// pub fn breakpoints_get_all(sled_path: &str) -> anyhow::Result<Vec<(String, String)>> {
//     let path = std::path::Path::new(sled_path);
//     // if path not exist, return None to avoid create db file
//     if !path.exists() {
//         return Ok(vec![]);
//     }
//     let db =
//         sled::open(path).map_err(|err| anyhow::anyhow!("sled open db file failed: {:?}", err))?;
//     let mut result = vec![];
//     for item in db.iter() {
//         let (key, value) = item?;
//         result.push((
//             String::from_utf8(key.to_vec())?,
//             String::from_utf8(value.to_vec())?,
//         ));
//     }
//     Ok(result)
// }
