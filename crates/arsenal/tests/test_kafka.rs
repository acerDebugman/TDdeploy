// use std::sync::Arc;

// use chrono::{Local, Utc};
// use arrow::array::TimestampNanosecondBuilder;
// use arrow_schema::{Field, Schema};


// #[tokio::test]
// async fn test_record_batch() -> anyhow::Result<()> {
//     tracing_subscriber::fmt::fmt()
//         .with_max_level(tracing::Level::TRACE)
//         .init();

//     let parser = r#"{
//                 "global": {
//                         "cache": {
//                                 "max_size": "1GB",
//                                 "location": "",
//                                 "on_fail": "skip"
//                         },
//                         "archive": {
//                                 "keep_days": "30d",
//                                 "max_size": "1GB",
//                                 "location": "",
//                                 "on_fail": "rotate"
//                         },
//                         "database_connection_error": "cache",
//                         "database_not_exist": "break",
//                         "table_not_exist": "retry",
//                         "primary_timestamp_overflow": "archive",
//                         "primary_timestamp_null": "archive",
//                         "primary_key_null": "archive",
//                         "table_name_length_overflow": "archive",
//                         "table_name_contains_illegal_char": {
//                                 "replace_to": ""
//                         },
//                         "variable_not_exist_in_table_name_template": {
//                                 "replace_to": ""
//                         },
//                         "field_name_not_found": "add_field",
//                         "field_name_length_overflow": "archive",
//                         "field_length_extend": true,
//                         "field_length_overflow": "archive",
//                         "ingesting_error": "skip",
//                         "connection_timeout_in_second": "30s"
//                 },
//                 "parse": {
//                         "value": {
//                                 "json": [
//                                         "$[\"dataType\"]=dataType",
//                                         "$[\"dataTime\"]=dataTime",
//                                         "$[\"saveTime\"]=saveTime",
//                                         "$[\"vin\"]=vin",
//                                         "$[\"payload\"][\"detectProtocol\"]=detectProtocol",
//                                         "$[\"payload\"][\"milState\"]=milState",
//                                         "$[\"payload\"][\"detectState\"]=detectState",
//                                         "$[\"payload\"][\"detectReadyState\"]=detectReadyState",
//                                         "$[\"payload\"][\"identifyCode\"]=identifyCode",
//                                         "$[\"payload\"][\"idVersion\"]=idVersion",
//                                         "$[\"payload\"][\"calibrateVerify\"]=calibrateVerify",
//                                         "$[\"payload\"][\"IUPR\"]=IUPR",
//                                         "$[\"payload\"][\"errorCodeCount\"]=errorCodeCount",
//                                         "$[\"payload\"][\"errorCodes\"]=errorCodes"
//                                 ],
//                                 "depth": 2,
//                                 "keep": true
//                         }
//                 },
//                 "model": {
//                         "name": "hq_obd_${vin}",
//                         "using": "hq_obd",
//                         "tags": [
//                                 "vin"
//                         ],
//                         "columns": [
//                                 "ts",
//                                 "detect_protocol",
//                                 "mil_state",
//                                 "detect_state",
//                                 "detect_ready_state",
//                                 "identify_code",
//                                 "id_version",
//                                 "calibrate_verify",
//                                 "iupr",
//                                 "error_code_count",
//                                 "error_codes"
//                         ]
//                 },
//                 "mutate": [{
//                         "map": {
//                                 "ts": {
//                                         "cast": "dataTime",
//                                         "as": "TIMESTAMP(ms)"
//                                 },
//                                 "detect_protocol": {
//                                         "cast": "detectProtocol",
//                                         "as": "INT"
//                                 },
//                                 "mil_state": {
//                                         "cast": "milState",
//                                         "as": "INT"
//                                 },
//                                 "detect_state": {
//                                         "cast": "detectState",
//                                         "as": "INT"
//                                 },
//                                 "detect_ready_state": {
//                                         "cast": "detectReadyState",
//                                         "as": "INT"
//                                 },
//                                 "identify_code": {
//                                         "cast": "identifyCode",
//                                         "as": "VARCHAR"
//                                 },
//                                 "id_version": {
//                                         "cast": "idVersion",
//                                         "as": "VARCHAR"
//                                 },
//                                 "calibrate_verify": {
//                                         "cast": "calibrateVerify",
//                                         "as": "VARCHAR"
//                                 },
//                                 "iupr": {
//                                         "cast": "IUPR",
//                                         "as": "VARCHAR"
//                                 },
//                                 "error_code_count": {
//                                         "cast": "errorCodeCount",
//                                         "as": "INT"
//                                 },
//                                 "error_codes": {
//                                         "cast": "errorCodes",
//                                         "as": "VARCHAR"
//                                 },
//                                 "vin": {
//                                         "cast": "vin",
//                                         "as": "VARCHAR"
//                                 }
//                         }
//                 }]
//         }"#;

