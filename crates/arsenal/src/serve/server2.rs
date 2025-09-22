/*
1. 测试了 do_get, do_put, do_exchange, do_action
2. do_action 需要先使用 list_actions 来获取支持的 action 类型
3. do_get 需要先 get_flight_info 来获取 schema + ticket， 发送 get_flight_info 客户端需要先准备 FlightDescriptor, 内部有 cmd;
do_get 方法两阶段的，主要用于多个服务器的场景，实现 do_flight_info 的服务器 类似一个资源注册服务器，
返回的 endpoint 才会被客户端跳转去获取实现 do_get 服务器的资源
 */
pub mod do_get {
    use arrow::array::{Int64Array, RecordBatch};
    use arrow::datatypes::{DataType, Field, Schema};
    use arrow_flight::{PollInfo, SchemaResult};
    use arrow_flight::{
        flight_service_server::FlightService,
        Action, ActionType, Criteria, Empty, FlightData, FlightDescriptor, FlightInfo,
        HandshakeRequest, HandshakeResponse, PutResult, Ticket,
    };
    use futures::stream::BoxStream;
    use std::sync::Arc;
    use tonic::{Request, Response, Status, Streaming};

    #[derive(Debug, Default)]
    pub struct MyFlightService;

    // type BoxStream<T> = Pin<Box<dyn Stream<Item = Result<T, Status>> + Send>>;

    #[tonic::async_trait]
    impl FlightService for MyFlightService {
        type HandshakeStream = BoxStream<'static, Result<HandshakeResponse, Status>>;
        type ListFlightsStream = BoxStream<'static, Result<FlightInfo, Status>>;
        type DoGetStream = BoxStream<'static, Result<FlightData, Status>>;
        type DoPutStream = BoxStream<'static, Result<PutResult, Status>>;
        type DoActionStream = BoxStream<'static, Result<arrow_flight::Result, Status>>;
        type ListActionsStream = BoxStream<'static, Result<ActionType, Status>>;
        type DoExchangeStream = BoxStream<'static, Result<FlightData, Status>>;

        /* ---------- 1. 握手（可放鉴权） ---------- */
        async fn handshake(
            &self,
            _request: Request<Streaming<HandshakeRequest>>,
        ) -> Result<Response<Self::HandshakeStream>, Status> {
            let resp = HandshakeResponse::default();
            Ok(Response::new(Box::pin(futures::stream::iter(vec![Ok(resp)]))))
        }

        /* ---------- 2. 根据请求返回 FlightInfo（包含 schema + ticket） ---------- */
        async fn get_flight_info(
            &self,
            _request: Request<FlightDescriptor>,
        ) -> Result<Response<FlightInfo>, Status> {
            let schema = Arc::new(Schema::new(vec![
                Field::new("id", DataType::Int64, false),
                Field::new("value", DataType::Int64, false),
            ]));
            // 生成一个 ticket，客户端后续凭它 do_get
            let ticket = Ticket {
                ticket: bytes::Bytes::from(b"demo_data".to_vec()),
            };
            let info = FlightInfo::new()
                .try_with_schema(&schema)
                .map_err(|e| Status::internal(e.to_string()))?
                .with_endpoint(
                    arrow_flight::FlightEndpoint::new()
                        .with_ticket(ticket)
                        .with_location("grpc://127.0.0.1:50051"),
                );
            Ok(Response::new(info))
        }

        

