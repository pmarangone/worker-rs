use anyhow::Error;
use deadpool_postgres::{Client, Config, ManagerConfig, Pool, RecyclingMethod, Runtime::Tokio1};
use tokio_postgres::NoTls;

use tracing::info;

use crate::models::transaction::TransactionModel;

pub async fn create_connection_pool() -> Result<Pool, Error> {
    let mut cfg = Config::new();
    cfg.host = Some("localhost".to_string()); // Replace with your database host
    cfg.dbname = Some("database".to_string());
    cfg.user = Some("user".to_string());
    cfg.password = Some("password".to_string());
    cfg.port = Some(5432);

    cfg.manager = Some(ManagerConfig {
        recycling_method: RecyclingMethod::Fast,
    });

    let pool = cfg.create_pool(Some(Tokio1), NoTls)?;
    let mut client = pool.get().await.expect("Failed to get client from pool");
    let _ = create_table(&mut client).await;

    info!("Pool connection succeeded!");
    Ok(pool)
}

// pub async fn get_client(pool: Pool) -> &mut Client {
//     let mut client = pool.get().await.expect("Failed to get client from pool");
//     client
// }

pub async fn create_table(client: &mut Client) -> Result<(), Error> {
    let res = client
        .batch_execute(
            "
    CREATE TABLE IF NOT EXISTS transactions (
        id      SERIAL PRIMARY KEY,
        amount  INTEGER NOT NULL,
        tx_id   INTEGER NOT NULL
    )
    ",
        )
        .await;

    match res {
        Ok(_) => info!("Table transactions created!"),
        Err(_) => info!("Failed to create transactions table"),
    }

    Ok(())
}

pub async fn insert_into_table(
    client: &mut Client,
    transaction: TransactionModel,
) -> Result<(), Error> {
    let id = transaction.id;
    client
        .execute(
            "INSERT INTO transactions (amount, tx_id) VALUES ($1, $2)",
            &[&transaction.amount, &id],
        )
        .await?;

    Ok(())
}
