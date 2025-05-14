CREATE TABLE annotations(
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    version_image_id INTEGER NOT NULL,
    class VARCHAR(255) NOT NULL,
    x REAL NOT NULL,
    y REAL NOT NULL,
    width REAL NOT NULL,
    height REAL NOT NULL,

    FOREIGN KEY (version_image_id) REFERENCES version_images(id)
);