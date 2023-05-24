use std::slice::SliceIndex;

use fancy_regex::Regex;
use gloo::{utils::document, console::log};
use yew::{Properties, Html, Callback, html, function_component};
use evalexpr::*;

use crate::{Value, Operation, OuterValue, runFitText};

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub value: OuterValue,
    pub text: String,
}

#[function_component(Button)]
pub fn button_press(props: &Props) -> Html {

    let onclick = Callback::from(|value: Value| {
        let output_element = document().get_element_by_id("output");

        let mut inner_value = None;
        let mut inner_operation = None;

        if output_element.clone().unwrap().inner_html().starts_with("Error: ") {
            document().get_element_by_id("output").unwrap().set_inner_html("");
        }

        match value.clone() {
            Value::Number(n) => inner_value = Some(n),
            Value::Operation(o) => inner_operation = Some(o),
        }

        if inner_value != None {
            let inner_html = &(output_element.clone().unwrap().inner_html().clone() + inner_value.unwrap().to_string().as_str());
            output_element.unwrap().set_inner_html((&inner_html).as_str());
        } else if inner_operation != None {
            let sign;
            match inner_operation.unwrap() {
                Operation::Addition => sign = "+",
                Operation::Subtraction => sign = "-",
                Operation::Multiplication => sign = "*",
                Operation::Division => sign = "/",
                Operation::Equal => sign = "",
                Operation::Clear => sign = "C",
                Operation::AllClear => sign = "AC",
                Operation::Decimal => sign = ".",
                Operation::OpenBracket => sign = "(",
                Operation::CloseBracket => sign = ")",
            }
            let inner_html = &(output_element.clone().unwrap().inner_html().clone() + sign);
            if sign == "" {
                let output;
                log!("innerhtml:", inner_html.as_str());

                //matches any number
                let regex = Regex::new("[-+]?([0-9]*[.])?[0-9]*([eE][-+]?[0-9]+)?").unwrap();
                let numbers = regex.find_iter(inner_html.as_str());

                let mut new_numbers: Vec<String> = vec![];
                for number in numbers {
                    let base = number.as_ref().unwrap().as_str().to_owned().clone();
                    if number.as_ref().unwrap().as_str().ends_with(".") {
                        let both = base + "0";
                        new_numbers.push(both.clone());
                    } else if number.as_ref().unwrap().as_str().contains(".") {
                        new_numbers.push(base.clone());
                    } else {
                        let both = base + ".0";
                        new_numbers.push(both.clone());
                    }
                }

                let numbers = regex.find_iter(inner_html.as_str());
                let mut new_output = inner_html.clone();
                let mut index = 0;
                let mut offset = 0;
                for number in numbers {
                    new_output = (&new_output[0..number.as_ref().unwrap().start() + offset]).to_string() + new_numbers.get(index).unwrap().as_str() + &new_output[number.as_ref().unwrap().end() + offset..new_output.len()];
                    offset += new_numbers.get(index).unwrap().len() - (number.as_ref().unwrap().end() - number.as_ref().unwrap().start());
                    index += 1;
                }

                log!("new output:", &new_output);

                fn test<T: AsRef<str>>(inp: &[T]) {
                    for x in inp { log!(x.as_ref()) }
                }

                // test(&new_numbers);

                //potential regex: https://regex101.com/r/3DrUWA/1
                //let regex = Regex::new("(?<!\.)\b[0-9]+\b(?!\.)").unwrap();
                //TODO: add regex to add .0 after all numbers if a decimal is  not already present rather than just .0 at the last number. to test try 1/2/2
                match eval_float(&new_output) {
                    Ok(v) => output = v.to_string(),
                    Err(e) => output = String::from("Error: ") + e.to_string().as_str(),
                }
                output_element.unwrap().set_inner_html((&output).as_str());
            } else if sign == "AC" {
                output_element.unwrap().set_inner_html("");
            } else if sign == "C" {
                output_element.unwrap().set_inner_html(&(document().get_element_by_id("output").unwrap().inner_html().as_str()[0..document().get_element_by_id("output").unwrap().inner_html().as_str().len() - 1]));
            } else {
                output_element.unwrap().set_inner_html((&inner_html).as_str());
            }
        }
        runFitText();
    });

    
    html! {
            <div class="middle-button" style="height: 100%; padding: 0.5vmin; box-sizing: border-box; border-radius: inherit;">
                <button class="text inner-button" style="border-radius: inherit;" onclick={match props.value {
                    OuterValue::Value(v) => move |_|{ onclick.emit(v);},
                }}>{&props.text}</button>
            </div>
    }
}