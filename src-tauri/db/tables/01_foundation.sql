CREATE TABLE characters (
    id INTEGER PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    metatype VARCHAR(50) NOT NULL,
    player_name VARCHAR(100),
    status VARCHAR(30) CHECK (status IN ('Creation', 'Active', 'Archived')) DEFAULT 'Creation',
    created_at TEXT DEFAULT (datetime('now')),
    updated_at TEXT DEFAULT (datetime('now')),
    FOREIGN KEY (metatype) REFERENCES metatype(name)
);

CREATE TABLE modifiers (
    id INTEGER PRIMARY KEY,
    name VARCHAR(100) DEFAULT NULL,
    character_id INTEGER NOT NULL,
    origin_id INTEGER DEFAULT NULL,
    origin_type VARCHAR(50) DEFAULT NULL,
    target_key VARCHAR(50) NOT NULL,
    operation VARCHAR(20) NOT NULL CHECK(operation IN ('add', 'sub', 'mul', 'div', 'set')),
    value REAL NOT NULL,
    activation VARCHAR(50) CHECK(activation IN ('always', 'while_unconscious', 'while_stunned')) DEFAULT 'always',
    condition_id INTEGER DEFAULT NULL,
    priority INTEGER DEFAULT 100,
    stack_group VARCHAR(50) DEFAULT NULL,
    created_at TEXT DEFAULT (datetime('now')),
    updated_at TEXT DEFAULT (datetime('now')),
    FOREIGN KEY (character_id) REFERENCES characters(id) ON DELETE CASCADE,
    FOREIGN KEY (condition_id) REFERENCES conditions(id) ON DELETE CASCADE
);


CREATE INDEX idx_modifiers_character ON modifiers(character_id);
CREATE INDEX idx_modifiers_target ON modifiers(target_key);
CREATE INDEX idx_modifiers_origin ON modifiers(origin_type, origin_id);

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

CREATE INDEX idx_conditions_character ON conditions(character_id);
CREATE INDEX idx_conditions_active ON conditions(active);

-- TODO: Move all indexes into the correct files
CREATE INDEX idx_characters_status ON characters(status);

CREATE TABLE metatypes (
    id INTEGER PRIMARY KEY,
    name VARCHAR(100) NOT NULL UNIQUE,
    body_min INTEGER DEFAULT 1,
    body_max INTEGER DEFAULT 6,
    agility_min INTEGER DEFAULT 1,
    agility_max INTEGER DEFAULT 6,
    reaction_min INTEGER DEFAULT 1,
    reaction_max INTEGER DEFAULT 6,
    strength_min INTEGER DEFAULT 1,
    strength_max INTEGER DEFAULT 6,
    willpower_min INTEGER DEFAULT 1,
    willpower_max INTEGER DEFAULT 6,
    logic_min INTEGER DEFAULT 1,
    logic_max INTEGER DEFAULT 6,
    intuition_min INTEGER DEFAULT 1,
    intuition_max INTEGER DEFAULT 6,
    charisma_min INTEGER DEFAULT 1,
    charisma_max INTEGER DEFAULT 6,
    edge_min INTEGER DEFAULT 1,
    edge_max INTEGER DEFAULT 6,
    magical_type VARCHAR(20) CHECK(magical_type IN ('Magic', 'Resonance', 'Mundane')) DEFAULT 'Mundane',
    magic_min INTEGER DEFAULT 0,
    magic_max INTEGER DEFAULT 0,
    resonance_min INTEGER DEFAULT 0,
    resonance_max INTEGER DEFAULT 0
);

CREATE TABLE resources (
    id INTEGER PRIMARY KEY,
    name VARCHAR(50) NOT NULL,
    base_amount REAL DEFAULT 0,
    current_amount REAL DEFAULT 0,
    character_id INTEGER NOT NULL,
    FOREIGN KEY (character_id) REFERENCES characters(id) ON DELETE CASCADE
);

CREATE TABLE priority_grades (
    grade VARCHAR(1) PRIMARY KEY
);

CREATE TABLE metatypes_priority_grades (
    special_attribute_bonus INTEGER NOT NULL, -- Handles additional edge and magic / resonance
    metatype_name VARCHAR(100) NOT NULL,
    grade VARCHAR(1) NOT NULL,
    PRIMARY KEY (grade, metatype_name),
    FOREIGN KEY (metatype_name) REFERENCES metatypes(name) ON DELETE CASCADE,
    FOREIGN KEY (grade) REFERENCES priority_grades(grade) ON DELETE CASCADE
);

-- TODO: Implement attributes via modifiers
-- TODO: Implement qualities
-- TODO: Implement metavariants as metatypes that reference a metatype (new table maybe?)
