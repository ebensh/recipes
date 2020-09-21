use std::fmt;

pub struct RecipeStatic<'a> {
  pub name: &'a str,
  pub ingredients: &'a[&'a str],
}

#[derive(Debug)]
pub struct Recipe {
  pub name: String,
  pub ingredients: Vec<String>,
}

impl Recipe {
  #[allow(dead_code)]
  pub fn new(name: String, ingredients: Vec<String>) -> Self {
    Self {
      name,
      ingredients,
    }
  }

  #[allow(dead_code)]
  pub fn name(mut self, name: String) -> Self {
    self.name = name;
    self
  }

  #[allow(dead_code)]
  pub fn ingredients(mut self, ingredients: Vec<String>) -> Self {
    self.ingredients = ingredients;
    self
  }
}

impl Default for Recipe {
  fn default() -> Self {
    Self {
      name: "A recipe".to_string(),
      ingredients: vec!["Ingredient 1".to_string(),
                        "Ingredient 2".to_string()],
    }
  }
}

impl fmt::Display for Recipe {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "({}, {:?})", self.name, self.ingredients)
  }
}

impl From<&RecipeStatic<'_>> for Recipe {
  fn from(rhs: &RecipeStatic<'_>) -> Self {
    Self {
      name: rhs.name.to_string(),
      ingredients: rhs.ingredients.into_iter().map(|s| s.to_string()).collect(),
    }
  }
}