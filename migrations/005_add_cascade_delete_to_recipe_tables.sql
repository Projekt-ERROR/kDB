ALTER TABLE recipe_ingredients 
    DROP CONSTRAINT recipe_ingredients_recipe_id_fkey,
    ADD CONSTRAINT recipe_ingredients_recipe_id_fkey 
        FOREIGN KEY (recipe_id) REFERENCES recipes(id) ON DELETE CASCADE;

ALTER TABLE recipe_steps
    DROP CONSTRAINT recipe_steps_recipe_id_fkey,
    ADD CONSTRAINT recipe_steps_recipe_id_fkey 
        FOREIGN KEY (recipe_id) REFERENCES recipes(id) ON DELETE CASCADE;

ALTER TABLE recipe_tags
    DROP CONSTRAINT recipe_tags_recipe_id_fkey,
    ADD CONSTRAINT recipe_tags_recipe_id_fkey 
        FOREIGN KEY (recipe_id) REFERENCES recipes(id) ON DELETE CASCADE;
