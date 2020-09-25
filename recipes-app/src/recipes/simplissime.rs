use super::recipe::RecipeStatic;

pub static BULGUR_FILLED_TOMATOES: RecipeStatic<'static> = RecipeStatic {
  name: "Bulgur filled tomatoes",
  ingredients: &["Bulgur", "Tomatoes"],
};

pub static SALADE_DE_QUINOA_FRAICHEUR: RecipeStatic<'static> = RecipeStatic {
  name: "Salade de Quinoa Fraîcheur",
  ingredients: &[
    "Quinoa",
    "Raisin noir",
    "Concombre",
    "Menthe",
    "Huile d'olive",
    "Framboises",
  ],
};

#[allow(dead_code)]
pub static RECIPES: &[&RecipeStatic<'static>] =
  &[&BULGUR_FILLED_TOMATOES, &SALADE_DE_QUINOA_FRAICHEUR];
