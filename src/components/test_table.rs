use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct TestTableProps {
    pub args: Vec<String>,
    pub examples: Vec<Vec<String>>,
    pub hooks: Vec<UseStateHandle<String>>,
}

// Displays all of the preset examples
#[function_component(TestTable)]
pub fn test_table(props: &TestTableProps) -> Html {
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
fn create_row(data: &[String], toggle: UseStateHandle<String>) -> Html {
    data.iter()
        .enumerate()
        .map(|(i, val)| {
            html! {
                <>
                <td>{val}</td>
                if i == data.len() - 1 {
                    if toggle.to_string() == *"Pass".to_string() {
                        <td style={"color: green; border-color: black;"}>{toggle.to_string()}</td>
                    } else if toggle.to_string() == *"Fail".to_string() {
                        <td style={"color: red; border-color: black;"}>{toggle.to_string()}</td>
                    } else {
                        <td>{""}</td>
                    }
                }
                </>
            }
        })
        .collect::<Html>()
}