//         let parser = serde_json::from_str::<Parser>(parser)?;
//         // dbg!(&parser);

//         let gbk_bytes: &[u8] = &[
//                 0xD4, 0xDA, 0xB1, 0xB1, 0xBE, 0xA9, 0xD1, 0xA7, 0xCF, 0xB0, 0x72, 0x75, 0x73, 0x74,
//                 0xB1, 0xE0, 0xB3, 0xCC,
//         ];
//         let dt = Utc::now().format("%Y-%m-%d %H:%M:%S");
//         let save_dt = Local::now().format("%Y-%m-%d %H:%M:%S");
//         let message1 = serde_json::json!(
//                 {
//                 "dataType": "DATA_CVI_OBD",
//                 "dataTime": dt.to_string(),
//                 "saveTime": save_dt.to_string(),
//                 "vin": "YS2G6X237M5622467",
//                 "payload": {
//                         "detectProtocol": 12,
//                         "milState": 12,
//                         "detectState": 12,
//                         "detectReadyState": 12,
//                         "identifyCode": "LBZWANGXUD0NG0826",
//                         "idVersion": "200000000000000002",
//                         "calibrateVerify": "300000000000000003",
//                         "IUPR": "0000000000000000000000000000000",
//                         "errorCodeCount": 12,
//                         "errorCodes": ["0118002A", "0118002A", "0118002A"]
//                 }
//                 }
//         )
//         .to_string();

//         let dt = Local::now().format("%Y-%m-%d %H:%M:%S");
//         let message2 = serde_json::json!(
//                 {
//                 "dataType": "DATA_CVI_OBD",
//                 "dataTime": dt.to_string(),
//                 "saveTime": save_dt.to_string(),
//                 "vin": "YS2G6X237M5622467",
//                 "payload": {
//                         "detectProtocol": 3,
//                         "milState": 3,
//                         "detectState": 3,
//                         "detectReadyState": 3,
//                         "identifyCode": "LBZWANGXUD0NG0826",
//                         "idVersion": "200000000000000002",
//                         "calibrateVerify": "300000000000000003",
//                         "IUPR": "0000000000000000000000000000000",
//                         "errorCodeCount": 3,
//                         "errorCodes": ["0118002A", "0118002A", "0118002A"]
//                 }
//                 }
//         )
//         .to_string();
//         let mut msg_bytes = message2.as_bytes().to_vec();
//         msg_bytes.splice(140..140, gbk_bytes.to_vec());

//         // ("value", Binary, [message1.as_bytes(), &msg_bytes])
//         let batch = arrow::array::record_batch!(
//                 ("a", Utf8, ["123", "445"]),
//                 ("value", Binary, [&msg_bytes, &msg_bytes])
//         )
//         .unwrap();
//         // let batch = arrow::array::record_batch!(
//         //     ("value", Binary, [&msg_bytes, &msg_bytes])
//         // ).unwrap();

//         let rs = parser.transform_records(&batch)?;
//         println!("transformed records len: {}", rs.num_rows());
//         dbg!(&rs);
//         Ok(())
// }

// #[tokio::test]
// async fn test_record_batch_kafka() -> anyhow::Result<()> {
//         tracing_subscriber::fmt::fmt()
//         .with_max_level(tracing::Level::TRACE)
//         .init();

