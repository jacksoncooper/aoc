def main() -> None:
    from sys import argv
    with open(argv[1]) as puzzle:
        text = puzzle.read()
        board, path = flat_board_and_path(text)
        row, column, face = walk_board(board, path)
        password = 1000 * row + 4 * column + face
        print(f'part one: {password}')
        board, path = board_and_path(text)
        row, column, face = walk_board(board, path)
        password = 1000 * row + 4 * column + face
        print(f'part two: {password}')

from typing import Callable, Optional

Board = dict[tuple[int, int], list[Optional[tuple[Callable[[int], int], tuple[int, int]]]]]
Path = tuple[list[int], list[str]]

identity: Callable[[int], int] = lambda face: face

def make_flat_board(text: str) -> Board:
    from collections import defaultdict

    board: Board = defaultdict(lambda: [None, None, None, None])

    rows = take_rows(text)
    columns = take_columns(text)

    # Add edges connecting the interior of the board.

    for row_offset in range(len(rows)):
        for column_offset in range(len(columns)):
            if rows[row_offset][column_offset] == '.':
                if column_offset < len(columns) - 1 and rows[row_offset][column_offset + 1] == '.':   # East
                    board[(row_offset, column_offset)][0] = identity, (row_offset, column_offset + 1)
                if row_offset < len(rows) - 1 and rows[row_offset + 1][column_offset] == '.':         # South
                    board[(row_offset, column_offset)][1] = identity, (row_offset + 1, column_offset)
                if column_offset > 0 and rows[row_offset][column_offset - 1] == '.':                  # West
                    board[(row_offset, column_offset)][2] = identity, (row_offset, column_offset - 1)
                if row_offset > 0 and rows[row_offset - 1][column_offset] == '.':                     # North
                    board[(row_offset, column_offset)][3] = identity, (row_offset - 1, column_offset)

    # Add edges connecting the edges of the board to their opposite side.
    # TODO: Clean this copy-paste up.

    for row_offset, row in enumerate(rows):
        lower_column, upper_column = row.index('.'), row.rindex('.')
        left_unobstructed = row.find('#') == -1 or row.find('#') > lower_column
        right_unobstructed = row.rfind('#') == -1 or row.rfind('#') < upper_column
        if left_unobstructed and right_unobstructed:
            board[(row_offset, lower_column)][2] = identity, (row_offset, upper_column)
            board[(row_offset, upper_column)][0] = identity, (row_offset, lower_column)

    for column_offset, column in enumerate(columns):
        lower_row, upper_row = column.index('.'), column.rindex('.')
        left_unobstructed = column.find('#') == -1 or column.find('#') > lower_row
        right_unobstructed = column.rfind('#') == -1 or column.rfind('#') < upper_row
        if left_unobstructed and right_unobstructed:
            board[(lower_row, column_offset)][3] = identity, (upper_row, column_offset)
            board[(upper_row, column_offset)][1] = identity, (lower_row, column_offset)

    # Adjust the subscripts to match the 1-indexed labeling in the puzzle statement.

    column_offsets: dict[int, int] = { }
    for row_offset, row in enumerate(rows):
        column_offsets[row_offset] = min(filter(lambda offset: offset != -1, (row.find('.'), row.find('#'))))

    adjusted_board: Board = defaultdict(lambda: [None, None, None, None])

    adjust: Callable[[tuple[int, int]], tuple[int, int]] = (
        lambda rc: (rc[0] + 1, rc[1] - (column_offsets[rc[0]] - 1)))

    for cell, neighbors in board.items():
        adjusted_board[adjust(cell)] = [
            (neighbor[0], adjust(neighbor[1])) if neighbor else None
                for neighbor in neighbors ]

    return adjusted_board

