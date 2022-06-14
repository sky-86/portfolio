mod solution;

use solution::Solution;
use yew::prelude::*;

#[derive(Clone, PartialEq)]
struct Test {
    id: usize,
    dividend: i32,
    divisor: i32,
}

#[derive(Clone, Properties, PartialEq)]
struct TestListProps {
    tests: Vec<Test>,
}

#[function_component(TestList)]
fn test_list(TestListProps { tests }: &TestListProps) -> Html {
    tests.iter().map(|test| html! {
        <li>{format!("{}, {} == {}", test.dividend, test.divisor, Solution::divide(test.dividend, test.divisor))}</li>
    }).collect()
}

fn display_test() -> Html {
    let tests = vec![
        Test {
            id: 1,
            dividend: 15,
            divisor: 5,
        },
        Test {
            id: 2,
            dividend: 20,
            divisor: 10,
        },
        Test {
            id: 3,
            dividend: 5,
            divisor: 1,
        },
    ];

    html! {
        <ol>
            <TestList tests={tests}/>
        </ol>
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
    <>
    <h1>{"Hello World"}</h1>
    <div>
        <button onclick={Callback::from(|_| ())}>{"Click"}</button>
        { display_test()}
    </div>
    </>
    }
}

fn main() {
    yew::start_app::<App>();
}
