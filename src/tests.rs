#[cfg(test)]
mod token_tests {
    use crate::patterns::{Token, token_iterator, tokenize_format_string};

    #[test]
    fn tokenize_string_repeat() {
        let tokens = tokenize_format_string("hello{0-9}world");
        assert_eq!(tokens[1], Token::Repeat(48, 57, 48))
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
    use crate::mutation::{Action, Mutation, MutationResult, MutationSet};

    #[test]
    fn basic_mutations() {
        let mut mutation_result = MutationResult {
            original_word: String::from("word"),
            mutated_words: vec![],
        };

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

        mutation_set.perform(&mut mutation_result, "word");

        assert_eq!(mutation_result.mutated_words, vec!["abcdrowabc"])
    }

    #[test]
    fn advanced_mutation() {
        let mut mutation_result = MutationResult {
            original_word: String::from("word"),
            mutated_words: vec![],
        };

        let mutation_set = MutationSet {
            mutations: vec![Mutation {
                action: Action::Append(String::from("{0-9}")),
                times: 1,
                keep_original: false,
            }],
        };

        mutation_set.perform(&mut mutation_result, "word");

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
    use crate::{mutation::MutationResult, yaml_parser::get_mutation_sets};

    #[test]
    fn yaml_parse_test() {
        let mut mutation_result = MutationResult {
            original_word: String::from("word"),
            mutated_words: vec![],
        };

        let mutation_sets = get_mutation_sets(
            "name: alphabet
mutation_sets:
  - [ wipe, \"append:{a-z}\" ] # => a, b, c, ..., z",
        );

        mutation_sets[0].perform(&mut mutation_result, "word");

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
