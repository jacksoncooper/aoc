from typing import Literal, Optional

Elf = tuple[int, int]
Elves = set[Elf]
Movement = Literal['north', 'south', 'west', 'east']
Movements = list[Movement]

def main() -> None:
    from sys import argv
    from collections import deque
    from itertools import count

    with open(argv[1]) as puzzle_file:
        grove = puzzle_file.read()

        elves = read_elves(grove)
        movements: deque[Movement] = deque(['north', 'south', 'west', 'east'])

        for _ in range(10):
            simulate_round(elves, list(movements))
            movements.rotate(-1)
        print(f'part one: {available_soil(elves)}')

        elves = read_elves(grove)
        movements = deque(['north', 'south', 'west', 'east'])

        for round in count(1):
            diffused = simulate_round(elves, list(movements))
            if diffused is None:
                break
            movements.rotate(-1)

        print(f'part two: {round}')

def read_elves(grove: str) -> set[tuple[int, int]]:
    elves: set[tuple[int, int]] = set()
    for row_offset, row in enumerate(grove.splitlines()):
        for column_offset, column in enumerate(row):
            if row[column_offset] == '#':
                elves.add((row_offset, column_offset))
    return elves

def valid_movement(elves: Elves, elf: Elf, movement: Movement) -> bool:
    from itertools import chain

    north_vacancies = [ (-1, -1), (-1,  0), (-1,  1) ]
    south_vacancies = [ ( 1, -1), ( 1,  0), ( 1,  1) ]
    west_vacancies  = [ (-1, -1), ( 0, -1), ( 1, -1) ]
    east_vacancies  = [ ( -1, 1), ( 0,  1), ( 1,  1) ]

    stay_put = True
    for vacancy in chain(north_vacancies, south_vacancies, west_vacancies, east_vacancies):
        row, column = elf
        d_row, d_column = vacancy
        if (row + d_row, column + d_column) in elves:
            stay_put = False

    if stay_put:
        return False

    match movement:
        case 'north':
            vacancies = north_vacancies
        case 'south':
            vacancies = south_vacancies
        case 'west':
            vacancies = west_vacancies
        case 'east':
            vacancies = east_vacancies

    for vacancy in vacancies:
        row, column = elf
        d_row, d_column = vacancy
        if (row + d_row, column + d_column) in elves:
            return False

    return True

def movement_to_displacement(movement: Movement) -> tuple[int, int]:
    match movement:
        case 'north':
            return -1, 0
        case 'south':
            return 1, 0
        case 'west':
            return 0, -1
        case 'east':
            return 0, 1

def propose_movements(elves: Elves, movements: Movements) -> Optional[dict[tuple[int, int], list[Elf]]]:
    from collections import defaultdict

    # A mapping from a cell in the grove to the elves that have proposed to move there.
    proposed: defaultdict[tuple[int, int], list[Elf]] = defaultdict(list)

    for elf in elves:
        for movement in movements:
            if valid_movement(elves, elf, movement):
                row, column = elf
                d_row, d_column = movement_to_displacement(movement)
                destination = row + d_row, column + d_column
                proposed[destination].append(elf)
                break

    if len(proposed) < 1:
        return None

    return proposed

def simulate_round(elves: Elves, movements: Movements) -> Optional[Elves]:
    # First half of round: the elves propose a movement.
    proposed = propose_movements(elves, movements)

    # All elves have settled on a place to plant their star fruit.
    if proposed is None:
        return None

    for cell, waiting_elves in proposed.items():
        # Only one elf proposes to move to this cell, so move them. There is no conflict.
        if len(waiting_elves) == 1:
            elf = waiting_elves.pop() # ❗️ Danger, mutates `proposed.`
            elves.remove(elf)
            elves.add(cell)

    return elves

def frontier(elves: Elves) -> tuple[tuple[int, int], tuple[int, int]]:
    point = elves.pop()
    elves.add(point)    # ❗️ Forgive me this is ugly.

    row, column = point

    min_row, max_row = row, row
    min_column, max_column = column, column

    for row, column in elves:
        min_row = min(min_row, row)
        max_row = max(max_row, row)
        min_column = min(min_column, column)
        max_column = max(max_column, column)

    return (min_row, max_row), (min_column, max_column)

def show_grove(elves: Elves) -> str:
    rows: list[str] = []
    (min_row, max_row), (min_column, max_column) = frontier(elves)
    for row_offset in range(min_row, max_row + 1):
        row: list[str] = []
        for column_offset in range(min_column, max_column + 1):
            if (row_offset, column_offset) in elves:
                row.append('#')
            else:
                row.append('.')
        rows.append(''.join(row))
    return '\n'.join(rows)

def available_soil(elves: Elves) -> int:
    (min_row, max_row), (min_column, max_column) = frontier(elves)
    height = max_row - min_row + 1
    width = max_column - min_column + 1
    return height * width - len(elves)

if __name__ == '__main__':
    main()
