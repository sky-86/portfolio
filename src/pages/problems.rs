use yew::prelude::*;
use yew_router::prelude::*;
use crate::routes::*;
use super::divide_two_ints::DivideTwoInts;
use strum_macros::EnumString;
use std::fmt;

#[function_component(Problems)]
pub fn problems() -> Html {
    html! {
        <div class="container">
            <h1>{ "Problems" }</h1>
            <ul class="problems">
                <li><Link<Route>
                    to={Route::Problem{id: AvailableProblems::DivideTwoInts }}>
                        {"Divide Two Ints"}
                </Link<Route>></li>

                <li><Link<Route>
                    to={Route::Problem{id: AvailableProblems::AddTwoInts }}>
                        {"Add Two Ints"}
                </Link<Route>></li>

                <li><Link<Route>
                    to={Route::Problem{id: AvailableProblems::MultiplyTwoInts }}>
                        {"Multiply Two Ints"}
                </Link<Route>></li>
            </ul>
        </div>
    }
}

#[derive(Clone, Debug, PartialEq, EnumString)]
pub enum AvailableProblems {
    DivideTwoInts,
    AddTwoInts,
    MultiplyTwoInts,
}

impl fmt::Display for AvailableProblems {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", &self)
    }
}

#[derive(Properties, PartialEq)]
pub struct ProblemProps {
    pub id: Option<AvailableProblems>,
}

#[function_component(Problem)]
pub fn problem(props: &ProblemProps) -> Html {
    let id = match &props.id {
        Some(id) => id,
        None => return html! {}
    };

    match id {
        AvailableProblems::DivideTwoInts => html! {<DivideTwoInts />},
        AvailableProblems::AddTwoInts => html! {<DivideTwoInts />}, 
        AvailableProblems::MultiplyTwoInts => html! {<DivideTwoInts />}, 
    }
}