        /* ---------- 3. 真正的数据流 ---------- */
        async fn do_get(
            &self,
            _request: Request<Ticket>,
        ) -> Result<Response<Self::DoGetStream>, Status> {
            println!("do_get: {:?}", _request);
            let schema = Arc::new(Schema::new(vec![
                Field::new("id", DataType::Int64, false),
                Field::new("value", DataType::Int64, false),
            ]));
            // 造 2 个 RecordBatch 当演示
            let id_array = Arc::new(Int64Array::from(vec![1, 2, 3]));
            let val_array = Arc::new(Int64Array::from(vec![10, 20, 30]));
            let batch =
                RecordBatch::try_new(schema.clone(), vec![id_array, val_array]).unwrap();
            // let batch = RecordBatch::new_empty(schema);

            // 创建元信息
            let metadata = serde_json::json!({
                "source": "MyFlightService",
                "timestamp": chrono::Utc::now().to_rfc3339(),
                "row_count": 3,
                "query_id": "demo_query_001"
            });

            let metadata_bytes = serde_json::to_vec(&metadata).unwrap();

            let mut flight_data: Vec<FlightData> =
                arrow_flight::utils::batches_to_flight_data(&batch.schema(), vec![batch])
                    .map_err(|e| Status::internal(e.to_string()))?;
            // 在这里就会产生2个 FlightData, 第一个就是 schema 信息，具体看 batches_to_flight_data 里的实现
            // 注意和下面的 meta 信息区别开，meta 信息可以每个 FlightData 都带上
            // 第二个才是数据
            println!("flight_data len: {}, flight_data = {:?}", flight_data.len(), flight_data);

            // if flight_data.len() > 1 {
            //     flight_data.truncate(1);
            // }

            for data in flight_data.iter_mut() {
                data.app_metadata = metadata_bytes.clone().into();
            }
            let stream = futures::stream::iter(flight_data.into_iter().map(Ok));
            Ok(Response::new(Box::pin(stream)))
        }

        // 这里测试 do_put 方法
        // async fn do_put(
        //     &self,
        //     request: Request<Streaming<FlightData>>,
        // ) -> Result<Response<Self::DoPutStream>, Status> {
        //     let mut stream = request.into_inner();
        //     let mut schema: Option<Arc<Schema>> = None;
        //     let mut total_rows = 0usize;

        //     // 逐条 FlightData 处理
        //     while let Some(flight_data) = stream.next().await {
        //         let data = flight_data?;
        //         // 第一条含 schema
        //         if data.data_header.len() > 0 && schema.is_none() {
        //             schema = Some(Arc::new(
        //                 arrow_flight::utils::flight_data_to_schema(&data)
        //                     .map_err(|e| Status::internal(e.to_string()))?,
        //             ));
        //             println!("[Server] received schema: {:?}", schema);
        //         }
        //         // 含 body 的批次
        //         if !data.data_body.is_empty() {
        //             let batch = arrow_flight::utils::flight_data_to_arrow_batch(
        //                 &data,
        //                 schema.clone().unwrap(),
        //                 &[],
        //             )
        //             .map_err(|e| Status::internal(e.to_string()))?;
        //             total_rows += batch.num_rows();
        //             println!("[Server] received batch: {} rows", batch.num_rows());
        //         }
        //     }

        //     // 返回确认结果
        //     let result = PutResult {
        //         app_metadata: format!("uploaded {} rows", total_rows).into_bytes(),
        //     };
        //     let output = futures::stream::iter(vec![Ok(result)]);
        //     Ok(Response::new(Box::pin(output)))
        // }

        /* 其余方法用默认实现即可 */
        async fn poll_flight_info(
            &self,
            _request: Request<FlightDescriptor>,
        ) -> Result<Response<PollInfo>, Status> {
            Err(Status::unimplemented("Implement poll_flight_info"))
        }

        async fn get_schema(
            &self,
            _request: Request<FlightDescriptor>,
        ) -> Result<Response<SchemaResult>, Status> {
            Err(Status::unimplemented("Implement get_schema"))
        }

