use strum_macros::EnumString;
use yew::prelude::*;

use crate::solutions::divide_two_ints::Solution;

#[derive(PartialEq, Properties, Clone)]
pub struct ProblemTemplateProps {
    pub name: Option<String>,
    pub explanation: Option<String>,
    pub code: Option<String>,
    pub args: Option<Vec<String>>,
    pub examples: Option<Vec<Vec<String>>>,
}

#[function_component(ProblemTemplate)]
pub fn problem_template(props: &ProblemTemplateProps) -> Html {
    let name = &props.name.clone().unwrap_or_default();
    let explanation = &props.explanation.clone().unwrap_or_default();
    let code = &props.code.clone().unwrap_or_default();
    let args = &props.args.clone().unwrap_or_default();
    let examples = &props.examples.clone().unwrap_or_default();
    
    // state is used to store a mutable value
    let input_test_state = use_state_eq::<Option<TestOptions>, _>(|| None);
    let input_test_state_outer = input_test_state.clone();

    let run_input_test = Callback::from(move |_| {
        if Solution::test(5, 5, 1) {
            input_test_state.set(Some(TestOptions::Pass));
            web_sys::console::log_1(&"passed".into());
        } else {
            input_test_state.set(Some(TestOptions::Fail));
            web_sys::console::log_1(&"failed".into());
        }
    });

    let preset_test_state = use_state_eq::<Option<TestOptions>, _>(|| None);
    let preset_test_state_outer = preset_test_state.clone();

    let examples_inner = examples.clone();

    let run_preset_test = Callback::from(move |_| {
        for test in &examples_inner {
            web_sys::console::log_1(&format!("{:?}", test).into());

            if Solution::test(
                test[0].parse().unwrap(),
                test[1].parse().unwrap(),
                test[2].parse().unwrap(),
            ) {
                preset_test_state.set(Some(TestOptions::Pass));
                web_sys::console::log_1(&"passed".into());
            } else {
                preset_test_state.set(Some(TestOptions::Fail));
                web_sys::console::log_1(&"failed".into());
            }
        }
    });

    html! {
        <>
        <div id="header" class="container">
            <h1>{ name }</h1>
        </div>

        <div id="tests" class="container">
            <div id="test_examples" class="container">
                <h2>{"Examples"}</h2>
                    <div id="examples" class="container" >
                        <ListExamples args={args.clone()} examples={examples.clone()}  />
                    </div>

                    <div id="output" >
                    </div>
            </div>

            <div id="test_input" class="container">
                <TestInput args={args.clone()} />
            </div>

        </div>

        <div id="test_results" class="container">
            <button onclick={run_input_test}>{ "Run Input Test" }</button>
            <button onclick={run_preset_test} >{ "Run Preset Tests" }</button>

            <h1><TestResult state={*input_test_state_outer} /></h1>
            <h1><TestResult state={*preset_test_state_outer} /></h1>
        </div>

        <div id="code" class="container">
            <pre>
                <code>{code}</code>
            </pre>
        </div>
        </>
    }
}

#[derive(PartialEq, Properties)]
struct ListExamplesProps {
    args: Vec<String>,
    examples: Vec<Vec<String>>,
}

// Displays all of the preset examples
#[function_component(ListExamples)]
fn list_examples(props: &ListExamplesProps) -> Html {
    let args = props
        .args
        .iter()
        .map(|key| {
            html! {
                <label>{key}</label>
            }
        })
        .collect::<Html>();

    let examples = props
        .examples
        .iter()
        .flatten()
        .map(|key| {
            html! {
                <label>{key}</label>
            }
        })
        .collect::<Html>();

    html! {
        <>
            <div>{args}</div>
            <div>{examples}</div>
        </>
    }
}

#[derive(PartialEq, Properties)]
struct TestInputProps {
    args: Vec<String>,
}

// Displays the input boxes for manual testing
#[function_component(TestInput)]
fn test_input(props: &TestInputProps) -> Html {
    // oninput event for updating inputs

    let inputs = props
        .args
        .iter()
        .map(|arg| {
            html! {
                <>
                <label for={arg.to_string()}>{arg}</label>
                <input type="number" name={arg.to_string()} />
                </>
            }
        })
        .collect::<Html>();

    html! {
        <>{inputs}</>
    }
}

#[derive(PartialEq, Debug, Clone, EnumString, Copy)]
enum TestOptions {
    Pass,
    Fail,
}

#[derive(PartialEq, Properties)]
struct TestResultProps {
    state: Option<TestOptions>,
}

#[function_component(TestResult)]
fn test_result(props: &TestResultProps) -> Html {
    html! {
        <> {
            match props.state {
                Some(x) => format!("{:?}", x),
                None => "".to_string(),
            }
        } </>
    }
}
