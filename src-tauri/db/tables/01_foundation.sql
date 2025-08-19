CREATE TABLE IF NOT EXISTS characters (
    id INTEGER PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    metatype VARCHAR(50) NOT NULL,
    player_name VARCHAR(100),
    body INTEGER DEFAULT 1,
    agility INTEGER DEFAULT 1,
    reaction INTEGER DEFAULT 1,
    strength INTEGER DEFAULT 1,
    willpower INTEGER DEFAULT 1,
    logic INTEGER DEFAULT 1,
    intuition INTEGER DEFAULT 1,
    charisma INTEGER DEFAULT 1,
    edge INTEGER DEFAULT 1,
    magic INTEGER DEFAULT 0,
    resonance INTEGER DEFAULT 0,
    karma_total INTEGER DEFAULT 0,
    nuyen INTEGER DEFAULT 0,
    status VARCHAR(30) CHECK (status IN ('creation', 'active', 'archived')) DEFAULT 'creation',
    created_at TEXT DEFAULT (datetime('now')),
    updated_at TEXT DEFAULT (datetime('now')),
    FOREIGN KEY (metatype) REFERENCES metatype(name)
);

-- TODO: Move all indexes into the correct files
CREATE INDEX IF NOT EXISTS idx_characters_status ON characters(status);

CREATE TABLE IF NOT EXISTS metatypes (
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
    magical_type VARCHAR(20) DEFAULT 'mundane',
    magic_min INTEGER DEFAULT 0,
    magic_max INTEGER DEFAULT 0,
    resonance_min INTEGER DEFAULT 0,
    resonance_max INTEGER DEFAULT 0
);
