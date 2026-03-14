





windows 编译：

```
Installed 2 executables: kimi, kimi-cli 
warning: `C:\Users\Administrator.WIN-2OA23UM12TN\.local\bin` is not on your PATH. To use installed tools, run `$env:PATH = "C:\Users\Administrator.WIN-2OA23UM12TN\.local\bin;$env:PATH"` or `uv tool update
-shell`.      
```





```

PS C:\workspace\0\TDinternal> $env:PATH = "C:\Users\Administrator.WIN-2OA23UM12TN\.local\bin;$env:PATH"                                                                                                     
PS C:\workspace\0\TDinternal> kimi                                                                                                                                                                          
╭───────────────────────────────────────────────────────────────────────────────────╮                                                                                                                       
│                                                                                   │                                                                                                                       
│   ▐█▛█▛█▌  Welcome to Kimi Code CLI!                                              │                                                                                                                       
│   ▐█████▌  Send /help for help information.                                       │                                                                                                                       
│                                                                                   │                                                                                                                       
│  Directory: C:\workspace\0\TDinternal                                             │                                                                                                                       
│  Session: 5a009479-cdcb-4146-90ba-64f9f4665e36                                    │                                                                                                                       
│  Model: not set, send /login to login                                             │                                                                                                                       
│                                                                                   │                                                                                                                       
│  Tip: Kimi Code Web UI, a GUI version of Kimi Code, is now in technical preview.  │                                                                                                                       
│       Type /web to switch, or next time run `kimi web` directly.                  │                                                                                                                       
│                                                                                   │                                                                                                                       
╰───────────────────────────────────────────────────────────────────────────────────╯                                                                                                                       

Bye!
PS C:\workspace\0\TDinternal> uv tool update-shell
Updated PATH to include executable directory C:\Users\Administrator.WIN-2OA23UM12TN\.local\bin 
Restart your shell to apply changes                                                            
PS C:\workspace\0\TDinternal> Get-Command kimi 

CommandType     Name                                               Version    Source
-----------     ----                                               -------    ------
Application     kimi.exe                                           0.0.0.0    C:\Users\Administrator.WIN-2OA23UM12TN\.local\bin\kimi.exe

PS C:\workspace\0\TDinternal> kimi --version
kimi, version 1.21.0 
```









