from __future__ import annotations

import unittest

class Cube:
    def __init__(self, triple: tuple[int, int, int]):
        x, y, z = triple
        self.x = x
        self.y = y
        self.z = z

    def to_triple(self) -> tuple[int, int, int]:
        return self.x, self.y, self.z

    def __eq__(self, other: object) -> bool:
        if isinstance(other, Cube):
            return self.to_triple() == other.to_triple()
        return False

    def __hash__(self) -> int:
        return hash(self.to_triple())

    def adjacent_cubes(self) -> tuple[Cube, Cube, Cube, Cube, Cube, Cube]:
        x, y, z = self.to_triple()
        return (
            Cube((x    , y    , z - 1)),
            Cube((x    , y    , z + 1)),
            Cube((x - 1, y    , z    )),
            Cube((x + 1, y    , z    )),
            Cube((x    , y - 1, z    )),
            Cube((x    , y + 1, z    ))
        )

def read_puzzle(text: str) -> list[Cube]:
        triples = map(
            lambda line: tuple(map(lambda entry: int(entry), line.split(','))),
            text.splitlines()
        )
        return list(map(lambda t: Cube((t[0], t[1], t[2])), triples))

# Assumptions by looking at the input.
# 1. The cubes are all have positive coordinate entries, i.e., they lie in the first quadrant.
# 2. The cubes do not lie on an axis, i.e., no entry is zero.

def extrema(cubes: set[Cube]) -> tuple[tuple[int, int], tuple[int, int], tuple[int, int]]:
    min_x = min(map(lambda cube: cube.x, cubes))
    max_x = max(map(lambda cube: cube.x, cubes))
    min_y = min(map(lambda cube: cube.y, cubes))
    max_y = max(map(lambda cube: cube.y, cubes))
    min_z = min(map(lambda cube: cube.z, cubes))
    max_z = max(map(lambda cube: cube.z, cubes))

    return ((min_x, max_x), (min_y, max_y), (min_z, max_z))

def flood(cubes: set[Cube]) -> int:
    from collections import deque

    x_extrema, y_extrema, z_extrema = extrema(cubes)

    x_min, x_max = x_extrema
    y_min, y_max = y_extrema
    z_min, z_max = z_extrema

    external_area = 0

    start = Cube((x_min - 1, y_min - 1, z_min - 1))
    discovered: set[Cube] = set([ start ])
    staged: deque[Cube] = deque([ start ])

    while len(staged) > 0:
        cube = staged.pop()

        # The steam will expand to reach as much as possible [...] but never expanding diagonally.
        for adjacent_cube in cube.adjacent_cubes():
            if adjacent_cube not in discovered:
                is_air = adjacent_cube not in cubes
                in_bounds = (
                    x_min - 1 <= cube.x <= x_max + 1 and
                    y_min - 1 <= cube.y <= y_max + 1 and
                    z_min - 1 <= cube.z <= z_max + 1
                )
                if in_bounds:
                    if is_air:
                        discovered.add(adjacent_cube)
                        staged.append(adjacent_cube)
                    else:
                        external_area += 1

    return external_area

#---

def surface_area(cubes: set[Cube]) -> int:
    surfaces = 6 * len(cubes)

    for cube in cubes:
        for adjacent in cube.adjacent_cubes():
            if adjacent in cubes:
                surfaces -= 1

    return surfaces

def main(puzzle_path: str) -> None:
    with open(puzzle_path) as puzzle:
        cubes = set(read_puzzle(puzzle.read()))
        part_one = surface_area(cubes)
        part_two = flood(cubes)
        print(f'part one: {part_one}')
        print(f'part two: {part_two}')

class TestDroplet(unittest.TestCase):
    sample = '''2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5'''

    def test_sample_extrema(self) -> None:
        cubes = set(read_puzzle(TestDroplet.sample))
        self.assertEqual(
            extrema(cubes),
            ((1, 3), (1, 3), (1, 6))
        )

if __name__ == '__main__':
    from sys import argv
    if len(argv) == 2:
        main(argv[1])
    else:
        unittest.main()
