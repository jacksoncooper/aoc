#include <stddef.h>
#include <stdio.h>

#define LINE 128
#define ALPHABET 128

enum chunk_is {
    okay,
    incomplete,
    invalid
};

enum chunk_is validate_line(char *line, char *panicked_at)
{
    // Validate a line of the navigation subsystem.
    // Subscripting is dangerous; character literals are likely signed.

    static char closes[ALPHABET] = { '\0' };
    closes['('] = ')';
    closes['['] = ']';
    closes['{'] = '}';
    closes['<'] = '>';

    static char recents[LINE] = { '\0' };
    int head = 0;

    for (int i = 0; line[i] != '\0'; ++i) {
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

    char line[LINE + 1];
    while (fgets(line, LINE + 1, stdin) != NULL) {
        if (validate_line(line, &panicked_at) == invalid) {
            high_score += points[panicked_at];
        }
    }

    return high_score;
}

int main()
{
    printf("syntax error high score: %i\n", part_one());
    return 0;
}
