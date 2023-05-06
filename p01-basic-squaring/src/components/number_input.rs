use gloo::{utils::document};
use wasm_bindgen::JsCast;
use web_sys::{InputEvent, HtmlInputElement};
use yew::{Properties, function_component, Html, Callback, html};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub name: String,
}

#[function_component(NumberInput)]
pub fn text_input(props: &Props) -> Html {

    let oninput = Callback::from(|event: InputEvent| {
        let value = event.clone()
            .target()
            .unwrap()
            .unchecked_into::<HtmlInputElement>()
            .value();
        let num_value = value.parse::<i32>().unwrap_or(0);
        document().get_element_by_id("output").unwrap().set_inner_html((num_value as i64 * num_value as i64).to_string().as_str());
    });
    html! {
        <div>
            <div id="output"></div>
            <input type="number" name={props.name.clone()} oninput={oninput} />
        </div>
    }
}