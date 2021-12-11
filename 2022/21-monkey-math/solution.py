'''
root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32
'''

import itertools
from typing import Callable, Optional, cast

class Monkey:
    def __init__(self, name: str):
        self.name = name

class BinaryMonkey(Monkey):
    def __init__(self, name: str, left: str, op_infix: str, op: Callable[[float, float], float], right: str) -> None:
        super().__init__(name)
        self.left = left
        self.right = right
        self.op_infix = op_infix
        self.op = op

    def __str__(self) -> str:
        return f'{self.name}: {self.left} {self.op_infix} {self.right}'

class ValueMonkey(Monkey):
    def __init__(self, name: str, value: int) -> None:
        super().__init__(name)
        self.value = value

    def __str__(self) -> str:
        return f'{self.name}: {self.value}'

def to_op(op: str) -> Callable[[float, float], float]:
    match op:
        case '+':
            return lambda l, r: l + r
        case '-':
            return lambda l, r: l - r
        case '*':
            return lambda l, r: l * r
        case '/':
            return lambda l, r: l / r
        case _:
            raise ValueError(f"unexpected operator '{op}'")

def read_monkeys(text: str) -> dict[str, Monkey]:
    monkeys: dict[str, Monkey] = { }
    for line in text.splitlines():
        name, rest = line.split(': ')
        terms = rest.split()
        monkey: Optional[Monkey] = None
        if len(terms) > 1:
            left, op, right = terms
            monkey = BinaryMonkey(name, left, op, to_op(op), right)
        else:
            value, = terms
            monkey = ValueMonkey(name, int(value))
        monkeys[name] = monkey
    return monkeys

def evaluate(monkeys: dict[str, Monkey], name: str) -> float:
    monkey = monkeys[name]

    if isinstance(monkey, ValueMonkey):
        return monkey.value
   
    if isinstance(monkey, BinaryMonkey):
        return monkey.op(evaluate(monkeys, monkey.left), evaluate(monkeys, monkey.right))

    raise ValueError(f"unknown monkey '{monkey.__class__}'")

def find_humn(monkeys: dict[str, Monkey], name: str) -> Optional[list[str]]:
    monkey = monkeys[name]

    if isinstance(monkey, ValueMonkey):
        if monkey.name == 'humn':
            return [ 'humn' ]
        return None
   
    if isinstance(monkey, BinaryMonkey):
        look_left = find_humn(monkeys, monkey.left)
        look_right = find_humn(monkeys, monkey.right)
        if look_left is None and look_right is None:
            return None
        if look_left:
            return [ monkey.name ] + look_left
        assert look_right is not None
        return [ monkey.name ] + look_right

    raise ValueError(f"unknown monkey '{monkey.__class__}'")

def show_equation(monkeys: dict[str, Monkey], name: str) -> str:
    monkey = monkeys[name]

    if isinstance(monkey, ValueMonkey):
        if monkey.name == 'humn':
            return 'h'
        else:
            return str(monkey.value)

    if isinstance(monkey, BinaryMonkey):
        return f'({show_equation(monkeys, monkey.left)} {monkey.op_infix} {show_equation(monkeys, monkey.right)})'

    raise ValueError(f"unknown monkey '{monkey.__class__}'")

def main() -> None:
    from sys import argv
    with open(argv[1]) as puzzle:
        monkeys = read_monkeys(puzzle.read())

        # Part 1
        print(f"part one: {evaluate(monkeys, 'root')}")

        # Part 2
        root = cast(BinaryMonkey, monkeys['root'])

        path_to_humn = find_humn(monkeys, 'root')
        assert path_to_humn is not None

        first_step = path_to_humn[1]

        if first_step == root.left:
            child_with_humn = root.left
            child_with_no_humn = root.right
        else:
            child_with_humn = root.right
            child_with_no_humn = root.left

        equality_target: float = evaluate(monkeys, child_with_no_humn)

        for depth, name in zip(itertools.count(1), path_to_humn[1 : len(path_to_humn) - 1]):
            monkey = cast(BinaryMonkey, monkeys[name])

            next_step = path_to_humn[depth + 1]

            if next_step == monkey.left:
                child_with_humn = monkey.left
                child_with_no_humn = monkey.right
            else:
                child_with_humn = monkey.right
                child_with_no_humn = monkey.left

            known_operand = evaluate(monkeys, child_with_no_humn)

            match monkey.op_infix:
                case '+':
                    equality_target -= known_operand
                case '-':
                    if monkey.right == child_with_humn:
                        # We are solving for `monkey.right`.
                        equality_target = known_operand - equality_target
                    else:
                        # We are solving for `monkey.left`.
                        equality_target += known_operand
                case '*':
                    equality_target /= known_operand
                case '/':
                    if monkey.right == child_with_humn:
                        # We are solving for `monkey.right`.
                        equality_target = known_operand / equality_target
                    else:
                        # We are solving for `monkey.left`.
                        equality_target *= known_operand
                case _:
                    raise ValueError(f"unexpected operator '{monkey.op_infix}'")

        root.op = to_op('-')
        root.op_infix = '-'
        
        humn = cast(ValueMonkey, monkeys['humn'])
        
        humn.value = round(equality_target)

        print(f'part two: {equality_target}')
        print(f"part two check (should be 0): {evaluate(monkeys, 'root')}")

if __name__ == '__main__':
    main()
