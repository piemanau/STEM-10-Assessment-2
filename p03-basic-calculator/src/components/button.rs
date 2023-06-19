use evalexpr::*;
use fancy_regex::Regex;
use gloo::{console::warn, utils::document};
use yew::{function_component, html, Callback, Html, Properties};

use crate::{runFitText, Operation, OuterValue, Value};

// Parameters I can pass in to use for each individual button
#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub value: OuterValue,
    pub text: String,
}

// Making a regex variable this way to save allocations, just good practise. It only allocated on first use.
lazy_static::lazy_static! {
    static ref REGEX: Regex = Regex::new("[-+]?([0-9]*[.])?[0-9]*([eE][-+]?[0-9]+)?").unwrap();
}

#[function_component(Button)]
pub fn button_press(props: &Props) -> Html {
    // When the button is clicked it finds what button was clicked and does what it needs to.
    let onclick = Callback::from(|value: Value| {
        // Gets the output element
        let output_element = document().get_element_by_id("output");

        // Gets some variables ready
        let mut inner_value = None;
        let mut inner_operation = None;

        // Clears the output if the last button press equaled in an error
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

        // Sets the number or operation based on what button was clicked
        match value.clone() {
            Value::Number(n) => inner_value = Some(n),
            Value::Operation(o) => inner_operation = Some(o),
        }

        // If a number was pressed it just adds that to the end
        if inner_value != None {
            let inner_html = &(output_element.clone().unwrap().inner_html().clone()
                + inner_value.unwrap().to_string().as_str());
            output_element
                .unwrap()
                .set_inner_html((&inner_html).as_str());
        // If it was an operation
        } else if inner_operation != None {
            // Gets the button that was pressed, including signs
            let sign;
            match inner_operation.unwrap() {
                Operation::Addition => sign = "+",
                Operation::Subtraction => sign = "-",
                Operation::Multiplication => sign = "*",
                Operation::Division => sign = "/",
                // Equals is set to nothing otherwise it wont calculate
                Operation::Equal => sign = "",
                Operation::Clear => sign = "C",
                Operation::AllClear => sign = "AC",
                Operation::Decimal => sign = ".",
                Operation::OpenBracket => sign = "(",
                Operation::CloseBracket => sign = ")",
            }
            // Adds the sign to the end of the output
            let inner_html = &(output_element.clone().unwrap().inner_html().clone() + sign);

            // If the sign is equals it makes all the numbers floats and then calculates it and sets the output
            if sign == "" {
                let output;

                //Matches any number, example: 0, 1. or 2.1
                let numbers = REGEX.find_iter(inner_html.as_str());

                // Starts to turn all numbers to float style
                let mut new_numbers: Vec<String> = vec![];
                let mut new_output = inner_html.clone();
                // Index and offset to add new numbers to the new string correctly
                let mut index = 0;
                let mut offset = 0;
                // Loops over each number, converts to floating point number and adds to a string
                for number in numbers {
                    let base = number.as_ref().unwrap().as_str().to_owned().clone();
                    // Ends with . for example 1. and adds a zero at the end so it will be 1.0
                    if number.as_ref().unwrap().as_str().ends_with(".") {
                        let both = base + "0";
                        new_numbers.push(both.clone());
                    // If it is already in the correct format it just adds it to the list of numbers
                    } else if number.as_ref().unwrap().as_str().contains(".") {
                        new_numbers.push(base.clone());
                    // If it has no full stop then it adds .0 so 1 would become 1.0
                    } else {
                        let both = base + ".0";
                        new_numbers.push(both.clone());
                    }

                    // For each of the numbers it gets the spot it needs to be at in the new string (copy of old) and replaces each number and updates the index and offset for the next number
                    new_output = (&new_output[0..number.as_ref().unwrap().start() + offset])
                        .to_string()
                        + new_numbers.get(index).unwrap().as_str()
                        + &new_output[number.as_ref().unwrap().end() + offset..new_output.len()];
                    // Calculate the ranges first so the number is always 0 or positive because the type is unsigned meaning thet is the value goes into negative at anypoint the number will overflow into the other side of the min/max
                    offset += new_numbers.get(index).unwrap().len()
                        - (number.as_ref().unwrap().end() - number.as_ref().unwrap().start());
                    index += 1;
                }

                // Calculates the answer, if it cant be calculated then it put the error in the console.
                match eval_float(&new_output) {
                    Ok(v) => output = v.to_string(),
                    Err(e) => {
                        output = {
                            warn!(e.to_string());
                            String::from("Could not calculate. Check console for error.")
                        }
                    }
                }

                // Sets the output to show the user
                output_element.unwrap().set_inner_html((&output).as_str());
            } else if sign == "AC" {
                // Clear the output
                output_element.unwrap().set_inner_html("");
            } else if sign == "C" {
                // Removes the the last digit if there is more than 0 digits
                if document()
                    .get_element_by_id("output")
                    .unwrap()
                    .inner_html()
                    .as_str()
                    .len()
                    > 0
                {
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
                }
            } else {
                // In all other cases default to doing nothing.
                output_element
                    .unwrap()
                    .set_inner_html((&inner_html).as_str());
            }
        }
        // Resize the text to fit the width
        runFitText();
    });

    html! {
        //html to add for each button.
        <div class="middle-button">
            // Pattern match an enum to parse in a custom parameter because the borrow can be a pain sometimes
            <div class="inner-button" style="border-radius: inherit;" onclick={match props.value {OuterValue::Value(v) => move |_|{ onclick.emit(v);},}}>
                <p class="text center" style="user-select: none; margin: 0;">{&props.text}</p>
            </div>
        </div>
    }
}
