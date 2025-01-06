use std::sync::Arc;

use apalis::prelude::*;
use apalis_redis::{Config, RedisStorage};
use db::primary_op::{create_table, insert_into_table};
use deadpool_postgres::Pool;
use models::transaction::TransactionModel;
use serde::{Deserialize, Serialize};
use tracing::{error, info};

mod db;
mod models;

struct State {
    pool: Pool,
}

/// A function called for every job
async fn send_tx(job: TransactionModel, state: Data<Arc<State>>) -> Result<(), Error> {
    info!("Called send_tx!");

    let mut client = state
        .pool
        .get()
        .await
        .expect("Failed to get client from pool");

    let insert_result = insert_into_table(&mut client, job).await;
    match insert_result {
        Ok(_) => {
            info!("Tx insertion succeeded!")
        }
        Err(err) => {
            error!("Tx insertion failed: {:?}", err)
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    tracing_subscriber::fmt::init();
    let redis_url = "redis://localhost:6379";

    let pool = db::primary_op::create_connection_pool().await?;
    let state = Arc::new(State { pool });

    match apalis_redis::connect(redis_url).await {
        Ok(conn) => {
            info!("Redis connection succeeded!");

            let config_storage = Config::default();
            let conf = config_storage.set_namespace("email-worker");
            let storage = RedisStorage::new_with_config(conn, conf);
            WorkerBuilder::new("email-worker")
                .data(state)
                .backend(storage)
                .build_fn(send_tx)
                .run()
                .await;
        }
        Err(err) => {
            error!("could not connect to redis {:?}", err);
        }
    }

    Ok(())
}