def make_board(text: str) -> Board:
    from collections import defaultdict

    board: Board = defaultdict(lambda: [None, None, None, None])

    rows = take_rows(text)
    columns = take_columns(text)

    # Add edges connecting the interior of the board.

    for row_offset in range(len(rows)):
        for column_offset in range(len(columns)):
            if rows[row_offset][column_offset] == '.':
                if column_offset < len(columns) - 1 and rows[row_offset][column_offset + 1] == '.':   # East
                    board[(row_offset, column_offset)][0] = identity, (row_offset, column_offset + 1)
                if row_offset < len(rows) - 1 and rows[row_offset + 1][column_offset] == '.':         # South
                    board[(row_offset, column_offset)][1] = identity, (row_offset + 1, column_offset)
                if column_offset > 0 and rows[row_offset][column_offset - 1] == '.':                  # West
                    board[(row_offset, column_offset)][2] = identity, (row_offset, column_offset - 1)
                if row_offset > 0 and rows[row_offset - 1][column_offset] == '.':                     # North
                    board[(row_offset, column_offset)][3] = identity, (row_offset - 1, column_offset)

    # Add edges connecting the edges of the board to their opposite side.
    # TODO: We're hardcoding this because Anshuman says the inputs have the same 'origami'.
    # In other words, they all the same shape -- we don't have to derive a general solution for
    # folding a cube that involves connecting adjacent sides.

    # ⬜️🟫🟫  _21
    # ⬜️🟫    _3
    # 🟫🟫    54
    # 🟫      6

    side_width = len(rows[-1].strip())

    for i in range(side_width):
        # Top edge of (2) to left edge of (6).
        two_r, two_c = 0, side_width + i
        six_r, six_c = 3 * side_width + i, 0
        if rows[0][side_width + i] != '#' and rows[3 * side_width + i][0] != '#':
            board[(two_r, two_c)][3] = lambda _: 0, (six_r, six_c)
            board[(six_r, six_c)][2] = lambda _: 1, (two_r, two_c)

        # Left edge of (2) to left edge of (5).
        two_r, two_c = i, side_width
        five_r, five_c = 2 * side_width + (side_width - 1 - i), 0
        if rows[two_r][two_c] != '#' and rows[five_r][five_c] != '#':
            board[(two_r, two_c)][2] = lambda _: 0, (five_r, five_c)
            board[(five_r, five_c)][2] = lambda _: 0, (two_r, two_c)

        # Left edge of (3) to top edge of (5).
        three_r, three_c = side_width + i, side_width
        five_r, five_c = 2 * side_width, i
        if rows[three_r][three_c] != '#' and rows[five_r][five_c] != '#':
            board[(three_r, three_c)][2] = lambda _: 1, (five_r, five_c)
            board[(five_r, five_c)][3] = lambda _: 0, (three_r, three_c)

        # Top edge of (1) to bottom edge of (6).
        one_r, one_c = 0, 2 * side_width + i
        six_r, six_c = 4 * side_width - 1, i
        if rows[one_r][one_c] != '#' and rows[six_r][six_c] != '#':
            board[(one_r, one_c)][3] = lambda _: 3, (six_r, six_c)
            board[(six_r, six_c)][1] = lambda _: 1, (one_r, one_c)

        # Right edge of (1) to right edge of (4).
        one_r, one_c = i, 3 * side_width - 1
        four_r, four_c = 2 * side_width + (side_width - 1 - i), 2 * side_width - 1
        if rows[one_r][one_c] != '#' and rows[four_r][four_c] != '#':
            board[(one_r, one_c)][0] = lambda _: 2, (four_r, four_c)
            board[(four_r, four_c)][0] = lambda _: 2, (one_r, one_c)

        # Bottom edge of (1) to right edge of (3).
        one_r, one_c = side_width - 1, 2 * side_width + i
        three_r, three_c = side_width + i, 2 * side_width - 1
        if rows[one_r][one_c] != '#' and rows[three_r][three_c] != '#':
            board[(one_r, one_c)][1] = lambda _: 2, (three_r, three_c)
            board[(three_r, three_c)][0] = lambda _: 3, (one_r, one_c)

        # Bottom edge of (4) to right edge of (6).
        four_r, four_c = 3 * side_width - 1, side_width + i
        six_r, six_c = 3 * side_width + i, side_width - 1
        if rows[four_r][four_c] != '#' and rows[six_r][six_c] != '#':
            board[(four_r, four_c)][1] = lambda _: 2, (six_r, six_c)
            board[(six_r, six_c)][0] = lambda _: 3, (four_r, four_c)

    # Adjust the subscripts to match the 1-indexed labeling in the puzzle statement.

    column_offsets: dict[int, int] = { }
    for row_offset, row in enumerate(rows):
        column_offsets[row_offset] = min(filter(lambda offset: offset != -1, (row.find('.'), row.find('#'))))

    adjusted_board: Board = defaultdict(lambda: [None, None, None, None])

    adjust: Callable[[tuple[int, int]], tuple[int, int]] = (
        lambda rc: (rc[0] + 1, rc[1] - (column_offsets[rc[0]] - 1)))

    for cell, neighbors in board.items():
        adjusted_board[adjust(cell)] = [
            (neighbor[0], adjust(neighbor[1])) if neighbor else None
                for neighbor in neighbors ]

    return adjusted_board

def make_path(text: str) -> Path:
    # TODO: Heinous. Invariant: `l` and `l_prime` form an exclusive interval containing an integer
    # after the inner `while` statement.
    l, l_prime = 0, 1
    motions: list[int] = []
    turns: list[str] = []
    while l_prime < len(text):
        while l_prime < len(text) and not text[l_prime].isalpha(): l_prime += 1
        motions.append(int(text[l : l_prime]))
        if l_prime < len(text):
            turns.append(text[l_prime])
        l = l_prime + 1
        l_prime = l + 1
    return motions, turns

def walk_board(board: Board, path: Path) -> tuple[int, int, int]:
    #       ---East----South---West-----North---- right turn -->
    faces = [ (0, 1), (1, 0), (0, -1), (-1, 0) ]
    face = 0

    def left_turn() -> int:
        nonlocal face
        face = (face - 1) % len(faces)
        return face

    def right_turn() -> int:
        nonlocal face
        face = (face + 1) % len(faces)
        return face

    row, column = (1, 1)
    row_direction, column_direction = faces[face]
    motions, turns = path

    for motion, turn in zip(motions, turns):
        for _ in range(motion):
            step = board[(row, column)][face]
            if step:
                row, column = step[1]
                face = step[0](face)
            else:
                break
        face = right_turn() if turn == 'R' else left_turn()

    last_motion = motions[-1]
    for _ in range(last_motion):
        step = board[(row, column)][face]
        if step:
            row, column = step[1]
            face = step[0](face)
        else:
            break

    return row, column, face

def take_rows(text: str) -> list[str]:
    from typing import Callable
    rows = text.splitlines()
    number_of_columns = max(len(row) for row in rows)
    implicit: Callable[[str], int] = lambda row: number_of_columns - len(row)
    return [ row if implicit(row) < 1 else row + ' ' * implicit(row) for row in rows ]

def take_columns(text: str) -> list[str]:
    rows = text.splitlines()
    number_of_columns = max(len(row) for row in rows)
    return [ ''.join([ rows[r][c] if c < len(rows[r]) else ' ' for r in range(len(rows)) ])
        for c in range(number_of_columns) ]

def flat_board_and_path(text: str) -> tuple[Board, Path]:
    board, path = text.split('\n\n')
    return make_flat_board(board), make_path(path)

def board_and_path(text: str) -> tuple[Board, Path]:
    board, path = text.split('\n\n')
    return make_board(board), make_path(path)

if __name__ == '__main__':
    main()
