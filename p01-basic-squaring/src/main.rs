use wasm_bindgen::JsCast;
use web_sys::{InputEvent, HtmlInputElement};
use yew::{function_component, Html, Callback, html, use_state};

#[function_component]
fn App() -> Html {
    
    // Value that goes on the output element
    let value = use_state(|| 0);

    // When input is detected it squares the number and puts it on the output element
    let oninput: Callback<InputEvent> = {

        // Create a copy of the value variable
        let value = value.clone();

       Callback::from(move |event: InputEvent| {
        // Grabs the input value
        let num_value = event
            .target()
            .unwrap()
            .unchecked_into::<HtmlInputElement>()
            .value();

            // Parses the input to i128
            let num_value = num_value.parse::<i128>().unwrap_or(0);

            // Squares the number
            let num_value = num_value * num_value;

            // Updates the output
            value.set(num_value);
        })
    };


    // Html for title, input and output
    html! {
        <div>
            // Header (title)
            <h1>{"Squaring Calculator"}</h1>
            // Input element, calls input when it gets some input
            <input type="number" name={"input"} {oninput} placeholder="Number"/>
            // Output element, dereference the value
            <div id="output">{ *value }</div>
        </div>
    }
}


// Main function, where the code starts and runs the app.
fn main() {
    yew::Renderer::<App>::new().render();
}