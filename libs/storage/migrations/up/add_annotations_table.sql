CREATE TABLE classes(
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    names VARCHAR(255) NOT NULL,
    version_id INTEGER NOT NULL,

    FOREIGN KEY (version_id) REFERENCES versions(id)
);

CREATE TABLE annotations(
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    version_image_id INTEGER NOT NULL,
    class_id INTEGER NOT NULL,
    x REAL NOT NULL,
    y REAL NOT NULL,
    width REAL NOT NULL,
    height REAL NOT NULL,

    FOREIGN KEY (class_id) REFERENCES classes(id),
    FOREIGN KEY (version_image_id) REFERENCES version_images(id)
);