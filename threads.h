#ifndef THREADS_H
#define THREADS_H

#include "tokens.h"

typedef struct {
    Token *toks;
    int thread_id;
    int toks_length;
    int start_index;
    int end_index;
} ThreadWork;

void distribute_threads(Token *toks, int toks_length, int threads_length);

#endif
