pub mod divide_two_ints;
pub mod two_sum;

use strum_macros::EnumString;
use std::fmt;
use yew_hooks::prelude::*;

// all available solutions
#[derive(Clone, Debug, PartialEq, EnumString)]
pub enum Solutions {
    DivideTwoInts,
    TwoSum,
}

impl fmt::Display for Solutions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", &self)
    }
}

// defines a single generic solution
#[derive(Clone, Debug, PartialEq)]
pub struct Solution {
    pub name: String,
    pub explanation: String,
    pub code: String,
    pub args: Vec<String>,
    pub examples: Vec<Vec<String>>,
    pub enum_ref: Solutions,
}

impl Solution {
    pub fn run(&self, hooks: &Vec<UseToggleHandle<bool>>) {
        match self.enum_ref {
            Solutions::DivideTwoInts => divide_two_ints::DivideTwoInts::default().run_tests(hooks),
            Solutions::TwoSum => two_sum::TwoSum::default().run_tests(hooks),
        }
    }
}

/*
vec[5, 5, 1]                 = vec[DivideTwoIntsArgs]
vec[vec[2,3,4], 6, vec[0,2]] = vec[TwoSumArgs]
*/
