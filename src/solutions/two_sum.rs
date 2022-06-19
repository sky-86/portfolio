use std::fmt;
use yew_hooks::prelude::*;

use super::{Solution, Solutions};

pub struct TwoSum {
    pub name: String,
    pub explanation: String,
    pub code: String,
    pub args: Vec<String>,
    pub examples: Vec<Vec<TwoSumArgs>>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TwoSumArgs {
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

impl TwoSum {
    pub fn default() -> TwoSum {
        let code = include_str!("two_sum.code").to_string();
        let examples = vec![
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
        ];

        TwoSum {
            name: "Two Sum".into(),
            explanation: "this is a explanation".into(),
            code,
            args: vec!["numbers".into(), "target".into(), "answer".into()],
            examples,
        }
    }

    pub fn convert(&self) -> Solution {
        let mut examples: Vec<Vec<String>> = Vec::new();
        for test in self.examples.clone() {
            let mut row: Vec<String> = Vec::new();
            for x in test {
                row.push(x.to_string());
            }
            examples.push(row);
        }

        Solution {
            name: self.name.clone(),
            explanation: self.explanation.clone(),
            code: self.code.clone(),
            args: self.args.clone(),
            examples,
            enum_ref: Solutions::TwoSum,
        }
    }

    pub fn run_tests(&self, hooks: &[UseToggleHandle<bool>]) {
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

            let result = TwoSum::two_sum(num, target);
            web_sys::console::log_1(&format!("{:?}", result).into());
            if result == answer {
                // update boolean hook; passed
                hooks[i].toggle();
            }
        }
    }

    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for (i, x) in nums.iter().enumerate() {
            for (j, y) in nums.iter().enumerate() {
                if i != j && x + y == target {
                    return vec![i as i32, j as i32];
                }
            }
        }
        return vec![-1, -1];
    }
}
