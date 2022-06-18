use yew::prelude::*;
use yew_router::prelude::*;
use strum_macros::EnumString;
use std::fmt;

use crate::routes::*;
use crate::routes::problem_template::ProblemTemplate;
use crate::solutions::divide_two_ints::Solution;

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
             </ul>
        </div>
    }
}

#[derive(Clone, Debug, PartialEq, EnumString)]
pub enum AvailableProblems {
    DivideTwoInts,
    //AddTwoInts,
    //MultiplyTwoInts,
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

    let data = Solution::get_info();

    let name = data.name;
    let explanation = data.explanation;
    let code = data.code;
    let args = data.args;
    let examples = data.examples;

    match id {
        AvailableProblems::DivideTwoInts => html! {
            <ProblemTemplate {name} {explanation} {code} {args} {examples} />
        },
        //AvailableProblems::AddTwoInts => html! {<ProblemTemplate />}, 
        //AvailableProblems::MultiplyTwoInts => html! {<ProblemTemplate />}, 
    }
}
