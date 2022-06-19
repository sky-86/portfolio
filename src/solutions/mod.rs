pub mod divide_two_ints;
pub mod two_sum;
pub mod max_product_word_len;

use std::fmt;
use strum_macros::EnumString;
use yew::prelude::*;

// all available solutions
#[derive(Clone, Debug, PartialEq, EnumString)]
pub enum Solutions {
    DivideTwoInts,
    TwoSum,
    MaxProductWordLengths,
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
    pub fn run(&self, hooks: &[UseStateHandle<String>]) {
        match self.enum_ref {
            Solutions::DivideTwoInts => divide_two_ints::DivideTwoInts::default().run_tests(hooks),
            Solutions::TwoSum => two_sum::TwoSum::default().run_tests(hooks),
            Solutions::MaxProductWordLengths => max_product_word_len::MaxProductWordLengths::default().run_tests(hooks),
        }
    }
}

/*
vec[5, 5, 1]                 = vec[DivideTwoIntsArgs]
vec[vec[2,3,4], 6, vec[0,2]] = vec[TwoSumArgs]
*/
