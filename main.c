#include "tokens.h"
#include "threads.h"

Token tokens[2] = {
    {0, 20},
    {0, 30}
};

int main() {
    debug_tokens(tokens, 2);
    distribute_threads(tokens, 2, 2);
    return 0;
}
