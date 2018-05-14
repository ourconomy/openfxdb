CREATE TABLE upstreams (
    id                  TEXT NOT NULL,
    created             TEXT NOT NULL,
    effect_id           TEXT NOT NULL,
    effect_version      INTEGER NOT NULL,
    upstream_effect_id  TEXT,
    upstream_effect     TEXT,
    number              INTEGER,
    transfer_unit       TEXT,
    amount              FLOAT,
    comment             TEXT,
    PRIMARY KEY (id, effect_id, effect_version),
    FOREIGN KEY (effect_id, effect_version) REFERENCES effects(id,version)
);
