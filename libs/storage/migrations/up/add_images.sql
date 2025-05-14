CREATE TABLE images(
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    names TEXT NOT NULL,
    width INTEGER NOT NULL,
    height INTEGER NOT NULL,
    paths TEXT NOT NULL
);

CREATE TABLE version_images(
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    version_id INTEGER NOT NULL,
    image_id INTEGER NOT NULL,

    FOREIGN KEY (version_id) REFERENCES versions(id),
    FOREIGN KEY (image_id) REFERENCES images(id)
);