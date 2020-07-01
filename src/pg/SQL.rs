pub static DROP_DUMMY_TABLE: &str = "DROP TABLE dummy_table";

pub static DUMMY_TABLE: &str = "
CREATE TABLE IF NOT EXISTS dummy_table (
    account_id       SERIAL PRIMARY KEY,
    nonce            INT NOT NULL,
    balance          BIGINT NOT NULL
    )";

pub static INSERT: &str = "
INSERT INTO dummy_table (nonce, balance) VALUES ($1, $2)
   ";

pub static BATCH_INSERT: &str = "
   INSERT INTO dummy_table (nonce, balance) VALUES (1, 2), (9, 20);
      ";

pub static SELECT: &str = "
SELECT account_id, nonce, balance FROM dummy_table
   ";

pub static UPDATE: &str = "
UPDATE dummy_table
SET nonce = $1
where
  account_id = $2
      ";
