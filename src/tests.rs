#[cfg(test)]
mod threading_tests {
    use crate::{pattern::tokenize_format_string, threading::distribute_token_iter_work};

    #[test]
    fn token_iter_work_distribution() {
        let tokens = tokenize_format_string("{0-9}");
        let token_iters = distribute_token_iter_work(&tokens, 10);

        for thread_i in 0..10 {
            assert_eq!(token_iters[thread_i].current_index, thread_i as u128);
            assert_eq!(token_iters[thread_i].end_index, (thread_i + 1) as u128);
        }
    }

    #[test]
    fn token_iter_work_distribution_with_remaining() {
        let tokens = tokenize_format_string("{0-9}");
        let token_iters = distribute_token_iter_work(&tokens, 3);

        assert_eq!(token_iters[0].current_index, 0);
        assert_eq!(token_iters[0].end_index, 4);

        for thread_i in 1..3 {
            assert_eq!(
                token_iters[thread_i].current_index,
                (thread_i * 3 + 1) as u128
            );
            assert_eq!(token_iters[thread_i].end_index, (thread_i * 3 + 4) as u128);
        }
    }
}

#[cfg(test)]
mod token_tests {
    use crate::pattern::{Token, token_iterator, tokenize_format_string};

    #[test]
    fn tokenize_string_repeat() {
        let tokens = tokenize_format_string("hello{0-9}world");
        assert_eq!(tokens[1], Token::NumRange('0' as u32, '9' as u32))
    }

    #[test]
    fn tokenize_string_string() {
        let tokens = tokenize_format_string("hello{0-9}world");
        assert_eq!(tokens[2], Token::String(String::from("world")))
    }

    #[test]
    fn properly_tokenize_double_brackets() {
        let tokens = tokenize_format_string("{{0-9}}");

        for token in tokens.iter() {
            println!("{}", token.to_string());
        }

        assert_eq!(tokens.len(), 3);

        assert_eq!(tokens[0], Token::String("{".to_string()));
        assert_eq!(tokens[2], Token::String("}".to_string()))
    }

    #[test]
    fn properly_tokenize_double_brackets_2() {
        let tokens = tokenize_format_string("{{{}}{0-9}}{}}}");

        for token in tokens.iter() {
            println!("{}", token.to_string());
        }

        assert_eq!(tokens.len(), 3);

        assert_eq!(tokens[0], Token::String("{{{}}".to_string()));
        assert_eq!(tokens[2], Token::String("}{}}}".to_string()))
    }

    #[test]
    fn tokenize_execute_letters() {
        let ac_toks = token_iterator(&tokenize_format_string("{a-z}{a-z}"));
        let result: Vec<String> = ac_toks.collect();

        assert_eq!(result.len(), 26 * 26)
    }

    #[test]
    fn tokenize_execute_ascii() {
        let ac_toks = token_iterator(&tokenize_format_string("{ -~}"));
        let result: Vec<String> = ac_toks.collect();

        assert_eq!(result.len(), 95)
    }
}

#[cfg(test)]
mod mutation_tests {
    use crate::mutation::{Action, Mutation, MutationSet};

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
                    action: Action::Append(String::from("abc")),
                    times: 1,
                    keep_original: false,
                },
                Mutation {
                    action: Action::Prepend(String::from("abc")),
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
                action: Action::Append(String::from("{0-9}")),
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
}

#[cfg(test)]
mod yaml_test {
    use crate::yaml_parser::parse_mutation_yaml;

    #[test]
    fn yaml_parse_test() {
        let mutation_sets = parse_mutation_yaml(
            "name: alphabet
mutation_sets:
  - [ wipe, \"append:{a-z}\" ] # => a, b, c, ..., z",
        );

        let mutation_result = mutation_sets[0].perform("word");

        assert_eq!(mutation_result.mutated_words.len(), 26);
    }
}

#[cfg(test)]
mod scrape_tests {
    use crate::website_scraper::extract_words;
    
    #[test]
    fn basic_scrape() {
        let html = "<!doctype html><html><head></head><body> \
        <div> <h1>Example Domain</h1> \
        <p>This domain is for use in illustrative examples in documents. You may use this \
        domain in literature without prior coordination or asking for permission.</p> \
        </div> \
        </body> \
        </html>";
        let words = extract_words(html);

        assert!(words.contains(&"domain".to_string()));
    }
    #[test]
    fn ignore_script_tag() {
        let html = "<!doctype html><html><head></head><body><script>Some javascript</script> \
        <div> <h1>Example Domain</h1> \
        <p>This domain is for use in illustrative examples in documents. You may use this \
        domain in literature without prior coordination or asking for permission.</p> \
        </div> \
        </body> \
        </html>";
        let words = extract_words(html);

        assert!(words.contains(&"domain".to_string()));
        assert!(!words.contains(&"javascript".to_string()));
    }

    #[test]
    fn ignore_mulitple_script_tags() {
        let html = "<!doctype html><html><head></head><body><script>Some javascript</script> \
        <div> <h1>Example Domain</h1> \
        <p>This domain is for use in illustrative examples in documents. You may use this \
        domain in literature without prior coordination or asking for permission.</p> \
        </div> \
        <script>second script</script> \
        </body> \
        </html>";
        let words = extract_words(html);

        assert!(words.contains(&"domain".to_string()));
        assert!(!words.contains(&"javascript".to_string()));
        assert!(!words.contains(&"second".to_string()));
    }
    
    #[test]
    fn ignore_style_tag() {
        let html = "<html><body><style>body { color: red; }</style> \
        <p>visible text</p></body></html>";
        let words = extract_words(html);
        
        assert!(words.contains(&"visible".to_string()));
        // 'red' is < 4 chars so filtered anyway? "color" is 5.
        // "color" should be ignored.
        assert!(!words.contains(&"color".to_string()));
    }
}
