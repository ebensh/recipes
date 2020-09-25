#![recursion_limit="256"]

mod recipes;

use std::collections::VecDeque;
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use recipes::recipe::Recipe;
use recipes::{simplissime,websites};

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
      let recipes: Vec<Recipe> = websites::RECIPES.into_iter().map(|rs| Recipe::from(*rs)).collect();
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
            <p><span>{
              match self.recipes.front() {
                Some(recipe) => recipe.to_string(),
                None => "None".to_string(),
              }
            }</span></p>
            <button onclick=self.link.callback(|_| Msg::PrevRecipe)>{ "Previous Recipe" }</button>
            <button onclick=self.link.callback(|_| Msg::NextRecipe)>{ "Next Recipe" }</button>
          </>
        }
    }
}

/*
#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}
*/
#[wasm_bindgen(start)]
pub fn run_app() -> Result<(), JsValue> {
    let window = web_sys::window().expect("global `window` does not exist");
    let document = window.document().expect("global `window` does not have `document`");
    let body = document.body().expect("`document` does not have a body");
    let children = body.children();
    let div_main = children.named_item("main").expect("missing div with id `main`");

    App::<Model>::new().mount(div_main);

    Ok(())
}