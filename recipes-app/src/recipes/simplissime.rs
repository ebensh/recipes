use super::recipe::RecipeStatic;

pub static BULGUR_FILLED_TOMATOES: RecipeStatic<'static> = RecipeStatic {
  name: "Bulgur filled tomatoes",
  ingredients: &["Bulgur", "Tomatoes"]
};

#[allow(dead_code)]
pub static RECIPES: &[&RecipeStatic<'static>] = &[
  &BULGUR_FILLED_TOMATOES
];