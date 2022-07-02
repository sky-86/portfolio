use yew::prelude::*;

use crate::pages::problems::pages::*;
use crate::pages::problems::problem_list::*;
use crate::pages::problems::problem::*;
use crate::pages::problems::pages::Example;

#[derive(Properties, PartialEq, Clone)]
pub struct ProblemRouteProps {
    pub id: Option<ProblemEnum>,
}

// renders the correct project page based on the id
// build the data and display page
#[function_component(ProblemRoute)]
pub fn problem_route(props: &ProblemRouteProps) -> Html {
    let id = match &props.id {
        Some(id) => id,
        None => return html! {},
    };
    let props;

    // is their a better way to do this??
    // like id::get_props()
    match id {
        ProblemEnum::DivideTwoInts => {
            props = divide_two_ints::DivideTwoIntsExamples::get_props();
            html! {<Problem ..props />}
        }
        ProblemEnum::TwoSum => {
            props = two_sum::TwoSumExamples::get_props();
            html! {<Problem ..props />}
        }
        ProblemEnum::MaxProductWordLengths => {
            props = max_product_word_len::MaxProductWordLengthsExamples::get_props();
            html! {<Problem ..props />}
        }
    }
}
