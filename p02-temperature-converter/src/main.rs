use wasm_bindgen::JsCast;
use web_sys::{HtmlInputElement, InputEvent};
use yew::{function_component, html, use_state, Callback, Html};

#[function_component]
fn App() -> Html {
    // Sets up defaults for the temperatures
    let celcius = use_state(|| 50.);
    let kelvin = use_state(|| 323.15);
    let fahrenheit = use_state(|| 122.);
    let freedom = use_state(|| 122.);
    let rankine = use_state(|| 581.66);
    let pietemp = use_state(|| 54.51);

    // Gets the value from the slider and updates the temperatures accordingly
    let oninput: Callback<InputEvent> = {
        // Copy variables to update the values of the temperatures
        let celcius = celcius.clone();
        let kelvin = kelvin.clone();
        let fahrenheit = fahrenheit.clone();
        let freedom = freedom.clone();
        let rankine = rankine.clone();
        let pietemp = pietemp.clone();

        // Gets the slider value and updates the temperatures accordingly
        Callback::from(move |event: InputEvent| {
            // Gets the slider value
            let value = event
                .clone()
                .target()
                .unwrap()
                .unchecked_into::<HtmlInputElement>()
                .value();

            //Parses the value to a float
            let value = value.parse::<f64>().unwrap_or(0.);

            // Calculates the temp for "freedom units" and fahrenheit
            let freedom_temp = (((1.8 * value) + 32.) * 100.).round() / 100.;

            // Calculates all the values for the outputs and sets them
            celcius.set(value);
            kelvin.set(((value + 273.15) * 100.).round() / 100.);
            fahrenheit.set(freedom_temp);
            freedom.set(freedom_temp);
            rankine.set((((value + 273.15) * 1.8) * 100.).round() / 100.);
            pietemp.set(((((value / 43.229041) - 3.321) * 34.234) * 100.).round() / 100.);
        })
    };

    // Basic HTMl
    html! {
        // Used for centering
        <div class="outer">
            // Used for centering
            <div class="inner">
                // All the outputs, uses the format macro to concatenate the output after the name, e.g. celcius
                <div id={"output-celcius"} class="text">{ format!("Celcius: {}", *celcius) }</div>
                <div id={"output-kelvin"} class="text">{ format!("Kelvin: {}", *kelvin) }</div>
                <div id={"output-farenheit"} class="text">{ format!("Fahrenheit: {}", *fahrenheit) }</div>
                <div id={"output-freedom"} class="text">{ format!("Freedom Units: {}", *freedom) }</div>
                <div id={"output-rankine"} class="text">{ format!("Rankine: {}", *rankine) }</div>
                <div id={"output-pietemp"} class="text">{ format!("Pietemp: {}", *pietemp) }</div>
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
