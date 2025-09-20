CREATE TABLE resources (
    id INTEGER PRIMARY KEY,
    name VARCHAR(50) NOT NULL,
    base_amount REAL DEFAULT 0,
    current_amount REAL DEFAULT 0,
    character_id INTEGER NOT NULL,
    FOREIGN KEY (character_id) REFERENCES characters(id) ON DELETE CASCADE
);