        async fn do_put(
            &self,
            _request: Request<Streaming<FlightData>>,
        ) -> Result<Response<Self::DoPutStream>, Status> {
            unimplemented!()
        }
        async fn do_action(
            &self,
            _request: Request<Action>,
        ) -> Result<Response<Self::DoActionStream>, Status> {
            unimplemented!()
        }
        async fn list_actions(
            &self,
            _request: Request<Empty>,
        ) -> Result<Response<Self::ListActionsStream>, Status> {
            unimplemented!()
        }
        async fn list_flights(
            &self,
            _request: Request<Criteria>,
        ) -> Result<Response<Self::ListFlightsStream>, Status> {
            unimplemented!()
        }
        async fn do_exchange(
            &self,
            _request: Request<Streaming<FlightData>>,
        ) -> Result<Response<Self::DoExchangeStream>, Status> {
            Err(Status::unimplemented("Implement do_exchange"))
        }
    }

    pub async fn start_server() -> anyhow::Result<()> {
        let addr = "127.0.0.1:50051".parse()?;
        let service = MyFlightService::default();
        println!("Arrow Flight server listening on {}", addr);

        tonic::transport::Server::builder()
            .add_service(arrow_flight::flight_service_server::FlightServiceServer::new(service))
            .serve(addr)
            .await?;
        Ok(())
    }
}

pub mod do_put {
    use arrow::datatypes::{DataType, Field, Schema};
    use arrow_flight::{PollInfo, SchemaResult};
    use arrow_flight::{
        flight_service_server::{FlightService, FlightServiceServer},
        Action, ActionType, Criteria, Empty, FlightData, FlightDescriptor, FlightInfo,
        HandshakeRequest, HandshakeResponse, PutResult, Ticket,
    };
    use futures::stream::BoxStream;
    use tokio_stream::StreamExt;
    use std::collections::HashMap;
    use std::sync::Arc;
    use tonic::{transport::Server, Request, Response, Status, Streaming};

    #[derive(Debug, Default)]
    pub struct MyFlightService;

    #[tonic::async_trait]
    impl FlightService for MyFlightService {
        type HandshakeStream = BoxStream<'static, Result<HandshakeResponse, Status>>;
        type ListFlightsStream = BoxStream<'static, Result<FlightInfo, Status>>;
        type DoGetStream = BoxStream<'static, Result<FlightData, Status>>;
        type DoPutStream = BoxStream<'static, Result<PutResult, Status>>;
        type DoActionStream = BoxStream<'static, Result<arrow_flight::Result, Status>>;
        type ListActionsStream = BoxStream<'static, Result<ActionType, Status>>;
        type DoExchangeStream = BoxStream<'static, Result<FlightData, Status>>;

        /* ---------- 1. 握手（可放鉴权） ---------- */
        async fn handshake(
            &self,
            _request: Request<Streaming<HandshakeRequest>>,
        ) -> Result<Response<Self::HandshakeStream>, Status> {
            let resp = HandshakeResponse::default();
            Ok(Response::new(Box::pin(futures::stream::iter(vec![Ok(resp)]))))
        }

        /* ---------- 2. 根据请求返回 FlightInfo（包含 schema + ticket） ---------- */
        async fn get_flight_info(
            &self,
            _request: Request<FlightDescriptor>,
        ) -> Result<Response<FlightInfo>, Status> {
            let schema = Arc::new(Schema::new(vec![
                Field::new("id", DataType::Int64, false),
                Field::new("value", DataType::Int64, false),
            ]));
            // 生成一个 ticket，客户端后续凭它 do_get
            let ticket = Ticket {
                ticket: bytes::Bytes::from(b"demo_data".to_vec()),
            };
            let info = FlightInfo::new()
                .try_with_schema(&schema)
                .map_err(|e| Status::internal(e.to_string()))?
                .with_endpoint(
                    arrow_flight::FlightEndpoint::new()
                        .with_ticket(ticket)
                        .with_location("grpc://127.0.0.1:50051"),
                );
            Ok(Response::new(info))
        }

