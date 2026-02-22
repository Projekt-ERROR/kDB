CREATE TABLE IF NOT EXISTS ingredients (
    id                  UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name                TEXT NOT NULL UNIQUE,
    category            TEXT,
    default_unit        TEXT,
    shelf_life_days     INT,
    storage             TEXT,
    created_at  TIMESTAMP NOT NULL DEFAULT NOW()
);
