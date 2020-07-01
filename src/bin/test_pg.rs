extern crate pg_playground;
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use pg_playground::pg::SQL;
use std::str::FromStr;
use tokio_postgres::{Config, NoTls};
type ConnectionPool = Pool<PostgresConnectionManager<NoTls>>;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::from_str("postgresql://postgres:dev@127.0.0.1:5432")?;

    let pg_mgr = PostgresConnectionManager::new(config, tokio_postgres::NoTls);

    let pool = match Pool::builder().build(pg_mgr).await {
        Ok(pool) => pool,
        Err(e) => panic!("builder error: {:?}", e),
    };

    // drop_tables(&pool).await;
    create_tables(&pool).await?;
    Ok(())
}

pub async fn create_tables(pool: &ConnectionPool) -> Result<(), Box<dyn Error>> {
    let connection = pool.get().await?;
    connection.execute(SQL::DUMMY_TABLE, &[]).await?;
    // connection.execute(SQL::CREATE_ACCOUNT_STATE, &[]).await?;
    Ok(())
}

pub async fn drop_tables(pool: &ConnectionPool) -> Result<(), Box<dyn Error>> {
    let connection = pool.get().await?;
    connection.execute(SQL::DROP_ACCOUNT_OWNER, &[]).await?;
    connection.execute(SQL::DROP_ACCOUNT_STATE, &[]).await?;

    Ok(())
}
