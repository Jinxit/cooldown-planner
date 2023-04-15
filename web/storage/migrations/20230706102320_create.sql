CREATE TABLE IF NOT EXISTS store
(
    key        STRING  NOT NULL,
    key_type   INTEGER NOT NULL,
    value_type INTEGER NOT NULL,
    value      TEXT    NOT NULL,
    live_until INTEGER,
    PRIMARY KEY (key, key_type, value_type)
);
