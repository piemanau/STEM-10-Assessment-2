use yew::prelude::*;
use yew::{html};

mod components;

use crate::components::number_input::*;

#[function_component]
fn App() -> Html {
    html! {
        <div class="container">
            <NumberInput name="input"/>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}