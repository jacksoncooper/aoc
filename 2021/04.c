#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define maximum_boards 512
#define board_width 5
#define number_length 2
#define draws 100
// #define draws 27

int *line_of_integers(FILE *stream, size_t how_many, size_t length)
{
    // Store at most `how_many` numbers, 1 newline, and 1 null.
    size_t line_length = (length + 1) * how_many + 2;

    char *line = (char *) malloc(line_length);
    fgets(line, line_length, stream);
    if (line == NULL) return NULL;

    const char delimeters[] = " ,\n";

    const char *more = strtok(line, delimeters);
    if (more == NULL) return NULL;

    int *numbers = (int *) malloc(sizeof(int) * how_many);

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

        // Assumes we cannot have a partially constructed board.
        if (winners == NULL) return NULL;

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
        for (int column = 0; column < board_width; ++column) {
            Pair pair = board[row][column];
            printf("%2i (%c) ", pair[0], pair[1] ? '*' : '_');
        }
        printf("\n");
    }
}

void delete(Board board)
{
    for (int row = 0; row < board_width; ++row) {
        for (int column = 0; column < board_width; ++column) {
            free(board[row][column]);
        }
        free(board[row]);
    }
    free(board);
}

bool row_win(Board board, int row)
{
    for (int column = 0; column < board_width; ++column)
        if (!board[row][column][1]) return false;
    return true;
}

bool column_win(Board board, int column)
{
    for (int row = 0; row < board_width; ++row)
        if (!board[row][column][1]) return false;
    return true;
}

bool win_at(Board board, int row_intersect, int column_intersect)
{
    if (row_win(board, row_intersect)) return true;
    if (column_win(board, column_intersect)) return true;
    return false;
}

Pair find(Board board, int value) {
    for (int row = 0; row < board_width; ++row) {
        for (int column = 0; column < board_width; ++column) {
            if (board[row][column][0] == value) {
                Pair pair = (Pair) malloc(sizeof(int) * 2);
                pair[0] = row, pair[1] = column;
                return pair;
            }
        }
    }
    return NULL;
}

int score(Board board) {
    int score = 0;
    for (int row = 0; row < board_width; ++row) {
        for (int column = 0; column < board_width; ++column) {
                Pair pair = board[row][column];
                if (!pair[1]) score += pair[0];
        }
    }
    return score;
}

int main()
{
    int *cage = line_of_integers(stdin, draws, number_length);
    scanf("\n");

    Board boards[maximum_boards];
    Board new_board;

    int number_of_players = 0;
    while ((new_board = new(stdin)) != NULL) {
        boards[number_of_players++] = new_board;
        scanf("\n");
    }

    bool *turned_in = (bool *) malloc(sizeof(bool) * number_of_players);
    for (int player = 0; player < number_of_players; ++player)
        turned_in[player] = false;

    for (int draw = 0; draw < draws; ++draw) {
        int call = cage[draw];
        for (int player = 0; player < number_of_players; ++player) {
            Board board = boards[player];
            Pair pair;

            if ((pair = find(board, call)) != NULL) {
                int row = pair[0], column = pair[1];

                boards[player][row][column][1] = true;

                if (!turned_in[player] && win_at(board, row, column)) {
                    turned_in[player] = true;
                    int unmarked = score(board);
                    printf(
                        "Win! on draw %i at %i for player %i with score %i.\n",
                        draw + 1, call, player + 1, unmarked * call
                    );
                    show(board);
                    printf("\n");
                }
            }
        }
    }

    free(cage);
    free(turned_in);
 
    for (int player = 0; player < number_of_players; ++player)
        delete(boards[player]);

    return 0;
}
