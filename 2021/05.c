#include <stdio.h>
#include <stdlib.h>

#define max_lines 1000
#define min(a, b) ((a) < (b) ? (a) : (b))
#define max(a, b) ((a) < (b) ? (b) : (a))

void munch_whitespace(FILE *stream)
{
    char c;
    while ((c = getc(stream)) == ' ' || c == '\t' || c == '\n');
    ungetc(c, stream);
}

void munch_right_arrow(FILE *stream)
{
    char c = getc(stream);
    if (c != '-') { ungetc(c, stream); return; }
    c = getc(stream);
    if (c != '>') { ungetc(c, stream); return; }
}

struct line
{
    int x_1, x_2;
    int y_1, y_2;
};

void arrange(struct line *line) {
    if (line -> x_1 > line -> x_2) {
        int t = line -> x_1; line -> x_1 = line -> x_2, line -> x_2 = t;
        t = line -> y_1; line -> y_1 = line -> y_2, line -> y_2 = t;
    }
}

struct line *read_line(FILE *stream)
{
    int x_1, x_2, y_1, y_2;
    munch_whitespace(stream);
    if (fscanf(stream, "%i,%i", &x_1, &y_1) != 2) return NULL;
    munch_whitespace(stream);
    munch_right_arrow(stream);
    if (fscanf(stream, "%i,%i", &x_2, &y_2) != 2) return NULL;
    struct line *new_line = (struct line *) malloc(sizeof(struct line));
    new_line -> x_1 = x_1; new_line -> x_2 = x_2;
    new_line -> y_1 = y_1; new_line -> y_2 = y_2;
    return new_line;
}

struct line *read_lines(FILE *stream, int *how_many)
{
    struct line *lines = (struct line *) malloc(sizeof(struct line) * max_lines);
    int line = 0; struct line *new_line;
    while ((new_line = read_line(stdin)) != NULL)
        lines[line++] = *new_line;
    *how_many = line;
    return lines;
}

void show(FILE* stream, struct line *line)
{
    fprintf(
        stream, "(%i, %i) -> (%i, %i)",
        line -> x_1, line -> y_1, line -> x_2, line -> y_2
    );
}

int main()
{
    int how_many; struct line *lines = read_lines(stdout, &how_many);
    printf("read %i lines\n", how_many);

    int maximum_x = 0, maximum_y = 0;
    for (int line = 0; line < how_many; ++line) {
        struct line current = lines[line];
        int x_1 = current.x_1, x_2 = current.x_2;
        int y_1 = current.y_1, y_2 = current.y_2;
        int current_x = max(x_1, x_2), current_y = max(y_1, y_2);
        maximum_x = max(maximum_x, current_x);
        maximum_y = max(maximum_y, current_y);
    }
    printf("maximum_x: %i, maximum_y: %i\n", maximum_x, maximum_y);
    ++maximum_x, ++maximum_y;

    for (int line = 0; line < 4; ++line)
        show(stdout, &lines[line]), printf("\n");
    printf("(rest elided)\n");

    int* *rows = (int* *) malloc(sizeof(int *) * maximum_y);
    for (int row = 0; row < maximum_y; ++row) {
        int *new_row = (int *) malloc(sizeof(int) * maximum_x);
        for (int column = 0; column < maximum_x; ++column) new_row[column] = 0;
        rows[row] = new_row;
    }

    for (int line = 0; line < how_many; ++line) {
        struct line current = lines[line];
        arrange(&current);
        int x_1 = current.x_1, x_2 = current.x_2;
        int y_1 = current.y_1, y_2 = current.y_2;
        int rise = y_2 - y_1, run = x_2 - x_1;
    
        // Vertical line.
        int minimum_y = min(y_1, y_2), maximum_y = max(y_1, y_2);
        if (!run) for (int row = minimum_y; row <= maximum_y; ++row)
            rows[row][x_1 = x_2] += 1;

        // Horizontal line.
        // int minimum_x = min(x_1, x_2), maximum_x = max(x_1, x_2);
        // if (!rise) for (int column = minimum_x; column <= maximum_x; ++column)
        //     rows[y_1 = y_2][column] += 1;

        // Due to limitations of the vent mapping system, the value of `slope`
        // can only be zero, negative one, or one.
            
        else {
            int slope = rise / run;
            int p_x = x_1, p_y = y_1;
            for (; p_x != x_2 || p_y != y_2; p_x += 1, p_y += slope)
                rows[p_y][p_x] += 1;
            rows[p_y][p_x] += 1;
        }
    }

    int poor_visibility = 0;
    for (int row = 0; row < maximum_y; ++row) {
        for (int column = 0; column < maximum_x; ++column) {
            int visibility = rows[row][column];
            if (visibility > 1) ++poor_visibility;
            if (visibility) printf("%i", visibility);
            else printf(".");
        }
        printf("\n");
    }
    printf("points with poor visibility: %i\n", poor_visibility);

    for (int row = 0; row < maximum_y; ++row) free(rows[row]);
    free(rows);

    free(lines);

    return 0;
}
