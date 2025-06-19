#ifndef TOKENS_H
#define TOKENS_H

typedef struct {
    int current;
    int end;
} Token;

int calculate_total_generations(Token *toks, int toks_length);
void print_token(Token *tok);
void debug_tokens(Token *toks, int toks_length);
void index_to_token_combination(int index, Token *base, int toks_length, int *out);

#endif
