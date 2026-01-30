



1.  decode 用的地方：
   1. 在写入的时候，新的数据会边编码为 SSdbRaw 类型，然后解码，去和老的数据比较，然后调用 update, insert, 等 FP
   2. 执行完后删除后，才会调用:  delete FP



2. encode 调用的地方：

   ```
   sdbFile.c 里 sdbWriteFileImp()
   ```

   



3. 还有个地方应该也调用了：decode FP， 应该是在从文件中加载 序列化数据 出来， 放到 HashMap 中的时候！否则 hashMap 就应该是空的



4. c 里的 hashmap 都是可以指向的！ 返回的指针，指向了 value , 但是 value 不可以随便释放，因为会释放具体的值，所以要使用 hashmap 专用的方法释放，但是读取是没有问题的！





5.  c 语言的 浅拷贝 和 深拷贝 非常重要 !!  放入 hashmap,  array 都是浅拷贝！ 所以如果要 深拷贝， 

   1. 要么序列化，
   2. 要么注意 深层次 的指针传递不要释放，
   3. 要么自己编写对应的 clone 函数

   



