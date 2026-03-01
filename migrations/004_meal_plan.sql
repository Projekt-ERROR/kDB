CREATE TABLE meal_plan (
  id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  plan_date   DATE NOT NULL,
  meal_slot   TEXT NOT NULL CHECK (meal_slot IN ('breakfast', 'lunch', 'dinner', 'snack')),
  recipe_id   UUID REFERENCES recipes(id) ON DELETE SET NULL,
  custom_meal TEXT,
  notes       TEXT,
  created_at  TIMESTAMPTZ DEFAULT now(),
  CONSTRAINT recipe_or_custom CHECK (
    recipe_id IS NOT NULL OR custom_meal IS NOT NULL
  ),
  UNIQUE (plan_date, meal_slot)
);
