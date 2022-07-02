use yew::prelude::*;

use super::super::problem::*;
use super::super::problem_list::*;


#[derive(Clone, Debug, PartialEq)]
pub struct DivideTwoIntsExamples {
    examples: Vec<Vec<i32>>,
}

// need to use traits
impl DivideTwoIntsExamples {
    pub fn defualt() -> ProblemProps {
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

        // this is so wrong fix this !!
        let example_ref = DivideTwoIntsExamples::get_examples();
        let example_string = DivideTwoIntsExamples::get_examples_string(&example_ref);

        ProblemProps {
            name: "Divide Two Ints".into(),
            explanation,
            code: include_str!("text/divide_two_ints.code").into(),
            args: vec!["arg1".into(), "arg2".into(), "answer".into()],
            id: ProblemEnum::DivideTwoInts,
            examples: example_string,
        }
    }

    pub fn get_examples() -> DivideTwoIntsExamples {
        DivideTwoIntsExamples {
            examples: vec![
                vec![5, 5, 1],
                vec![25, -5, -5],
                vec![5, 5, 0],
                vec![872703948, 379, 2302648],
            ],
        }
    }

    pub fn get_examples_string(examples: &DivideTwoIntsExamples) -> Vec<Vec<String>> {
        let mut values: Vec<Vec<String>> = Vec::new();
        for test in &examples.examples {
            let mut row: Vec<String> = Vec::new();
            for x in test {
                row.push(x.to_string());
            }
            values.push(row);
        }
        values
    }

    pub fn run_tests(&self, hooks: &[UseStateHandle<String>]) {
        let examples = &self.examples;

        for (i, test) in examples.iter().enumerate() {
            let result = DivideTwoIntsExamples::divide(test[0], test[1]);
            web_sys::console::log_1(&format!("{:?}", result).into());
            if result == test[2] {
                // update boolean hook; passed
                hooks[i].set("Pass".to_string());
            } else {
                hooks[i].set("Fail".to_string());
            }
        }
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
