use super::recipe::Recipe;

pub static BULGUR_FILLED_TOMATOES: Recipe<'static> = Recipe {
  name: "Bulgur filled tomatoes",
  ingredients: &["Bulgur", "Tomatoes"]
};

#[allow(dead_code)]
pub static RECIPES: &[&Recipe<'static>] = &[
  &BULGUR_FILLED_TOMATOES
];