        /* ---------- 3. 真正的数据流 ---------- */
        // 这里测试 do_put 方法
        async fn do_put(
            &self,
            request: Request<Streaming<FlightData>>,
        ) -> Result<Response<Self::DoPutStream>, Status> {
            let mut stream = request.into_inner();
            let mut schema: Option<Arc<Schema>> = None;
            let mut total_rows = 0usize;

            // 逐条 FlightData 处理
            while let Some(flight_data) = stream.next().await {
                let data = flight_data?;
                // 第一条含 schema
                if data.data_header.len() > 0 && schema.is_none() {
                    schema = Some(
                        crate::serve::server2::flight_data_to_schema(&data)
                            .map_err(|e| Status::internal(e.to_string())).unwrap(),
                    );
                    println!("[Server] received schema: {:?}", schema);
                }
                // 含 body 的批次
                if !data.data_body.is_empty() {
                    let batch = arrow_flight::utils::flight_data_to_arrow_batch(
                        &data,
                        schema.clone().unwrap(),
                        &HashMap::new(),
                    )
                    .map_err(|e| Status::internal(e.to_string()))?;
                    total_rows += batch.num_rows();
                    println!("[Server] received batch: {} rows", batch.num_rows());
                }
            }

            // 返回确认结果
            let result = PutResult {
                app_metadata: bytes::Bytes::from(format!("uploaded {} rows", total_rows).into_bytes()),
            };
            let output = futures::stream::iter(vec![Ok(result)]);
            Ok(Response::new(Box::pin(output)))
        }

        /* 其余方法用默认实现即可 */
        async fn poll_flight_info(
            &self,
            _request: Request<FlightDescriptor>,
        ) -> Result<Response<PollInfo>, Status> {
            Err(Status::unimplemented("Implement poll_flight_info"))
        }

        async fn get_schema(
            &self,
            _request: Request<FlightDescriptor>,
        ) -> Result<Response<SchemaResult>, Status> {
            Err(Status::unimplemented("Implement get_schema"))
        }

        async fn do_get(
            &self,
            _request: Request<Ticket>,
        ) -> Result<Response<Self::DoGetStream>, Status> {
            Err(Status::unimplemented("Implement do_get"))
        }

        // async fn do_put(
        //     &self,
        //     _request: Request<Streaming<FlightData>>,
        // ) -> Result<Response<Self::DoPutStream>, Status> {
        //     unimplemented!()
        // }
        async fn do_action(
            &self,
            _request: Request<Action>,
        ) -> Result<Response<Self::DoActionStream>, Status> {
            unimplemented!()
        }
        async fn list_actions(
            &self,
            _request: Request<Empty>,
        ) -> Result<Response<Self::ListActionsStream>, Status> {
            unimplemented!()
        }
        async fn list_flights(
            &self,
            _request: Request<Criteria>,
        ) -> Result<Response<Self::ListFlightsStream>, Status> {
            unimplemented!()
        }
        async fn do_exchange(
            &self,
            _request: Request<Streaming<FlightData>>,
        ) -> Result<Response<Self::DoExchangeStream>, Status> {
            Err(Status::unimplemented("Implement do_exchange"))
        }
    }

    pub async fn start_server() -> anyhow::Result<()> {
        let addr = "127.0.0.1:50051".parse()?;
        let service = MyFlightService::default();
        println!("Arrow Flight server listening on {}", addr);

        tonic::transport::Server::builder()
            .add_service(arrow_flight::flight_service_server::FlightServiceServer::new(service))
            .serve(addr)
            .await?;
        Ok(())
    }
}


pub mod do_exchange {
    use arrow::array::{Int64Array, RecordBatch};
    use arrow::datatypes::{DataType, Field, Schema};
    use arrow_flight::{PollInfo, SchemaResult};
    use arrow_flight::{
        flight_service_server::{FlightService, FlightServiceServer},
        Action, ActionType, Criteria, Empty, FlightData, FlightDescriptor, FlightInfo,
        HandshakeRequest, HandshakeResponse, PutResult, Ticket,
    };
    use futures::stream::BoxStream;
    use tokio_stream::StreamExt;
    use std::collections::HashMap;
    use std::sync::Arc;
    use tonic::{transport::Server, Request, Response, Status, Streaming};

