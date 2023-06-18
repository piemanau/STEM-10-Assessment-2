use gloo::{utils::document};
use wasm_bindgen::JsCast;
use web_sys::{InputEvent, HtmlInputElement};
use yew::{function_component, Html, Callback, html};

#[function_component]
fn App() -> Html {
    
    let oninput = Callback::from(|event: InputEvent| {
        let value = event.clone()
            .target()
            .unwrap()
            .unchecked_into::<HtmlInputElement>()
            .value();
        let num_value = value.parse::<i128>().unwrap_or(0);
        document().get_element_by_id("output").unwrap().set_inner_html((num_value * num_value).to_string().as_str());
    });

    html! {
        <div>
            <h1>{"Squaring Calculator"}</h1>
            <input type="number" name={"input"} oninput={oninput} placeholder="Number"/>
            <div id="output"></div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}