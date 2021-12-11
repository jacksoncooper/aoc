import unittest

from typing import Optional
from typing import cast

def snafu_digit_to_decimal(digit: str) -> Optional[int]:
    match digit:
        case '=': return -2
        case '-': return -1
        case '0': return  0
        case '1': return  1
        case '2': return  2
        case  _ : return None

def decimal_digit_to_snafu(digit: int) -> Optional[str]:
    match digit:
        case -2: return '='
        case -1: return '-'
        case  0: return '0'
        case  1: return '1'
        case  2: return '2'
        case _: return None

def snafu_to_decimal(snafu: str) -> int:
    decimal = 0

    for place, digit in enumerate(reversed(snafu)):
        digit_as_decimal = snafu_digit_to_decimal(digit)
        assert digit_as_decimal is not None
        decimal += pow(5, place) * digit_as_decimal

    return decimal

def decimal_to_snafu(decimal: int) -> str:
    from math import floor, log

    # We can have at most two times any power of five in a single digit.

    # e.g., 2022_10 = 31042_5
    #       3210      43210

    # 2022_10
    #     = 3 * pow(5, 4) + 1 * pow(5, 3) + 0 * pow(5, 2) + 4 * pow(5, 1) + 2 * pow(5, 0)
    #     = 3 * pow(5, 4) + ...
    #     This doesn't work, can't rid ourselves of larger powers of 5 using smaller powers.
    #     = 2 * pow(5, 4) + 5 * pow(5, 3) + ...
    #     This does work.
    #     = 3 * pow(5, 4) + ...
    #     = pow(5, 5) - 5 * pow(5, 4) + 3 * pow(5, 4) + ...
    #     = pow(5, 5) - 2 * pow(5, 4) + ...

    def go(power: int, decimal: int) -> tuple[int, list[int]]:
        if (power < 0):
            return False, []

        digit = floor(decimal / pow(5, power))

        carry, rest = go(power - 1, decimal - digit * pow(5, power))

        if digit > 2:
            # `digit` is 3 or 4. Add `pow(5, power + 1)` to compensate.

            # `digit` - 5` is:
            #  -2 if `digit` is 3.
            #  -1 if `digit` is 4.

            return True, [digit - 5 + carry] + rest
        else:
            # `digit` is 0, 1, or 2.
          
            # 3 * pow(5, p)
            # = pow(5, p + 1) - 5 * pow(5, p) + 3 * pow(5, p)
            # = pow(5, p + 1) - 2 * pow(5, p)

            if digit > 1 and carry == 1:
                return True, [-2] + rest
                
            return False, [digit + carry] + rest

    # TODO: Yuck.
    if decimal <= 0:
        return '0'

    largest_power = floor(log(decimal, 5))
    carry, rest = go(largest_power, decimal)

    digits = rest
    if carry == 1:
        digits = [ 1 ] + rest

    readable_digits = list(map(decimal_digit_to_snafu, digits))

    for digit in readable_digits:
        assert digit is not None

    return ''.join(cast(list[str], readable_digits))

'''
  Decimal          SNAFU
        1              1
        2              2
        3             1=
        4             1-
        5             10
        6             11
        7             12
        8             2=
        9             2-
       10             20
       15            1=0
       20            1-0
     2022         1=11-2
    12345        1-0---0
314159265  1121-1110-1=0
'''

class TestBob(unittest.TestCase):
    def test_to_decimal(self) -> None:
        self.assertEqual(snafu_to_decimal('0'),  0)
        self.assertEqual(snafu_to_decimal('1'),  1)
        self.assertEqual(snafu_to_decimal('2'),  2)
        self.assertEqual(snafu_to_decimal('1='), 3)
        self.assertEqual(snafu_to_decimal('1-'), 4)
        self.assertEqual(snafu_to_decimal('10'), 5)

        self.assertEqual(snafu_to_decimal('1121-1110-1=0'), 314159265)

    def test_to_snafu(self) -> None:
        self.assertEqual(decimal_to_snafu(0), '0')
        self.assertEqual(decimal_to_snafu(1), '1')
        self.assertEqual(decimal_to_snafu(2), '2')
        self.assertEqual(decimal_to_snafu(3), '1=')
        self.assertEqual(decimal_to_snafu(4), '1-')
        self.assertEqual(decimal_to_snafu(5), '10')
        self.assertEqual(decimal_to_snafu(6), '11')
        self.assertEqual(decimal_to_snafu(7), '12')
        self.assertEqual(decimal_to_snafu(8), '2=')

        self.assertEqual(decimal_to_snafu(2022), '1=11-2')
        self.assertEqual(decimal_to_snafu(314159265), '1121-1110-1=0')

if __name__ == '__main__':
    # unittest.main()

    from sys import argv
    # Part one.
    with open(argv[1]) as puzzle_file:
        fuel_requirements: list[int] = []
        for side_of_burner in puzzle_file:
            fuel_requirements.append(snafu_to_decimal(side_of_burner.strip()))
        print(f'part one: {decimal_to_snafu(sum(fuel_requirements))}')

