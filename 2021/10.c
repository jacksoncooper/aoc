#include <limits.h>
#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>

#define LINE 128
#define LINES 128
#define ALPHABET 128

enum chunk_is {
    okay,
    incomplete,
    invalid
};

enum chunk_is validate_line(char *line, char *panicked_at, char *unmatched)
{
    // Validate a line of the navigation subsystem.
    // Subscripting is dangerous; character literals are likely signed.

    static char closes[ALPHABET] = { '\0' };
    closes['('] = ')';
    closes['['] = ']';
    closes['{'] = '}';
    closes['<'] = '>';

    char recents[LINE] = { '\0' };
    int head = 0;

    for (int i = 0; line[i] != '\n'; ++i) {
        char current = line[i];
        if (current == '(' || current == '[' || current == '{' || current == '<') {
            recents[head++] = current;
        } else {
            char recent = recents[--head];
            if (closes[recent] != current) {
                *panicked_at = current;
                return invalid;
            }
        }
    }

    if (head == 0) return okay;

    int j = 0;
    for (int i = head - 1; i >= 0; --i, ++j)
        unmatched[j] = recents[i];
    unmatched[j] = '\0';

    return incomplete;
}

int part_one()
{
    static int points[ALPHABET] = { 0 };
    points[')'] =     3;
    points[']'] =    57;
    points['}'] =  1197;
    points['>'] = 25137;

    int high_score = 0;
    char panicked_at = '\0';
    char line[LINE + 2], unmatched[LINE + 1];

    while (fgets(line, LINE + 2, stdin) != NULL) {
        if (validate_line(line, &panicked_at, unmatched) == invalid) {
            high_score += points[panicked_at];
        }
    }

    return high_score;
}

int less_than(const void* l, const void* r)
{
     long int a = *((long int *) l);
     long int b = *((long int *) r);

     if (a == b) return  0;
     if (a <  b) return -1;
                 return  1;
}

long int part_two()
{
    static long int points[ALPHABET] = { 0 };
    points['('] = 1;
    points['['] = 2;
    points['{'] = 3;
    points['<'] = 4;

    long int scores[LINES] = { 0 };
    char panicked_at = '\0';
    char line[LINE + 2], unmatched[LINE + 1];

    long int score = 0;
    while (fgets(line, LINE + 2, stdin) != NULL) {
        if (validate_line(line, &panicked_at, unmatched) == incomplete) {
            for (int i = 0; unmatched[i] != '\0'; ++i) {
                scores[score] *= 5L;
                scores[score] += points[unmatched[i]];
            }
            ++score;
        }
    }

    qsort(scores, score, sizeof(long int), less_than);

    // Guaranteed odd number of incomplete lines.
    return scores[(score - 1) / 2];
}

int main()
{
    // printf("syntax error high score: %i\n", part_one());
    printf("autocomplete high score: %li\n", part_two());
    return 0;
}
