

1. create table d0 using meters tags(7, "California.PaloAlto");
2. insert into d1 using meters tags(7, "California.PaloAlto") values(1.0, 10, 2.0);
insert into d1 using meters tags(7, "California.PaloAlto") values(now(), 1.0, 10, 2.0);
3. ?? select * from meters 
4. create database db vgroups 3 buffer 10 replica 3 keep 3650 duration 20;
5. create stable meters(ts timestamp, current float, voltage int, phase float) tags(groupid int, location varchar(24))  ;


select location, avg(current) from meters partition by location;
select location, avg(current) from meters group by location partition by location, groupid;


create stable meters(ts timestamp, current float, voltage float, phrase float) tags(groupid int, location varchar(64));

create table `meters` (`ts` TIMESTAMP,`id` INT,`voltage` INT,`v_blob` BINARY(32)) tags (`groupid` INT,`location` BINARY(128));


insert into d1 using meters tags(7, "California.PaloAlto") values(now, 1.0, 10, '123423');
insert into d1 using meters tags(7, "California.PaloAlto") values(now, 1.0, 10, '333333');


create stable meters(ts timestamp, current float, voltage float, phrase float) tags(groupid int, location varchar(64));


taos> desc st2;
             field              |          type          |   length    |        note        |     encode     |    compress    |     level      |
================================================================================================================================================
 ts                             | TIMESTAMP              |           8 |                    | delta-i        | lz4            | medium         |
 name                           | NCHAR                  |          10 |                    | disabled       | zstd           | medium         |
 addr                           | VARCHAR                |         128 |                    | disabled       | zstd           | medium         |
 val                            | INT                    |           4 | TAG                | disabled       | disabled       | disabled       |
Query OK, 4 row(s) in set (0.005372s)
