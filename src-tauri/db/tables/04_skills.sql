CREATE TABLE skills (
    id INTEGER PRIMARY KEY,
    name VARCHAR(100) NOT NULL UNIQUE,
    attribute VARCHAR(50) NOT NULL,
    can_default BOOLEAN DEFAULT FALSE,
    skill_group VARCHAR(100) DEFAULT NULL,
    FOREIGN KEY (skill_group) REFERENCES skill_groups(name) ON DELETE CASCADE
);

CREATE TABLE skill_groups (
    id INTEGER PRIMARY KEY,
    name VARCHAR(100) NOT NULL UNIQUE
);
