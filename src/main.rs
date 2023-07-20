use yew_router::prelude::*;
use yew::prelude::*;

use crate::projects::basic_squaring::BasicSquaring;
use crate::projects::temperature_converter::TemperatureConverter;
use crate::projects::basic_calculator::BasicCalculator;
use crate::projects::base_converter::{BaseConverter, BaseConverterInstructions};
use crate::projects::time_converter::TimeConverter;
mod projects;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/10/stem/at2/")]
    Home,
    #[at("/10/stem/at2/basic-squaring")]
    BasicSquaring,
    #[at("/10/stem/at2/temperature-converter")]
    TemperatureConverter,
    #[at("/10/stem/at2/basic-calculator")]
    BasicCalculator,
    #[at("/10/stem/at2/base-converter")]
    BaseConverter,
    #[at("/10/stem/at2/base-converter/instructions")]
    BaseConverterInstructions,
    #[at("/10/stem/at2/time-converter")]
    TimeConverter,
    #[not_found]
    #[at("/10/stem/at2/404")]
    NotFound,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { 
            <div class="center">
                <h1 style="width: 100%" class="text">{ "Which project would you like to see?" }</h1>
                <Link<Route> to={Route::BasicSquaring}><button class="home-button">{ "Basic Squaring" }</button></Link<Route>>
                <Link<Route> to={Route::TemperatureConverter}><button class="home-button">{ "Temperature Converter" }</button></Link<Route>>
                <Link<Route> to={Route::BasicCalculator}><button class="home-button">{ "Basic Calculator" }</button></Link<Route>>
                <Link<Route> to={Route::BaseConverter}><button class="home-button">{ "Base Converter" }</button></Link<Route>>
                <Link<Route> to={Route::TimeConverter}><button class="home-button">{ "Time Converter" }</button></Link<Route>>
            </div>
        },
        Route::BasicSquaring => html! { <BasicSquaring /> },
        Route::TemperatureConverter => html! { <TemperatureConverter /> },
        Route::BasicCalculator => html! { <BasicCalculator /> },
        Route::BaseConverter => html! { <BaseConverter /> },
        Route::BaseConverterInstructions => html! { <BaseConverterInstructions /> },
        Route::TimeConverter => html! { <TimeConverter /> },
        Route::NotFound => html! {
            <div class="center" style="width: 100px;">
                <p class="text center-horizontally"><br />{ "404" }</p>
                <Link<Route> to={Route::Home}><button class="single-home-button">{ "Go back home" }</button></Link<Route>>
            </div>
        },
    }
}

#[function_component(Main)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} /> // <- must be child of <BrowserRouter>
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<Main>::new().render();
}
