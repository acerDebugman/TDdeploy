



1. c 的对象释放，就是 array, hashMap, json 都是浅拷贝，存在 二级指针 的对象就需要自己写程序释放， array, hashmap 里的对象依赖容器自己释放。这样就容易区分 内存 指针的走向；

   写程序就心里就知道哪些指针在哪里共享，在哪里会被修改！







内存泄露代码：

```
      if (pOptions->parser) {
        taosMemFreeClear(pOptions->parser);
      }
      pOptions->parser = taosMemoryCalloc(1, pOptions->parserLen + 1);
```

