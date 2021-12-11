from typing import Literal, Optional

Cell = tuple[int, int]
Direction = Literal['north', 'east', 'south', 'west']
Blizzards = dict[Cell, set[Direction]]

State = tuple[
    tuple[int, int], # Dimensions of the valley, i.e., (rows, columns).
    tuple[int, int], # Starting location of the expedition.
    tuple[int, int], # Target location of the expedition.
    int,             # Age of the expedition in minutes.
    Cell,            # The location of the expedition, i.e., (row, column).
]

def read_valley(text: str) -> tuple[tuple[int, int], Cell, Cell, Blizzards]:
    from collections import defaultdict
    from itertools import count

    lines = text.splitlines()
    rows, columns = len(lines), len(lines[0])

    blizzards: Blizzards = defaultdict(set)
    for row_offset, line in zip(count(1), lines[1 : -1]):
        for column_offset, cell in zip(count(1), line[1 : -1]):
            match cell:
                case '^':
                    direction: Direction = 'north'
                case '>':
                    direction = 'east'
                case 'v':
                    direction = 'south'
                case '<':
                    direction = 'west'
                case _:
                    continue
            blizzards[(row_offset, column_offset)].add(direction)

    return (rows, columns), (0, 1), (rows - 1, columns - 2), blizzards

def make_blizzard_states(size: tuple[int, int], blizzards: Blizzards) -> list[Blizzards]:
    # Consider this valley.

    #.###### -+
    #>>.<^<#  |- h = 6
    #.<..<<#  |
    #>v.><>#  |
    #<^v^^>#  |
    ######.# -+

    # Any of the north-south moving blizzards will return to their original position in
    # `height - 2` units of time. Any of the east-west blizzards will return to their original
    # position in `width - 2` units of time.

    # Above, `height - 2` is 4 and `width - 2` is 6. So,

    # +-----+-----+-----+ (north-south)
    # +---+---+---+---+   (east-west)
    # 0     6    12
    #    (time)

    # ...the states repeat themselves every 12 units of time.

    from math import lcm

    rows, columns = size
    number_of_states = lcm(rows - 2, columns - 2)

    states: list[Blizzards] = []
    for state in range(number_of_states): 
        states.append(blizzards)
        blizzards = step_blizzards(size, blizzards)

    return states

def step_blizzards(size: tuple[int, int], blizzards: Blizzards) -> Blizzards:
    # Assumes there are no north and south blizzards in column 1 and column 6 because they would
    # leave the valley.

    # TODO: On a train so I can't verify that on my sample input at the moment.

    from collections import defaultdict

    rows, columns = size

    next_blizzards: Blizzards = defaultdict(set)

    for (row, column), directions in blizzards.items():
        for direction in directions:
            match direction:
                case 'north': # Never
                    if row > 1:
                        next_cell = row - 1, column
                    else:
                        next_cell = rows - 2, column
                case 'east': # Eat
                    if column < columns - 2:
                        next_cell = row, column + 1
                    else:
                        next_cell = row, 1
                case 'south': # Soggy
                    if row < rows - 2:
                        next_cell = row + 1, column
                    else:
                        next_cell = 1, column
                case 'west': # Waffles
                    if column > 1:
                        next_cell = row, column - 1
                    else:
                        next_cell = row, columns - 2
            next_blizzards[next_cell].add(direction)

    return next_blizzards

def step_expedition(state: State, blizzard_states: list[Blizzards]) -> list[State]:
    size, start, goal, age, expedition = state
    rows, columns = size
    row, column = expedition
    blizzards = blizzard_states[(age + 1) % len(blizzard_states)]

    expeditions: list[Cell] = []

    start_row, start_column = start
    goal_row, goal_column = goal

    at_start = expedition == start
    at_goal = expedition == goal

    # Move back to the starting location of the expedition.
    above_start = (row, column) == (start_row - 1, start_column)
    below_start = (row, column) == (start_row + 1, start_column)

    # Move to the ending location of the expedition.
    above_goal = (row, column) == (goal_row - 1, goal_column)
    below_goal = (row, column) == (goal_row + 1, goal_column)

    if (row, column) not in blizzards:
        expeditions.append((row, column))
    if below_start or below_goal or row > 1 and (row - 1, column) not in blizzards: # Never
        expeditions.append((row - 1, column))
    if not at_start and not at_goal and column < columns - 2 and (row, column + 1) not in blizzards: # Eat
        expeditions.append((row, column + 1))
    if above_start or above_goal or row < rows - 2 and (row + 1, column) not in blizzards: # Soggy
        expeditions.append((row + 1, column))
    if not at_start and not at_goal and column > 1 and (row, column - 1) not in blizzards: # Waffles
        expeditions.append((row, column - 1))

    return [
        (size, start, goal, age + 1, expedition)
            for expedition in expeditions
    ]

