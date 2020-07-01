pub static CREATE_ACCOUNT_OWNER: &str = "
CREATE TABLE IF NOT EXISTS account_owner(
    id              SERIAL PRIMARY KEY
    )";

pub static CREATE_ACCOUNT_DEFINITION: &str = "
CREATE TABLE IF NOT EXISTS account_definition (
    account_id      INT,
    batch_id        INT NOT NULL,
    PRIMARY KEY (account_id),
    FOREIGN KEY (account_id) REFERENCES account_owner(id)
    )";

pub static CREATE_ACCOUNT_STATE: &str = "
CREATE TABLE IF NOT EXISTS account_state (
    account_id      INT,
    nonce           INT NOT NULL,
    balance         BIGINT NOT NULL,
    PRIMARY KEY (account_id),
    FOREIGN KEY (account_id) REFERENCES account_owner(id)
    )";

pub static DROP_ACCOUNT_STATE: &str = "DROP TABLE account_state";
pub static DROP_ACCOUNT_OWNER: &str = "DROP TABLE account_owner";

pub static DUMMY_TABLE: &str = "
CREATE TABLE IF NOT EXISTS dummy_table (
    account_id       SERIAL PRIMARY KEY,
    nonce            INT NOT NULL,
    balance          BIGINT NOT NULL
    )";
