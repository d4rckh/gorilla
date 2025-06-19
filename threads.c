#include <stdio.h>
#include <stdlib.h>
#include <pthread.h>
#include <string.h>
#include "threads.h"
#include "tokens.h"

void *thread_func(void *arg)
{
    ThreadWork thread_work = *(ThreadWork *)arg;
    printf("Thread work = %u -> %u\n", thread_work.start_index, thread_work.end_index);

    int *out = malloc(sizeof(int) * thread_work.toks_length);
    char *buf = malloc(sizeof(char) * 2000);

    for (int i = thread_work.start_index; i < thread_work.end_index; i++)
    {
        memset(out, 0, sizeof(int) * thread_work.toks_length);
        memset(buf, 0, sizeof(char) * 2000);

        index_to_token_combination(i, thread_work.toks, thread_work.toks_length, out);

        generate_combination(thread_work.toks, out, thread_work.toks_length, buf);

        printf("%s\n", buf);
    }

    free(buf);
    free(out);

    return NULL;
}

void distribute_threads(Token *toks, int toks_length, int threads_length)
{
    int total = calculate_total_generations(toks, toks_length);
    int base = total / threads_length;
    int rem = total % threads_length;

    printf("Total generations: %d\n", total);
    printf("Generating %d threads with %d (+%d remainder)\n", threads_length, base, rem);

    pthread_t threads[100];
    ThreadWork *works = malloc(sizeof(ThreadWork) * 100);

    int index = 0;

    for (int i = 0; i < threads_length; i++)
    {
        int end_index = index + base + (i < rem ? 1 : 0);

        printf("Thread %u will generate %u -> %u\n", i, index, end_index);

        works[i].start_index = index;
        works[i].end_index = end_index;
        works[i].toks = toks;
        works[i].toks_length = toks_length;
        works[i].thread_id = i;

        pthread_create(&threads[i], NULL, thread_func, &works[i]);

        index = end_index;
    }

    for (int i = 0; i < threads_length; i++)
    {
        pthread_join(threads[i], NULL);
    }

    free(works);
}
