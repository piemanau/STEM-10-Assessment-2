use gloo::{utils::document, console::log};
use wasm_bindgen::{JsCast, prelude::wasm_bindgen};
use web_sys::{InputEvent, HtmlInputElement, Event, Element};
use yew::{Properties, function_component, Html, Callback, html};

use crate::{convert_from_base_to_base, Output, components::number_input};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub name: String,
    pub output: Output,
}

#[function_component(NumberInput)]
pub fn text_input(props: &Props) -> Html {

    let base_key = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ".trim();
    let oninput = Callback::from(|tuple: (InputEvent, String)| {
        let value = tuple.0
            .clone()
            .target()
            .unwrap()
            .unchecked_into::<HtmlInputElement>()
            .value();
        number_input::updateValue(document().get_element_by_id(tuple.1.to_lowercase().clone().as_ref()).unwrap(), convert_from_base_to_base(value, 16, 8, base_key));
        
        log!("Updated.");
    });
    
        html! {
            <div>
                <p>{props.name.clone()}</p>
                <input type="text" name={props.name.clone().to_lowercase()} id={props.name.clone().to_lowercase()} oninput={match props.output.clone() {Output::Value(v) => move |event: InputEvent| { oninput.emit((event, v.clone()));}}} />
            </div>
        
    }
}

#[wasm_bindgen(module = "/updatevalue.js")]
extern "C" {
    fn updateValue(element: Element, value: String);
}