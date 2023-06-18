use gloo::{utils::document};
use wasm_bindgen::JsCast;
use web_sys::{InputEvent, HtmlInputElement};
use yew::{function_component, Html, Callback, html};

#[function_component]
fn App() -> Html {
    
    // When input is detected it squares the number and puts it on the output element
    let oninput = Callback::from(|event: InputEvent| {

        // Grabs the input value
        let value = event.clone()
            .target()
            .unwrap()
            .unchecked_into::<HtmlInputElement>()
            .value();

        // Parses the input to i128
        let num_value = value.parse::<i128>().unwrap_or(0);

        // Updates the output and gets the square
        document().get_element_by_id("output").unwrap().set_inner_html((num_value * num_value).to_string().as_str());
    });


    // Html for title, input and output
    html! {
        <div>
            // Header (title)
            <h1>{"Squaring Calculator"}</h1>
            // Input element, calls input when it gets some input
            <input type="number" name={"input"} oninput={oninput} placeholder="Number"/>
            // Output element
            <div id="output">{"0"}</div>
        </div>
    }
}


// Main function, where the code starts and calls the app.
fn main() {
    yew::Renderer::<App>::new().render();
}