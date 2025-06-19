#include <stdio.h>
#include "tokens.h"

int calculate_total_generations(Token *toks, int toks_length) {
    int generations = 1;
    for (int i = 0; i < toks_length; i++) {
        generations *= toks[i].end - toks[i].current;
    }
    return generations;
}

void print_token(Token *tok) {
    printf("token(%d, %d)\n", tok->current, tok->end);
}

void debug_tokens(Token *toks, int toks_length) {
    printf("-- Total tokens: %d\n", toks_length);
    printf("-- Total generations: %u\n", calculate_total_generations(toks, toks_length));
    for (int i = 0; i < toks_length; i++) {
        print_token(&toks[i]);
    }
}

void index_to_token_combination(int index, Token *base, int toks_length, int *out) {
    for (int i = toks_length - 1; i >= 0; i--) {
        int range = base[i].end - base[i].current;
        out[i] = index % range + base[i].current;
        index /= range;
    }
}
