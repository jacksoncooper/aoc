// For input:
#define diagnostic_length 1000
#define entry_length 12

// For sample:
// #define diagnostic_length 12
// #define entry_length 5

#include <stdio.h>

enum common { more_zeros, more_ones, neither };

int most_common(char report[][entry_length], int digit)
{
    int zeros = 0, ones = 0;
    for (int entry = 0; entry < diagnostic_length; ++entry) {
        char value = report[entry][digit];
        if (value == '0') ++zeros;
        else if (value == '1') ++ones;
        else {
            printf(
                "abort: malformed value '%c' (entry %i, digit %i)\n",
                value, entry, digit
            );
            return -1;
        };
    }
    if (zeros < ones) return more_ones;
    else if (zeros == ones) return neither;
    else return more_zeros;
}

int power_consumption()
{
    // Returns -1 on malformed input.
    // Careful of the narrowing of an unsigned product to int.

    char report[diagnostic_length][entry_length];

    for (int entry = 0; entry < diagnostic_length; ++entry) {
        if (scanf("%s", report + entry) != 1) return -1;
    }

    unsigned gamma = 0, epsilon = 0;

    for (int digit = 0; digit < entry_length; ++digit) {
        enum common which = most_common(report, digit);
        if (which == neither) {
            printf("abort: malformed digit %i\n", digit + 1);
            return -1;
        }
        gamma = gamma << 1; epsilon = epsilon << 1;
        if (which == more_ones) gamma += 1;
        else epsilon += 1;
    }

    printf("gamma: %u, epsilon: %u\n", gamma, epsilon);
    return gamma * epsilon;
}

int main()
{
    printf("power consumption: %i\n", power_consumption());
    return 0;
}
