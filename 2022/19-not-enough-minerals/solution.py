# Our pack contains:
#  - One ore-collecting robot.
#  - One robot factory, which requires one minute to produce a robot given the required resources.
#
# Each robot can collect one of its resource type per minute.
#  - For example, a geode-cracking robot can crack one geode per minute.
#
# Goal: Maximize the number of open geodes for the elephants. A geode robot cracks geodes.
#
# Robot types:
#   1. Ore-collecting robot.
#   2. Clay-collecting robot.
#   3. Obsidian-collecting robot.
#   4. Geode-opening robot.

from enum import IntEnum
from functools import reduce

from typing import cast
from typing import Optional

class Resource(IntEnum):
    ORE = 0
    CLAY = 1
    OBSIDIAN = 2
    GEODE = 3

class Quantity:
    def __init__(self, amount: int, resource: Resource) -> None:
        self.resource = resource
        self.amount = amount

    def __eq__(self, other: object) -> bool:
        if isinstance(other, Quantity):
            return (
                self.amount == other.amount and
                self.resource == other.resource
            )
        return False

class Robot(IntEnum):
    ORE = 0
    CLAY = 1
    OBSIDIAN = 2
    GEODE = 3

Resources = list[int]
Robots = list[int]
Blueprint = dict[Robot, list[Quantity]]

class State:
    def __init__(self, resources: Resources, robots: Robots, minutes: int) -> None:
        self.resources = resources
        self.robots = robots
        self.minutes = minutes

    def __str__(self) -> str:
        return f'''minutes remaining: {self.minutes}
    resources: ore={self.resources[Resource.ORE]}; clay={self.resources[Resource.CLAY]}; obsidian={self.resources[Resource.OBSIDIAN]}; geodes={self.resources[Resource.GEODE]}
    robots: ore={self.robots[Robot.ORE]}; clay={self.robots[Robot.CLAY]}; obsidian={self.robots[Robot.OBSIDIAN]}; geodes={self.robots[Robot.GEODE]}'''

    def __hash__(self) -> int:
        return hash((
            self.resources[Resource.ORE], self.resources[Resource.CLAY],
            self.resources[Resource.OBSIDIAN], self.resources[Resource.GEODE],
            self.robots[Robot.ORE], self.robots[Robot.CLAY], self.robots[Robot.OBSIDIAN],
            self.robots[Robot.GEODE], self.minutes
        ))

def read_resource(text: str) -> Resource:
    match text:
        case 'ore':
            return Resource.ORE
        case 'clay':
            return Resource.CLAY
        case 'obsidian':
            return Resource.OBSIDIAN
        case _:
            raise RuntimeError(f"expected resource but got '{text}'")

def read_quantities(text: str) -> list[Quantity]:
    costs = []
    for clause in text.split(' and '):
        match clause.split(' '):
            case [amount, resource]:
                costs.append(Quantity(int(amount), read_resource(resource)))
            case _:
                raise RuntimeError('expect amount then resource')
    return costs

def read_blueprint(text: str) -> Blueprint:
    # Blueprint 1:
    #   Each ore robot costs 4 ore.
    #   Each clay robot costs 2 ore.
    #   Each obsidian robot costs 3 ore and 14 clay.
    #   Each geode robot costs 2 ore and 7 obsidian.
    #
    # (Formatted across multiple lines for legibility.)

    colon = text.find(': ')
    text = text[colon + 2 :]
    text = text.removesuffix('.')
    match text.split('. '):
        case [ore, clay, obsidian, geode]:
            blueprint = {}
            blueprint[Robot.ORE]      = read_quantities(ore.removeprefix('Each ore robot costs '))
            blueprint[Robot.CLAY]     = read_quantities(clay.removeprefix('Each clay robot costs '))
            blueprint[Robot.OBSIDIAN] = read_quantities(obsidian.removeprefix('Each obsidian robot costs '))
            blueprint[Robot.GEODE]    = read_quantities(geode.removeprefix('Each geode robot costs '))
            return blueprint
        case _:
            raise RuntimeError('expected 4 recipes')

def time_to_manufacture(blueprint: Blueprint, resources: Resources, robots: Robots, robot: Robot) -> int:
        if all(map(lambda q: resources[q.resource] >= q.amount, blueprint[robot])):
            return 1

        # We do not have enough of at least one item to manufacture the robot.
        from math import ceil
        return 1 + max(map(
            lambda q: ceil((q.amount - resources[q.resource]) / robots[robot_makes_resource(q.resource)]),
            blueprint[robot]
        ))

def robot_makes_resource(resource: Resource) -> Robot:
    match resource:
        case Resource.ORE:
            return Robot.ORE
        case Resource.CLAY:
            return Robot.CLAY
        case Resource.OBSIDIAN:
            return Robot.OBSIDIAN
        case Resource.GEODE:
            return Robot.GEODE

def resource_from_robot(robot: Robot) -> Optional[Resource]:
    match robot:
        case Robot.ORE:
            return Resource.ORE
        case Robot.CLAY:
            return Resource.CLAY
        case Robot.OBSIDIAN:
            return Resource.OBSIDIAN
        case Robot.GEODE:
            return None

