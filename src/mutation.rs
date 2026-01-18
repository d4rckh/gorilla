use std::{
    cmp::Ordering,
    fmt::{self, Display},
};

use crate::pattern::{Token, token_iterator, tokenize_format_string};

#[derive(Debug, Clone)]
pub enum Action {
    Prepend(String, Vec<Token>), // (original_string, pre_tokenized)
    Append(String, Vec<Token>),  // (original_string, pre_tokenized)
    Replace(String, String),

    FirstLetter,
    LowercaseAll,
    UppercaseAll,
    RemoveFirstLetter,
    RemoveLastLetter,
    Reverse,
    Capitalize,
    ToggleCase,

    // more debugging related
    Clone,
    Wipe,
    Remove,
    Nothing,

    // conditional, same effect as remove if conditions are not met
    // bool indicates if the condition should be negated
    IfCharacterLength(bool, Ordering, usize),
    IfContains(bool, String),
}

#[derive(Debug)]
pub enum MutationBuildError {
    ActionDoesNotExist,
    MissingArguments,
    InvalidArgument,
}

#[derive(Clone)]
pub struct Mutation {
    pub action: Action,
    pub times: usize,
    pub keep_original: bool,
}

#[derive(Clone)]
pub struct MutationSet {
    pub mutations: Vec<Mutation>,
}

pub struct MutationResult {
    pub mutated_words: Vec<String>,
}

impl MutationSet {
    pub fn test_size(&self) -> usize {
        self.perform("").mutated_words.len()
    }

    pub fn perform(&self, word: &str) -> MutationResult {
        let mut result: Vec<String> = vec![word.to_owned()];
        let mut buffer: Vec<String> = Vec::new();

        for mutation in &self.mutations {
            buffer.clear();
            for s in &result {
                mutation.perform(&mut buffer, s)
            }
            std::mem::swap(&mut result, &mut buffer);
        }

        MutationResult {
            mutated_words: result,
        }
    }

    pub fn empty_set() -> MutationSet {
        let mutation = Mutation {
            action: Action::Nothing,
            times: 1,
            keep_original: false,
        };
        MutationSet {
            mutations: vec![mutation],
        }
    }
}

impl Mutation {
    pub fn perform(&self, result: &mut Vec<String>, input: &str) {
        if self.keep_original {
            result.push(input.to_owned());
        }

        match &self.action {
            Action::Prepend(_, tokens) => {
                for word in token_iterator(tokens) {
                    result.push(format!("{}{}", word.repeat(self.times), input))
                }
            }
            Action::Append(_, tokens) => {
                for word in token_iterator(tokens) {
                    result.push(format!("{}{}", input, word.repeat(self.times)))
                }
            }
            Action::Replace(s, b) => {
                if input.contains(s) || !self.keep_original {
                    result.push(input.replace(s, b))
                }
            }
            Action::RemoveFirstLetter => {
                let mut chrs = input.chars();
                for _ in 0..self.times {
                    chrs.next();
                }
                result.push(chrs.as_str().to_string())
            }
            Action::RemoveLastLetter => {
                let mut chrs = input.chars();
                for _ in 0..self.times {
                    chrs.next_back();
                }
                result.push(chrs.as_str().to_string())
            }
            Action::IfCharacterLength(not, ord, number) => {
                if (input.len().cmp(number) == *ord) != *not {
                    result.push(input.to_owned())
                }
            }
            Action::IfContains(not, string) => {
                if input.contains(string) != *not {
                    result.push(input.to_owned())
                }
            }
            Action::FirstLetter => result.push(
                input
                    .chars()
                    .next()
                    .map_or(String::from(""), |x| x.to_string()),
            ),
            Action::Capitalize => {
                let mut chars = input.chars();
                if let Some(first) = chars.next() {
                    let mut new_string = first.to_uppercase().to_string();
                    new_string.push_str(chars.as_str().to_lowercase().as_str());
                    result.push(new_string);
                } else {
                    result.push(String::new());
                }
            }
            Action::ToggleCase => {
                let new_string: String = input
                    .chars()
                    .map(|c| {
                        if c.is_uppercase() {
                            c.to_lowercase().to_string()
                        } else {
                            c.to_uppercase().to_string()
                        }
                    })
                    .collect();
                result.push(new_string);
            }
            Action::Reverse => result.push(input.chars().rev().collect()),
            Action::UppercaseAll => result.push(input.to_uppercase()),
            Action::LowercaseAll => result.push(input.to_lowercase()),
            Action::Clone => result.append(&mut vec![input.to_owned(), input.to_owned()]),
            Action::Wipe => result.push(String::new()),
            Action::Nothing => result.push(input.to_owned()),
            Action::Remove => (),
        }
    }
}

