from collections import deque
from enum import Enum
import itertools
import sys
import unittest

Face = list[tuple[int, int]]

class Jet(Enum):
    LEFT = 1
    RIGHT = 2

class Cell(Enum):
    AIR = 1
    ROCK = 2

    def __str__(self) -> str:
        match self:
            case Cell.AIR:
                return '.'
            case Cell.ROCK:
                return '#'

class Wall(Enum):
    LEFT = 1
    RIGHT = 2
    BOTH = 3
    NONE = 4

class Chamber:
    def __init__(self, width: int):
        self.width = width
        self.rows: deque[list[Cell]] = deque()

    def height(self) -> int:
        return len(self.rows)

    def extend(self, n: int = 1) -> None:
        for _ in range(n):
            self.rows.appendleft([Cell.AIR] * self.width)

    def in_chamber(self, maybe_cell: tuple[int, int]) -> bool:
        # We will consider the chamber to be infinitely tall.
        return 0 <= maybe_cell[0] < self.width and maybe_cell[1] < self.height()

    def is_air(self, maybe_cell: tuple[int, int]) -> bool:
        if not self.in_chamber(maybe_cell):
            return False

        return (
            maybe_cell[1] < 0 or
            self[maybe_cell[1]][maybe_cell[0]] == Cell.AIR
        )

    def topology(self) -> tuple[int, ...]:
        first_rock = []
        for column in range(self.width):
            pebble = 0
            while pebble < self.height() and self[pebble][column] == Cell.AIR:
                pebble += 1
            first_rock.append(pebble)
        return tuple(first_rock)

    def __getitem__(self, i: int) -> list[Cell]:
        return self.rows[i]

    def __str__(self) -> str:
        readable_rows = [ ]
        floor = f"+{'-' * self.width}+"
        for row in self.rows:
            readable_rows.append(f"|{''.join(map(lambda cell: str(cell), row))}|")
        readable_rows.append(floor)
        return '\n'.join(readable_rows)

class Rock:
    def __init__(self, corner: tuple[int, int], height: int, left: Face, right: Face, bottom: Face):
        self.corner = corner
        self.height = height
        self.left_face = left
        self.right_face = right
        self.bottom_face = bottom

    def move_left(self, chamber: Chamber) -> bool:
        okay = self.okay_move(chamber, self.left_face, (-1, 0))
        if okay: self.corner = self.corner[0] - 1, self.corner[1]
        return okay

    def move_right(self, chamber: Chamber) -> bool:
        okay = self.okay_move(chamber, self.right_face, (1, 0))
        if okay: self.corner = self.corner[0] + 1, self.corner[1]
        return okay

    def move_down(self, chamber: Chamber) -> bool:
        okay = self.okay_move(chamber, self.bottom_face, (0, 1))

        if okay:
            self.corner = self.corner[0], self.corner[1] + 1
        else:
            if self.corner[1] < 0:
                chamber.extend(abs(self.corner[1]))
                self.corner = self.corner[0], 0

            features = itertools.chain(self.left_face, self.right_face, self.bottom_face)
            for face in map(self.absolute, features):
                chamber[face[1]][face[0]] = Cell.ROCK

        return okay

    def forms_cover(self, chamber: Chamber) -> bool:
        # We rely on the property that each rock is connected, so we may start at any feature.
        start = self.absolute(self.bottom_face[0])

        seen: set[tuple[int, int]] = set([ start ])
        discovered: deque[tuple[int, int]] = deque([start])
        connectivity = Wall.NONE

        def fold_connectivity(sample: Wall) -> None:
            nonlocal connectivity
            match (connectivity, sample):
                case (Wall.LEFT, Wall.RIGHT):
                    connectivity = Wall.BOTH
                case (Wall.RIGHT, Wall.LEFT):
                    connectivity = Wall.BOTH
                case (Wall.BOTH, _):
                    connectivity = Wall.BOTH
                case (Wall.NONE, _):
                    connectivity = sample

        while len(discovered) > 0:
            cell = discovered.popleft()

            if cell[0] <= 0:
                sample = Wall.LEFT
            elif cell[0] >= chamber.width - 1:
                sample = Wall.RIGHT
            else:
                sample = Wall.NONE

            fold_connectivity(sample)

            offsets = (
                (-1, -1), (0, -1), (1, -1),
                (-1,  0),          (1,  0),
                (-1,  1), (0,  1), (1,  1)
            )

            connected = map(lambda offset: (cell[0] + offset[0], cell[1] + offset[1]), offsets)
            connected_rock = filter(
                lambda cell: (
                    0 <= cell[0] < chamber.width and
                    0 <= cell[1] < chamber.height() and
                    chamber[cell[1]][cell[0]] == Cell.ROCK
                ),
                connected
            )
            unvisited = filter(lambda cell: cell not in seen, connected_rock)
            for cell in unvisited:
                seen.add(cell)
                discovered.append(cell)

        return connectivity == Wall.BOTH

    def okay_move(self, chamber: Chamber, face: Face, offset: tuple[int, int]) -> bool:
        maybe_air = map(
            lambda feature: self.absolute(
                (feature[0] + offset[0], feature[1] + offset[1])
            ),
            face
        )

        return all(map(chamber.is_air, maybe_air))

    def absolute(self, offset: tuple[int, int]) -> tuple[int, int]:
        return self.corner[0] + offset[0], self.corner[1] + offset[1]