def make_robot(blueprint: Blueprint, state: State, robot: Robot) -> Optional[State]:
    from copy import deepcopy

    # If we can't produce a resource, we can't produce this robot. Skip it.
    if any(map(lambda q: state.robots[robot_makes_resource(q.resource)] == 0, blueprint[robot])):
        return None

    # We take advantage of the robot factory. We know we must manufacturer some robot next until
    # we run out of time. Not using the factory for a minute is wasteful and clearly not
    # optimal.
    to_manufacture = time_to_manufacture(blueprint, state.resources, state.robots, robot)

    # If we don't have enough time to make this robot, skip it and make a different robot.
    if to_manufacture > state.minutes:
            return None
    # o:
    next_state = deepcopy(state)

    # Each robot takes one minute to collect its resource.
    next_state.resources[Resource.ORE] += to_manufacture * state.robots[Robot.ORE]
    next_state.resources[Resource.CLAY] += to_manufacture * state.robots[Robot.CLAY]
    next_state.resources[Resource.OBSIDIAN] += to_manufacture * state.robots[Robot.OBSIDIAN]
    next_state.resources[Resource.GEODE] += to_manufacture * state.robots[Robot.GEODE]

    # Subtract off the cost of manufacturing the robot.
    for q in blueprint[robot]:
        next_state.resources[q.resource] -= q.amount
    next_state.robots[robot] += 1

    next_state.minutes -= to_manufacture

    return next_state

def sufficient_resource(blueprint: Blueprint, state: State, resource: Resource) -> bool:
    most_needed: Optional[int] = None
    for quantities in blueprint.values():
        largest_amount = min(map(lambda q: q.amount, filter(lambda q: q.resource == resource, quantities)), default=None)
        options = (q for q in [most_needed, largest_amount] if q is not None)
        most_needed = max(options, default=None)

    # This resource is not required to make robots.
    if most_needed is None:
        return True

    # We have enough robots to make X of this resource per minute.
    # Any robot requires at most X of this resource to build.
    # We can only build one robot per minute.
    # So, it does not make sense to build more robots that produce this resource.

    return state.robots[robot_makes_resource(resource)] >= most_needed

def most_geodes_with_blueprint(blueprint: Blueprint, state: State) -> tuple[int, list[Robot]]:
    global_best: int = 0

    def go(blueprint: Blueprint, state: State) -> tuple[int, list[Robot]]:
        nonlocal global_best

        # u/boojum from Reddit pruned the search space by tracking the optimal solution found so far
        # and comparing it with the number of geodes we can produce in the best case.

        # In other words, with `g` geode robots and `n` minutes remaining, it's
        #     g + (g + 1) + (g + 2) + ... + (g + n - 1)
        # <-> n * g + (1 + 2 + ... + n - 1)
        # <-> g * n + ((n - 1) n) / 2

        # https://emojied.net/😡😟😾😿😀😤

        # My earlier attempt at pruning when we have enough of a resource to satisfy building the
        # most expensive robot for each remaining minute did nearly nothing, too loose.

        # Lesson learned: upper bounds on an entire solution work really well.

        upper_bound = (
            state.resources[Resource.GEODE]
            + state.robots[Robot.GEODE] * state.minutes
            + ((state.minutes - 1) * state.minutes) // 2
        )

        if upper_bound <= global_best:
            return global_best, []

        skipped: int = 0
        geodes: list[tuple[int, list[Robot]]] = []

        for robot in Robot.ORE, Robot.CLAY, Robot.OBSIDIAN, Robot.GEODE:
            if robot != Robot.GEODE and sufficient_resource(blueprint, state, cast(Resource, resource_from_robot(robot))):
                skipped += 1
            else:
                next_state = make_robot(blueprint, state, robot);
                if next_state is None:
                    skipped += 1
                else:
                    best, robots = go(blueprint, next_state)
                    geodes.append((best, [robot] + robots))

        if skipped == 4:
            make_geodes = state.resources[Resource.GEODE] + state.minutes * state.robots[Robot.GEODE]
            global_best = max(global_best, make_geodes)
            return make_geodes, []
        else:
            best, choices = max(geodes, key = lambda pair: pair[0])
            global_best = max(global_best, best)
            return best, choices

    return go(blueprint, state)

import unittest

class TestReading(unittest.TestCase):
    def test_read_quantities(self) -> None:
        self.assertEqual(
            read_quantities('3 ore and 4 clay'),
            [Quantity(3, Resource.ORE), Quantity(4, Resource.CLAY)]
        )

    def test_read_blueprint(self) -> None:
        blueprint = read_blueprint('Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.')
        self.assertTrue(len(blueprint) == 4)
        self.assertEqual(blueprint[Robot.ORE], [Quantity(4, Resource.ORE)])
        self.assertEqual(blueprint[Robot.CLAY], [Quantity(2, Resource.ORE)])
        self.assertEqual(blueprint[Robot.OBSIDIAN], [Quantity(3, Resource.ORE), Quantity(14, Resource.CLAY)])
        self.assertEqual(blueprint[Robot.GEODE], [Quantity(2, Resource.ORE), Quantity(7, Resource.OBSIDIAN)])

def main() -> None:
    from sys import argv

    if len(argv) == 2:
        with open(argv[1]) as puzzle_file:
            puzzle = puzzle_file.read().splitlines()
            blueprints = list(map(read_blueprint, puzzle))

            # Part 1
            quality = 0
            for id, blueprint in enumerate(blueprints):
                resources = [0, 0, 0, 0]
                robots = [1, 0, 0, 0]
                solution = most_geodes_with_blueprint(blueprint, State(resources, robots, 24))
                quality += (id + 1) * solution[0]

            print(f'part one: {quality}')

            # Part 2
            geodes: list[int] = []
            for id, blueprint in enumerate(list(blueprints)[:3]):
                resources = [0, 0, 0, 0]
                robots = [1, 0, 0, 0]
                solution = most_geodes_with_blueprint(blueprint, State(resources, robots, 32))
                geodes.append(solution[0])

            print(f'part two: {reduce(lambda r, l: r * l, geodes, 1)}')
    else:
        unittest.main()

if __name__ == '__main__':
    main()
