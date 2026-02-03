



内存泄露代码 经典代码：

```
      if (pOptions->parser) {
        taosMemFreeClear(pOptions->parser);
      }
      pOptions->parser = taosMemoryCalloc(1, pOptions->parserLen + 1);
```





