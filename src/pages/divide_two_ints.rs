use crate::routes::*;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(DivideTwoInts)]
pub fn divide_two_ints() -> Html {
    let history = use_history().unwrap();
    let goback = Callback::once(move |_| history.push(Route::Problems));

    html! {
        <div>
            <h1>{ "Divide Two Ints" }</h1>
            <button>{ "Test" }</button>
            <button onclick={goback} >{ "Go Back" }</button>
        </div>
    }
}

pub struct Solution;

impl Solution {
    pub fn test() {}

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
