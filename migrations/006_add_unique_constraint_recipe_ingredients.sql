ALTER TABLE recipe_ingredients 
    ADD CONSTRAINT recipe_ingredients_recipe_sort_unique 
    UNIQUE (recipe_id, sort_order);
