// For input:
#define diagnostic_length 1000
#define entry_length 12

// For sample:
// #define diagnostic_length 12
// #define entry_length 5

#include <stdbool.h>
#include <stddef.h>
#include <stdio.h>

enum common { more_zeros, more_ones, neither, malformed };

enum common most_common(char report[][entry_length], int digit, bool filter[entry_length])
{
    // TODO: The parameter `filter` is a horrible hack because I didn't read
    // through the examples and thought the ratings are a function of the bits
    // of each entry in the diagnostic report, instead of the ones that have
    // yet to be filtered.

    int zeros = 0, ones = 0;
    for (int entry = 0; entry < diagnostic_length; ++entry) {
        if (filter != NULL && !filter[entry]) continue;
        char value = report[entry][digit];
        if (value == '0') ++zeros;
        else if (value == '1') ++ones;
        else {
            printf(
                "abort: malformed value '%c' (entry %i, digit %i)\n",
                value, entry, digit
            );
            return malformed;
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
        enum common which = most_common(report, digit, NULL);
        if (which == malformed) return -1;
        if (which == neither) {
            printf("abort: malformed digit %i\n", digit + 1);
            return -1;
        }
        gamma = gamma << 1; epsilon = epsilon << 1;
        if (which == more_ones) gamma += 1;
        else epsilon += 1;
    }

    return gamma * epsilon;
}

void all_true(bool list[], size_t length)
{
    for (size_t i = 0; i < length; ++i) list[i] = true;
}

int yield_remaining(bool list[], int length)
{
    // Because we don't have an associative array in the standard library. :c

    int remaining = 0;
    int last_encountered = 0;

    for (int candidate = 0; candidate < length; ++candidate) {
        if (list[candidate]) {
            remaining += 1;
            last_encountered = candidate;
        }
    }

    if (remaining == 1) return last_encountered;
    return -1;
}

unsigned to_number(char entry[], size_t length)
{
    unsigned number = 0;
    for (size_t digit = 0; digit < length; ++digit) {
        number = number << 1;
        if (entry[digit] == '1') number += 1;
    }
    return number;
}

int life_support_rating()
{
    // Returns -1 on malformed input.
    // Careful of the narrowing of an unsigned product to int.

    char report[diagnostic_length][entry_length];

    for (int entry = 0; entry < diagnostic_length; ++entry) {
        if (scanf("%s", report + entry) != 1) return -1;
    }

    bool oxygen_candidates[diagnostic_length];
    all_true(oxygen_candidates, diagnostic_length);

    bool scrubber_candidates[diagnostic_length];
    all_true(scrubber_candidates, diagnostic_length);

    unsigned oxygen = 0, scrubber = 0;

    for (int digit = 0; digit < entry_length; ++digit) {
        enum common among_oxygen = most_common(report, digit, oxygen_candidates);
        if (among_oxygen == malformed) return -1;
        enum common among_scrubber = most_common(report, digit, scrubber_candidates);
        if (among_scrubber == malformed) return -1;

        for (int entry = 0; entry < diagnostic_length; ++entry) {
            char value = report[entry][digit];

            // For the oxygen generator, 
            if (among_oxygen == -1) return -1;
            bool zeros_common = among_oxygen == more_zeros && value == '0';
            bool ones_common = among_oxygen == more_ones && value == '1';
            bool neither_common = among_oxygen == neither && value == '1';
            bool oxygen_candidate = zeros_common || ones_common || neither_common;
            if (!oxygen_candidate) oxygen_candidates[entry] = false;

            // For the carbon dioxide scrubber,
            if (among_scrubber == -1) return -1;
            zeros_common = among_scrubber == more_zeros && value == '1';
            ones_common = among_scrubber == more_ones && value == '0';
            neither_common = among_scrubber == neither && value == '0';
            bool scrubber_candidate = zeros_common || ones_common || neither_common;
            if (!scrubber_candidate) scrubber_candidates[entry] = false;
        }

        int index;

        if (!oxygen && (index = yield_remaining(oxygen_candidates, diagnostic_length)) != -1)
            oxygen = to_number(report[index], entry_length);
            
        if (!scrubber && (index = yield_remaining(scrubber_candidates, diagnostic_length)) != -1)
            scrubber = to_number(report[index], entry_length);

        if (oxygen && scrubber) 
            return oxygen * scrubber;
    }

    return -1;
}

int main()
{
    printf("power consumption: %i\n", power_consumption());
    // printf("life support rating: %i\n", life_support_rating());
    return 0;
}
