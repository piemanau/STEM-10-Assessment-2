use evalexpr::eval;
use gloo::console::log;
use yew::prelude::*;
use yew::html;

use crate::components::button::Button;

mod components;

#[derive(PartialEq, Clone)]
pub enum State {
    NumberOne(CustomNumber),
    Operation(Operation),
    NumberTwo(CustomNumber),
}


#[derive(PartialEq, Clone)]
pub enum CustomNumber {
    Number(f64),
    None,
}


#[derive(PartialEq, Clone, Copy)]
pub enum Operation {
    Multiplication,
    Division,
    Subtraction,
    Addition,
    Equal,
    Clear,
    AllClear,
}

#[derive(PartialEq, Clone, Copy)]
pub enum Value {
    Number(i8),
    Operation(Operation),
}

#[derive(PartialEq, Clone, Copy)]
pub enum OuterValue {
    Value(Value),
}


#[function_component]
fn App() -> Html {
    html! {
        <div class="calculator">
            <div class="outer">
                <p class="output text" id="output">{""}</p>
                <script>{"window.fitText( document.getElementById(\"output\") )"}</script>
                <div class="button zero"><Button value={OuterValue::Value(Value::Number(0))} text="0"/></div>
                <div class="button one"><Button value={OuterValue::Value(Value::Number(1))} text="1"/></div>
                <div class="button two"><Button value={OuterValue::Value(Value::Number(2))} text="2"/></div>
                <div class="button three"><Button value={OuterValue::Value(Value::Number(3))} text="3"/></div>
                <div class="button four"><Button value={OuterValue::Value(Value::Number(4))} text="4"/></div>
                <div class="button five"><Button value={OuterValue::Value(Value::Number(5))} text="5"/></div>
                <div class="button six"><Button value={OuterValue::Value(Value::Number(6))} text="6"/></div>
                <div class="button seven"><Button value={OuterValue::Value(Value::Number(7))} text="7"/></div>
                <div class="button eight"><Button value={OuterValue::Value(Value::Number(8))} text="8"/></div>
                <div class="button nine"><Button value={OuterValue::Value(Value::Number(9))} text="9"/></div>
                <div class="button addition"><Button value={OuterValue::Value(Value::Operation(Operation::Addition))} text="+"/></div>
                <div class="button subtraction"><Button value={OuterValue::Value(Value::Operation(Operation::Subtraction))} text="-"/></div>
                <div class="button division"><Button value={OuterValue::Value(Value::Operation(Operation::Division))} text="/"/></div>
                <div class="button multiplication"><Button value={OuterValue::Value(Value::Operation(Operation::Multiplication))} text="*"/></div>
                <div class="button equals"><Button value={OuterValue::Value(Value::Operation(Operation::Equal))} text="="/></div>
                <div class="button clear"><Button value={OuterValue::Value(Value::Operation(Operation::Clear))} text="C"/></div>
                <div class="button allclear"><Button value={OuterValue::Value(Value::Operation(Operation::AllClear))} text="AC"/></div>
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}