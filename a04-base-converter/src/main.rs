use yew::prelude::*;

mod components;

use crate::components::number_input::*;

#[derive(PartialEq, Clone)]
pub enum Output {
    Value(String),
}

#[function_component]
fn App() -> Html {
    html! {
        <div>
            <NumberInput name="First" output={Output::Value(String::from("Second"))}/>
            <NumberInput name="Second" output={Output::Value(String::from("First"))}/>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

fn convert_from_base_ten(mut number: u32, base: u32, base_key: &str) -> String {
    if number == 0 {
        return base_key[0..1].to_owned();
    }

    let mut output = String::from("");

    while number != 0 {
        let l;
        (number, l) = (number / base, number % base);
        output = String::from(&base_key[l as usize..l as usize + 1 as usize]) + &output;
    }

    output
}

fn convert_to_base_ten(number: String, base: u32, base_key: &str) -> u32 {
    let mut output: u32 = 0;

    for character in number.chars() {
        //TODO: remove unwrap
        let num = base_key.find(character).unwrap();
        output *= base;
        output += num as u32;
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
