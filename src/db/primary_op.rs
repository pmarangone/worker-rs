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
    info!("Pool connection succeeded!");
    Ok(pool)
}

// pub async fn get_client(pool: Pool) -> &mut Client {
//     let mut client = pool.get().await.expect("Failed to get client from pool");
//     client
// }

pub async fn create_table(client: &mut Client) -> Result<(), Error> {
    client
        .batch_execute(
            "
    CREATE TABLE IF NOT EXISTS transactions (
        id      SERIAL PRIMARY KEY,
        amount  INTEGER NOT NULL,
        tx_id   INTEGER NOT NULL
    )
    ",
        )
        .await?;

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

pub async fn query_all(client: Client) -> Result<(), Error> {
    // let mut users: Vec<User> = vec![];

    // for row in client
    //     .query(
    //         "SELECT id, name, surname, description, age FROM person",
    //         &[],
    //     )
    //     .await?
    // {
    //     let id: i32 = row.get(0);
    //     let name: &str = row.get(1);
    //     let surname: &str = row.get(2);
    //     let description: &str = row.get(3);
    //     let age: i32 = row.get(4);

    //     // let data: Option<&[u8]> = row.get(2);

    //     users.push(User {
    //         name: name.to_string(),
    //         surname: surname.to_string(),
    //         description: description.to_string(),
    //         age,
    //     });
    // }

    // Ok(Json(users))
    Ok(())
}
