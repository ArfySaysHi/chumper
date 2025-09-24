CREATE TABLE priority_bundles (
    id INTEGER PRIMARY KEY,
    name VARCHAR(100) DEFAULT NULL,
    grade VARCHAR(1) CHECK(grade IN ('A', 'B', 'C', 'D', 'E', '*')) DEFAULT '*',
    system VARCHAR(100) DEFAULT 'Core',
    parent_id INTEGER DEFAULT NULL,
    created_at TEXT DEFAULT (datetime('now')),
    updated_at TEXT DEFAULT (datetime('now')),
    FOREIGN KEY (parent_id) REFERENCES priority_bundles(id) ON DELETE SET NULL
);

CREATE TABLE priority_bundle_modifiers (
    id INTEGER PRIMARY KEY,
    bundle_id INTEGER NOT NULL,
    target_key VARCHAR(200) NOT NULL, -- e.g. "body", "nuyen", "attributes.points"
    operation VARCHAR(20) NOT NULL CHECK(operation IN ('Add', 'Sub', 'Mul', 'Div', 'Set')),
    value VARCHAR(200) NOT NULL,
    created_at TEXT DEFAULT (datetime('now')),
    updated_at TEXT DEFAULT (datetime('now')),
    FOREIGN KEY (bundle_id) REFERENCES priority_bundles(id) ON DELETE CASCADE
);

CREATE TABLE priority_bundle_skills (
    id INTEGER PRIMARY KEY,
    bundle_id INTEGER NOT NULL,
    attribute VARCHAR(3) NOT NULL,
    amount INTEGER DEFAULT 0,
    rating INTEGER DEFAULT 0,
    FOREIGN KEY (bundle_id) REFERENCES priority_bundles(id) ON DELETE CASCADE
);

CREATE TABLE priority_bundle_metatypes (
    id INTEGER PRIMARY KEY,
    bundle_id INTEGER NOT NULL,
    special_points INTEGER NOT NULL,
    name INTEGER NOT NULL,
    FOREIGN KEY (bundle_id) REFERENCES priority_bundles(id) ON DELETE CASCADE,
    FOREIGN KEY (name) REFERENCES metatypes(name) ON DELETE CASCADE
);

CREATE TABLE priority_bundle_qualities (
    id INTEGER PRIMARY KEY,
    bundle_id INTEGER NOT NULL,
    name VARCHAR(100) NOT NULL,
    FOREIGN KEY (bundle_id) REFERENCES priority_bundles(id) ON DELETE CASCADE,
    FOREIGN KEY (name) REFERENCES qualities(name) ON DELETE CASCADE
);