impl Display for Mutation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.times > 1 {
            write!(f, "{}x ", self.times)?;
        }

        match &self.action {
            Action::Prepend(s, _) => write!(f, "prepend: {}", s),
            Action::Append(s, _) => write!(f, "append: {}", s),
            Action::Replace(s, b) => write!(f, "replace: {} -> {}", s, b),
            Action::Reverse => write!(f, "reverse"),
            Action::Capitalize => write!(f, "capitalize"),
            Action::ToggleCase => write!(f, "toggle case"),
            Action::RemoveFirstLetter => write!(f, "remove 1st letter"),
            Action::RemoveLastLetter => write!(f, "remove last letter"),
            Action::Clone => write!(f, "clone"),
            Action::Wipe => write!(f, "wipe"),
            Action::Nothing => write!(f, "nothing"),
            Action::FirstLetter => write!(f, "first letter"),
            Action::UppercaseAll => write!(f, "uppercase all"),
            Action::LowercaseAll => write!(f, "lowercase all"),
            Action::Remove => write!(f, "remove"),

            Action::IfCharacterLength(not, ord, number) => {
                write!(f, "if length {:?} {} = {}", ord, number, !not)
            }
            Action::IfContains(not, string) => write!(f, "if contains {} = {}", string, !not),
        }?;

        if self.keep_original {
            write!(f, " (keeping original)")
        } else {
            write!(f, "")
        }
    }
}

macro_rules! check_action_args_length {
    ($action:expr, $requiredArgs:expr, $actualArgs:expr) => {
        if $actualArgs > ($requiredArgs - 1) {
            Ok($action)
        } else {
            Err(MutationBuildError::MissingArguments)
        }
    };
}

impl Action {
    pub fn from_string(
        action: &str,
        arguments: Vec<&str>,
        options: &str,
    ) -> Result<Action, MutationBuildError> {
        let argc = arguments.len();

        match action {
            "prepend" => {
                let s = arguments[0].to_owned();
                let tokens = tokenize_format_string(&s);
                check_action_args_length!(Action::Prepend(s, tokens), 1, argc)
            }
            "append" => {
                let s = arguments[0].to_owned();
                let tokens = tokenize_format_string(&s);
                check_action_args_length!(Action::Append(s, tokens), 1, argc)
            }
            "replace" => {
                check_action_args_length!(
                    Action::Replace(arguments[0].to_owned(), arguments[1].to_owned()),
                    2,
                    argc
                )
            }
            "if_length" => {
                check_action_args_length!(
                    {
                        let arg_chrs: Vec<char> = arguments[0].chars().collect();
                        let first_chr = arg_chrs.first().unwrap();

                        let ordering = match first_chr {
                            '>' => Ordering::Greater,
                            '<' => Ordering::Less,
                            '=' => Ordering::Equal,
                            _ => return Err(MutationBuildError::InvalidArgument),
                        };

                        let mut number_chrs = arguments[0].chars();
                        number_chrs.next();
                        let number: usize = number_chrs.as_str().parse().unwrap();

                        Action::IfCharacterLength(options.contains('!'), ordering, number)
                    },
                    1,
                    argc
                )
            }
            "if_contains" => {
                check_action_args_length!(
                    Action::IfContains(options.contains('!'), arguments[0].to_owned()),
                    1,
                    argc
                )
            }
            "reverse" => Ok(Action::Reverse),
            "capitalize" => Ok(Action::Capitalize),
            "toggle_case" => Ok(Action::ToggleCase),
            "clone" => Ok(Action::Clone),
            "wipe" => Ok(Action::Wipe),
            "1st_letter" => Ok(Action::FirstLetter),
            "nothing" => Ok(Action::Nothing),
            "uppercase_all" => Ok(Action::UppercaseAll),
            "lowercase_all" => Ok(Action::LowercaseAll),
            "remove_last_letter" => Ok(Action::RemoveLastLetter),
            "remove_first_letter" => Ok(Action::RemoveFirstLetter),
            "remove" => Ok(Action::Remove),
            _ => Err(MutationBuildError::ActionDoesNotExist),
        }
    }
}

