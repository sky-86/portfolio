use yew::prelude::*;
use std::fmt;

use super::{Solution, Solutions};

#[derive(Clone, Debug, PartialEq)]
pub struct MaxProductWordLengths {
    name: String,
    explanation: Html,
    code: String,
    args: Vec<String>,
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

impl MaxProductWordLengths {
    pub fn default() -> MaxProductWordLengths {
        let code = include_str!("text/max_product_word_len.code").to_string();

        let explanation = html! {
            <>
                <p id="explanation">
                    {"Given a string array "}
                    <code>{"words"}</code>
                    {", return the maximum value of "}
                    <code>{"length(word[i]) * length(word[j])"}</code>
                    {" where the two words do not share common letters. If no such two words exist, return 0."}
                </p>
                <a href="https://leetcode.com/problems/maximum-product-of-word-lengths/" target="_blank" rel="noopener noreferrer">{"https://leetcode.com/problems/maximum-product-of-word-lengths/"}</a>
            </>
        };
        
        let examples = vec![
            vec![
                Args::Words(vec!["abcw","baz","foo","bar","xtfn","abcdef"].iter().map(|s| s.to_string()).collect()),
                Args::Answer(16),
            ],
            vec![
                Args::Words(vec!["a","ab","abc","d","cd","bcd","abcd"].iter().map(|s| s.to_string()).collect()),
                Args::Answer(4),
            ],
            vec![
                Args::Words(vec!["a","aa","aaa","aaaa"].iter().map(|s| s.to_string()).collect()),
                Args::Answer(0),
            ],
            vec![
                Args::Words(vec!["a","aa","aaa","aaaa"].iter().map(|s| s.to_string()).collect()),
                Args::Answer(5),
            ],
            vec![
                Args::Words(vec!["a","aa","aaa","aaaa","a","aa","aaa","aaaa","a","aa","aaa","aaaa","a","aa","aaa","aaaa","a","aa","aaa","aaaa","a","aa","aaa","aaaa","a","aa","aaa","aaaa","a","aa","aaa","aaaa","a","aa","aaa","aaaa","a","aa","aaa","aaaa","a","aa","aaa","aaaa","a","aa","aaa","aaaa","a","aa","aaa","aaaa","a","aa","aaa","aaaa","a","aa","aaa","aaaa","a","aa","aaa","aaaa","a","aa","aaa","aaaa","a","aa","aaa","aaaa","a","aa","aaa","aaaa","a","aa","aaa","aaaa","a","aa","aaa","aaaa","a","aa","aaa","aaaa","a","aa","aaa","aaaa","a","aa","aaa","aaaa","a","aa","aaa","aaaa","a","aa","aaa","aaaa","a","aa","aaa","aaaa",].iter().map(|s| s.to_string()).collect()),
                Args::Answer(0),
            ],
        ];

        MaxProductWordLengths {
            name: "Max Product of Word Lengths".into(),
            explanation,
            code,
            args: vec!["words".into(), "answer".into()],
            examples,
        }
    }

    pub fn convert(&self) -> Solution {
        let mut examples: Vec<Vec<String>> = Vec::new();
        for test in self.examples.clone() {
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
            examples.push(row);
        }
        Solution {
            name: self.name.clone(),
            explanation: self.explanation.clone(),
            code: self.code.clone(),
            args: self.args.clone(),
            examples,
            enum_ref: Solutions::MaxProductWordLengths,
        }
    }

    pub fn run_tests(&self, hooks: &[UseStateHandle<String>]) {
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

            let result = MaxProductWordLengths::max_product(words);
            web_sys::console::log_1(&format!("{:?}", result).into());
            if result == answer {
                // update boolean hook; passed
                hooks[i].set("Pass".to_string());
            } else {
                hooks[i].set("Fail".to_string());
            }
        }
    }


    pub fn max_product(words: Vec<String>) -> i32 {
        let no_common_letters = |s1: &str, s2: &str| -> bool {
            let mut bitmask1 = 0;
            let mut bitmask2 = 0;
            for c in s1.chars() {
                bitmask1 |= 1 << (c as i32 - 'a' as i32);
            }
            for c in s2.chars() {
                bitmask2 |= 1 << (c as i32 - 'a' as i32);
            }

            (bitmask1 & bitmask2) == 0
        };

        let mut largest = 0;

        for (ie, i) in words.iter().enumerate() {
            for (je, j) in words.iter().enumerate() {
                if ie != je && no_common_letters(i, j) {
                    largest = std::cmp::max(largest, i.len() * j.len());
                }
            }
        }

        largest as i32
    }
}