    #[derive(Debug, Default)]
    pub struct MyFlightService;

    #[tonic::async_trait]
    impl FlightService for MyFlightService {
        type HandshakeStream = BoxStream<'static, Result<HandshakeResponse, Status>>;
        type ListFlightsStream = BoxStream<'static, Result<FlightInfo, Status>>;
        type DoGetStream = BoxStream<'static, Result<FlightData, Status>>;
        type DoPutStream = BoxStream<'static, Result<PutResult, Status>>;
        type DoActionStream = BoxStream<'static, Result<arrow_flight::Result, Status>>;
        type ListActionsStream = BoxStream<'static, Result<ActionType, Status>>;
        type DoExchangeStream = BoxStream<'static, Result<FlightData, Status>>;

        /* ---------- 1. 握手（可放鉴权） ---------- */
        async fn handshake(
            &self,
            _request: Request<Streaming<HandshakeRequest>>,
        ) -> Result<Response<Self::HandshakeStream>, Status> {
            let resp = HandshakeResponse::default();
            Ok(Response::new(Box::pin(futures::stream::iter(vec![Ok(resp)]))))
        }

        /* ---------- 2. 根据请求返回 FlightInfo（包含 schema + ticket） ---------- */
        // 客户端也可以不用请求
        async fn get_flight_info(
            &self,
            _request: Request<FlightDescriptor>,
        ) -> Result<Response<FlightInfo>, Status> {
            let schema = Arc::new(Schema::new(vec![
                Field::new("id", DataType::Int64, false),
                Field::new("value", DataType::Int64, false),
            ]));
            // 生成一个 ticket，客户端后续凭它 do_get
            let ticket = Ticket {
                ticket: bytes::Bytes::from(b"demo_data".to_vec()),
            };
            let info = FlightInfo::new()
                .try_with_schema(&schema)
                .map_err(|e| Status::internal(e.to_string()))?
                .with_endpoint(
                    arrow_flight::FlightEndpoint::new()
                        .with_ticket(ticket)
                        .with_location("grpc://127.0.0.1:50051"),
                );
            Ok(Response::new(info))
        }

