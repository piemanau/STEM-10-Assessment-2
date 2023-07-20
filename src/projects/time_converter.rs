use yew::prelude::*;
use web_sys::{HtmlInputElement, InputEvent};
use wasm_bindgen::JsCast;

#[function_component(TimeConverter)]
pub(crate) fn time_converter() -> Html {
    let total = use_state(|| 50);
    let hours = use_state(|| 0);
    let minutes = use_state(|| 50);

     // Gets the value from the slider and updates the outputs accordingly
     let oninput: Callback<InputEvent> = {
        // Copy variables to update the values of the outputs
        let total = total.clone();
        let hours = hours.clone();
        let minutes = minutes.clone();

        // Gets the slider value and updates the outputs accordingly
        Callback::from(move |event: InputEvent| {
            // Gets the slider value
            let value = event
                .clone()
                .target()
                .unwrap()
                .unchecked_into::<HtmlInputElement>()
                .value();

            //Parses the value to a unsigned 64 bit integer
            let value = value.parse::<u64>().unwrap_or(0);

            // Calculates all the values for the outputs and sets them
            total.set(value);
            hours.set(value / 60);
            minutes.set(value % 60);
        })
    };

    // Basic html
    html! {
        <div class="center">
            // Outputs
            <div class="text">{ format!("Total Minutes: {}", *total) }</div>
            // Output with the second number having a leading 0
            <div class="text">{ format!("Hours and minutes: {}:{:02}", *hours, *minutes) }</div>
            // Slider
            <input style="width: 550px;" class="slider" type="range" min="0" max="400" name={"slider"} oninput={oninput} />
        </div>
    }
}
