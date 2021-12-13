// For input:
#define diagnostic_length 1000
#define entry_length 12

// For sample:
// #define diagnostic_length 12
// #define entry_length 5

#include <stdio.h>

int power_consumption()
{
    // Returns -1 on malformed input.
    // Careful of the narrowing of an unsigned product to int.

    char diagnostic_report[diagnostic_length][entry_length + 1];

    for (int entry = 0; entry < diagnostic_length; ++entry) {
        if (scanf("%s", diagnostic_report + entry) != 1) return -1;
    }

    unsigned gamma = 0, epsilon = 0;

    for (int digit = 0; digit < entry_length; ++digit) {
        int zeros = 0, ones = 0;
        for (int entry = 0; entry < diagnostic_length; ++entry) {
            char value = diagnostic_report[entry][digit];
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
        if (zeros == ones) {
            printf("abort: malformed digit %i\n", digit + 1);
            return -1;
        }
        gamma = gamma << 1; epsilon = epsilon << 1;
        if (zeros < ones) gamma += 1;
        else epsilon += 1;
    }

    return gamma * epsilon;
}

int main()
{
    printf("power consumption: %i\n", power_consumption());
    return 0;
}
