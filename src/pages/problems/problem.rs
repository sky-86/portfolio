use yew::prelude::*;

use super::pages::{divide_two_ints::DivideTwoIntsExamples, max_product_word_len::MaxProductWordLengthsExamples};
use super::pages::two_sum::TwoSumExamples;
use super::problem_list::*;
use crate::components::{code::Code, test_table::TestTable};
use super::pages::Example;

//fn get_problem() ->

// defines a single generic solution
#[derive(Clone, Debug, PartialEq, Properties)]
pub struct ProblemProps {
    pub name: String,
    pub explanation: Html,
    pub code: String,
    pub args: Vec<String>,
    pub examples: Vec<Vec<String>>,
    pub id: ProblemEnum,
}

// generic run tests function;
// ****gotta be a better way to do this, trait?
impl ProblemProps {
    pub fn run(&self, hooks: &[UseStateHandle<String>]) {
        match self.id {
            ProblemEnum::DivideTwoInts => {
                DivideTwoIntsExamples::new().run_tests(hooks);
            }
            ProblemEnum::TwoSum => {
                TwoSumExamples::new().run_tests(hooks);
            }
            ProblemEnum::MaxProductWordLengths => {
                MaxProductWordLengthsExamples::new().run_tests(hooks);
            }
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
