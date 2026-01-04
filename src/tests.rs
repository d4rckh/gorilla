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
        assert_eq!(tokens[1], Token::Repeat(48, 57))
    }

    #[test]
    fn tokenize_string_string() {
        let tokens = tokenize_format_string("hello{0-9}world");
        assert_eq!(tokens[2], Token::String(String::from("world")))
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
    use crate::website_scraper::just_body_html_content;
    #[test]
    fn basic_scrape() {
        let html = "<!doctype html><html><head></head></body> \
        <div> <h1>Example Domain</h1> \
        <p>This domain is for use in illustrative examples in documents. You may use this \
        domain in literature without prior coordination or asking for permission.</p> \
        </div> \
        </body> \
        </html>";
        let content = just_body_html_content(html);

        assert!(content.contains("domain"));
    }
    #[test]
    fn ignore_script_tag() {
        let html = "<!doctype html><html><head></head></body><script>Some javascript</script> \
        <div> <h1>Example Domain</h1> \
        <p>This domain is for use in illustrative examples in documents. You may use this \
        domain in literature without prior coordination or asking for permission.</p> \
        </div> \
        </body> \
        </html>";
        let content = just_body_html_content(html);

        assert!(content.contains("domain"));
        assert!(!content.contains("javascript"));
    }

    #[test]
    fn ignore_mulitple_script_tags() {
        let html = "<!doctype html><html><head></head></body><script>Some javascript</script> \
        <div> <h1>Example Domain</h1> \
        <p>This domain is for use in illustrative examples in documents. You may use this \
        domain in literature without prior coordination or asking for permission.</p> \
        </div> \
        <script>second script</script> \
        </body> \
        </html>";
        let content = just_body_html_content(html);

        assert!(content.contains("domain"));
        assert!(!content.contains("javascript"));
        assert!(!content.contains("second"));
    }
}
