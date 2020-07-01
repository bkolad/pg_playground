extern crate pg_playground;
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use pg_playground::pg::SQL;
use std::str::FromStr;
use tokio_postgres::{Config, Connection, NoTls};
type ConnectionPool = Pool<PostgresConnectionManager<NoTls>>;
use std::error::Error;

pub struct Account {
    nonce: i32,
    balance: i64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::from_str("postgresql://postgres:dev@127.0.0.1:5432")?;

    let pg_mgr = PostgresConnectionManager::new(config, tokio_postgres::NoTls);

    let pool = match Pool::builder().build(pg_mgr).await {
        Ok(pool) => pool,
        Err(e) => panic!("builder error: {:?}", e),
    };

    // drop_tables(&pool).await;
    // create_tables(&pool).await?;
    let acc = Account {
        nonce: 10,
        balance: 33,
    };

    let acc2 = Account {
        nonce: 101,
        balance: 2333,
    };
    let accs = vec![acc, acc2];
    update_prepared(&pool).await?;
    batch_insert(&pool, accs).await?;
    read(&pool).await?;
    Ok(())
}

pub async fn read(pool: &ConnectionPool) -> Result<(), Box<dyn Error>> {
    let connection = pool.get().await?;

    for row in connection.query(SQL::SELECT, &[]).await? {
        let id: i32 = row.get(0);
        let nonce: i32 = row.get(1);
        let balance: i64 = row.get(2);
        println!("ROW: id {:?}, n {:?}, b {:?}", id, nonce, balance);
    }
    Ok(())
}

pub async fn update(pool: &ConnectionPool) -> Result<(), Box<dyn Error>> {
    let connection = pool.get().await?;
    let k: i32 = 9;
    let id = 3;
    connection.execute(SQL::UPDATE, &[&k, &id]).await?;
    Ok(())
}

pub async fn update_prepared(pool: &ConnectionPool) -> Result<(), Box<dyn Error>> {
    let connection = pool.get().await?;
    let stmt = connection.prepare(SQL::UPDATE).await?;
    let k: i32 = 99999;
    let id = 2;
    connection.execute(&stmt, &[&k, &id]).await?;

    Ok(())
}

pub async fn insert_prepared(pool: &ConnectionPool, acc: &Account) -> Result<(), Box<dyn Error>> {
    let connection = pool.get().await?;
    let stmt = connection.prepare(SQL::INSERT).await?;

    connection
        .execute(&stmt, &[&acc.nonce, &acc.balance])
        .await?;

    Ok(())
}

pub async fn insert(pool: &ConnectionPool, acc: &Account) -> Result<(), Box<dyn Error>> {
    let connection = pool.get().await?;
    connection
        .execute(SQL::INSERT, &[&acc.nonce, &acc.balance])
        .await?;

    Ok(())
}

pub async fn batch_insert(pool: &ConnectionPool, accs: Vec<Account>) -> Result<(), Box<dyn Error>> {
    let mut connection = pool.get().await?;
    let trs = connection.transaction().await?;
    let sql: &str = &generate(accs);

    trs.execute(sql, &[]).await?;
    trs.commit().await?;
    Ok(())
}

pub async fn create_tables(pool: &ConnectionPool) -> Result<(), Box<dyn Error>> {
    let connection = pool.get().await?;
    connection.execute(SQL::DUMMY_TABLE, &[]).await?;
    Ok(())
}

pub async fn drop_tables(pool: &ConnectionPool) -> Result<(), Box<dyn Error>> {
    let connection = pool.get().await?;
    connection.execute(SQL::DROP_DUMMY_TABLE, &[]).await?;

    Ok(())
}

fn generate(accs: Vec<Account>) -> String {
    let mut e = "INSERT INTO dummy_table (nonce, balance) VALUES
    "
    .to_string();

    for i in 0..accs.len() - 1 {
        let acc = &accs[i];
        let v = format!("({}, {}), ", acc.nonce, acc.balance);
        e.push_str(&v)
    }
    let acc = &accs[accs.len() - 1];
    let v = format!("({}, {});", acc.nonce, acc.balance);
    e.push_str(&v);
    println!("E: {}", e);
    e
}
