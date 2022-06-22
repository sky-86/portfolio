use yew::prelude::*;

use crate::routes::problem_template::ProblemTemplate;
use crate::solutions::*;
use crate::components::LinkRoute;

#[function_component(Problems)]
pub fn problems() -> Html {
    html! {
        <div class="container">
            <h1>{ "LeetCode Problems" }</h1>
            <ul class="problems">
                <li><LinkRoute to={Solutions::DivideTwoInts} name={"Divide Two Ints"} /></li>
                <li><LinkRoute to={Solutions::TwoSum} name={"Two Sum"} /></li>
                <li><LinkRoute to={Solutions::MaxProductWordLengths} name={"Max Product of Word Lengths"} /></li>
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
        Solutions::MaxProductWordLengths => {
            let temp_solution = max_product_word_len::MaxProductWordLengths::default();
            let solution = Some(temp_solution.convert());
            html! {<ProblemTemplate {solution}  />}
        }

    }
}
