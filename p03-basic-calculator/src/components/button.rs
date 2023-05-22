use gloo::{utils::document, console::log};
use yew::{Properties, Html, Callback, html, function_component};
use evalexpr::*;

use crate::{Value, Operation, OuterValue};

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

        if output_element.clone().unwrap().inner_html().starts_with("Could not calculate.") {
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
                Operation::Modulo => sign = "%",
                Operation::OpenBracket => sign = "(",
                Operation::CloseBracket => sign = ")",
            }
            let inner_html = &(output_element.clone().unwrap().inner_html().clone() + sign);
            if sign == "" {
                let output;
                log!(inner_html.as_str());
                match eval_float(&(inner_html.clone().to_owned() + ".0")) {
                    Ok(v) => output = v.to_string(),
                    Err(_) => output = String::from("Could not calculate."),
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
    });

    
    html! {
            <div class="middle-button" style="height: 100%; padding: 0.5vmin; box-sizing: border-box; border-radius: inherit;">
                <button class="text inner-button" style="border-radius: inherit;" onclick={match props.value {
                    OuterValue::Value(v) => move |_|{ onclick.emit(v);},
                }}>{&props.text}</button>
            </div>
    }
}