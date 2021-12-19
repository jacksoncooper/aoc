#include <stdbool.h>
#include <stdio.h>
#include <string.h>

#define direction_length 8

int depth_distance_product()
{
    /*        8
     *        v
     * forward_
     * down____
     * up______
     */

    // Stops reading pairs on malformed input.

    int depth = 0, distance = 0;
    char direction[direction_length];
    int size = 0;

    while (true) {
        if (scanf("%s", direction) != 1) break;
        if (scanf("%i", &size) != 1) break;

        if (!strncmp("forward", direction, direction_length - 1)) {
            distance += size;
        } else if (!strncmp("down", direction, direction_length - 1)) {
            depth += size;
        } else if (!strncmp("up", direction, direction_length - 1)) {
            depth -= size;
        } else break;
    }

    return depth * distance;
}

int depth_distance_product_with_aim()
{
    /*        8
     *        v
     * forward_
     * down____
     * up______
     */

    // Stops reading pairs on malformed input.

    int depth = 0, distance = 0, aim = 0;
    char direction[direction_length];
    int size = 0;

    while (true) {
        if (scanf("%s", direction) != 1) break;
        if (scanf("%i", &size) != 1) break;

        if (!strncmp("forward", direction, direction_length - 1)) {
            depth += aim * size;
            distance += size;
        } else if (!strncmp("down", direction, direction_length - 1)) {
            aim += size;
        } else if (!strncmp("up", direction, direction_length - 1)) {
            aim -= size;
        } else break;
    }

    return depth * distance;
}

int main()
{
    printf(
        "Product of depth and distance: %i\n",
        depth_distance_product_with_aim()
    );

    return 0;
}
