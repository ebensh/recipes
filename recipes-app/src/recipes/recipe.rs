use std::fmt;

#[derive(Debug)]
pub struct Recipe<'a> {
  pub name: &'a str,
  pub ingredients: &'a[&'a str],
}

impl<'a> Recipe<'a> {
  #[allow(dead_code)]
  pub fn new(name: &'a str, ingredients: &'a[&'a str]) -> Self {
    Self {
      name: &name,
      ingredients: ingredients,
    }
  }

  #[allow(dead_code)]
  pub fn name(mut self, name: &'a str) -> Self {
    self.name = name;
    self
  }

  #[allow(dead_code)]
  pub fn ingredients(mut self, ingredients: &'a[&'a str]) -> Self {
    self.ingredients = ingredients;
    self
  }
}

impl<'a> Default for Recipe<'a> {
  fn default() -> Self {
    Self {
      name: "A recipe",
      ingredients: &["Ingredient 1", "Ingredient 2"],
    }
  }
}

impl<'a> fmt::Display for Recipe<'a> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "({}, {:?})", self.name, self.ingredients)
  }
}