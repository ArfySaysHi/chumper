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
