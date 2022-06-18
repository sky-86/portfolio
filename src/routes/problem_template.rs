use strum_macros::EnumString;
use yew::prelude::*;
use yew_hooks::prelude::*;

use crate::solutions::divide_two_ints::Solution;

#[derive(PartialEq, Properties, Clone)]
pub struct ProblemTemplateProps {
    pub name: Option<String>,
    pub explanation: Option<String>,
    pub code: Option<String>,
    pub args: Option<Vec<String>>,
    pub examples: Option<Vec<Vec<i32>>>,
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

    let examples_inner = examples.clone();

    let examples_len = examples.len();
    let mut hooks: Vec<UseToggleHandle<bool>> = Vec::new();

    for _i in 0..examples_len {
        let temp_hook = use_bool_toggle(false);
        hooks.push(temp_hook);
    }
    let hooks_outer = hooks.clone();

    let run_preset_test = Callback::from(move |_| {
        for (i, test) in examples_inner.iter().enumerate() {
            web_sys::console::log_1(&format!("{:?}", test).into());

            if Solution::test(test[0], test[1], test[2]) {
                hooks[i].toggle();
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
                        <table>
                            <ListExamples args={args.clone()} 
                        examples={examples.clone()} 
                        hooks={hooks_outer}  />
                        </table>
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
    examples: Vec<Vec<i32>>,
    hooks: Vec<UseToggleHandle<bool>>,
}

// Displays all of the preset examples
#[function_component(ListExamples)]
fn list_examples(props: &ListExamplesProps) -> Html {
    let mut arg_list = props.args.clone();
    arg_list.push("Output".to_string());
    let args = arg_list
        .iter()
        .map(|arg| {
            html! {
                <th>{arg}</th>
            }
        })
        .collect::<Html>();

    let hooks = &props.hooks;
    web_sys::console::log_1(&format!("{:?}", hooks).into());

    let examples = props
        .examples
        .iter()
        .enumerate()
        .map(|(i, vec)| {
            html! {
                <tr>{create_row(vec, hooks[i].clone())}</tr>
            }
        })
        .collect::<Html>();

    html! {
        <>
            <tr>{args}</tr>
            <>{examples}</>
        </>
    }
}

fn create_row(data: &[i32], toggle: UseToggleHandle<bool>) -> Html {
    data.iter()
        .enumerate()
        .map(|(i, val)| {
            html! {
                <>
                <td>{val}</td>
                if i == data.len() - 1 {
                    <td class="test_result">{*toggle}</td>
                }
                </>
            }
        })
        .collect::<Html>()
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
