use gloo::{utils::document};
use wasm_bindgen::JsCast;
use web_sys::{InputEvent, HtmlInputElement};
use yew::{function_component, Html, Callback, html};

#[function_component]
fn App() -> Html {
    // Gets the value from the slider
    let oninput = Callback::from(|event: InputEvent| {
        let value = event.clone()
            .target()
            .unwrap()
            .unchecked_into::<HtmlInputElement>()
            .value();

        //Parses the value to a float
        let num_value = value.parse::<f64>().unwrap_or(0.);

        // Calculates all the values for the outputs and sets them
        document().get_element_by_id("output-celcius").unwrap().set_inner_html(&("Celcius: ".to_string() + num_value.to_string().as_str()));
        document().get_element_by_id("output-kelvin").unwrap().set_inner_html(&("Kelvin: ".to_string() + &(((num_value + 273.15) * 100.).round() / 100.).to_string().as_str()));
        let freedom: String = ((((1.8 * num_value) + 32.) * 100.).round() / 100.).to_string();
        document().get_element_by_id("output-farenheit").unwrap().set_inner_html(&("Farenheight: ".to_string() + &freedom));
        document().get_element_by_id("output-freedom").unwrap().set_inner_html(&("Freedom Units: ".to_string() + &freedom));
        document().get_element_by_id("output-rankine").unwrap().set_inner_html(&("Rankine: ".to_string() + &((((num_value + 273.15) * 1.8) * 100.).round() / 100.).to_string().as_str()));
        let temp = ((num_value / 43.229041) - 3.321) * 34.234;
        document().get_element_by_id("output-pietemp").unwrap().set_inner_html(&("Pietemp: ".to_string() + &((((temp) * 100.).round() / 100.).to_string().as_str())));
    });

    // Basic HTMl
    html! {
        // Used for centering
        <div class="outer">
            // Used for centering
            <div class="inner">
                // All the outputs
                <div id={"output-celcius"} class="text">{"Celcius: 50"}</div>
                <div id={"output-kelvin"} class="text">{"Kelvin: 323.15"}</div>
                <div id={"output-farenheit"} class="text">{"Farenheight: 122"}</div>
                <div id={"output-freedom"} class="text">{"Freedom Units: 122"}</div>
                <div id={"output-rankine"} class="text">{"Rankine: 581.66"}</div>
                <div id={"output-pietemp"} class="text">{"Pietemp: 54.51"}</div>
                // Slider, oninput runs the calculation code when moved
                <input class="slider" type="range" min="-50" max="200" name={"slider"} oninput={oninput} />
            </div>
        </div>
    }
}

// Where the app starts and runs the code.
fn main() {
    yew::Renderer::<App>::new().render();
}