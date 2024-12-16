-- Your SQL goes here
CREATE TABLE
    marketplace_processor_status (
        processor VARCHAR(50) NOT NULL,
        last_success_version BIGINT NOT NULL,
        last_updated TIMESTAMP NOT NULL DEFAULT NOW (),
        last_transaction_timestamp TIMESTAMP NULL,
        PRIMARY KEY (processor)
    );