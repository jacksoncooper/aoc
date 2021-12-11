from __future__ import annotations

from typing import Callable, Optional

class File:
    def __init__(self, values: list[int]) -> None:
        where_is: dict[int, Value] = { }

        first = values[0]
        head = Value(first, 0)
        origin = head
        last = head

        where_is[0] = head

        for i in range(1, len(values)):
            value = values[i]
            next_last = Value(value, i, last, None)
            last.forward = next_last
            where_is[i] = next_last
            if value == 0:
                origin = next_last
            last = next_last

        last.forward = head
        head.backward = last

        self.origin = origin
        self.size = len(values)
        self.where_is = where_is

    def mix(self, subscript: int) -> None:
        to_mix = self.where_is[subscript]
        value = to_mix.value
        forward_offset = abs(value) % (self.size - 1)

        if forward_offset == 0:
            return

        advance: Callable[[Value], Optional[Value]] = (lambda v: v.forward) if value > 0 else (lambda v: v.backward)
        for _ in range(forward_offset):
            to_swap: Optional[Value] = advance(to_mix)
            assert to_mix is not None
            assert to_swap is not None
            if to_mix is self.origin:
                self.origin = to_swap
            elif to_swap is self.origin:
                self.origin = to_mix
            self.where_is[to_mix.sub], self.where_is[to_swap.sub] = to_swap, to_mix
            to_mix.value, to_swap.value = to_swap.value, to_mix.value
            to_mix.sub, to_swap.sub = to_swap.sub, to_mix.sub
            to_mix = to_swap

    def sub(self, subscript: int) -> int:
        origin = self.origin
        forward_offset = subscript
        head: Optional[Value] = origin
        for _ in range(forward_offset):
            assert head is not None
            head = head.forward
        assert head is not None
        return head.value

    def to_list(self) -> list[int]:
        values: list[int] = [ self.origin.value ]
        finger: Optional[Value] = self.origin.forward
        for _ in range(self.size - 1):
            assert finger is not None
            values.append(finger.value)
            finger = finger.forward
        return values

class Value:
    def __init__(self, value: int, sub: int, backward: Optional[Value] = None, forward: Optional[Value] = None) -> None:
        self.value = value
        self.sub = sub
        self.forward = forward
        self.backward = backward

def main() -> None:
    from sys import argv
    with open(argv[1]) as puzzle:
        values = [int(line) for line in puzzle.readlines()]
        print(f'part one: {part_one(values)}')
        print(f'part two: {part_two(values)}')

def part_one(values: list[int]) -> int:
    file = File(values)
    for i in range(len(values)):
        file.mix(i)
    return file.sub(1000) + file.sub(2000) + file.sub(3000)

def part_two(values: list[int]) -> int:
    decryption_key = 811589153
    file = File(list(map(lambda value: value * decryption_key, values)))
    for _ in range(10):
        for i in range(len(values)):
            file.mix(i)
    return file.sub(1000) + file.sub(2000) + file.sub(3000)

import unittest

class TestFile(unittest.TestCase):
    def test_new_file(self) -> None:
        sample_file = File([1, 2, -3, 3, -2, 0, 4])
        self.assertEqual(
            sample_file.to_list(),
            [0, 4, 1, 2, -3, 3, -2]
        )

        after_negative_3 = sample_file.where_is[2].forward
        assert after_negative_3 is not None
        self.assertEqual(
            after_negative_3.value,
            3
        )
        before_negative_3 = sample_file.where_is[2].backward
        assert before_negative_3 is not None
        self.assertEqual(
            before_negative_3.value,
            2
        )

        after_4 = sample_file.where_is[6].forward
        assert after_4 is not None
        self.assertEqual(
            after_4.value,
            1
        )
        before_4 = sample_file.where_is[6].backward
        assert before_4 is not None
        self.assertEqual(
            before_4.value,
            0
        )

        after_1 = sample_file.where_is[0].forward
        assert after_1 is not None
        self.assertEqual(
            after_1.value,
            2
        )
        before_1 = sample_file.where_is[0].backward
        assert before_1 is not None
        self.assertEqual(
            before_1.value,
            4
        )

    def test_sample_mix(self) -> None:
        sample_file = File([1, 2, -3, 3, -2, 0, 4])
        self.assertEqual(
            sample_file.to_list(),
            [0, 4, 1, 2, -3, 3, -2]
        #   [1, 2, -3, 3, -2, 0, 4]
        )

        sample_file.mix(0)
        self.assertEqual(
            sample_file.to_list(),
            [0, 4, 2, 1, -3, 3, -2]
        #   [2, 1, -3, 3, -2, 0, 4]
        )

        sample_file.mix(1)
        self.assertEqual(
            sample_file.to_list(),
            [0, 4, 1, -3, 2, 3, -2]
        #   [1, -3, 2, 3, -2, 0, 4]
        )

        sample_file.mix(2)
        self.assertEqual(
            sample_file.to_list(),
            [0, 4, 1, 2, 3, -2, -3]
        #   [1, 2, 3, -2, -3, 0, 4]
        )

        sample_file.mix(3)
        self.assertEqual(
            sample_file.to_list(),
            [0, 3, 4, 1, 2, -2, -3]
        #   [1, 2, -2, -3, 0, 3, 4]
        )

        sample_file.mix(4)
        self.assertEqual(
            sample_file.to_list(),
            [0, 3, 4, -2, 1, 2, -3]
        #   [1, 2, -3, 0, 3, 4, -2]
        )

        sample_file.mix(5)
        self.assertEqual(
            sample_file.to_list(),
            [0, 3, 4, -2, 1, 2, -3]
        #   [1, 2, -3, 0, 3, 4, -2]
        )

        sample_file.mix(6)
        self.assertEqual(
            sample_file.to_list(),
            [0, 3, -2, 1, 2, -3, 4]
        #   [1, 2, -3, 4, 0, 3, -2]
        )

if __name__ == '__main__':
    main()