//         let parser = r#"{
//             "global": {
//                     "cache": {
//                             "max_size": "1GB",
//                             "location": "",
//                             "on_fail": "skip"
//                     },
//                     "archive": {
//                             "keep_days": "30d",
//                             "max_size": "1GB",
//                             "location": "",
//                             "on_fail": "rotate"
//                     },
//                     "database_connection_error": "cache",
//                     "database_not_exist": "break",
//                     "table_not_exist": "retry",
//                     "primary_timestamp_overflow": "archive",
//                     "primary_timestamp_null": "archive",
//                     "primary_key_null": "archive",
//                     "table_name_length_overflow": "archive",
//                     "table_name_contains_illegal_char": {
//                             "replace_to": ""
//                     },
//                     "variable_not_exist_in_table_name_template": {
//                             "replace_to": ""
//                     },
//                     "field_name_not_found": "add_field",
//                     "field_name_length_overflow": "archive",
//                     "field_length_extend": true,
//                     "field_length_overflow": "archive",
//                     "ingesting_error": "skip",
//                     "connection_timeout_in_second": "30s"
//             },
//             "parse": {
//                     "value": {
//                             "json": [
//                                     "$[\"dataType\"]=dataType",
//                                     "$[\"dataTime\"]=dataTime",
//                                     "$[\"saveTime\"]=saveTime",
//                                     "$[\"vin\"]=vin",
//                                     "$[\"payload\"][\"detectProtocol\"]=detectProtocol",
//                                     "$[\"payload\"][\"milState\"]=milState",
//                                     "$[\"payload\"][\"detectState\"]=detectState",
//                                     "$[\"payload\"][\"detectReadyState\"]=detectReadyState",
//                                     "$[\"payload\"][\"identifyCode\"]=identifyCode",
//                                     "$[\"payload\"][\"idVersion\"]=idVersion",
//                                     "$[\"payload\"][\"calibrateVerify\"]=calibrateVerify",
//                                     "$[\"payload\"][\"IUPR\"]=IUPR",
//                                     "$[\"payload\"][\"errorCodeCount\"]=errorCodeCount",
//                                     "$[\"payload\"][\"errorCodes\"]=errorCodes"
//                             ],
//                             "depth": 2,
//                             "keep": true
//                     }
//             },
//             "model": {
//                     "name": "hq_obd_${vin}",
//                     "using": "hq_obd",
//                     "tags": [
//                             "vin"
//                     ],
//                     "columns": [
//                             "ts",
//                             "detect_protocol",
//                             "mil_state",
//                             "detect_state",
//                             "detect_ready_state",
//                             "identify_code",
//                             "id_version",
//                             "calibrate_verify",
//                             "iupr",
//                             "error_code_count",
//                             "error_codes"
//                     ]
//             },
//             "mutate": [{
//                     "map": {
//                             "ts": {
//                                     "cast": "dataTime",
//                                     "as": "TIMESTAMP(ms)"
//                             },
//                             "detect_protocol": {
//                                     "cast": "detectProtocol",
//                                     "as": "INT"
//                             },
//                             "mil_state": {
//                                     "cast": "milState",
//                                     "as": "INT"
//                             },
//                             "detect_state": {
//                                     "cast": "detectState",
//                                     "as": "INT"
//                             },
//                             "detect_ready_state": {
//                                     "cast": "detectReadyState",
//                                     "as": "INT"
//                             },
//                             "identify_code": {
//                                     "cast": "identifyCode",
//                                     "as": "VARCHAR"
//                             },
//                             "id_version": {
//                                     "cast": "idVersion",
//                                     "as": "VARCHAR"
//                             },
//                             "calibrate_verify": {
//                                     "cast": "calibrateVerify",
//                                     "as": "VARCHAR"
//                             },
//                             "iupr": {
//                                     "cast": "IUPR",
//                                     "as": "VARCHAR"
//                             },
//                             "error_code_count": {
//                                     "cast": "errorCodeCount",
//                                     "as": "INT"
//                             },
//                             "error_codes": {
//                                     "cast": "errorCodes",
//                                     "as": "VARCHAR"
//                             },
//                             "vin": {
//                                     "cast": "vin",
//                                     "as": "VARCHAR"
//                             }
//                     }
//             }]
//         }"#;

//         let parser = serde_json::from_str::<Parser>(parser)?;
//         // dbg!(&parser);

//         let gbk_bytes: &[u8] = &[
//             0xD4, 0xDA, 0xB1, 0xB1, 0xBE, 0xA9, 0xD1, 0xA7, 0xCF, 0xB0, 0x72, 0x75, 0x73, 0x74,
//             0xB1, 0xE0, 0xB3, 0xCC,
//         ];
//         let dt = Utc::now().format("%Y-%m-%d %H:%M:%S");
//         let save_dt = Local::now().format("%Y-%m-%d %H:%M:%S");
//         let message1 = serde_json::json!(
//             {
//                 "dataType": "DATA_CVI_OBD",
//                 "dataTime": dt.to_string(),
//                 "saveTime": save_dt.to_string(),
//                 "vin": "YS2G6X237M5622467",
//                 "payload": {
//                     "detectProtocol": 12,
//                     "milState": 12,
//                     "detectState": 12,
//                     "detectReadyState": 12,
//                     "identifyCode": "LBZWANGXUD0NG0826",
//                     "idVersion": "200000000000000002",
//                     "calibrateVerify": "300000000000000003",
//                     "IUPR": "0000000000000000000000000000000",
//                     "errorCodeCount": 12,
//                     "errorCodes": ["0118002A", "0118002A", "0118002A"]
//                 }
//             }
//         ).to_string();

