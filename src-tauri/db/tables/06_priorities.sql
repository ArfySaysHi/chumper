CREATE TABLE priority_grades (
    grade VARCHAR(1) PRIMARY KEY
);

CREATE TABLE priority_bundles (
    id INTEGER PRIMARY KEY,
    name VARCHAR(100) DEFAULT NULL,
    grade VARCHAR(1) DEFAULT '*',
    menu_order INTEGER DEFAULT 0,
    parent_bundle_id INTEGER DEFAULT NULL,
    system VARCHAR(100) DEFAULT 'Core',
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
