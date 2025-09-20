CREATE TABLE qualities (
    id INTEGER PRIMARY KEY,
    name VARCHAR(100) NOT NULL UNIQUE,
    resource_name VARCHAR(50) NOT NULL,
    cost REAL NOT NULL DEFAULT 0,
    category VARCHAR(50) NOT NULL CHECK(category IN ('Positive', 'Negative')),
    FOREIGN KEY (resource_name) REFERENCES resources(name) ON DELETE RESTRICT
);

CREATE TABLE quality_modifiers (
    id INTEGER PRIMARY KEY,
    quality_id INTEGER NOT NULL,
    target_key VARCHAR(50) NOT NULL,
    operation VARCHAR(20) NOT NULL CHECK(operation IN ('Add', 'Sub', 'Mul', 'Div', 'Set')),
    value_formula VARCHAR(200) NOT NULL,
    activation VARCHAR(50) DEFAULT 'always',
    priority INTEGER DEFAULT 100,
    FOREIGN KEY (quality_id) REFERENCES qualities(id) ON DELETE CASCADE
);
