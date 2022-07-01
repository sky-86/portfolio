use yew::prelude::*;

use super::pages::*;
use super::problem_list::*;
use crate::components::{code::Code, test_table::TestTable};

#[derive(Properties, PartialEq, Clone)]
pub struct ProblemRouteProps {
    pub id: Option<ProblemEnum>,
}

// renders the correct project page based on the id
#[function_component(ProblemRoute)]
pub fn project_route(props: &ProblemRouteProps) -> Html {
    let id = match &props.id {
        Some(id) => id,
        None => return html! {}
    };
    let props;

    match id {
        ProblemEnum::DivideTwoInts => {
            let explanation = html! {
                <>
                    <p>
                        {"Given two integers "}
                        <code>{"dividend"}</code>
                        {" and "}
                        <code>{"divisor"}</code>
                        {", divide two integers without using multiplication, division, and mod operator."}
                    </p>
                    <a href="https://leetcode.com/problems/divide-two-integers/" target="_blank" rel="noopener noreferrer">{"https://leetcode.com/problems/divide-two-integers/"}</a>
                </>
            };

            props = ProblemProps {
                name: "Divide Two Ints".into(),
                explanation,
                code: include_str!("pages/text/divide_two_ints.code").into(),
                args: vec!["arg1".into(), "arg2".into(), "answer".into()],
                examples: vec![vec!["1".into(),"2".into(),"3".into()], 
                    vec!["1".into(),"2".into(),"3".into()], 
                    vec!["1".into(),"2".into(),"3".into()]],
                id: ProblemEnum::DivideTwoInts,
            };
        },
        ProblemEnum::TwoSum => {
            props = ProblemProps {
                name: "Two Sum".into(),
                explanation: html! {},
                code: include_str!("pages/text/two_sum.code").into(),
                args: vec!["arg1".into(), "arg2".into(), "answer".into()],
                examples: vec![vec!["1".into(),"2".into(),"3".into()], 
                    vec!["1".into(),"2".into(),"3".into()], 
                    vec!["1".into(),"2".into(),"3".into()]],
                id: ProblemEnum::DivideTwoInts,
            };
        }
        ProblemEnum::MaxProductWordLengths => {
            props = ProblemProps {
                name: "Max Product".into(),
                explanation: html! {},
                code: include_str!("pages/text/max_product_word_len.code").into(),
                args: vec!["arg1".into(), "arg2".into(), "answer".into()],
                examples: vec![vec!["1".into(),"2".into(),"3".into()], 
                    vec!["1".into(),"2".into(),"3".into()], 
                    vec!["1".into(),"2".into(),"3".into()]],
                id: ProblemEnum::DivideTwoInts,
            };
        }
    }

    html! {<Problem ..props />}
}

// defines a single generic solution
#[derive(Clone, Debug, PartialEq, Properties)]
pub struct ProblemProps {
    pub name: String,
    pub explanation: Html,
    pub code: String,
    pub args: Vec<String>,
    pub examples: Vec<Vec<String>>,
    pub id: ProblemEnum,
    pub problem_ref: 
}

// generic run tests function;
// ****gotta be a better way to do this, trait?
impl ProblemProps {
    pub fn run(&self, hooks: &[UseStateHandle<String>]) {
        match self.id {
            ProblemEnum::DivideTwoInts => divide_two_ints::DivideTwoInts::default().run_tests(hooks),
            ProblemEnum::TwoSum => two_sum::TwoSum::default().run_tests(hooks),
            ProblemEnum::MaxProductWordLengths => max_product_word_len::MaxProductWordLengths::default().run_tests(hooks),
        }
    }
}

#[function_component(Problem)]
pub fn problem(props: &ProblemProps) -> Html {
    // unwrap Properties
    let solution = props.clone();

    // a vec of hooks; used for the test results; pass or fail
    let mut hooks: Vec<UseStateHandle<String>> = Vec::new();
    // fill vec with hooks
    for _i in 0..solution.examples.len() {
        let temp_hook = use_state_eq(|| "empty".to_string());
        hooks.push(temp_hook);
    }

    // gets passed into the list examples component
    let hooks_outer = hooks.clone();
    let hooks_inner = hooks.clone();

    let solution_inner = solution.clone();
    // onclick callback; runs the tests
    let run_preset_test = Callback::from(move |_| {
        solution_inner.run(&hooks_inner);
    });

    html! {
        <>
        <div>
            <h1>{ solution.name }</h1>
        </div>

        <div>
            {solution.explanation}
        </div>

        <div>
            <div>
                <h2>{"Examples"}</h2>
                    <div>
                        <table>
                            <TestTable args={solution.args.clone()} 
                                examples={solution.examples.clone()} 
                                hooks={hooks_outer}  />
                        </table>
                    </div>
            </div>
        </div>

        <div>
            <button onclick={run_preset_test} >{ "Run Tests" }</button>
        </div>
        <Code code={solution.code} />
        </>
    }
}
