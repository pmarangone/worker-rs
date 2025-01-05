use apalis::prelude::*;
use apalis_redis::{Config, RedisStorage};
use serde::{Deserialize, Serialize};
use tracing::{error, info};

#[derive(Debug, Deserialize, Serialize)]
pub struct Email {
    pub to: String,
}

/// A function called for every job
async fn send_email(job: Email, data: Data<usize>) -> Result<(), Error> {
    info!("Called send_email!");
    /// execute job
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt::init();
    let redis_url = "redis://localhost:6379";
    match apalis_redis::connect(redis_url).await {
        Ok(conn) => {
            info!("Connection succeeded!");
            let config_storage = Config::default();
            let conf = config_storage.set_namespace("email-worker");
            let mut storage = RedisStorage::new_with_config(conn, conf);
            // let _ = produce_route_jobs(&mut storage).await;
            WorkerBuilder::new("email-worker")
                .data(0usize)
                .backend(storage)
                .build_fn(send_email)
                .run()
                .await;
        }
        Err(err) => {
            error!("could not connect to redis {:?}", err);
        }
    }

    Ok(())
}
