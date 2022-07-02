use yew::prelude::*;

use super::super::problem::*;
use super::super::problem_list::*;
use super::Example;

#[derive(Clone, Debug, PartialEq)]
pub struct DivideTwoIntsExamples {
    examples: Vec<Vec<i32>>,
}

impl Example for DivideTwoIntsExamples {
    fn new() -> Self {
        DivideTwoIntsExamples {
            examples: vec![
                vec![5, 5, 1],
                vec![25, -5, -5],
                vec![5, 5, 0],
                vec![872703948, 379, 2302648],
            ],
        }
    }

    fn get_props() -> ProblemProps {
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

        let examples = DivideTwoIntsExamples::new().get_example_string();

        ProblemProps {
            name: "Divide Two Ints".into(),
            explanation,
            code: include_str!("text/divide_two_ints.code").into(),
            args: vec!["arg1".into(), "arg2".into(), "answer".into()],
            id: ProblemEnum::DivideTwoInts,
            examples,
        }
    }

    fn get_example_string(&self) -> Vec<Vec<String>> {
        let mut values: Vec<Vec<String>> = Vec::new();
        for test in &self.examples {
            let mut row: Vec<String> = Vec::new();
            for x in test {
                row.push(x.to_string());
            }
            values.push(row);
        }
        values
    }

    fn run_tests(&self, hooks: &[UseStateHandle<String>]) {
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
}
