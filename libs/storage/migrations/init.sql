PRAGMA foreign_keys = ON;
CREATE TABLE IF NOT EXISTS _migrations(
    id INT PRIMARY KEY NOT NULL,
    up TEXT NOT NULL,
    down TEXT NOT NULL,
    applied BOOLEAN NOT NULL
);