use super::recipe::RecipeStatic;

pub static PERFECT_GUACAMOLE: RecipeStatic = RecipeStatic{
  // https://www.simplyrecipes.com/recipes/perfect_guacamole/
  name: "Perfect Guacamole",
  ingredients: &["2 ripe avocados", "Salt", "Fresh lime juice"],
};

pub static PERFECT_HARD_BOILED_EGGS: RecipeStatic = RecipeStatic{
  // https://www.simplyrecipes.com/recipes/perfect_guacamole/
  name: "Perfect Hard Boiled Eggs",
  ingredients: &["1 egg"],
};

#[allow(dead_code)]
pub static RECIPES: &[&RecipeStatic<'static>] = &[
  &PERFECT_GUACAMOLE,
  &PERFECT_HARD_BOILED_EGGS
];