def part_one() -> Optional[int]:
    from sys import argv

    with open(argv[1]) as puzzle_file:
        puzzle = puzzle_file.read()
        size, start, goal, blizzards = read_valley(puzzle)

        initial_state = size, start, goal, 0, start 
        blizzard_states = make_blizzard_states(size, blizzards)

        success = fastest_time_to(initial_state, blizzard_states)
        if success:
            time, _ = success
            return time

    return None

def part_two() -> Optional[int]:
    from sys import argv

    with open(argv[1]) as puzzle_file:
        # TODO: This mess assumes that the state of the first expedition to reach the goal is the
        # optimal state needed to turn around and reach the start in the shortest time, and so on.
        # This happens to be true of the sample input and my input, but is probably not true of all
        # inputs.

        # There could be multiple expeditions that reach the goal at the same earliest time, or a
        # later one whose state is more amenable to turning around.

        # From a Redditor: this is not true, because the first expedition to reach the goal can
        # always WAIT at the goal until the weather improves.

        puzzle = puzzle_file.read()
        size, start, goal, blizzards = read_valley(puzzle)

        initial_state = size, start, goal, 0, start 
        blizzard_states = make_blizzard_states(size, blizzards)

        success = fastest_time_to(initial_state, blizzard_states)
        if success:
            time, next_state = success
            size, start, goal, age, expedition = next_state

            success = fastest_time_to((size, goal, start, age, expedition), blizzard_states)
            if success:
                time, next_state = success
                size, start, goal, age, expedition = next_state

                success = fastest_time_to((size, goal, start, age, expedition), blizzard_states)
                if success:
                    time, _ = success
                    return time
    return None

def fastest_time_to(state: State, blizzard_states: list[Blizzards]) -> Optional[tuple[int, State]]:
        from collections import deque

        size, start, goal, age, expedition = state

        # An expedition always has the option of WAIT'ing in place. So, it never makes sense to
        # revisit a cell unless a blizzard forced the expedition to leave that cell.

        cycles: set[tuple[Cell, int]] = set([ (start, age) ])

        expeditions: deque[State] = deque([ state ])

        while len(expeditions) > 0:
            this_expedition = expeditions.popleft()
            # print(show_expedition(this_expedition, blizzard_states))
            _, _, goal, age, location = this_expedition
            if location == goal:
                return age, this_expedition
            branches = step_expedition(this_expedition, blizzard_states)
            for branch in branches:
                size, start, goal, age, location = branch
                if (location, age % len(blizzard_states)) not in cycles:
                    expeditions.append((size, start, goal, age, location))
                    cycles.add((location, age % len(blizzard_states)))

        return None

def show_expedition(state: State, blizzard_states: list[Blizzards]) -> str:
    (rows, columns), start, goal, age, expedition = state

    expedition_row, expedition_column = expedition
    start_row, start_column = start
    goal_row, goal_column = goal

    lines: list[list[str]] = [ [ '#' for _ in range(columns) ] for _ in range(rows) ]

    lines[start_row][start_column] = '.'
    lines[goal_row][goal_column] = '.'

    for row in range(1, rows - 1):
        for column in range(1, columns - 1):
            lines[row][column] = '.'

    lines[expedition_row][expedition_column] = 'E'

    for (row, column), directions in blizzard_states[age % len(blizzard_states)].items():
        if len(directions) > 1:
            lines[row][column] = str(len(directions))
        else:
            direction = directions.pop() # ❗️ Nasty mutability hack again.
            directions.add(direction)
            match direction:
                case 'north':
                    symbol = '^'
                case 'east':
                    symbol = '>'
                case 'south':
                    symbol = 'v'
                case 'west':
                    symbol = '<'
            lines[row][column] = symbol

    return f'minute {age}, E at ({expedition_row}, {expedition_column})\n' + '\n'.join(''.join(line) for line in lines)

def main() -> None:
    print(f'part one: {part_one()}')
    print(f'part two: {part_two()}')

if __name__ == '__main__':
    main()
