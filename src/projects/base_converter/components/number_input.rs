use gloo::utils::document;
use wasm_bindgen::JsCast;
use web_sys::{HtmlInputElement, InputEvent};
use yew::{function_component, html, Callback, Html, Properties};

use crate::projects::base_converter::{convert_from_base_to_base, updateValue, Output};

// Properties passed in when adding the component to HTML
#[derive(Properties, PartialEq)]
pub struct Props {
    pub name: String,
    pub output: Output,
    pub value: String,
}

// The actual component that returns HTML
#[function_component(NumberInput)]
pub fn text_input(props: &Props) -> Html {

    // On input for the input elements
    let oninput = Callback::from(|tuple: (InputEvent, String)| {
        // Get a bunch of values from HTML elements
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

        // If atleast one of the bases is 1 return and do nothing
        if base_in == String::from("1") || base_out == String::from("1") {
            return;
        }

        // Get more values
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

        // Deconstruct tuple into 2 seperate variables
        let (event, output) = tuple;

        // If the output goes to both
        if output.starts_with("Both") {
            // Splits the outputs into a tuple
            let outputs = output.split_whitespace().collect::<Vec<&str>>();
            let outputs = (
                outputs.get(1).unwrap().to_ascii_lowercase(),
                outputs.get(2).unwrap().to_ascii_lowercase(),
            );

            // Gets elements of both of the outputs
            let value_one = document().get_element_by_id(&outputs.0).unwrap();
            let value_two = document().get_element_by_id(&outputs.1).unwrap();

            // Updates the values of the outputs aswell as calculating them
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
        // If it is not to both outputs
        } else {
            // Gets the value of the input element, one of the numbers going in
            let value = event
                .clone()
                .target()
                .unwrap()
                .unchecked_into::<HtmlInputElement>()
                .value();

            // If the input is from numberone
            if event
                .target()
                .unwrap()
                .unchecked_into::<HtmlInputElement>()
                .id()
                == String::from("numberone")
            {
                // Calculates and updates the value
                updateValue(
                    document()
                        .get_element_by_id(output.to_lowercase().clone().as_ref())
                        .unwrap(),
                    convert_from_base_to_base(
                        value,
                        base_in.parse().unwrap(),
                        base_out.parse().unwrap(),
                        &base_key_in,
                        &base_key_out,
                    ),
                );
            // If it is numberone
            } else {
                // Calculates and updates the other output
                updateValue(
                    document()
                        .get_element_by_id(output.to_lowercase().clone().as_ref())
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
        // Update previous values, used if there is an invalid input somewhere
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

    // Output/input HTML
    html! {
            <div style="display: flex; align-items: center; width: 80vmin">
                // Name element
                <div class="inner-input">
                    <p>{props.name.clone()}</p>
                </div>
                // Input element
                <input class="inner-input" type="text" value={props.value.clone()} prevvalue={props.value.clone()} placeholder={props.name.clone()} name={props.name.clone().split_whitespace().collect::<Vec<&str>>().join("").to_lowercase()} id={props.name.clone().split_whitespace().collect::<Vec<&str>>().join("").to_lowercase()} oninput={match props.output.clone() {Output::Value(v) => move |event: InputEvent| { oninput.emit((event, v.clone()));}}} />
            </div>
    }
}