pub fn parse_mutation_string(mutation_strings: &Vec<String>) -> Vec<Mutation> {
    let mut mutations: Vec<Mutation> = vec![];

    for mutation_string in mutation_strings {
        let mut mutation_split: Vec<&str> = mutation_string
            .split(':')
            // .into_iter()
            .map(|x| x.trim())
            .collect();
        let mut mutation_action = mutation_split[0];
        let mut mutation_runtimes: usize = 1;
        let mut mutation_options: &str = "";

        if mutation_action.contains(' ') {
            let mutation_action_split: Vec<&str> = mutation_action.split(' ').collect();
            if mutation_action_split.len() > 1 {
                mutation_action = mutation_action_split.last().unwrap();
                match mutation_action_split[0].parse() {
                    Ok(f) => mutation_runtimes = f,
                    Err(_) => mutation_options = mutation_action_split[0],
                }
            }
            if mutation_action_split.len() > 2 {
                mutation_options = mutation_action_split[1];
            }
        }

        mutation_split.remove(0);

        match Action::from_string(mutation_action, mutation_split, mutation_options) {
            Ok(m) => mutations.push(Mutation {
                action: m,
                times: mutation_runtimes,
                keep_original: mutation_options.contains('k'),
            }),
            Err(e) => {
                crate::logging::warning(&format!(
                    "couldn't build mutation {} ({:?})",
                    mutation_action, e
                ));
            }
        }
    }

    mutations
}

#[cfg(test)]
mod tests {
    use super::{Action, Mutation, MutationSet, parse_mutation_string};
    use crate::pattern::tokenize_format_string;

    #[test]
    fn basic_mutations() {
        let mutation_set = MutationSet {
            mutations: vec![
                Mutation {
                    action: Action::Reverse,
                    times: 1,
                    keep_original: false,
                },
                Mutation {
                    action: Action::Append(String::from("abc"), tokenize_format_string("abc")),
                    times: 1,
                    keep_original: false,
                },
                Mutation {
                    action: Action::Prepend(String::from("abc"), tokenize_format_string("abc")),
                    times: 1,
                    keep_original: false,
                },
            ],
        };

        let mutation_result = mutation_set.perform("word");

        assert_eq!(mutation_result.mutated_words, vec!["abcdrowabc"])
    }

    #[test]
    fn advanced_mutation() {
        let mutation_set = MutationSet {
            mutations: vec![Mutation {
                action: Action::Append(String::from("{0-9}"), tokenize_format_string("{0-9}")),
                times: 1,
                keep_original: false,
            }],
        };

        let mutation_result = mutation_set.perform("word");

        assert_eq!(
            mutation_result.mutated_words,
            vec![
                "word0", "word1", "word2", "word3", "word4", "word5", "word6", "word7", "word8",
                "word9"
            ]
        )
    }

    #[test]
    fn capitalize_and_toggle_case_mutations() {
        let cap_set = MutationSet {
            mutations: vec![Mutation {
                action: Action::Capitalize,
                times: 1,
                keep_original: false,
            }],
        };
        assert_eq!(cap_set.perform("word").mutated_words, vec!["Word"]);
        assert_eq!(cap_set.perform("WORD").mutated_words, vec!["Word"]);

        let toggle_set = MutationSet {
            mutations: vec![Mutation {
                action: Action::ToggleCase,
                times: 1,
                keep_original: false,
            }],
        };
        assert_eq!(toggle_set.perform("WoRd").mutated_words, vec!["wOrD"]);
    }

    #[test]
    fn clone_mutation() {
        let clone_set = MutationSet {
            mutations: vec![Mutation {
                action: Action::Clone,
                times: 1,
                keep_original: false,
            }],
        };
        assert_eq!(
            clone_set.perform("test").mutated_words,
            vec!["test", "test"]
        );
    }

    #[test]
    fn wipe_mutation() {
        let wipe_set = MutationSet {
            mutations: vec![Mutation {
                action: Action::Wipe,
                times: 1,
                keep_original: false,
            }],
        };
        assert_eq!(wipe_set.perform("test").mutated_words, vec![""]);
    }