class Minus(Rock):
    def __init__(self) -> None:
        super().__init__((2, -4), 1,
            [(0, 0)],
            [(3, 0)],
            [(0, 0), (1, 0), (2, 0), (3, 0)]
        )

class Plus(Rock):
    def __init__(self) -> None:
        super().__init__((2, -6), 3,
            [(1, 0), (0, 1), (1, 2)],
            [(1, 0), (2, 1), (1, 2)],
            [(0, 1), (1, 2), (2, 1)]
        )

class Chair(Rock):
    def __init__(self) -> None:
        super().__init__((2, -6), 3,
            [(0, 2)],
            [(2, 0), (2, 1), (2, 2)],
            [(0, 2), (1, 2), (2, 2)]
        )

class Pipe(Rock):
    def __init__(self) -> None:
        super().__init__((2, -7), 4,
        [(0, 0), (0, 1), (0, 2), (0, 3)],
        [(0, 0), (0, 1), (0, 2), (0, 3)],
        [(0, 3)]
        )

class Box(Rock):
    def __init__(self) -> None:
        super().__init__((2, -5), 2,
            [(0, 0), (0, 1)],
            [(1, 0), (1, 1)],
            [(0, 1), (1, 1)]
        )

def to_jets(text: str) -> list[Jet]:
    jets = []
    for symbol in text:
        if symbol == '<':
            jets.append(Jet.LEFT)
        elif symbol == '>':
            jets.append(Jet.RIGHT)
    return jets

class TestMovement(unittest.TestCase):
    def test_plus_in_empty_chamber(self) -> None:
        chamby = Chamber(7)
        self.assertEqual(chamby.height(), 0)

        plusy = Plus()

        for _ in range(2):
            self.assertTrue(plusy.move_left(chamby))
        self.assertFalse(plusy.move_left(chamby))

        for _ in range(4):
            self.assertTrue(plusy.move_right(chamby))
        self.assertFalse(plusy.move_right(chamby))

        for _ in range(2):
            self.assertTrue(plusy.move_left(chamby))

        for _ in range(3):
            self.assertTrue(plusy.move_down(chamby))
        self.assertFalse(plusy.move_down(chamby))

        self.assertEqual(chamby.height(), 3)

        self.assertEqual(chamby[0][3], Cell.ROCK)
        self.assertEqual(chamby[1][2], Cell.ROCK)
        self.assertEqual(chamby[1][3], Cell.AIR)
        self.assertEqual(chamby[1][4], Cell.ROCK)
        self.assertEqual(chamby[2][3], Cell.ROCK)

    def test_plus_then_minus(self) -> None:
        chamby = Chamber(7)
        self.assertEqual(chamby.height(), 0)

        plusy = Plus()

        self.assertTrue(plusy.move_right(chamby))

        for _ in range(3):
            self.assertTrue(plusy.move_down(chamby))
        self.assertFalse(plusy.move_down(chamby))

        self.assertEqual(chamby[0][4], Cell.ROCK)
        self.assertEqual(chamby[1][3], Cell.ROCK)
        self.assertEqual(chamby[1][4], Cell.AIR)
        self.assertEqual(chamby[1][5], Cell.ROCK)
        self.assertEqual(chamby[2][4], Cell.ROCK)

        minusy = Minus()

        for _ in range(2):
            self.assertTrue(minusy.move_left(chamby))

        for _ in range(4):
            self.assertTrue(minusy.move_down(chamby))
        self.assertFalse(minusy.move_down(chamby))

        self.assertEqual(chamby.height(), 3)

        self.assertEqual(chamby[0][0], Cell.ROCK)
        self.assertEqual(chamby[0][1], Cell.ROCK)
        self.assertEqual(chamby[0][2], Cell.ROCK)
        self.assertEqual(chamby[0][3], Cell.ROCK)

    def test_cover_detection(self) -> None:
        chamby = Chamber(7)
        self.assertEqual(chamby.height(), 0)

        boxy = Box()

        self.assertTrue(boxy.move_right(chamby))
        for _ in range(3):
            self.assertTrue(boxy.move_down(chamby))
        self.assertFalse(boxy.move_down(chamby))

        self.assertEqual(chamby.height(), 2)
        self.assertFalse(boxy.forms_cover(chamby))

        chairy = Chair()

        for _ in range(2):
            self.assertTrue(chairy.move_right(chamby))

        for _ in range(3):
            self.assertTrue(chairy.move_down(chamby))
        self.assertFalse(chairy.move_down(chamby))

        self.assertFalse(chairy.forms_cover(chamby))
        self.assertEqual(chamby.height(), 5)

        plusy = Plus()

        for _ in range(6):
            self.assertTrue(plusy.move_down(chamby))
        self.assertFalse(plusy.move_down(chamby))

        self.assertEqual(chamby.height(), 5)
        self.assertFalse(plusy.forms_cover(chamby))

        minusy = Minus()

        for _ in range(2):
            self.assertTrue(minusy.move_left(chamby))

        for _ in range(3):
            self.assertTrue(minusy.move_down(chamby))
        self.assertFalse(minusy.move_down(chamby))

        self.assertEqual(chamby.height(), 6)

        self.assertTrue(minusy.forms_cover(chamby))

