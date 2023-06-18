use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::Element;
use yew::prelude::*;
use yew_router::prelude::*;

mod components;

use crate::components::number_input::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/instructions")]
    Instructions,
}

#[derive(PartialEq, Clone)]
pub enum Output {
    Value(String),
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => home(),
        Route::Instructions => html! {
           <p>{"Instructions"}</p>
        }
    }
}

#[function_component]
fn App() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} /> // <- must be child of <BrowserRouter>
        </BrowserRouter>
    }
}


fn home() -> Html {
    html! {
        <div>
            <NumberInput name="Number One" value="" output={Output::Value(String::from("NumberTwo"))}/>
            <NumberInput name="Base In" value="10" output={Output::Value(String::from("Both NumberOne NumberTwo"))}/>
            <NumberInput name="Base Key In" value="0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ" output={Output::Value(String::from("Both NumberOne NumberTwo"))}/>
            <NumberInput name="Number Two" value="" output={Output::Value(String::from("NumberOne"))}/>
            <NumberInput name="Base Out" value="2" output={Output::Value(String::from("Both NumberOne NumberTwo"))}/>
            <NumberInput name="Base Key Out" value="0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ" output={Output::Value(String::from("Both NumberOne NumberTwo"))}/>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

fn convert_from_base_ten(mut number: u128, base: u32, base_key: &str) -> String {
    if number == 0 {
        return base_key[0..1].to_owned();
    }

    let mut output = String::from("");

    while number != 0 {
        let l;
        (number, l) = (number / base as u128, number % base as u128);
        output = String::from(&base_key[l as usize..l as usize + 1 as usize]) + &output;
    }

    output
}

fn convert_to_base_ten(number: String, base: u32, base_key: &str) -> u128 {
    let mut output: u128 = 0;

    for character in number.chars() {
        //TODO: remove unwrap
        let num = base_key.find(character).unwrap();
        output *= base as u128;
        output += num as u128;
    }

    output
}

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
