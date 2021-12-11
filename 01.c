#include <stdio.h>

int number_of_increases()
{
    int reference, ahead;

    scanf("%i", &reference);

    int increases = 0;
    while (scanf("%i", &ahead) != EOF) {
        if (reference < ahead)
            ++increases;
        reference = ahead;
    }

    return increases;
}

int main()
{
    int increases = number_of_increases();

    printf("Number of increases: %i\n", increases);

    return 0;
}
