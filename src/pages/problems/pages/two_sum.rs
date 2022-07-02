use std::fmt;
use yew::prelude::*;

use crate::pages::problems::problem_list::ProblemEnum;
use super::super::problem::ProblemProps;
use super::Example;

pub struct TwoSumExamples {
    examples: Vec<Vec<TwoSumArgs>>,
}

#[derive(Clone, Debug, PartialEq)]
enum TwoSumArgs {
    Numbers(Vec<i32>),
    Target(i32),
    Answer(Vec<i32>),
}

impl fmt::Display for TwoSumArgs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            TwoSumArgs::Numbers(x) => write!(f, "{:?}", x),
            TwoSumArgs::Target(x) => write!(f, "{:?}", x),
            TwoSumArgs::Answer(x) => write!(f, "{:?}", x),
        }
    }
}


impl Example for TwoSumExamples {
    fn new() -> Self {
        TwoSumExamples {
            examples: vec![
                vec![
                    TwoSumArgs::Numbers(vec![2, 7, 11, 15]),
                    TwoSumArgs::Target(9),
                    TwoSumArgs::Answer(vec![0, 1]),
                ],
                vec![
                    TwoSumArgs::Numbers(vec![3, 2, 4]),
                    TwoSumArgs::Target(6),
                    TwoSumArgs::Answer(vec![1, 2]),
                ],
                vec![
                    TwoSumArgs::Numbers(vec![3, 2, 4]),
                    TwoSumArgs::Target(6),
                    TwoSumArgs::Answer(vec![0, 2]),
                ],

            ]
        }
    }

    fn get_props() -> ProblemProps {
        let explanation = html! {
            <>
                <p>
                    {"Given an array of integers "}
                    <code>{"nums"}</code>
                    {" and an integer "}
                    <code>{"target"}</code>
                    {", return indices of the two numbers such that they add up to "}
                    <code>{"target"}</code>
                    {"."}
                </p>
                <a href="https://leetcode.com/problems/two-sum/" target="_blank" rel="noopener noreferrer">{"https://leetcode.com/problems/two-sum/"}</a>
            </>
        };

        let examples = TwoSumExamples::new().get_example_string();

        ProblemProps {
            name: "Two Sum".into(),
            explanation,
            code: include_str!("text/two_sum.code").into(),
            args: vec!["1".into(),"1".into(),"a".into()],
            id: ProblemEnum::TwoSum,
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
            let mut num = vec![0; 1];
            let mut target = 0;
            let mut answer = vec![0; 1];
            for y in test {
                match y {
                    TwoSumArgs::Numbers(x) => num = x.clone(),
                    TwoSumArgs::Target(x) => target = *x,
                    TwoSumArgs::Answer(x) => answer = x.clone(),
                }
            }

            let result = TwoSumExamples::two_sum(num, target);
            web_sys::console::log_1(&format!("{:?}", result).into());
            if result == answer {
                // update boolean hook; passed
                hooks[i].set("Pass".to_string());
            } else {
                hooks[i].set("Fail".to_string());
            }
        }
    }
}
