use gloo::console::log;
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::Element;
use yew::prelude::*;
use yew_router::prelude::*;

mod components;

use crate::components::number_input::*;

// Specifies what path of the URL points to each arm in the enum
#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/instructions")]
    Instructions,
}

// Output for each of the inputs
#[derive(PartialEq, Clone)]
pub enum Output {
    Value(String),
}

// Gets the url and runs the relevant code
fn switch(routes: Route) -> Html {
    match routes {
        // If it is at "home" it renders the home code
        Route::Home => home(),
        // If it is at intructions it will render the instructions
        Route::Instructions => html! {
            // Instruction elements
            <div class="container">
                <div>
                    <h1 class="header">{"Base Converter Instructions"}</h1>
                    <p class="text" style="font-weight: bold">{"Don't reload this page, doesn't work for now"}</p>
                    <p class="text">{"Number one and number two are the numbers you are converting to and from."}</p>
                    <p class="text">{"Base in and base out are the bases you are converting to and from, base 10 to base 2 is decimal to binary."}</p>
                    <p class="text">{"Base key in and base key out are the \"keys\" you use to convert to and from. "}<br/><br/>{"\"A\" in base 11 is 10 by default but if you change the 11th index to \"B\", 10 would be equal to \"B\" instead of \"A\"."}</p>
                    <Link<Route> to={Route::Home}><button class="button">{ "Go to converter" }</button></Link<Route>>
                </div>
            </div>
        },
    }
}

// Where the main function calls, it calles the switch function
#[function_component]
fn App() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

// The "home" function, the code where the converter is
fn home() -> Html {
    html! {
        <div class="container">
            <div class="background">
                // Inputs and outputs
                <NumberInput name="Number One" value="0" output={Output::Value(String::from("NumberTwo"))}/>
                <NumberInput name="Base In" value="10" output={Output::Value(String::from("Both NumberOne NumberTwo"))}/>
                <NumberInput name="Base Key In" value="0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ" output={Output::Value(String::from("Both NumberOne NumberTwo"))}/>
                <NumberInput name="Number Two" value="0" output={Output::Value(String::from("NumberOne"))}/>
                <NumberInput name="Base Out" value="2" output={Output::Value(String::from("Both NumberOne NumberTwo"))}/>
                <NumberInput name="Base Key Out" value="0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ" output={Output::Value(String::from("Both NumberOne NumberTwo"))}/>
                // Button to go to the intructions
                <Link<Route> to={Route::Instructions}><button class="button inner-input">{ "Go to instructions" }</button></Link<Route>>
            </div>
        </div>
    }
}

// Where the code starts, runs the app
fn main() {
    yew::Renderer::<App>::new().render();
}

// Converts from base 10
fn convert_from_base_ten(mut number: u128, base: u32, base_key: &str) -> String {
    // If the number is zero it just returns the first value in base key
    if number == 0 {
        return base_key[0..1].to_owned();
    }

    let mut output = String::from("");

    // While the number isnt 0 it gets the modulo of the number and adds it to the output using the base key and divides the number by the base
    while number != 0 {
        let l;
        (number, l) = (number / base as u128, number % base as u128);
        output = String::from(&base_key[l as usize..l as usize + 1 as usize]) + &output;
    }

    // Returns the output
    output
}

// Convert to base 10
fn convert_to_base_ten(number: String, base: u32, base_key: &str) -> u128 {
    let mut output: u128 = 0;

    // For each character the in the string it gets the value
    for character in number.chars() {
        let num;
        match base_key.find(character) {
            Some(v) => num = v,
            None => {
                output = 0;
                log!("Error: invalid input");
                break;
            }
        }
        // Once it gets the value it times it by the base (output * 10 in base 10) and adds the number
        output *= base as u128;
        output += num as u128;
    }

    // Returns the output
    output
}

// Convert from any base to any other base using the other 2 conversion methods
fn convert_from_base_to_base(
    number: String,
    base_in: u32,
    base_out: u32,
    base_key_in: &str,
    base_key_out: &str,
) -> String {
    convert_from_base_ten(
        convert_to_base_ten(number, base_in, base_key_in),
        base_out,
        base_key_out,
    )
}

#[wasm_bindgen(module = "/updatevalue.js")]
extern "C" {
    fn updateValue(element: Element, value: String);
}

// Test the base conversions, if they all pass it should work
#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_convert_from_base_ten() {
        let base_key = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ".trim();
        assert_eq!(convert_from_base_ten(100, 8, base_key), String::from("144"));
    }

    #[test]
    fn test_convert_to_base_ten() {
        let base_key = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ".trim();
        assert_eq!(convert_to_base_ten(String::from("144"), 8, base_key), 100);
    }

    #[test]
    fn test_convert_from_base_to_base() {
        let base_key = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ".trim();
        assert_eq!(
            convert_from_base_to_base(String::from("144"), 8, 16, base_key, base_key),
            String::from("64")
        );
    }

    #[test]
    fn test_convert_from_base_ten_with_letter() {
        let base_key = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ".trim();
        assert_eq!(convert_from_base_ten(255, 16, base_key), String::from("FF"));
    }

    #[test]
    fn test_convert_from_base_to_base_with_letter() {
        let base_key = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ".trim();
        assert_eq!(
            convert_from_base_to_base(String::from("FF"), 16, 20, base_key, base_key),
            String::from("CF")
        );
    }
}
