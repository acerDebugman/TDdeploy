use std::io::Write;



/*
验证：当 select! 的某个分支完成时，​​其他分支会被立即取消​​。
结论：是的，当 tokio::select! 的某个分支完成时，​​其他分支会被立即取消​​。
当宏执行完毕时，所有未完成的 Future 会被丢弃（Drop），触发它们的清理逻辑。



```
let handle1 = tokio::spawn(async_op1());
let handle2 = tokio::spawn(async_op2());

tokio::select! {
    _ = handle1 => { /* 处理结果 1 */ },
    _ = handle2 => { /* 处理结果 2 */ },
}
```

 */
#[tokio::main]
async fn main() {
    async fn test_select() {
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    }

    let cancel_token = tokio_util::sync::CancellationToken::new();
        
    tokio::select! {
        _ = test_select() => {
            println!("task1: test_select finished");
        }
        _ = async {
            cancel_token.cancelled().await;
            println!("task2: cancel_token is cancelled");
        } => {}
        _ = async {
            loop {
                if cancel_token.is_cancelled() {
                    println!("cancel_token is cancelled");
                    break;
                }
                println!("task3: running 1 sectimeout");
                tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            }
            cancel_token.cancel();
        } => {}
    }
    
    println!("main finished");

    tokio::signal::ctrl_c().await.unwrap();
    println!("main: ctrl_c");
    cancel_token.cancel();
    println!("main: cancel_token");
    std::io::stdout().flush().unwrap();

    tokio::signal::ctrl_c().await.unwrap();
}