//         let dt = Local::now().format("%Y-%m-%d %H:%M:%S");
//         let message2 = serde_json::json!(
//             {
//                 "dataType": "DATA_CVI_OBD",
//                 "dataTime": dt.to_string(),
//                 "saveTime": save_dt.to_string(),
//                 "vin": "YS2G6X237M5622467",
//                 "payload": {
//                     "detectProtocol": 3,
//                     "milState": 3,
//                     "detectState": 3,
//                     "detectReadyState": 3,
//                     "identifyCode": "LBZWANGXUD0NG0826",
//                     "idVersion": "200000000000000002",
//                     "calibrateVerify": "300000000000000003",
//                     "IUPR": "0000000000000000000000000000000",
//                     "errorCodeCount": 3,
//                     "errorCodes": ["0118002A", "0118002A", "0118002A"]
//                 }
//             }
//         ).to_string();
//         let mut msg_bytes = message2.as_bytes().to_vec();
//         msg_bytes.splice(140..140, gbk_bytes.to_vec());

//         // ("value", Binary, [message1.as_bytes(), &msg_bytes])
//         // let batch = arrow::array::record_batch!(
//         //     ("a", Utf8, ["123", "445"]),
//         //     ("value", Binary, [&msg_bytes, &msg_bytes])
//         // ).unwrap();
//         let flat_columns = vec![
//             Field::new(
//                 "ts",
//                 DataType::Timestamp(arrow::datatypes::TimeUnit::Nanosecond, None),
//                 false,
//             ),
//             Field::new("topic", DataType::Utf8, false),
//             Field::new("partition", DataType::Int32, false),
//             Field::new("offset", DataType::Int64, false),
//             Field::new("key", DataType::Binary, true),
//             Field::new("value", DataType::Binary, true),
//         ];

//         let schema = Arc::new(Schema::new(flat_columns));

//         let mut timestamp = TimestampNanosecondBuilder::new();
//         let mut topic = StringBuilder::new();
//         let mut partition: arrow::array::PrimitiveBuilder<arrow::datatypes::Int32Type> =
//             Int32Builder::new();
//         let mut offset = Int64Builder::new();
//         let mut key = BinaryBuilder::new();
//         let mut value = BinaryBuilder::new();

//         timestamp.append_value(
//             Utc::now()
//                 .timestamp_nanos_opt()
//                 .expect("Get now timestamp in nanosecond should always success"),
//         );
//         topic.append_value("test1");
//         partition.append_value(0);
//         offset.append_value(203);
//         key.append_value(b"abc");
//         value.append_value(msg_bytes.as_slice());

//         timestamp.append_value(
//             Utc::now()
//                 .timestamp_nanos_opt()
//                 .expect("Get now timestamp in nanosecond should always success"),
//         );
//         topic.append_value("test1");
//         partition.append_value(0);
//         offset.append_value(203);
//         key.append_value(b"abc");
//         value.append_value(msg_bytes.as_slice());

//         let batch = arrow::array::RecordBatch::try_new(
//             schema.clone(),
//             vec![
//                 Arc::new(timestamp.finish()),
//                 Arc::new(topic.finish()),
//                 Arc::new(partition.finish()),
//                 Arc::new(offset.finish()),
//                 Arc::new(key.finish()),
//                 Arc::new(value.finish()),
//             ],
//         )?;

//         let rs = parser.transform_records(&batch)?; 
//         println!("transformed records len: {}", rs.num_rows());
//         dbg!(&rs);
//         Ok(())
// }


// #[test]
//     fn parse_json_array_test2() -> anyhow::Result<()> {
//         tracing_subscriber::fmt::fmt()
//             .with_max_level(tracing::Level::DEBUG)
//             .init();

//         let json = r#"{
//         "json": [
//             "$[\"dataType\"]=dataType",
//             "$[\"dataTime\"]=dataTime",
//             "$[\"saveTime\"]=saveTime",
//             "$[\"vin\"]=vin",
//             "$[\"payload\"][\"speed\"]=speed",
//             "$[\"payload\"][\"atmoPres\"]=atmoPres",
//             "$[\"payload\"][\"outPutWrest\"]=outPutWrest",
//             "$[\"payload\"][\"rubWrest\"]=rubWrest",
//             "$[\"payload\"][\"rev\"]=rev",
//             "$[\"payload\"][\"fuelVelocityFlow\"]=fuelVelocityFlow",
//             "$[\"payload\"][\"upperSCRNOxOutPut\"]=upperSCRNOxOutPut",
//             "$[\"payload\"][\"downSCRNOxOutPut\"]=downSCRNOxOutPut",
//             "$[\"payload\"][\"percentReactant\"]=percentReactant",
//             "$[\"payload\"][\"airInput\"]=airInput",
//             "$[\"payload\"][\"temperSCRInput\"]=temperSCRInput",
//             "$[\"payload\"][\"temperSCROutput\"]=temperSCROutput",
//             "$[\"payload\"][\"DPFDifferentPress\"]=DPFDifferentPress",
//             "$[\"payload\"][\"temperCoolant\"]=temperCoolant",
//             "$[\"payload\"][\"percentOil\"]=percentOil",
//             "$[\"payload\"][\"fixState\"]=fixState",
//             "$[\"payload\"][\"longitude\"]=longitude",
//             "$[\"payload\"][\"latitude\"]=latitude",
//             "$[\"payload\"][\"mileage\"]=mileage"
//         ],
//         "depth": 2,
//         "keep": true
//         }"#;
//         let json_parser: Json = serde_json::from_str(json).unwrap();
//         // dbg!(&json_parser);

