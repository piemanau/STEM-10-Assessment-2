use evalexpr::*;
use fancy_regex::Regex;
use gloo::{console::warn, utils::document};
use yew::{function_component, html, Callback, Html, Properties};

use crate::{runFitText, Operation, OuterValue, Value};

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub value: OuterValue,
    pub text: String,
}

lazy_static::lazy_static! {
    static ref REGEX: Regex = Regex::new("[-+]?([0-9]*[.])?[0-9]*([eE][-+]?[0-9]+)?").unwrap();
}

#[function_component(Button)]
pub fn button_press(props: &Props) -> Html {
    let onclick = Callback::from(|value: Value| {
        let output_element = document().get_element_by_id("output");

        let mut inner_value = None;
        let mut inner_operation = None;

        if output_element
            .clone()
            .unwrap()
            .inner_html()
            .starts_with("Could not calculate. Check console for error.")
        {
            document()
                .get_element_by_id("output")
                .unwrap()
                .set_inner_html("");
        }

        match value.clone() {
            Value::Number(n) => inner_value = Some(n),
            Value::Operation(o) => inner_operation = Some(o),
        }

        if inner_value != None {
            let inner_html = &(output_element.clone().unwrap().inner_html().clone()
                + inner_value.unwrap().to_string().as_str());
            output_element
                .unwrap()
                .set_inner_html((&inner_html).as_str());
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

                //matches any number
                let numbers = REGEX.find_iter(inner_html.as_str());

                let mut new_numbers: Vec<String> = vec![];
                let mut new_output = inner_html.clone();
                let mut index = 0;
                let mut offset = 0;
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
                    new_output = (&new_output[0..number.as_ref().unwrap().start() + offset])
                        .to_string()
                        + new_numbers.get(index).unwrap().as_str()
                        + &new_output[number.as_ref().unwrap().end() + offset..new_output.len()];
                    //calculate the ranges first so the number is always 0 or positive because the type is unsigned meaning thet is the value goes into negative at anypoint the number will overflow into the other side of the min/max
                    offset += new_numbers.get(index).unwrap().len()
                        - (number.as_ref().unwrap().end() - number.as_ref().unwrap().start());
                    index += 1;
                }

                //TODO: potentialy add an option for a full error or just something like "Could not calculate."
                match eval_float(&new_output) {
                    Ok(v) => output = v.to_string(),
                    Err(e) => {
                        output = {
                            warn!(e.to_string());
                            String::from("Could not calculate. Check console for error.")
                        }
                    }
                }
                output_element.unwrap().set_inner_html((&output).as_str());
            } else if sign == "AC" {
                output_element.unwrap().set_inner_html("");
            } else if sign == "C" {
                output_element.unwrap().set_inner_html(
                    &(document()
                        .get_element_by_id("output")
                        .unwrap()
                        .inner_html()
                        .as_str()[0..document()
                        .get_element_by_id("output")
                        .unwrap()
                        .inner_html()
                        .as_str()
                        .len()
                        - 1]),
                );
            } else {
                output_element
                    .unwrap()
                    .set_inner_html((&inner_html).as_str());
            }
        }
        runFitText();
    });

    html! {
        //html to add for each button.
        <div class="middle-button">
            <div class="inner-button" style="border-radius: inherit;" onclick={match props.value {OuterValue::Value(v) => move |_|{ onclick.emit(v);},}}>
                <p class="text center" style="user-select: none; margin: 0;">{&props.text}</p>
            </div>
        </div>
    }
}