        /* ---------- 3. 真正的数据流 ---------- */
        // 这里测试 do_exchange 方法
        async fn do_exchange(
            &self,
            request: Request<Streaming<FlightData>>,
        ) -> Result<Response<Self::DoExchangeStream>, Status> {
            let mut upstream = request.into_inner();

            // 先收 Schema（第一条 FlightData）
            let schema_header = upstream
                .next()
                .await
                .ok_or_else(|| Status::internal("missing schema"))??;
            let schema = crate::serve::server2::flight_data_to_schema(&schema_header)
                    .map_err(|e| Status::internal(e.to_string()))?;
            println!("[Server] received schema: {:?}", schema);

            // 构造输出 Schema：原列 + 2 列汇总
            let out_schema = Arc::new(Schema::new(vec![
                Field::new("rows", DataType::Int64, false),
                Field::new("sum_id", DataType::Int64, false),
            ]));

            // 返回流：先吐 Schema header
            let schema_flight = arrow_flight::utils::batches_to_flight_data(
                out_schema.as_ref(),
                Default::default(),
            ).unwrap().remove(0); //这里 remove 0 是因为这就是一个 schema batch，协议只用发一个 schema 就可以, 没有第二个
            let (tx, rx) = tokio::sync::mpsc::channel::<Result<FlightData, Status>>(4);
            tx.send(Ok(schema_flight)).await.unwrap();

            // 后台任务：边收边算，边回传结果
            let out_schema_clone = out_schema.clone();
            tokio::spawn(async move {
                let mut total_rows = 0i64;
                let mut sum_id = 0i64;
                while let Some(flight_data) = upstream.next().await {
                    let data = flight_data.unwrap();
                    if data.data_body.is_empty() {
                        continue; // 纯 header 帧
                    }
                    let batch = arrow_flight::utils::flight_data_to_arrow_batch(
                        &data,
                        schema.clone(),
                        &HashMap::new(),
                    )
                    .unwrap();
                    let rows = batch.num_rows() as i64;
                    let id_array = batch
                        .column(0)
                        .as_any()
                        .downcast_ref::<Int64Array>()
                        .unwrap();
                    let sum: i64 = id_array.iter().map(|v| v.unwrap_or(0)).sum();
                    total_rows += rows;
                    sum_id += sum;

                    // 每收到一批，立刻返回一个汇总 batch
                    let out_batch = vec![RecordBatch::try_new(
                        out_schema_clone.clone(),
                        vec![
                            Arc::new(Int64Array::from(vec![rows])),
                            Arc::new(Int64Array::from(vec![sum])),
                        ],
                    ).unwrap()];
                    let flight_resp = arrow_flight::utils::batches_to_flight_data(
                        &out_schema_clone.clone(),
                        out_batch,
                    ).unwrap();
                    for fd in flight_resp {
                        tx.send(Ok(fd)).await.unwrap();
                    }
                }
                // 最后再给一次全局汇总（可选）
                let final_batch = RecordBatch::try_new(
                    out_schema_clone.clone(),
                    vec![
                        Arc::new(Int64Array::from(vec![total_rows])),
                        Arc::new(Int64Array::from(vec![sum_id])),
                    ],
                )
                .unwrap();
                let final_flight =
                    arrow_flight::utils::batches_to_flight_data(&out_schema_clone, vec![final_batch]).unwrap();
                for fd in final_flight {
                    tx.send(Ok(fd)).await.unwrap();
                }
            });

            Ok(Response::new(Box::pin(
                tokio_stream::wrappers::ReceiverStream::new(rx),
            )))

        }

        /* 其余方法用默认实现即可 */
        async fn poll_flight_info(
            &self,
            _request: Request<FlightDescriptor>,
        ) -> Result<Response<PollInfo>, Status> {
            Err(Status::unimplemented("Implement poll_flight_info"))
        }

        async fn get_schema(
            &self,
            _request: Request<FlightDescriptor>,
        ) -> Result<Response<SchemaResult>, Status> {
            Err(Status::unimplemented("Implement get_schema"))
        }

        async fn do_get(
            &self,
            _request: Request<Ticket>,
        ) -> Result<Response<Self::DoGetStream>, Status> {
            Err(Status::unimplemented("Implement do_get"))
        }

        async fn do_put(
            &self,
            _request: Request<Streaming<FlightData>>,
        ) -> Result<Response<Self::DoPutStream>, Status> {
            unimplemented!()
        }
        async fn do_action(
            &self,
            _request: Request<Action>,
        ) -> Result<Response<Self::DoActionStream>, Status> {
            unimplemented!()
        }
        async fn list_actions(
            &self,
            _request: Request<Empty>,
        ) -> Result<Response<Self::ListActionsStream>, Status> {
            unimplemented!()
        }
        async fn list_flights(
            &self,
            _request: Request<Criteria>,
        ) -> Result<Response<Self::ListFlightsStream>, Status> {
            unimplemented!()
        }
        // async fn do_exchange(
        //     &self,
        //     _request: Request<Streaming<FlightData>>,
        // ) -> Result<Response<Self::DoExchangeStream>, Status> {
        //     Err(Status::unimplemented("Implement do_exchange"))
        // }
    }

    pub async fn start_server() -> anyhow::Result<()> {
        let addr = "127.0.0.1:50051".parse()?;
        let service = MyFlightService::default();
        println!("Arrow Flight server listening on {}", addr);

        tonic::transport::Server::builder()
            .add_service(arrow_flight::flight_service_server::FlightServiceServer::new(service))
            .serve(addr)
            .await?;
        Ok(())
    }
}


