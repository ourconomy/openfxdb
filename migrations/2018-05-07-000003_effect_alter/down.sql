ALTER TABLE effects RENAME TO effectscopy;
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
INSERT INTO effects SELECT id, created, version, current, title, description, origin, homepage, license FROM effectscopy;
DROP TABLE effectscopy;
