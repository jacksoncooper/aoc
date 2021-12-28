#include <ctype.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdio.h>

// #define WIDTH   10
// #define HEIGHT   5
// #define PAIRS  100

#define WIDTH   100
#define HEIGHT  100
#define PAIRS  1000

typedef int (*Terrain)[WIDTH];
Terrain make_terrain(FILE *stream)
{
    static int terrain[HEIGHT][WIDTH];

    for (int row = 0; row < HEIGHT; ++row) {
        for (int column = 0; column < WIDTH; ++column) {
            int height = fgetc(stream);
            if (!isdigit(height)) return NULL;
            terrain[row][column] = height - '0';
        }
        fgetc(stream);
    }

    return terrain;
}

typedef int (*Points)[2];
Points minimums(Terrain terrain, int *length)
{
    static int points[PAIRS][2];

    *length = 0;
    for (int row = 0; row < HEIGHT; ++row) {
        for (int column = 0; column < WIDTH; ++column) {
            int height       = terrain[row][column];
            bool left_slope  = column <= 0          || terrain[row    ][column - 1] > height;
            bool right_slope = column >= WIDTH  - 1 || terrain[row    ][column + 1] > height;
            bool up_slope    = row    <= 0          || terrain[row - 1][column    ] > height;
            bool down_slope  = row    >= HEIGHT - 1 || terrain[row + 1][column    ] > height;
            if (left_slope && right_slope && up_slope && down_slope)
                points[*length][0] = row, points[*length][1] = column, ++*length;
        }
    }

    return points;
}

int risk_levels(Terrain terrain, Points minimums, int length)
{
    int sum = 0;
    for (int i = 0; i < length; ++i) {
        int *point = minimums[i];
        int row = point[0], column = point[1];
        sum += terrain[row][column] + 1;
    }
    return sum;
}

int go(Terrain terrain, int row, int column, int previous)
{
    if (column < 0 || column >= WIDTH) return 0;
    if (row < 0 || row >= HEIGHT) return 0;
    int height = terrain[row][column];
    if (height > 8 || height < previous) return 0;
    terrain[row][column] = 9; // Danger!
    return 1 + go(terrain, row    , column - 1, height)
             + go(terrain, row    , column + 1, height)
             + go(terrain, row - 1, column    , height)
             + go(terrain, row + 1, column    , height);
}

int *basin_sizes(Terrain terrain, Points minimums, int length)
{
    // Each local minimum has exactly one basin.
    static int sizes[PAIRS];

    for (int i = 0; i < length; ++i) {
        int *point = minimums[i];
        sizes[i] = go(terrain, point[0], point[1], 0);
    }

    return sizes;
}

int product_of_maximum(int *sizes, int length, int passes)
{
    int maximums = 1;
    for (int pass = 0; pass < passes; ++pass) {
        int maximum = 1, at;
        for (int i = 0; i < length; ++i)
            if (sizes[i] > maximum) at = i, maximum = sizes[i];
        maximums *= maximum;
        sizes[at] = 0; // Danger!
    }
    return maximums;
}

int main()
{
    Terrain terrain = make_terrain(stdin);

    if (terrain == NULL) { printf("error: bad read\n"); return 0; }

    // for (int row = 0; row < HEIGHT; ++row) {
    //     for (int column = 0; column < WIDTH; ++column) {
    //         printf("%i", terrain[row][column]);
    //     }
    //     printf("\n");
    // }

    int length = 0;
    Points points = minimums(terrain, &length); 

    // for (int i = 0; i < length; ++i)
    //     printf("(%i, %i)\n", points[i][0] + 1, points[i][1] + 1);

    printf("sum of risk levels: %i\n", risk_levels(terrain, points, length));

    int *sizes = basin_sizes(terrain, points, length);

    printf("product of basins: %i\n", product_of_maximum(sizes, length, 3));
}
