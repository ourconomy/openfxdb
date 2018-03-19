CREATE TABLE effects (
    id          TEXT NOT NULL,
    created     INTEGER NOT NULL,
    version     INTEGER NOT NULL,
    current     BOOLEAN NOT NULL,
    title       TEXT NOT NULL,
    description TEXT NOT NULL,
    origin      TEXT,
    homepage    TEXT,
    license     TEXT,
    PRIMARY KEY (id, version)
);

CREATE TABLE effect_tag_relations (
    effect_id      TEXT NOT NULL,
    effect_version INTEGER NOT NULL,
    tag_id        TEXT NOT NULL,
    PRIMARY KEY (effect_id, effect_version, tag_id),
    FOREIGN KEY (effect_id, effect_version) REFERENCES effects(id,version),
    FOREIGN KEY (tag_id) REFERENCES tags(id)
);
