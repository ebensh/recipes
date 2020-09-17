use std::fmt;

#[derive(Debug)]
pub struct Recipe<'a> {
  name: &'a str,
  ingredients: Vec<&'a str>,
}

impl<'a> Recipe<'a> {
  pub fn new(name: &'a str, ingredients: Vec<&'a str>) -> Self {
    Self {
      name: &name,
      ingredients: ingredients,
    }
  }

  pub fn name(mut self, name: &'a str) -> Self {
    self.name = name;
    self
  }

  pub fn ingredients(mut self, ingredients: Vec<&'a str>) -> Self {
    self.ingredients = ingredients;
    self
  }
}

impl<'a> Default for Recipe<'a> {
  fn default() -> Self {
    Self {
      name: "A recipe",
      ingredients: vec!["Ingredient 1", "Ingredient 2"],
    }
  }
}

impl<'a> fmt::Display for Recipe<'a> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "({}, {:?})", self.name, self.ingredients)
  }
}