def simulate(n: int) -> int:
    with open(sys.argv[1]) as puzzle:
        jets = to_jets(puzzle.read())
        jets_iter = zip(itertools.cycle(range(len(jets))), itertools.cycle(jets))

        rocks = [Minus, Plus, Chair, Pipe, Box]
        rocks_iter = zip(itertools.cycle(range(len(rocks))), itertools.cycle(rocks))

        chamber = Chamber(7)
        tower_height = 0

        states: dict[tuple[int, int, tuple[int, ...]], tuple[int, int]] = { }

        i = 0
        while i < n:
            rock_index, rock_constructor = next(rocks_iter)
            rock = rock_constructor()

            okay = True
            while okay:
                jet_index, jet = next(jets_iter)

                state = (rock_index, jet_index, chamber.topology())

                if state in states:
                    # Here we've detected a cycle! That is, at some point in the past before we
                    # dropped a rock of a particular shape, we were at position `jet_index` in our
                    # circular list of jets, and the topology of the chamber was something. Now,
                    # we see that same topology at that same `jet_index` before we drop a rock of
                    # the same shape! The jet cycle completely determines the tower, so we can
                    # can replay this cycle over and over.

                    # Why are we guaranteed that this simulation produces a cycle? I have no idea.
                    # I had to go to the subreddit looking for a hint to Part 2.

                    # https://emojied.net/😶😗😲😩😈😾

                    # How many rocks did we drop during the cycle?
                    # I think there should be a way to do this without storing `i`. Those indices
                    # get huge!
                    cycle_rocks = i - states[state][0]

                    cycle_height = chamber.height() - states[state][1]

                    cycles_remaining = (n - i) // cycle_rocks
                    tower_height += cycles_remaining * cycle_height
                    i += cycles_remaining * cycle_rocks

                states[state] = i, chamber.height()

                match jet:
                    case Jet.LEFT:
                        rock.move_left(chamber)
                    case Jet.RIGHT:
                        rock.move_right(chamber)
                okay = rock.move_down(chamber)

            # while rock.forms_cover(chamber):
            #     tower_height += 1
            #     chamber.rows.pop()

            # print(f'{((i + 1) / n) * 100 : 10.4} percent', end='')
            # print('\r', end='')

            i += 1

        tower_height += chamber.height()
        # print(chamber)
        return tower_height

if __name__ == '__main__':
    # Return[s ...] the maximum depth of the Python interpreter stack [...in bytes?].
    # import sys
    # print(sys.getrecursionlimit())

    # unittest.main()
    print(f'part one: {simulate(2022)}')
    print(f'part two: {simulate(1000000000000)}')
