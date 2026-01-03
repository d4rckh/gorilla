use std::vec;

use crate::patterns::{Token, TokenIter, calculate_total_generations, token_iterator_from_start_end};

pub fn distribute_token_iter_work(tokens: &Vec<Token>, threads_n: u128) -> Vec<TokenIter> {
    let mut result: Vec<TokenIter> = vec![];

    let total_generations = calculate_total_generations(&tokens);
    
    let base = total_generations / threads_n;
    let rem = total_generations % threads_n;

    let mut start_i = 0;
    
    for thread_i in 0..threads_n {
        let end_i = start_i + base + (if thread_i < rem { 1 } else { 0 });
    
        result.push(
            token_iterator_from_start_end(tokens, start_i, end_i)
        );
    
        start_i = end_i;
    }

    result
}