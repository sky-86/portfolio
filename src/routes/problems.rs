use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::*;
use crate::routes::problem_template::ProblemTemplate;
use crate::solutions::*;

#[function_component(Problems)]
pub fn problems() -> Html {
    html! {
        <div class="container">
            <h1>{ "Problems" }</h1>
            <ul class="problems">
                <li><Link<Route> to={Route::Problem{id: Solutions::DivideTwoInts }}>
                    {"Divide Two Ints"}
                </Link<Route>></li>

                <li><Link<Route> to={Route::Problem{id: Solutions::TwoSum }}>
                    {"Two Sum"}
                </Link<Route>></li>
             </ul>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct ProblemProps {
    pub id: Option<Solutions>,
}

#[function_component(Problem)]
pub fn problem(props: &ProblemProps) -> Html {
    let id = match &props.id {
        Some(id) => id,
        None => return html! {}
    };

    match id {
        Solutions::DivideTwoInts => {
            let temp_solution = divide_two_ints::DivideTwoInts::default();
            let solution = Some(temp_solution.convert());
            html! {<ProblemTemplate {solution}  />}
        },
        Solutions::TwoSum => {
            let temp_solution = two_sum::TwoSum::default();
            let solution = Some(temp_solution.convert());
            html! {<ProblemTemplate {solution}  />}
        }
    }
}
