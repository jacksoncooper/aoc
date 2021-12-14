// TODO: I have no idea what's wrong here I'm switching to Rust.

#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define board_width 5
#define number_length 2
#define draws 27

int *line_of_integers(FILE *stream, size_t how_many, size_t length)
{
    // Store at most `how_many` numbers, 1 newline, and 1 null.
    size_t line_length = (length + 1) * how_many + 2;

    char *line = (char *) malloc(line_length);

    fgets(line, line_length, stream);

    const char delimeters[] = " ,\n";

    const char *more = strtok(line, delimeters);
    if (more == NULL) return NULL;

    int *numbers = (int *) malloc(how_many);

    size_t at = 0;
    numbers[at++] = atoi(more);

    while ((more = strtok(NULL, delimeters)) != NULL) {
        numbers[at++] = atoi(more);
        if (at == how_many) break;
    }

    free(line);

    return numbers;
}

typedef int *Pair;
typedef Pair *Row;
typedef Row *Board;

Board new(FILE *stream)
{
    Board rows = (Board) malloc(sizeof(Row) * board_width);

    for (int line = 0; line < board_width; ++line) {
        int *winners = line_of_integers(stream, board_width, number_length);
        Row row = (Row) malloc(sizeof(Pair) * board_width);

        for (int winner = 0; winner < board_width; ++winner) {
            Pair pair = (Pair) malloc(sizeof(int) * 2);
            pair[0] = winners[winner], pair[1] = false;
            row[winner] = pair;
        }

        rows[line] = row;
        free(winners);
    }

    return rows;
}

void show(Board board)
{
    for (int row = 0; row < board_width; ++row) {
        for (int entry = 0; entry < board_width; ++entry) {
            Pair pair = board[row][entry];
            printf("%2i (%c) ", pair[0], pair[1] ? '*' : '_');
        }
        printf("\n");
    }
}

int main()
{
    int *numbers = line_of_integers(stdin, draws, number_length);
    for (int draw = 0; draw < draws; ++draw) printf("%i ", numbers[draw]);
    printf("\n");
    free(numbers);

    printf("\n");

    scanf("\n");
    Board first_board = new(stdin);
    show(first_board);

    printf("\n");

    scanf("\n");
    Board second_board = new(stdin);
    show(second_board);

    printf("\n");

    scanf("\n");
    Board third_board = new(stdin);
    show(third_board);

    return 0;
}
