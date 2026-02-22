CREATE TABLE IF NOT EXISTS kitchen (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    ingredient_id   UUID NOT NULL REFERENCES ingredients(id) ON DELETE CASCADE,
    quantity        FLOAT NOT NULL,
    unit            TEXT NOT NULL,
    purchased_on    DATE NOT NULL DEFAULT CURRENT_DATE,
    expires_on      DATE,
    opened          BOOLEAN NOT NULL DEFAULT FALSE,
    created_at      TIMESTAMP NOT NULL DEFAULT NOW()
);
