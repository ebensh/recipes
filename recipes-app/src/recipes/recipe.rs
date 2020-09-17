#[derive(Debug)]
pub struct Recipe<'a> {
  name: &'a str,
  ingredients: Vec<&'a str>,
}

impl Recipe<'_> {
  fn new(name: &'_ str, ingredients: Vec<&'_ str>) -> Self {
    Self {
      name: &name,
      ingredients: ingredients,
    }
  }

  fn name(mut self, name: &'a str) -> Self {
    self.name = name;
    self
  }

  fn ingredients(mut self, ingredients: Vec<&'a str>) -> Self {
    self.ingredients = ingredients;
    self
  }
}

impl Default for Recipe<'a> {
  fn default() -> Self {
    Self {
      name: "A recipe",
      ingredients: vec!["Ingredient 1", "Ingredient 2"],
    }
  }
}
