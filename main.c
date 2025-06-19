#include "tokens.h"
#include "threads.h"

Token tokens[3] = {
    {0,     1,      TEXT,       {.text = "administrator"}},
    {0,     10,     COUNT,      {.start = 1}},
    {0,     26,     LETTER,     {.start = 'a'}}};

int main()
{
    distribute_threads(tokens, 3, 3);
    return 0;
}
