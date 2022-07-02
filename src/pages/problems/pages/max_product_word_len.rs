use std::fmt;
use yew::prelude::*;

use super::super::problem::*;
use super::super::problem_list::*;
use super::Example;

#[derive(Clone, Debug, PartialEq)]
pub struct MaxProductWordLengthsExamples {
    examples: Vec<Vec<Args>>,
}

#[derive(Clone, Debug, PartialEq)]
enum Args {
    Words(Vec<String>),
    Answer(i32),
}

impl fmt::Display for Args {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Args::Words(x) => write!(f, "{:?}", x),
            Args::Answer(x) => write!(f, "{:?}", x),
        }
    }
}

impl Example for MaxProductWordLengthsExamples {
    fn new() -> Self {
        MaxProductWordLengthsExamples {
            examples: vec![
                vec![
                    Args::Words(
                        vec!["abcw", "baz", "foo", "bar", "xtfn", "abcdef"]
                            .iter()
                            .map(|s| s.to_string())
                            .collect(),
                    ),
                    Args::Answer(16),
                ],
                vec![
                    Args::Words(
                        vec!["a", "ab", "abc", "d", "cd", "bcd", "abcd"]
                            .iter()
                            .map(|s| s.to_string())
                            .collect(),
                    ),
                    Args::Answer(4),
                ],
                vec![
                    Args::Words(
                        vec!["a", "aa", "aaa", "aaaa"]
                            .iter()
                            .map(|s| s.to_string())
                            .collect(),
                    ),
                    Args::Answer(0),
                ],
                vec![
                    Args::Words(
                        vec!["a", "aa", "aaa", "aaaa"]
                            .iter()
                            .map(|s| s.to_string())
                            .collect(),
                    ),
                    Args::Answer(5),
                ],
                vec![
                    Args::Words(
                        vec![
                            "a", "aa", "aaa", "aaaa", "a", "aa", "aaa", "aaaa", "a", "aa", "aaa",
                            "aaaa", "a", "aa", "aaa", "aaaa", "a", "aa", "aaa", "aaaa", "a", "aa",
                            "aaa", "aaaa", "a", "aa", "aaa", "aaaa", "a", "aa", "aaa", "aaaa", "a",
                            "aa", "aaa", "aaaa", "a", "aa", "aaa", "aaaa", "a", "aa", "aaa",
                            "aaaa", "a", "aa", "aaa", "aaaa", "a", "aa", "aaa", "aaaa", "a", "aa",
                            "aaa", "aaaa", "a", "aa", "aaa", "aaaa", "a", "aa", "aaa", "aaaa", "a",
                            "aa", "aaa", "aaaa", "a", "aa", "aaa", "aaaa", "a", "aa", "aaa",
                            "aaaa", "a", "aa", "aaa", "aaaa", "a", "aa", "aaa", "aaaa", "a", "aa",
                            "aaa", "aaaa", "a", "aa", "aaa", "aaaa", "a", "aa", "aaa", "aaaa", "a",
                            "aa", "aaa", "aaaa", "a", "aa", "aaa", "aaaa", "a", "aa", "aaa",
                            "aaaa",
                        ]
                        .iter()
                        .map(|s| s.to_string())
                        .collect(),
                    ),
                    Args::Answer(0),
                ],
            ],
        }
    }

    fn get_props() -> ProblemProps {
        let explanation = html! {
            <>
                <p>
                    {"Given a string array "}
                    <code>{"words"}</code>
                    {", return the maximum value of "}
                    <code>{"length(word[i]) * length(word[j])"}</code>
                    {" where the two words do not share common letters. If no such two words exist, return 0."}
                </p>
                <a href="https://leetcode.com/problems/maximum-product-of-word-lengths/" target="_blank" rel="noopener noreferrer">{"https://leetcode.com/problems/maximum-product-of-word-lengths/"}</a>
            </>
        };

        let examples = MaxProductWordLengthsExamples::new().get_example_string();

        ProblemProps {
            name: "Two Sum".into(),
            explanation,
            code: include_str!("text/max_product_word_len.code").into(),
            args: vec!["1".into(), "2".into(), "answer".into()],
            id: ProblemEnum::MaxProductWordLengths,
            examples,
        }
    }

    fn get_example_string(&self) -> Vec<Vec<String>> {
        let mut values: Vec<Vec<String>> = Vec::new();
        for test in &self.examples {
            let mut row: Vec<String> = Vec::new();
            for s in test {
                let mut s = s.to_string();
                // if string is to long, shorten it
                if s.len() > 100 {
                    let slice1 = s[0..83].to_string();
                    let slice2 = s[s.len() - 10..].to_string();
                    s = format!("{} ... {}", slice1, slice2);
                }

                row.push(s);
            }
            values.push(row);
        }
        values
    }

    fn run_tests(&self, hooks: &[UseStateHandle<String>]) {
        let examples = &self.examples;

        for (i, test) in examples.iter().enumerate() {
            let mut words = vec![String::new(); 1];
            let mut answer = 0;
            for y in test {
                match y {
                    Args::Words(x) => words = x.clone(),
                    Args::Answer(x) => answer = *x,
                }
            }

            let result = MaxProductWordLengthsExamples::max_product(words);
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