pub mod do_action {
    use arrow::array::{Int64Array, RecordBatch};
    use arrow::datatypes::{DataType, Field, Schema};
    use arrow_flight::{PollInfo, SchemaResult};
    use arrow_flight::{
        flight_service_server::{FlightService, FlightServiceServer},
        Action, ActionType, Criteria, Empty, FlightData, FlightDescriptor, FlightInfo,
        HandshakeRequest, HandshakeResponse, PutResult, Ticket,
    };
    use futures::stream::BoxStream;
    use tokio_stream::StreamExt;
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};
    use tonic::{transport::Server, Request, Response, Status, Streaming};

    #[derive(Debug, Default)]
    pub struct MyFlightService {
        // 内存表：演示 drop_table
        tables: Arc<Mutex<std::collections::HashMap<String, Vec<u8>>>>,
    }

    impl MyFlightService {
        fn drop_table(&self, name: &str) -> Result<String, Status> {
            let mut t = self.tables.lock().unwrap();
            if t.remove(name).is_some() {
                Ok(format!("table '{}' dropped", name))
            } else {
                Err(Status::not_found(format!("table '{}' not found", name)))
            }
        }
    
        fn shutdown(&self) -> Result<String, Status> {
            // 实际生产里这里发退出信号；示例直接返回
            Ok("server shutting down".to_string())
        }
    }

    #[tonic::async_trait]
    impl FlightService for MyFlightService {
        type HandshakeStream = BoxStream<'static, Result<HandshakeResponse, Status>>;
        type ListFlightsStream = BoxStream<'static, Result<FlightInfo, Status>>;
        type DoGetStream = BoxStream<'static, Result<FlightData, Status>>;
        type DoPutStream = BoxStream<'static, Result<PutResult, Status>>;
        type DoActionStream = BoxStream<'static, Result<arrow_flight::Result, Status>>;
        type ListActionsStream = BoxStream<'static, Result<ActionType, Status>>;
        type DoExchangeStream = BoxStream<'static, Result<FlightData, Status>>;

        /* ---------- 1. 握手（可放鉴权） ---------- */
        async fn handshake(
            &self,
            _request: Request<Streaming<HandshakeRequest>>,
        ) -> Result<Response<Self::HandshakeStream>, Status> {
            let resp = HandshakeResponse::default();
            Ok(Response::new(Box::pin(futures::stream::iter(vec![Ok(resp)]))))
        }

        // 客户端也可以不用请求
        async fn get_flight_info(
            &self,
            _request: Request<FlightDescriptor>,
        ) -> Result<Response<FlightInfo>, Status> {
            unimplemented!()
        }

        /* ---------- 2. 列出所有支持的 action ---------- */
        async fn list_actions(
            &self,
            _request: Request<Empty>,
        ) -> Result<Response<Self::ListActionsStream>, Status> {
            let actions = vec![
                ActionType {
                    r#type: "drop_table".to_string(),
                    description: "Delete a table from memory".to_string(),
                },
                ActionType {
                    r#type: "shutdown".to_string(),
                    description: "Graceful shutdown".to_string(),
                },
            ];
            let output = futures::stream::iter(actions.into_iter().map(Ok));
            Ok(Response::new(Box::pin(output)))
        }

        /* ---------- 3. 真正的数据流 ---------- */
        // 这里测试 do_action 方法
        async fn do_action(
            &self,
            request: Request<Action>,
        ) -> Result<Response<Self::DoActionStream>, Status> {
            let action = request.into_inner();
            let results: Vec<Result<arrow_flight::Result, Status>> = match action.r#type.as_str() {
                "drop_table" => {
                    let table_name = String::from_utf8_lossy(&action.body);
                    match self.drop_table(&table_name) {
                        Ok(msg) => vec![Ok(arrow_flight::Result { body: bytes::Bytes::from(msg.into_bytes()) })],
                        Err(e) => vec![Err(e)],
                    }
                }
                "shutdown" => {
                    // 可以返回多条结果
                    vec![
                        Ok(arrow_flight::Result {
                            body: bytes::Bytes::from(b"shutting down...".to_vec()),
                        }),
                        Ok(arrow_flight::Result {
                            body: bytes::Bytes::from(b"bye".to_vec()),
                        }),
                    ]
                }
                _ => vec![Err(Status::invalid_argument(format!(
                    "unknown action: {}",
                    action.r#type
                )))],
            };
            let output = futures::stream::iter(results);
            Ok(Response::new(Box::pin(output)))

        }

        
        /* 其余方法用默认实现即可 */
        async fn poll_flight_info(
            &self,
            _request: Request<FlightDescriptor>,
        ) -> Result<Response<PollInfo>, Status> {
            Err(Status::unimplemented("Implement poll_flight_info"))
        }

        async fn get_schema(
            &self,
            _request: Request<FlightDescriptor>,
        ) -> Result<Response<SchemaResult>, Status> {
            Err(Status::unimplemented("Implement get_schema"))
        }

        async fn do_get(
            &self,
            _request: Request<Ticket>,
        ) -> Result<Response<Self::DoGetStream>, Status> {
            Err(Status::unimplemented("Implement do_get"))
        }

        async fn do_put(
            &self,
            _request: Request<Streaming<FlightData>>,
        ) -> Result<Response<Self::DoPutStream>, Status> {
            unimplemented!()
        }
        // async fn do_action(
        //     &self,
        //     _request: Request<Action>,
        // ) -> Result<Response<Self::DoActionStream>, Status> {
        //     unimplemented!()
        // }
        // async fn list_actions(
        //     &self,
        //     _request: Request<Empty>,
        // ) -> Result<Response<Self::ListActionsStream>, Status> {
        //     unimplemented!()
        // }
        async fn list_flights(
            &self,
            _request: Request<Criteria>,
        ) -> Result<Response<Self::ListFlightsStream>, Status> {
            unimplemented!()
        }
        async fn do_exchange(
            &self,
            _request: Request<Streaming<FlightData>>,
        ) -> Result<Response<Self::DoExchangeStream>, Status> {
            Err(Status::unimplemented("Implement do_exchange"))
        }
    }

    pub async fn start_server() -> anyhow::Result<()> {
        let addr = "127.0.0.1:50051".parse()?;
        let service = MyFlightService::default();
        println!("Arrow Flight server listening on {}", addr);

        tonic::transport::Server::builder()
            .add_service(arrow_flight::flight_service_server::FlightServiceServer::new(service))
            .serve(addr)
            .await?;
        Ok(())
    }
}


/* ---------- 4. 启动服务端 ---------- */
pub async fn start_server2() -> anyhow::Result<()> {
    // do_get::start_server().await?;    
    // do_put::start_server().await?;    
    // do_exchange::start_server().await?;
    do_action::start_server().await?;
    Ok(())
}


pub fn flight_data_to_schema(flight_data: &arrow_flight::FlightData) -> anyhow::Result<arrow_schema::SchemaRef> {
    let schema = flight_data;
    let message = arrow::ipc::root_as_message(&schema.data_header[..])
        .map_err(|_| arrow_schema::ArrowError::CastError("Cannot get root as message".to_string()))?;

    let ipc_schema: arrow_ipc::Schema = message
        .header_as_schema()
        .ok_or_else(|| arrow_schema::ArrowError::CastError("Cannot get header as Schema".to_string()))?;
    let schema = arrow::ipc::convert::fb_to_schema(ipc_schema);
    let schema = std::sync::Arc::new(schema);

    Ok(schema)
}