use yew::prelude::*;
use yew_hooks::prelude::*;

use crate::solutions::Solution;

#[derive(PartialEq, Properties, Clone)]
pub struct ProblemTemplateProps {
    pub solution: Option<Solution>,
}

#[function_component(ProblemTemplate)]
pub fn problem_template(props: &ProblemTemplateProps) -> Html {
    // unwrap Properties
    let solution = props.solution.clone().unwrap();

    // a vec of hooks; used for the test results; pass or fail
    let mut hooks: Vec<UseToggleHandle<bool>> = Vec::new();
    // fill vec with hooks
    for _i in 0..solution.examples.len() {
        let temp_hook = use_bool_toggle(false);
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
        <div id="header" class="container">
            <h1>{ solution.name }</h1>
        </div>

        <div id="tests" class="container">
            <div id="test_examples" class="container">
                <h2>{"Examples"}</h2>
                    <div id="examples" class="container" >
                        <table>
                            <ListExamples args={solution.args.clone()} 
                        examples={solution.examples.clone()} 
                        hooks={hooks_outer}  />
                        </table>
                    </div>
            </div>
        </div>

        <div id="test_results" class="container">
            <button onclick={run_preset_test} >{ "Run Preset Tests" }</button>
        </div>

        <div id="code" class="container">
            <pre>
                <code>{solution.code}</code>
            </pre>
        </div>
        </>
    }
}

#[derive(PartialEq, Properties)]
struct ListExamplesProps {
    args: Vec<String>,
    examples: Vec<Vec<String>>,
    hooks: Vec<UseToggleHandle<bool>>,
}

// Displays all of the preset examples
#[function_component(ListExamples)]
fn list_examples(props: &ListExamplesProps) -> Html {
    // creates the headers for the table of tests
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

    // creates the table rows, adds the test results to the end
    let examples = props
        .examples
        .iter()
        .enumerate()
        .map(|(i, vec)| {
            html! {
                <tr>{create_row(vec, props.hooks[i].clone())}</tr>
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

// helper function for creating the table
fn create_row(data: &[String], toggle: UseToggleHandle<bool>) -> Html {
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