    #[test]
    fn parsing_mutation_string() {
        let mutations =
            parse_mutation_string(&vec!["reverse".to_string(), "append:123".to_string()]);
        assert_eq!(mutations.len(), 2);
        match mutations[0].action {
            Action::Reverse => assert!(true),
            _ => assert!(false, "Expected Reverse"),
        }
        match &mutations[1].action {
            Action::Append(s, _) => assert_eq!(s, "123"),
            _ => assert!(false, "Expected Append"),
        }
    }

    #[test]
    fn remove_letters_edge_cases() {
        // Remove First
        let rm_first = MutationSet {
            mutations: vec![Mutation {
                action: Action::RemoveFirstLetter,
                times: 100, // Excessive
                keep_original: false,
            }],
        };
        assert_eq!(rm_first.perform("short").mutated_words, vec![""]);

        // Remove Last
        let rm_last = MutationSet {
            mutations: vec![Mutation {
                action: Action::RemoveLastLetter,
                times: 100, // Excessive
                keep_original: false,
            }],
        };
        assert_eq!(rm_last.perform("short").mutated_words, vec![""]);
    }

    #[test]
    fn first_letter_mutation() {
        let first = MutationSet {
            mutations: vec![Mutation {
                action: Action::FirstLetter,
                times: 1,
                keep_original: false,
            }],
        };
        assert_eq!(first.perform("word").mutated_words, vec!["w"]);
        assert_eq!(first.perform("").mutated_words, vec![""]);
    }

    #[test]
    fn casing_mutations() {
        let upper = MutationSet {
            mutations: vec![Mutation {
                action: Action::UppercaseAll,
                times: 1,
                keep_original: false,
            }],
        };
        assert_eq!(upper.perform("MixedCase").mutated_words, vec!["MIXEDCASE"]);

        let lower = MutationSet {
            mutations: vec![Mutation {
                action: Action::LowercaseAll,
                times: 1,
                keep_original: false,
            }],
        };
        assert_eq!(lower.perform("MixedCase").mutated_words, vec!["mixedcase"]);
    }

    #[test]
    fn replace_mutation_edge() {
        let rep = MutationSet {
            mutations: vec![Mutation {
                action: Action::Replace("foo".to_string(), "bar".to_string()),
                times: 1,
                keep_original: false,
            }],
        };
        // If "foo" not found, formatting logic returns original if keep_original=false?
        // Logic: if !keep_original -> push(input.replace). input.replace returns input if not found.
        // So it returns ["baz"].
        assert_eq!(rep.perform("baz").mutated_words, vec!["baz"]);

        // With keep_original, should return original
        let rep_keep = MutationSet {
            mutations: vec![Mutation {
                action: Action::Replace("foo".to_string(), "bar".to_string()),
                times: 1,
                keep_original: true,
            }],
        };
        assert_eq!(rep_keep.perform("baz").mutated_words, vec!["baz"]);
    }

    #[test]
    fn conditional_mutations() {
        // Length > 3
        let if_len_gt = MutationSet {
            mutations: vec![Mutation {
                action: Action::IfCharacterLength(false, std::cmp::Ordering::Greater, 3), // not negated
                times: 1,
                keep_original: false,
            }],
        };
        assert_eq!(if_len_gt.perform("four").mutated_words, vec!["four"]);
        assert_eq!(if_len_gt.perform("two").mutated_words.is_empty(), true);

        // Contains "a"
        let if_contains = MutationSet {
            mutations: vec![Mutation {
                action: Action::IfContains(false, "a".to_string()),
                times: 1,
                keep_original: false,
            }],
        };
        assert_eq!(if_contains.perform("apple").mutated_words, vec!["apple"]);
        assert_eq!(if_contains.perform("berry").mutated_words.is_empty(), true);
    }
    #[test]
    fn remove_mutation() {
        let remove = MutationSet {
            mutations: vec![Mutation {
                action: Action::Remove,
                times: 1,
                keep_original: false,
            }],
        };
        // Remove action returns nothing (should empty the result for that path)
        assert_eq!(remove.perform("anything").mutated_words.is_empty(), true);
    }

    #[test]
    fn nothing_mutation() {
        let nothing = MutationSet {
            mutations: vec![Mutation {
                action: Action::Nothing,
                times: 1,
                keep_original: false,
            }],
        };
        // Nothing action returns input as is
        assert_eq!(
            nothing.perform("unchanged").mutated_words,
            vec!["unchanged"]
        );
    }
}
