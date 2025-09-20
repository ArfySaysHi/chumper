CREATE TABLE characters (
    id INTEGER PRIMARY KEY,
    name VARCHAR(100),
    player_name VARCHAR(100),
    status VARCHAR(30) CHECK (status IN ('Creation', 'Active', 'Archived')) DEFAULT 'Creation',
    metatype_id INTEGER NOT NULL,
    created_at TEXT DEFAULT (datetime('now')),
    updated_at TEXT DEFAULT (datetime('now')),
    FOREIGN KEY (metatype_id) REFERENCES metatypes(id) ON DELETE RESTRICT
);

CREATE TABLE character_priorities (
    id INTEGER PRIMARY KEY,
    bundle_name VARCHAR(100) NOT NULL,
    grade VARCHAR(1) NOT NULL,
    priority_system VARCHAR(100) DEFAULT 'Core',
    character_id INTEGER NOT NULL,
    FOREIGN KEY (character_id) REFERENCES characters(id) ON DELETE CASCADE
);

CREATE TABLE character_qualities (
    id INTEGER PRIMARY KEY,
    character_id INTEGER NOT NULL,
    quality_id INTEGER NOT NULL,
    rating INTEGER DEFAULT 1,
    FOREIGN KEY (character_id) REFERENCES characters(id) ON DELETE CASCADE,
    FOREIGN KEY (quality_id) REFERENCES qualities(id) ON DELETE CASCADE
);
