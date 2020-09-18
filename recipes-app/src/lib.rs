mod recipes;

use wasm_bindgen::prelude::*;
use yew::prelude::*;
use recipes::recipe::Recipe;
use recipes::simplissime;

struct Model {
    link: ComponentLink<Self>,
    value: i64,
}

enum Msg {
  AddOne,
  SubOne,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            value: 0,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => self.value += 1,
            Msg::SubOne => self.value -= 1
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
      let recipe: &Recipe<'_> = &simplissime::BULGUR_FILLED_TOMATOES;
      let recipe: Recipe<'_> = Recipe::new("test", &["ing1", "ing2"]);

        html! {
            <div>
            <p><span>{recipe}</span></p>
            <button onclick=self.link.callback(|_| Msg::SubOne)>{ "-1" }</button>
            <button onclick=self.link.callback(|_| Msg::AddOne)>{ "+1" }</button>
                <p>{ self.value }</p>
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}