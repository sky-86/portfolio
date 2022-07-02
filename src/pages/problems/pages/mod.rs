// Problem Pages
pub mod functions;
pub mod divide_two_ints;
pub mod two_sum;
pub mod max_product_word_len;

use yew::prelude::*;
use super::problem::ProblemProps;

pub trait Example {
    fn new() -> Self;
    fn get_props() -> ProblemProps;
    fn get_example_string(&self) -> Vec<Vec<String>>;
    fn run_tests(&self, hooks: &[UseStateHandle<String>]);
}