//         let flat_columns = vec![
//             Field::new(
//                 "ts",
//                 DataType::Timestamp(arrow::datatypes::TimeUnit::Nanosecond, None),
//                 false,
//             ),
//             Field::new("key", DataType::Binary, true),
//             Field::new("value", DataType::Binary, false),
//         ];
//         let schema = Arc::new(Schema::new(flat_columns));

//         let mut timestamp = TimestampNanosecondBuilder::new();
//         let mut key = BinaryBuilder::new();
//         let mut value = BinaryBuilder::new();

//         let gbk_bytes: &[u8] = &[
//             0xD4, 0xDA, 0xB1, 0xB1, 0xBE, 0xA9, 0xD1, 0xA7, 0xCF, 0xB0, 0x72, 0x75, 0x73, 0x74,
//             0xB1, 0xE0, 0xB3, 0xCC,
//         ];

//         let dt = Local::now().format("%Y-%m-%d %H:%M:%S");
//         let save_dt = Local::now().format("%Y-%m-%d %H:%M:%S");
//         let message = serde_json::json!(
//             {
//                 "ts": Local::now().timestamp_millis(),
//                 "dataType": "DATA_CVI_OBD",
//                 "dataTime": dt.to_string(),
//                 "saveTime": save_dt.to_string(),
//                 "vin": "YS2G6X237M5622467",
//                 "payload": {
//                     "detectProtocol": 2,
//                     "milState": 10,
//                     "detectState": 2,
//                     "detectReadyState": 2,
//                     "identifyCode": "LBZWANGXUD0NG0826",
//                     "idVersion": "200000000000000002",
//                     "calibrateVerify": "300000000000000003",
//                     "IUPR": "0000000000000000000000000000000",
//                     "errorCodeCount": 1,
//                     "errorCodes": ["0118002A", "0118002A", "0118002A"]
//                 }
//             }
//         ).to_string();
//         let mut msg_bytes = message.as_bytes().to_vec();
//         msg_bytes.splice(140..140, gbk_bytes.to_vec());

//         timestamp.append_value(Local::now().timestamp_nanos_opt().unwrap());
//         key.append_value(b"key");
//         value.append_value(br#"{"dataType":"DATA_CVI_OBD","dataTime":"2025-06-27 10:49:48","saveTime":"2025-06-27 10:55:34","vin":"YS2G6X237M5622467","payload":{"detectProtocol":2,"milState":2,"detectState":2,"detectReadyState":2,"identifyCode":"LBZWANGXUD0NG0826","idVersion":"200000000000000002","calibrateVerify":"300000000000000003","IUPR":"000000000000000000000000000000000000","errorCodeCount":3,"errorCodes":["0118002A","0118002A","0118002A"]}}"#);

//         timestamp.append_value(Local::now().timestamp_nanos_opt().unwrap());
//         key.append_value(b"key");
//         value.append_value(msg_bytes);

//         let batch = RecordBatch::try_new(
//             schema.clone(),
//             vec![
//                 Arc::new(timestamp.finish()),
//                 Arc::new(key.finish()),
//                 Arc::new(value.finish()),
//             ],
//         )?;

//         let (_, field) = schema.fields().find("value").unwrap();
//         let array = batch.column_by_name("value").unwrap();

//         let (batch, _) = json_parser
//             .parse_array(field, array)
//             .map_err(|error| {
//                 panic!("json parser parse_array error: {:?}", error);
//             })
//             .unwrap();

//         dbg!(&batch);
//         // println!(
//         //     "\nfinal batch: {:?}",
//         //     arrow::util::pretty::pretty_format_batches(&[batch])?.to_string()
//         // );
//         assert_eq!(batch.num_rows(), 1);
//         Ok(())
//     }