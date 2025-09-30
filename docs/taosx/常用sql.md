```
select `precision`, `keep` from information_schema.ins_databases where name = database();
```


taosx 里的时间戳范围：最小不能小于 tsdb 设置的 keep 时间 之前，比如 keep 10d,10d,10d duration 1d; 插入的数据时间小于 now - 10d 就会报 Timestamp out of range 或者 the primary timestamp 1667677500000 overflow；其中 Timestamp out of range 是 tsdb 报错的；
大于 now + 365d 就会报： the primary timestamp 1880602200000 overflow


有一个时间戳不对，就会导致这一批的 sql 的数据都无法写入；
insert into t1 using meters tags(10) values(1759194000000, 11),(1880602200000000, 11);
```
taos> select * from meters;
           ts            |   current   |   groupid   |
======================================================
 2025-09-28 11:01:06.915 |          10 |          10 |
 2025-09-30 09:00:00.000 |          10 |          10 |
 2025-09-30 11:00:53.596 |          10 |          10 |
 2025-10-02 11:01:12.323 |          10 |          10 |
 2026-01-08 11:01:28.483 |          10 |          10 |
 2028-06-26 11:01:35.812 |          10 |          10 |
 2029-08-05 13:30:00.000 |          10 |          10 |
 2053-02-15 11:02:40.837 |          10 |          10 |
Query OK, 8 row(s) in set (0.007290s)

taos> insert into t1 using meters tags(10) values(1759194000000, 10),(1880602200000, 10);
Insert OK, 2 row(s) affected (0.001621s)

taos> insert into t1 using meters tags(10) values(1759194000000, 11),(1880602200000000, 11);

DB error: Timestamp data out of range [0x8000060B] (0.001148s)
taos> select * from meters;
           ts            |   current   |   groupid   |
======================================================
 2025-09-28 11:01:06.915 |          10 |          10 |
 2025-09-30 09:00:00.000 |          10 |          10 |
 2025-09-30 11:00:53.596 |          10 |          10 |
 2025-10-02 11:01:12.323 |          10 |          10 |
 2026-01-08 11:01:28.483 |          10 |          10 |
 2028-06-26 11:01:35.812 |          10 |          10 |
 2029-08-05 13:30:00.000 |          10 |          10 |
 2053-02-15 11:02:40.837 |          10 |          10 |
Query OK, 8 row(s) in set (0.007739s)

taos> insert into t1 using meters tags(10) values(1759194000000, 11);
Insert OK, 1 row(s) affected (0.001743s)

taos> select * from meters;
           ts            |   current   |   groupid   |
======================================================
 2025-09-28 11:01:06.915 |          10 |          10 |
 2025-09-30 09:00:00.000 |          11 |          10 |
 2025-09-30 11:00:53.596 |          10 |          10 |
 2025-10-02 11:01:12.323 |          10 |          10 |
 2026-01-08 11:01:28.483 |          10 |          10 |
 2028-06-26 11:01:35.812 |          10 |          10 |
 2029-08-05 13:30:00.000 |          10 |          10 |
 2053-02-15 11:02:40.837 |          10 |          10 |
Query OK, 8 row(s) in set (0.007321s)

```