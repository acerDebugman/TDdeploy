use std::sync::Arc;


/**
 * 多线程打开 期望报错:
 * in main: Err(sled open db file failed: Io(Custom { kind: Other, error: "could not acquire lock on \"/home/algo/tmp/east_airplane/breakpoints/db\": Os { code: 11, kind: WouldBlock, message: \"Resource temporarily unavailable\" }" }))
 */
pub async fn ts7443_breakpoint_main() -> anyhow::Result<()> {
    // let path = "/home/algo/tmp/east_airplane/breakpoints";
    // let _ = tokio::spawn(async move {
    //     let rs = breakpoints_get_all(path).await;
    //     println!("in spawn: {:?}", rs);
    //     tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    // });
    // // let _ = jd.await?;
    // tokio::time::sleep(std::time::Duration::from_secs(3)).await;

    // let rs = breakpoints_get_all(path).await;
    // println!("in main: {:?}", rs);

    // test_arc_strong_count().await;
    test_arc_weak().await;
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

pub async fn test_arc_weak() {
    let d = std::sync::Arc::new(String::from("abc"));
    let d_weak = Arc::downgrade(&d);
    // println!("d init strong count: {:?}, d: {:?}", Arc::weak_count(&d), d);
    println!("d init weak count: {:?}, d: {:?}", Arc::weak_count(&d), d);
    let d = unsafe { 
        let ptr = Arc::into_raw(d);
        Arc::decrement_strong_count(ptr); //这里先减1, 后面直接再使用，可能就会 panic 了，因为 release 了
        // Arc::increment_strong_count(ptr); //先加一没有问题
        Arc::from_raw(ptr)
    };
    // println!("d strong count after decrement: {:?}, d: {:?}", Arc::strong_count(&d), d);

    tokio::spawn({
        let d1 = d_weak.upgrade().unwrap();
        async move {
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            println!("d1 strong count: {:?}, d1: {:?}", Arc::strong_count(&d1), d1);
            tokio::time::sleep(std::time::Duration::from_secs(3)).await;
        }
    });

    tokio::spawn({
        let d2 = d_weak.upgrade().unwrap();
        async move {
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            println!("d2 strong count: {:?}, d2: {:?}", Arc::strong_count(&d2), d2);
            tokio::time::sleep(std::time::Duration::from_secs(3)).await;
        }
    });

    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
    println!("d strong count: {:?}, d: {:?}", Arc::strong_count(&d), d);

    tokio::time::sleep(std::time::Duration::from_secs(7)).await;
    println!("d strong count: {:?}, d: {:?}", Arc::strong_count(&d), d);

}




pub async fn test_arc_strong_count() {
    let d = std::sync::Arc::new(String::from("abc"));
    println!("d init strong count: {:?}, d: {:?}", Arc::strong_count(&d), d);
    let d = unsafe { 
        let ptr = Arc::into_raw(d);
        Arc::decrement_strong_count(ptr); //这里先减1, 后面直接再使用，可能就会 panic 了，因为 release 了
        // Arc::increment_strong_count(ptr); //先加一没有问题
        Arc::from_raw(ptr)
    };
    println!("d strong count after decrement: {:?}, d: {:?}", Arc::strong_count(&d), d);

    tokio::spawn({
        let d1 = d.clone();
        async move {
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            println!("d1 strong count: {:?}, d1: {:?}", Arc::strong_count(&d1), d1);
            tokio::time::sleep(std::time::Duration::from_secs(3)).await;
        }
    });

    tokio::spawn({
        let d2 = d.clone();
        async move {
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            println!("d2 strong count: {:?}, d2: {:?}", Arc::strong_count(&d2), d2);
            tokio::time::sleep(std::time::Duration::from_secs(3)).await;
        }
    });

    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
    println!("d strong count: {:?}, d: {:?}", Arc::strong_count(&d), d);

    tokio::time::sleep(std::time::Duration::from_secs(7)).await;
    println!("d strong count: {:?}, d: {:?}", Arc::strong_count(&d), d);

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
