CREATE TABLE modifiers (
    id INTEGER PRIMARY KEY,
    name VARCHAR(100) DEFAULT NULL,
    character_id INTEGER NOT NULL,
    origin_id INTEGER DEFAULT NULL,
    origin_type VARCHAR(50) DEFAULT NULL,
    target_key VARCHAR(50) NOT NULL,
    operation VARCHAR(20) NOT NULL CHECK(operation IN ('add', 'sub', 'mul', 'div', 'set')),
    value_formula VARCHAR(200) NOT NULL,
    activation VARCHAR(50) CHECK(activation IN ('always', 'while_unconscious', 'while_stunned')) DEFAULT 'always',
    condition_id INTEGER DEFAULT NULL,
    priority INTEGER DEFAULT 100,
    stack_group VARCHAR(50) DEFAULT NULL,
    created_at TEXT DEFAULT (datetime('now')),
    updated_at TEXT DEFAULT (datetime('now')),
    FOREIGN KEY (character_id) REFERENCES characters(id) ON DELETE CASCADE,
    FOREIGN KEY (condition_id) REFERENCES conditions(id) ON DELETE CASCADE
);

CREATE TABLE conditions (
    id INTEGER PRIMARY KEY,
    character_id INTEGER DEFAULT NULL,
    source_type VARCHAR(50) DEFAULT NULL,
    source_id INTEGER DEFAULT NULL,
    name VARCHAR(200) NOT NULL,
    description TEXT DEFAULT NULL,
    active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TEXT DEFAULT (datetime('now')),
    updated_at TEXT DEFAULT (datetime('now')),
    FOREIGN KEY (character_id) REFERENCES characters(id) ON DELETE CASCADE
);
