

launch.json 配置：

```
        {
            "type": "lldb",
            "request": "launch",
            "name": "taos",
            "program": "${workspaceFolder}/debug/build/bin/taos",
            "args": [],
            "cwd": "${workspaceFolder}",
            "env": {
                "RUST_BACKTRACE": "1",
                "RUST_LOG": "debug",
                "TAOS_DATA_DIR": "/tmp/taos/",
                "LD_LIBRARY_PATH": "${workspaceFolder}/debug/build/lib/",
                "ASAN_OPTIONS": "detect_odr_violation=0"
            }
        },
        {
            "type": "lldb",
            "request": "launch",
            "name": "taosd",
            "program": "${workspaceFolder}/debug/build/bin/taosd",
            "args": [],
            "cwd": "${workspaceFolder}",
            "env": {
                "RUST_BACKTRACE": "1",
                "RUST_LOG": "debug",
                "TAOS_DATA_DIR": "/tmp/taos/",
                "LD_LIBRARY_PATH": "${workspaceFolder}/debug/build/lib/",
                "ASAN_OPTIONS": "detect_odr_violation=0"
            }
        }
```



调试控制台使用查看内存命令：

```
memory read -f s -count 30 pReq->pCont
```



