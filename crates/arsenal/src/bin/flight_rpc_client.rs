use std::sync::Arc;

use arrow::array::{Int64Array, RecordBatch};
use arrow_flight::flight_service_client::FlightServiceClient;
use arrow_flight::{FlightData, FlightDescriptor, Ticket};
use arrow_schema::{ArrowError, DataType, Field, Schema, SchemaRef};
use futures::StreamExt;
use tonic::{Request, Status};
// use arrow_flight::flight_service_client::FlightServiceClient;
// use arrow_flight::{FlightData, FlightDescriptor};
use futures::stream;
// use tonic::Request;



#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    // let addr = "[::1]:50051".parse()?;
    // let client = FlightServiceClient::connect(addr).await?;
    // client_do_get().await?;
    // client_do_put().await?;
    // client_do_exchange().await?;
    client_do_action().await?;
    Ok(())
}

/* ---------- 5. 客户端：握手 → get_info → do_get ---------- */
async fn client_do_get() -> anyhow::Result<()> {
    let mut client = FlightServiceClient::connect("http://127.0.0.1:50051").await?;

    // 1. 握手（空握手即可）
    let hs_req = Request::new(futures::stream::iter(vec![
        arrow_flight::HandshakeRequest::default(),
    ]));
    let hs_resp = client.handshake(hs_req).await?;
    println!("handshake ok: {:?}", hs_resp.metadata());

    // 2. 拿 FlightInfo（包含 schema & ticket）
    let fd = FlightDescriptor::new_cmd(b"select now();".to_vec());
    let info_resp = client.get_flight_info(Request::new(fd)).await?;
    let info = info_resp.into_inner();
    // println!("info = {:?}", info);
    // println!("schema = {:?}", info.try_get_schema()?);

    // 3. 凭 ticket 拉数据
    for ep in info.endpoint {
        println!("ep = {:?}", ep);
        println!();
        if let Some(ticket) = ep.ticket {
            let mut stream = client.do_get(ticket).await?.into_inner();
            let mut msg_idx = -1;
            while let Some(Ok(data)) = stream.next().await {
                msg_idx += 1;
                if msg_idx == 0 {
                    if data.data_header.is_empty() {
                        println!("first msg data header is empty");
                        continue;
                    }
                    match flight_data_to_schema(&data) {
                        Ok(schema) => {
                            println!("first msg schema = {:?}", schema);
                        }
                        Err(e) => {
                            println!("first msg schema err = {:?}", e);
                        }
                    }
                    continue;
                }
                println!();
                // 这里打印 meta data 信息
                println!("received flight meta data: {:?}", data.app_metadata);
                // 这里打印 body 长度，实际可解码为 RecordBatch
                println!("received flight body data: {} bytes", data.data_body.len());
            }
        }
    }
    Ok(())
}

/* ---------- 5. 客户端：握手 → get_info → do_out ---------- */
async fn client_do_put() -> anyhow::Result<()> {
    let mut client = FlightServiceClient::connect("http://127.0.0.1:50051").await?;

    // 1. 构造 Schema
    let schema = Arc::new(arrow_schema::Schema::new(vec![
        Field::new("id", DataType::Int64, false),
        Field::new("value", DataType::Int64, false),
    ]));

    // 2. 造 2 个 RecordBatch
    let batch1 = RecordBatch::try_new(
        schema.clone(),
        vec![
            Arc::new(Int64Array::from(vec![1, 2, 3])),
            Arc::new(Int64Array::from(vec![10, 20, 30])),
        ],
    )?;
    let batch2 = RecordBatch::try_new(
        schema.clone(),
        vec![
            Arc::new(Int64Array::from(vec![4, 5])),
            Arc::new(Int64Array::from(vec![40, 50])),
        ],
    )?;

    // 3. 转成 FlightData 流（第 1 条带 schema header）
    let flights: Vec<FlightData> =
        arrow_flight::utils::batches_to_flight_data(&schema, vec![batch1, batch2])
            .map_err(|e| Status::internal(e.to_string()))?;

    // 4. 上传 & 接收确认
    // let descriptor = FlightDescriptor::new_cmd(b"do_put_demo".to_vec());
    let outbound = stream::iter(flights);
    let response = client
        .do_put(Request::new(outbound))
        .await?
        .into_inner();
    let mut put_stream = response;
    while let Some(result) = put_stream.next().await {
        let r = result?;
        println!("[Client] server confirmed: {:?}",
                 String::from_utf8_lossy(&r.app_metadata));
    }
    Ok(()) 
}


