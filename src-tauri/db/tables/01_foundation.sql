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

CREATE TABLE skill_groups (
    id INTEGER PRIMARY KEY,
    name VARCHAR(100) NOT NULL UNIQUE
);

CREATE TABLE skills (
    id INTEGER PRIMARY KEY,
    name VARCHAR(100) NOT NULL UNIQUE,
    attribute VARCHAR(50) NOT NULL,
    can_default BOOLEAN DEFAULT FALSE,
    skill_group VARCHAR(100) DEFAULT NULL,
    FOREIGN KEY (skill_group) REFERENCES skill_groups(name) ON DELETE CASCADE
);

CREATE TABLE qualities (
    id INTEGER PRIMARY KEY,
    name VARCHAR(100) NOT NULL UNIQUE,
    resource_name VARCHAR(50) NOT NULL,
    cost REAL NOT NULL DEFAULT 0,
    category VARCHAR(50) NOT NULL CHECK(category IN ('Positive', 'Negative')),
    FOREIGN KEY (resource_name) REFERENCES resources(name) ON DELETE RESTRICT
);

-- Convert into modifiers later
CREATE TABLE quality_effects (
    id INTEGER PRIMARY KEY,
    quality_id INTEGER NOT NULL,
    target_key VARCHAR(50) NOT NULL,
    operation VARCHAR(20) NOT NULL,
    value_formula VARCHAR(200) NOT NULL,
    activation VARCHAR(50) DEFAULT 'always',
    priority INTEGER DEFAULT 100,
    FOREIGN KEY (quality_id) REFERENCES qualities(id) ON DELETE CASCADE
);

CREATE TABLE character_qualities (
    id INTEGER PRIMARY KEY,
    character_id INTEGER NOT NULL,
    quality_id INTEGER NOT NULL,
    rating INTEGER DEFAULT 1,
    FOREIGN KEY (character_id) REFERENCES characters(id) ON DELETE CASCADE,
    FOREIGN KEY (quality_id) REFERENCES qualities(id) ON DELETE CASCADE
);

CREATE TABLE priority_grades (
    grade VARCHAR(1) PRIMARY KEY
);

CREATE TABLE priority_bundles (
    id INTEGER PRIMARY KEY,
    name VARCHAR(100) DEFAULT NULL,
    grade VARCHAR(1) DEFAULT '*',
    menu_order INTEGER DEFAULT 0,
    parent_bundle_id INTEGER DEFAULT NULL,
    created_at TEXT DEFAULT (datetime('now')),
    updated_at TEXT DEFAULT (datetime('now')),
    FOREIGN KEY (parent_bundle_id) REFERENCES priority_bundles(id) ON DELETE CASCADE
);

CREATE TABLE priority_bundle_modifiers (
    id INTEGER PRIMARY KEY,
    grade VARCHAR(1) NOT NULL DEFAULT '*',
    bundle_id INTEGER NOT NULL,
    target_key VARCHAR(200) NOT NULL, -- e.g. "body", "nuyen", "attributes.points"
    operation VARCHAR(20) NOT NULL CHECK(operation IN ('add', 'sub', 'mul', 'div', 'set')),
    value VARCHAR(200) NOT NULL,
    created_at TEXT DEFAULT (datetime('now')),
    updated_at TEXT DEFAULT (datetime('now')),
    FOREIGN KEY (bundle_id) REFERENCES priority_bundles(id) ON DELETE CASCADE
);

CREATE TABLE priority_bundle_skills (
    id INTEGER PRIMARY KEY,
    grade VARCHAR(1) NOT NULL DEFAULT '*',
    bundle_id INTEGER NOT NULL,
    attribute VARCHAR(3) NOT NULL,
    amount INTEGER DEFAULT 0,
    rating INTEGER DEFAULT 0
);

-- TODO: Create the metatype priority grades import for the frontend modal info
CREATE TABLE priority_bundle_metatypes (
    id INTEGER PRIMARY KEY,
    bundle_id INTEGER NOT NULL,
    grade VARCHAR(1) DEFAULT '*',
    special_points INTEGER NOT NULL,
    name VARCHAR(100) NOT NULL,
    FOREIGN KEY (name) REFERENCES metatypes(name) ON DELETE CASCADE
);

CREATE TABLE priority_bundle_qualities (
    id INTEGER PRIMARY KEY,
    bundle_id INTEGER NOT NULL,
    grade VARCHAR(1) DEFAULT '*',
    name INTEGER NOT NULL,
    FOREIGN KEY (name) REFERENCES qualities(name) ON DELETE CASCADE
);

CREATE TABLE metatype_qualities (
    id INTEGER PRIMARY KEY,
    metatype_name VARCHAR(100) NOT NULL,
    quality_name VARCHAR(100) NOT NULL,
    default_rating INTEGER DEFAULT 1,
    created_at TEXT DEFAULT (datetime('now')),
    updated_at TEXT DEFAULT (datetime('now')),
    FOREIGN KEY (metatype_name) REFERENCES metatypes(name) ON DELETE CASCADE,
    FOREIGN KEY (quality_name) REFERENCES qualities(name) ON DELETE CASCADE,
    UNIQUE (metatype_name, quality_name)
);
