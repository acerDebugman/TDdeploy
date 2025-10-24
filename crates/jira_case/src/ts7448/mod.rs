pub mod producer;
pub mod consumer;
pub mod consumer2;
pub mod consumer3;
pub mod consumer4;
pub mod batching;
pub mod reader;
pub mod round_trip;

pub async fn ts7448_main() -> anyhow::Result<()> {

    // test_futures_ordered().await?;
    // test_futures_unordered().await?;
    consumer::consumer_main1().await?;

    Ok(())
}


use futures::stream::{FuturesOrdered, FuturesUnordered, StreamExt};
use prost_types::Any; // StreamExt 提供 next()
use std::{pin::Pin, time::Duration};
use tokio::time::sleep;

/*
一起输出,会等最慢的一起输出：
slow
fast
mid
 */
async fn test_futures_ordered() -> anyhow::Result<()> {
    let mut fo = FuturesOrdered::new();

    // 故意把“耗时更长”的任务先 push
    fo.push_back(Box::pin(async { sleep(Duration::from_secs(3)).await; "slow" }) as Pin<Box<dyn Future<Output = &'static str> + Send>>);
    fo.push_back(Box::pin(async { sleep(Duration::from_secs(1)).await; "fast" }) as Pin<Box<dyn Future<Output = &'static str> + Send>>);
    fo.push_back(Box::pin(async { sleep(Duration::from_secs(2)).await; "mid"  }) as Pin<Box<dyn Future<Output = &'static str> + Send>>);

    // 按完成顺序打印, 所有都完成了才会打印
    while let Some(res) = fo.next().await {
        println!("{res}");
    }

    Ok(())
}


/*
先后输出：
fast
mid
slow
 */
async fn test_futures_unordered() -> anyhow::Result<()> {
    let mut fuord = FuturesUnordered::new();

    // 故意把“耗时更长”的任务先 push
    fuord.push(Box::pin(async { sleep(Duration::from_secs(3)).await; "slow" }) as Pin<Box<dyn Future<Output = &'static str> + Send>>);
    fuord.push(Box::pin(async { sleep(Duration::from_secs(1)).await; "fast" }) as Pin<Box<dyn Future<Output = &'static str> + Send>>);
    fuord.push(Box::pin(async { sleep(Duration::from_secs(2)).await; "mid"  }) as Pin<Box<dyn Future<Output = &'static str> + Send>>);

    // 按完成顺序打印
    while let Some(res) = fuord.next().await {
        println!("{res}");
    }

    Ok(())
}

