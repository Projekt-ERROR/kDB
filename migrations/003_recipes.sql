CREATE TABLE recipes (
  id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  name            TEXT NOT NULL UNIQUE,
  description     TEXT,
  servings        INTEGER NOT NULL DEFAULT 4,
  prep_time_mins  INTEGER,
  cook_time_mins  INTEGER,
  total_time_mins INTEGER GENERATED ALWAYS AS (
                    COALESCE(prep_time_mins, 0) + COALESCE(cook_time_mins, 0)
                  ) STORED,
  difficulty      TEXT CHECK (difficulty IN ('easy', 'medium', 'hard')),
  cuisine         TEXT,
  meal_type       TEXT CHECK (meal_type IN ('breakfast', 'lunch', 'dinner', 'snack', 'dessert')),
  source          TEXT,
  created_at      TIMESTAMPTZ DEFAULT now(),
  updated_at      TIMESTAMPTZ DEFAULT now()
);

CREATE OR REPLACE FUNCTION set_updated_at()
RETURNS TRIGGER AS $$
BEGIN
  NEW.updated_at = now();
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER recipes_updated_at
  BEFORE UPDATE ON recipes
  FOR EACH ROW EXECUTE FUNCTION set_updated_at();

CREATE TABLE recipe_ingredients (
  id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  recipe_id       UUID NOT NULL REFERENCES recipes(id) ON DELETE CASCADE,
  ingredient_id   UUID REFERENCES ingredients(id) ON DELETE SET NULL,
  custom_name     TEXT,
  quantity        NUMERIC NOT NULL,
  unit            TEXT,
  preparation     TEXT,
  optional        BOOLEAN DEFAULT false,
  sort_order      INTEGER NOT NULL DEFAULT 0,
  CONSTRAINT ingredient_or_custom CHECK (
    ingredient_id IS NOT NULL OR custom_name IS NOT NULL
  )
);

CREATE TABLE recipe_steps (
  id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  recipe_id       UUID NOT NULL REFERENCES recipes(id) ON DELETE CASCADE,
  step_number     INTEGER NOT NULL,
  instruction     TEXT NOT NULL,
  timer_mins      INTEGER,
  UNIQUE (recipe_id, step_number)
);

CREATE TABLE recipe_tags (
  recipe_id       UUID NOT NULL REFERENCES recipes(id) ON DELETE CASCADE,
  tag             TEXT NOT NULL,
  PRIMARY KEY (recipe_id, tag)
);
