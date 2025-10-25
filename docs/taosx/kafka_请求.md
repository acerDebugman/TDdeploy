
flat:
```
{
    "parser": {
        "parse": {
            "value": {
                "udt": "let v3 = data[\"location\"].name();\n\n[\n#{\"ts\": data[\"ts\"], \"val\": v3.sub_string(0,2), \"dev_id\": data[\"id\"]},\n#{\"ts\": data[\"ts\"], \"val\": v3.sub_string(2,2), \"dev_id\": data[\"id\"]},\n#{\"ts\": data[\"ts\"], \"val\": v3.sub_string(4,2), \"dev_id\": data[\"id\"]}\n]"
            }
        }
    },
    "input": [
        {
            "topic": "topic",
            "partition": "partition",
            "offset": "offset",
            "key": "key",
            "value": "{\"groupid\":1,\"id\":1,\"location\":\"BeiJing\",\"ts\":1756446333120,\"voltage\":1.7000000476837158}"
        },
        {
            "topic": "topic",
            "partition": "partition",
            "offset": "offset",
            "key": "key",
            "value": "{\"groupid\":0,\"id\":0,\"location\":\"BeiJing\",\"ts\":1756446334139,\"voltage\":3.700000047683716}"
        },
        {
            "topic": "topic",
            "partition": "partition",
            "offset": "offset",
            "key": "key",
            "value": "{\"groupid\":0,\"id\":0,\"location\":\"BeiJing\",\"ts\":1756446335668,\"voltage\":6.699999809265137}"
        },
        {
            "topic": "topic",
            "partition": "partition",
            "offset": "offset",
            "key": "key",
            "value": "{\"groupid\":1,\"id\":1,\"location\":\"BeiJing\",\"ts\":1756446336179,\"voltage\":7.699999809265137}"
        },
        {
            "topic": "topic",
            "partition": "partition",
            "offset": "offset",
            "key": "key",
            "value": "{\"groupid\":0,\"id\":0,\"location\":\"BeiJing\",\"ts\":1756446337199,\"voltage\":9.699999809265137}"
        }
    ]
}
```

返回:

```
[
    {
        "fields": [
            {
                "name": "topic",
                "scope": "Unspecified",
                "type": "varchar(128)",
                "arrow_type": "Utf8"
            },
            {
                "name": "partition",
                "scope": "Unspecified",
                "type": "varchar(128)",
                "arrow_type": "Utf8"
            },
            {
                "name": "offset",
                "scope": "Unspecified",
                "type": "varchar(128)",
                "arrow_type": "Utf8"
            },
            {
                "name": "key",
                "scope": "Unspecified",
                "type": "varchar(128)",
                "arrow_type": "Utf8"
            },
            {
                "name": "dev_id",
                "scope": "Unspecified",
                "type": "bigint",
                "arrow_type": "Int64"
            },
            {
                "name": "ts",
                "scope": "Unspecified",
                "type": "bigint",
                "arrow_type": "Int64"
            },
            {
                "name": "val",
                "scope": "Unspecified",
                "type": "varchar(128)",
                "arrow_type": "Utf8"
            }
        ],
        "columns": [
            [
                "topic",
                "partition",
                "offset",
                "key",
                1,
                1756446333120,
                "Be"
            ],
            [
                "topic",
                "partition",
                "offset",
                "key",
                1,
                1756446333120,
                "iJ"
            ],
            [
                "topic",
                "partition",
                "offset",
                "key",
                1,
                1756446333120,
                "in"
            ],
            [
                "topic",
                "partition",
                "offset",
                "key",
                0,
                1756446334139,
                "Be"
            ],
            [
                "topic",
                "partition",
                "offset",
                "key",
                0,
                1756446334139,
                "iJ"
            ],
            [
                "topic",
                "partition",
                "offset",
                "key",
                0,
                1756446334139,
                "in"
            ],
            [
                "topic",
                "partition",
                "offset",
                "key",
                0,
                1756446335668,
                "Be"
            ],
            [
                "topic",
                "partition",
                "offset",
                "key",
                0,
                1756446335668,
                "iJ"
            ],
            [
                "topic",
                "partition",
                "offset",
                "key",
                0,
                1756446335668,
                "in"
            ],
            [
                "topic",
                "partition",
                "offset",
                "key",
                1,
                1756446336179,
                "Be"
            ],
            [
                "topic",
                "partition",
                "offset",
                "key",
                1,
                1756446336179,
                "iJ"
            ],
            [
                "topic",
                "partition",
                "offset",
                "key",
                1,
                1756446336179,
                "in"
            ],
            [
                "topic",
                "partition",
                "offset",
                "key",
                0,
                1756446337199,
                "Be"
            ],
            [
                "topic",
                "partition",
                "offset",
                "key",
                0,
                1756446337199,
                "iJ"
            ],
            [
                "topic",
                "partition",
                "offset",
                "key",
                0,
                1756446337199,
                "in"
            ]
        ]
    }
]
```
