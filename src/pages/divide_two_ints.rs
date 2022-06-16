use crate::routes::*;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;
use strum_macros::EnumString;

#[derive(PartialEq, Debug, Clone, EnumString, Copy)]
enum TestState {
    Pass,
    Fail,
}

#[derive(PartialEq, Properties)]
struct ViewTestProps {
    state: Option<TestState>,
}

#[function_component(ViewTest)]
fn view_test(props: &ViewTestProps) -> Html {
    html! {
        <> {
            match props.state {
                Some(x) => format!("{:?}", x),
                None => "".to_string(),
            }
        } </>
    }
}

#[function_component(DivideTwoInts)]
pub fn divide_two_ints() -> Html {
    //used for navigation
    //let history = use_history().unwrap();
    //let goback = Callback::once(move |_| history.push(Route::Problems));

    // set refs for each input box
    // refs store a value from an element
    let dividend_ref = NodeRef::default();
    let dividend_ref_outer = dividend_ref.clone();
    let divisor_ref = NodeRef::default();
    let divisor_ref_outer = divisor_ref.clone();
    let answer_ref = NodeRef::default();
    let answer_ref_outer = answer_ref.clone();

    // state is used to store a mutable value
    let test_state = use_state_eq::<Option<TestState>, _>(|| None);
    let test_state_outer = test_state.clone();
    
    // run test
    let onclick = Callback::from(move |_| {
        let dividend = dividend_ref.cast::<HtmlInputElement>().unwrap().value();
        let divisor = divisor_ref.cast::<HtmlInputElement>().unwrap().value();
        let answer = answer_ref.cast::<HtmlInputElement>().unwrap().value();

        if Solution::test(dividend.parse::<i32>().unwrap(), 
            divisor.parse::<i32>().unwrap(), answer.parse::<i32>().unwrap()) {
            test_state.set(Some(TestState::Pass));
            web_sys::console::log_1(&"passed".into());
        } else {
            test_state.set(Some(TestState::Fail));
            web_sys::console::log_1(&"failed".into());
        }
    });

    // import raw text file
    let contents = include_str!("../divide_text.in");

    html! {
        <>
        <div class="container">
            <h1>{ "Divide Two Ints" }</h1>
</div>
        <div class="container">
            <label for="dividend">{"Dividend:"}</label>
            <input type="number" name="dividend" ref={dividend_ref_outer}/>
            <label for="divisor">{"Divisor:"}</label>
            <input type="number" name="divisor" ref={divisor_ref_outer}/>
            <label for="answer">{"Answer:"}</label>
            <input type="number" name="answer" ref={answer_ref_outer}/>
            <button {onclick}>{ "Run Test" }</button>
            <h1><ViewTest state={*test_state_outer} /></h1>
            <pre>
                <code>{contents}</code>
            </pre>
        </div>
        </>
    }
}

pub struct Solution;

impl Solution {
    pub fn test(dividend: i32, divisor: i32, answer: i32) -> bool {
        let temp = Solution::divide(dividend, divisor);
        if temp == answer {
            return true
        }
        false
    }

    pub fn divide(mut dividend: i32, mut divisor: i32) -> i32 {
        // handles an edge case
        if dividend == -2147483648 && divisor == -1 {
            return 2147483647;
        }

        // if negative and positive answer will be negative
        let mut positives = 0;
        if dividend.is_positive() {
            dividend = -dividend;
            positives += 1;
        }
        if divisor.is_positive() {
            divisor = -divisor;
            positives += 1;
        }

        let mut quotient: i32 = 0;
        let mut doubled: i32 = divisor;
        if -2147483648 - divisor < divisor {
            while doubled + doubled > dividend {
                if quotient == 0 {
                    quotient = 2;
                } else {
                    quotient += quotient;
                }

                doubled += doubled;
                if -2147483648 - doubled > doubled {
                    break;
                }
            }
            if doubled != divisor {
                dividend -= doubled;
            }
        }

        // subtract the divisor the dividend until it reaches the floor
        while dividend - divisor < 0 {
            dividend -= divisor;
            quotient += 1;
        }

        if dividend == divisor {
            quotient += 1;
        }

        if positives == 1 {
            quotient = -quotient;
        }
        quotient
    }
}
