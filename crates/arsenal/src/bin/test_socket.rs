use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

/*  
另起一个终端，输入：nc 127.0.0.1 9999
乱敲字符，会在另一个终端中显示出来，同时这个终端会显示：
srv-0
srv-1
srv-2
...

这就验证了全双工通信：
1. 客户端可以向服务端发送数据
2. 服务端可以向客户端发送数据
3. 服务端可以关闭连接，客户端会收到 EOF 错误
4. 客户端可以关闭连接，服务端会收到 EOF 错误
*/
fn handle(mut stream: TcpStream) {
    let mut s2 = stream.try_clone().unwrap();
    // 线程 A：持续写
    thread::spawn(move || {
        let mut n = 0u64;
        loop {
            let buf = format!("srv-{}\n", n);
            stream.write_all(buf.as_bytes()).unwrap();
            n += 1;
            std::thread::sleep(std::time::Duration::from_millis(1000));
        }
    });

    // 线程 B：持续读
    let mut buf = [0u8; 1024];
    loop {
        let n = s2.read(&mut buf).unwrap();
        print!("{}", String::from_utf8_lossy(&buf[..n]));
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:9999").unwrap();
    for stream in listener.incoming() {
        handle(stream.unwrap());
    }
}