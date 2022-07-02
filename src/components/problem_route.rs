use yew::prelude::*;

use crate::pages::problems::pages::*;
use crate::pages::problems::problem_list::*;
use crate::pages::problems::problem::*;

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

    match id {
        ProblemEnum::DivideTwoInts => {
            props = divide_two_ints::DivideTwoIntsExamples::defualt();
            html! {<Problem ..props />}
        }
        ProblemEnum::TwoSum => {
            html! {}
            //                name: "Two Sum".into(),
            //                explanation: html! {},
            //                code: include_str!("pages/text/two_sum.code").into(),
            //                args: vec!["arg1".into(), "arg2".into(), "answer".into()],
            //                examples: vec![vec!["1".into(),"2".into(),"3".into()],
            //                    vec!["1".into(),"2".into(),"3".into()],
            //                    vec!["1".into(),"2".into(),"3".into()]],
            //                id: ProblemEnum::DivideTwoInts,
            //            };
        }
        ProblemEnum::MaxProductWordLengths => {
            html! {}
            //            props = ProblemProps {
            //                name: "Max Product".into(),
            //                explanation: html! {},
            //                code: include_str!("pages/text/max_product_word_len.code").into(),
            //                args: vec!["arg1".into(), "arg2".into(), "answer".into()],
            //                examples: vec![vec!["1".into(),"2".into(),"3".into()],
            //                    vec!["1".into(),"2".into(),"3".into()],
            //                    vec!["1".into(),"2".into(),"3".into()]],
            //                id: ProblemEnum::DivideTwoInts,
            //            };
        }
    }
}
