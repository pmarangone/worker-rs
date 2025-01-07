use std::sync::Arc;

use apalis::prelude::*;
use apalis_amqp::AmqpBackend;
use apalis_redis::RedisStorage;
use db::primary_op::insert_into_table;
use deadpool_postgres::Pool;
use models::transaction::TransactionModel;
use tracing::{error, info};

mod db;
mod models;

struct State {
    pool: Pool,
}

/// A function called for every job
async fn send_tx(job: TransactionModel, state: Data<u32>) -> Result<(), Error> {
    info!("Called send_tx!");

    Ok(())
}

pub async fn connect_to_rabbitmq(addr: &str) -> lapin::Result<lapin::Connection> {
    let conn = lapin::Connection::connect(addr, lapin::ConnectionProperties::default()).await?;
    tracing::debug!("Connected to RabbitMQ");
    Ok(conn)
}

// async fn worker_redis(state: Arc<State>, conf: apalis_redis::Config) {
// let redis_url = "redis://localhost:6379";
//     match apalis_redis::connect(redis_url).await {
//         Ok(conn) => {
//             info!("Redis connection succeeded!");

//             let storage = RedisStorage::new_with_config(conn, conf);
//             WorkerBuilder::new("email-worker")
//                 .data(state)
//                 .backend(storage)
//                 .build_fn(send_tx)
//                 .run()
//                 .await;
//         }
//         Err(err) => {
//             error!("could not connect to redis {:?}", err);
//         }
//     }
// }

//This can be in another part of the program or another application eg a http server
pub async fn amqp_produce_route_jobs() -> Result<(), anyhow::Error> {
    info!("Called producer!");

    let amqp_addr = "amqp://127.0.0.1:5672/%2f";

    let conn = connect_to_rabbitmq(&amqp_addr).await?;
    let amqp_channel = conn.create_channel().await?;

    let amqp_queue = amqp_channel
        .queue_declare(
            "transaction_queue",
            lapin::options::QueueDeclareOptions::default(),
            lapin::types::FieldTable::default(),
        )
        .await?;

    let config_storage = apalis_amqp::utils::Config::new("transaction_queue");
    let mut backend: AmqpBackend<TransactionModel> =
        AmqpBackend::new_with_config(amqp_channel, amqp_queue, config_storage);

    match backend
        .enqueue(TransactionModel { id: 1, amount: 42 })
        .await
    {
        Ok(_) => {
            info!("Job request sent successfully.");
        }
        Err(err) => {
            error!("Job request failed. {:?}", err);
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    tracing_subscriber::fmt::init();

    let amqp_addr = "amqp://127.0.0.1:5672/%2f";

    let pool = db::primary_op::create_connection_pool().await?;
    let state = Arc::new(State { pool });

    let conn = connect_to_rabbitmq(&amqp_addr).await?;
    let amqp_channel = conn.create_channel().await?;

    let amqp_queue = amqp_channel
        .queue_declare(
            "transaction_queue",
            lapin::options::QueueDeclareOptions::default(),
            lapin::types::FieldTable::default(),
        )
        .await?;

    let config_storage = apalis_amqp::utils::Config::new("transaction_queue");
    let mut backend: AmqpBackend<TransactionModel> =
        AmqpBackend::new_with_config(amqp_channel, amqp_queue, config_storage);

    backend
        .enqueue(TransactionModel { id: 1, amount: 42 })
        .await
        .unwrap();

    amqp_produce_route_jobs().await?;

    Monitor::new()
        .register(
            WorkerBuilder::new("transaction_queue")
                .data(0u32)
                .backend(backend)
                .build_fn(send_tx),
        )
        .run()
        .await?;

    // info!("{:?}", backend.config());
    // info!("{:?}", backend.queue());
    // info!("{:?}", backend.channel());

    Ok(())
}
