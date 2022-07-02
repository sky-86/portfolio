use yew::prelude::*;
use yew_router::prelude::*;
use std::fmt;
use strum_macros::EnumString;

use crate::route::Route;

// can i use a macro instead of include_str!

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
