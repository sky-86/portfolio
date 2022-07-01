use yew::prelude::*;
use yew_router::prelude::*;
use std::fmt;
use strum_macros::EnumString;

use super::pages::*;
use crate::route::Route;
use super::pages::*;

// all available problems
#[derive(Clone, Debug, PartialEq, EnumString)]
pub enum ProblemEnum {
    DivideTwoInts,
    TwoSum,
    MaxProductWordLengths,
}

// converts enum name to string,
// need for displaying the url
impl fmt::Display for ProblemEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", &self)
    }
}

#[function_component(ProblemList)]
pub fn problem_list() -> Html {
    html! {
        <div>
            <h1>{ "LeetCode Problems" }</h1>
            <ul>
                <li><Link<Route> to={Route::Problem{id: ProblemEnum::DivideTwoInts}}>
                    {"Divide Two Ints"}
                </Link<Route>></li>
                <li><Link<Route> to={Route::Problem{id: ProblemEnum::TwoSum}}>
                   {"Two Sum"} 
                </Link<Route>></li>
                <li><Link<Route> to={Route::Problem{id: ProblemEnum::MaxProductWordLengths}}>
                   {"Max Product of Word Lengths"} 
                </Link<Route>></li>
             </ul>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct ProblemRouteProps {
    pub id: Option<ProblemEnum>,
}

#[function_component(ProblemRoute)]
pub fn problem_route(props: &ProblemRouteProps) -> Html {
    let id = match &props.id {
        Some(id) => id,
        None => return html! {}
    };

    match id {
        ProblemEnum::DivideTwoInts => {
            let temp_solution = divide_two_ints::DivideTwoInts::default();
            let solution = Some(temp_solution.convert());
            html! {<ProblemTemplate {solution}  />}
        },
        ProblemEnum::TwoSum => {
            let temp_solution = two_sum::TwoSum::default();
            let solution = Some(temp_solution.convert());
            html! {<ProblemTemplate {solution}  />}
        }
        ProblemEnum::MaxProductWordLengths => {
            let temp_solution = max_product_word_len::MaxProductWordLengths::default();
            let solution = Some(temp_solution.convert());
            html! {<ProblemTemplate {solution}  />}
        }

    }
}
