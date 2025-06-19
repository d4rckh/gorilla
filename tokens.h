#ifndef TOKENS_H
#define TOKENS_H

typedef enum {
    TEXT,
    COUNT,
    LETTER
} TokenType;

typedef union {
    char* text;
    int start;
} TokenInfo;

typedef struct {
    int current;
    int end;
    TokenType type;
    TokenInfo info;
} Token;

int calculate_total_generations(Token *toks, int toks_length);
void print_token(Token *tok);
void debug_tokens(Token *toks, int toks_length);
void index_to_token_combination(int index, Token *base, int toks_length, int *out);
void generate_combination(Token *toks, int *indices, int length, char *buf);

#endif // TOKENS_H
