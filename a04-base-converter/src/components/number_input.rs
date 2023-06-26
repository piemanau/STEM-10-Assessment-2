use gloo::utils::document;
use wasm_bindgen::JsCast;
use web_sys::{HtmlInputElement, InputEvent};
use yew::{function_component, html, Callback, Html, Properties};

use crate::{convert_from_base_to_base, updateValue, Output};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub name: String,
    pub output: Output,
    pub value: String,
}

#[function_component(NumberInput)]
pub fn text_input(props: &Props) -> Html {
    let oninput = Callback::from(|tuple: (InputEvent, String)| {
        let base_in = document()
            .get_element_by_id("basein")
            .unwrap()
            .unchecked_into::<HtmlInputElement>()
            .value();
        let base_out = document()
            .get_element_by_id("baseout")
            .unwrap()
            .unchecked_into::<HtmlInputElement>()
            .value();

        if base_in == String::from("1") || base_out == String::from("1") {
            return;
        }

        let base_key_in = document()
            .get_element_by_id("basekeyin")
            .unwrap()
            .unchecked_into::<HtmlInputElement>()
            .value();
        let base_key_out = document()
            .get_element_by_id("basekeyout")
            .unwrap()
            .unchecked_into::<HtmlInputElement>()
            .value();

        let prev_base_key_in = document()
            .get_element_by_id("basekeyin")
            .unwrap()
            .get_attribute("prevvalue");
        let prev_base_key_out = document()
            .get_element_by_id("basekeyout")
            .unwrap()
            .get_attribute("prevvalue");
        let prev_base_in = document()
            .get_element_by_id("basein")
            .unwrap()
            .get_attribute("prevvalue");
        let prev_base_out = document()
            .get_element_by_id("baseout")
            .unwrap()
            .get_attribute("prevvalue");

        if tuple.1.starts_with("Both") {
            let outputs = tuple.1.split_whitespace().collect::<Vec<&str>>();
            let outputs = (
                outputs.get(1).unwrap().to_ascii_lowercase(),
                outputs.get(2).unwrap().to_ascii_lowercase(),
            );

            let value_one = document().get_element_by_id(&outputs.0).unwrap();
            let value_two = document().get_element_by_id(&outputs.1).unwrap();

            updateValue(
                document().get_element_by_id(&outputs.0).unwrap(),
                convert_from_base_to_base(
                    value_one.unchecked_into::<HtmlInputElement>().value(),
                    prev_base_in.unwrap().parse().unwrap(),
                    base_in.parse::<u32>().unwrap().clone(),
                    &prev_base_key_in.unwrap(),
                    &base_key_in,
                ),
            );

            updateValue(
                document().get_element_by_id(&outputs.1).unwrap(),
                convert_from_base_to_base(
                    value_two.unchecked_into::<HtmlInputElement>().value(),
                    prev_base_out.unwrap().parse().unwrap(),
                    base_out.parse().unwrap(),
                    &prev_base_key_out.unwrap(),
                    &base_key_out,
                ),
            );
        } else {
            let value = tuple
                .0
                .clone()
                .target()
                .unwrap()
                .unchecked_into::<HtmlInputElement>()
                .value();

            if tuple
                .0
                .target()
                .unwrap()
                .unchecked_into::<HtmlInputElement>()
                .id()
                == String::from("numberone")
            {
                updateValue(
                    document()
                        .get_element_by_id(tuple.1.to_lowercase().clone().as_ref())
                        .unwrap(),
                    convert_from_base_to_base(
                        value,
                        base_in.parse().unwrap(),
                        base_out.parse().unwrap(),
                        &base_key_in,
                        &base_key_out,
                    ),
                );
            } else {
                updateValue(
                    document()
                        .get_element_by_id(tuple.1.to_lowercase().clone().as_ref())
                        .unwrap(),
                    convert_from_base_to_base(
                        value,
                        base_out.parse().unwrap(),
                        base_in.parse().unwrap(),
                        &base_key_out,
                        &base_key_in,
                    ),
                );
            }
        }
        // Update previous values
        let _prev_base_key_in = document()
            .get_element_by_id("basekeyin")
            .unwrap()
            .set_attribute("prevvalue", &base_key_in);
        let _prev_base_key_out = document()
            .get_element_by_id("basekeyout")
            .unwrap()
            .set_attribute("prevvalue", &base_key_out);
        let _prev_base_in = document()
            .get_element_by_id("basein")
            .unwrap()
            .set_attribute("prevvalue", &base_in);
        let _prev_base_out = document()
            .get_element_by_id("baseout")
            .unwrap()
            .set_attribute("prevvalue", &base_out);
    });

    html! {
            <div style="display: flex; align-items: center; width: 80vmin">
                <div class="inner-input">
                    <p>{props.name.clone()}</p>
                </div>
                <input class="inner-input" type="text" value={props.value.clone()} prevvalue={props.value.clone()} placeholder={props.name.clone()} name={props.name.clone().split_whitespace().collect::<Vec<&str>>().join("").to_lowercase()} id={props.name.clone().split_whitespace().collect::<Vec<&str>>().join("").to_lowercase()} oninput={match props.output.clone() {Output::Value(v) => move |event: InputEvent| { oninput.emit((event, v.clone()));}}} />
            </div>
    }
}
