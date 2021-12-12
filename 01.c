#include <stdio.h>

#define window_size 3

int increases()
{
    int reference, next;

    if (scanf("%i", &reference) != 1) return 0;

    int increases = 0;
    while (scanf("%i", &next) == 1) {
        if (reference < next)
            ++increases;
        reference = next;
    }

    return increases;
}

int sliding_increases()
{
    int reference, next, insert = 0;
    int window[window_size];

    if (scanf("%i %i %i", window, window + 1, window + 2) != window_size)
        return 0;

    reference = window[0] + window[1] + window[2];

    int increases = 0;
    while (scanf("%i", &next) == 1) {
        int oldest = window[insert], newest = next;

        window[insert] = newest;
        insert = insert + 1 > window_size - 1 ? 0 : insert + 1;

        int next_three = -oldest + reference + newest;
        if (reference < next_three)
            ++increases;
        reference = next_three;
    }
    
    return increases;
}

int main()
{
    // printf("Number of increases: %i\n", increases());
    printf("Number of sliding increases: %i\n", sliding_increases());

    return 0;
}
