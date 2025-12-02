#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include "uv.h"

typedef struct {
    uv_write_t req;
    uv_buf_t buf;
} write_req_t;

static uv_loop_t *loop;
static uv_tcp_t client;

void on_write(uv_write_t *req, int status) {
    if (status < 0) {
        fprintf(stderr, "Write error: %s\n", uv_strerror(status));
    }
    // free_req((write_req_t*)req);
    free((write_req_t*)req);
}

void on_connect(uv_connect_t *req, int status) {
    if (status < 0) {
        fprintf(stderr, "Connect error: %s\n", uv_strerror(status));
        free(req);
        return;
    }
    free(req);

    const char *msg = "Hello from C!";
    write_req_t *wr = (write_req_t*)malloc(sizeof(write_req_t));
    wr->buf = uv_buf_init((char*)msg, strlen(msg));
    
    uv_write((uv_write_t*)wr, (uv_stream_t*)&client, &wr->buf, 1, on_write);
}

void on_read(uv_stream_t *stream, ssize_t nread, const uv_buf_t *buf) {
    if (nread < 0) {
        if (nread != UV_EOF) {
            fprintf(stderr, "Read error: %s\n", uv_strerror(nread));
        }
        uv_close((uv_handle_t*)stream, NULL);
        free(buf->base);
        return;
    }

    if (nread > 0) {
        printf("Rust says: %.*s\n", (int)nread, buf->base);
    }

    free(buf->base);
}

void alloc_buffer(uv_handle_t *handle, size_t suggested_size, uv_buf_t *buf) {
    buf->base = (char*)malloc(suggested_size);
    buf->len = suggested_size;
}

int main() {
    loop = uv_default_loop();
    uv_tcp_init(loop, &client);

    struct sockaddr_in addr;
    uv_ip4_addr("127.0.0.1", 12345, &addr);

    uv_connect_t *connect_req = (uv_connect_t*)malloc(sizeof(uv_connect_t));
    uv_tcp_connect(connect_req, &client, (const struct sockaddr*)&addr, on_connect);
    
    uv_read_start((uv_stream_t*)&client, alloc_buffer, on_read);
    
    return uv_run(loop, UV_RUN_DEFAULT);
}