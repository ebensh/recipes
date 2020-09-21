#![recursion_limit="256"]

mod recipes;

use std::collections::VecDeque;
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use recipes::recipe::Recipe;
use recipes::simplissime;

#[derive(Default)]
struct RecipeSlot {
  label: String,
  recipe: Option<Recipe>,
}

struct Model {
    link: ComponentLink<Self>,
    recipes: VecDeque<Recipe>,
    recipe_slots: [RecipeSlot;7],
}

enum Msg {
  PrevRecipe,
  NextRecipe,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let recipes: Vec<Recipe> = simplissime::RECIPES.into_iter().map(|rs| Recipe::from(*rs)).collect();
        Self {
            link,
            recipes: VecDeque::from(recipes),
            recipe_slots: [
              RecipeSlot{ label: "Sunday".to_string(), recipe: None },
              RecipeSlot{ label: "Monday".to_string(), recipe: None },
              RecipeSlot{ label: "Tuesday".to_string(), recipe: None },
              RecipeSlot{ label: "Wednesday".to_string(), recipe: None },
              RecipeSlot{ label: "Thursday".to_string(), recipe: None },
              RecipeSlot{ label: "Friday".to_string(), recipe: None },
              RecipeSlot{ label: "Saturday".to_string(), recipe: None },
            ],
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::PrevRecipe => self.recipes.rotate_left(1),
            Msg::NextRecipe => self.recipes.rotate_right(1),
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
      html! {
          <>
            <span>{ "Test "}</span>
            <div id="header">
              <span>{ "Test header" }</span>
            </div>
            <div id="sidebar">
              <span>{ "Test sidebar" }</span>
            </div>
            <div>
              <p><span>{
                match self.recipes.front() {
                  Some(recipe) => recipe.to_string(),
                  None => "None".to_string(),
                }
              }</span></p>
              <button onclick=self.link.callback(|_| Msg::PrevRecipe)>{ "Previous Recipe" }</button>
              <button onclick=self.link.callback(|_| Msg::NextRecipe)>{ "Next Recipe" }</button>
            </div>
          </>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}