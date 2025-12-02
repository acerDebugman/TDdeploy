use libuv_sys2::*;
use std::ffi::{c_void, CString};
use std::mem;
use std::os::raw::{c_char, c_int};
use std::ptr;

// 客户端数据上下文
struct ClientData {
    _client: *mut uv_tcp_t,
    write_req: *mut uv_write_t,
}

unsafe extern "C" fn alloc_buffer(_handle: *mut uv_handle_t, suggested_size: usize, buf: *mut uv_buf_t) {
    let base = libc::malloc(suggested_size) as *mut c_char;
    (*buf).base = base;
    (*buf).len = suggested_size;
}

unsafe extern "C" fn on_write(req: *mut uv_write_t, _status: c_int) {
    let _data = Box::from_raw((*req).data as *mut ClientData);
    libc::free(req as *mut c_void);
}

unsafe extern "C" fn on_read(stream: *mut uv_stream_t, nread: isize, buf: *const uv_buf_t) {
    if nread > 0 {
        // 打印接收到的消息
        let msg = std::slice::from_raw_parts((*buf).base as *const u8, nread as usize);
        println!("C says: {}", String::from_utf8_lossy(msg));

        // 发送回复
        let reply = CString::new("Hello from Rust!").unwrap();
        let write_buf = uv_buf_init(reply.as_ptr() as *mut i8, unsafe { reply.as_bytes().len()  as u32 });
        
        let write_req = libc::malloc(mem::size_of::<uv_write_t>()) as *mut uv_write_t;
        (*write_req).data = Box::into_raw(Box::new(ClientData {
            _client: stream as *mut uv_tcp_t,
            write_req,
        })) as *mut c_void;
        
        uv_write(write_req, stream, &write_buf, 1, Some(on_write));
    } else if nread < 0 {
        uv_close(stream as *mut uv_handle_t, None);
    }
    
    libc::free((*buf).base as *mut c_void);
}

unsafe extern "C" fn on_new_connection(server: *mut uv_stream_t, status: c_int) {
    if status < 0 { return; }

    let loop_ = (*server).loop_;
    let client = libc::malloc(mem::size_of::<uv_tcp_t>()) as *mut uv_tcp_t;
    uv_tcp_init(loop_, client);
    
    if uv_accept(server, client as *mut uv_stream_t) == 0 {
        uv_read_start(client as *mut uv_stream_t, Some(alloc_buffer), Some(on_read));
    } else {
        uv_close(client as *mut uv_handle_t, None);
    }
}

fn main() {
    unsafe {
        let loop_ = uv_default_loop();
        let server = libc::malloc(mem::size_of::<uv_tcp_t>()) as *mut uv_tcp_t;
        uv_tcp_init(loop_, server);
        
        let mut addr: sockaddr_in = mem::zeroed();
        let addr_str = CString::new("0.0.0.0").unwrap();
        uv_ip4_addr(addr_str.as_ptr(), 12345, &mut addr);
        
        uv_tcp_bind(server, &addr as *const _ as *const sockaddr, 0);
        uv_listen(server as *mut uv_stream_t, 128, Some(on_new_connection));
        
        println!("Rust server listening on port 12345");
        uv_run(loop_, uv_run_mode_UV_RUN_DEFAULT);
    }
}