async fn client_do_exchange() -> anyhow::Result<()> {
    let mut client = FlightServiceClient::connect("http://127.0.0.1:50051").await?;

    // 1. 握手（空握手即可）
    let hs_req = Request::new(futures::stream::iter(vec![
        arrow_flight::HandshakeRequest::default(),
    ]));
    let hs_resp = client.handshake(hs_req).await?;
    println!("handshake ok: {:?}", hs_resp.metadata());

    // 1. 输入 Schema
    let schema = Arc::new(arrow_schema::Schema::new(vec![Field::new("id", DataType::Int64, false)]));
    let schema_flight =
        arrow_flight::utils::batches_to_flight_data(schema.as_ref(), Default::default())?.remove(0);

    // 2. 造 2 个 RecordBatch
    let batch1 = RecordBatch::try_new(
        schema.clone(),
        vec![Arc::new(Int64Array::from(vec![1, 2, 3]))],
    )?;
    let batch2 = RecordBatch::try_new(
        schema.clone(),
        vec![Arc::new(Int64Array::from(vec![10, 20]))],
    )?;

    // 3. 转成 FlightData 流（schema 在前）
    let mut flights = vec![schema_flight];
    flights.extend(
        arrow_flight::utils::batches_to_flight_data(schema.as_ref(), vec![batch1, batch2]).unwrap()
    );
    let outbound = stream::iter(flights.into_iter());

    // 4. 发起 DoExchange 并同时读返回流
    let response = client.do_exchange(Request::new(outbound)).await?;
    let mut inbound = response.into_inner();

    println!("[Client] waiting for server summaries...");
    while let Some(data) = inbound.next().await {
        let flight_data = data?;
        if flight_data.data_body.is_empty() {
            println!("empty data body, this is schema");
            continue;
        }
        let batch = arrow_flight::utils::flight_data_to_arrow_batch(
            &flight_data,
            Arc::new(Schema::new(vec![
                Field::new("rows", DataType::Int64, false),
                Field::new("sum_id", DataType::Int64, false),
            ])),
            &Default::default(),
        )?;
        println!("[Client] server returned: {}", arrow::util::pretty::pretty_format_batches(&[batch])?);
    }
    Ok(())
}



async fn client_do_action() -> anyhow::Result<()> {
    use arrow_flight::flight_service_client::FlightServiceClient;
    use arrow_flight::{Action, Empty};
    use futures::StreamExt;

    let mut client = FlightServiceClient::connect("http://127.0.0.1:50051").await?;

    // 1. 先枚举支持的动作
    let list_resp = client.list_actions(Request::new(Empty {})).await?;
    println!("[Client] Server supports actions:");
    let mut list_stream = list_resp.into_inner();
    while let Some(action_type) = list_stream.next().await {
        let at = action_type?;
        println!("  - {} : {}", at.r#type, at.description);
    }

    // 2. 调用 drop_table
    let drop_action = Action {
        r#type: "drop_table".to_string(),
        body: bytes::Bytes::from(b"test_table".to_vec()),
    };
    println!("\n[Client] calling drop_table('test_table')");
    let drop_resp = client.do_action(Request::new(drop_action)).await?;
    let mut drop_stream = drop_resp.into_inner();
    while let Some(result) = drop_stream.next().await {
        let result = result?;
        let msg = String::from_utf8_lossy(&result.body);
        println!("[Client] drop_table result: {}", msg);
    }

    // 3. 调用 shutdown（服务端返回多条结果）
    let shutdown_action = Action {
        r#type: "shutdown".to_string(),
        body: bytes::Bytes::from(b"".to_vec()),
    };
    println!("\n[Client] calling shutdown");
    let shutdown_resp = client.do_action(Request::new(shutdown_action)).await?;
    let mut shutdown_stream = shutdown_resp.into_inner();
    while let Some(result) = shutdown_stream.next().await {
        let result = result?;
        let msg = String::from_utf8_lossy(&result.body);
        println!("[Client] shutdown result: {}", msg);
    }

    Ok(())
}

pub fn flight_data_to_schema(flight_data: &FlightData) -> anyhow::Result<SchemaRef> {
    let schema = flight_data;
    let message = arrow::ipc::root_as_message(&schema.data_header[..])
        .map_err(|_| arrow_schema::ArrowError::CastError("Cannot get root as message".to_string()))?;

    let ipc_schema: arrow_ipc::Schema = message
        .header_as_schema()
        .ok_or_else(|| ArrowError::CastError("Cannot get header as Schema".to_string()))?;
    let schema = arrow::ipc::convert::fb_to_schema(ipc_schema);
    let schema = Arc::new(schema);

    Ok(schema)
}