import * as fs from "node:fs";
import * as process from "node:process";
import assert from "node:assert";

type Color = 'red' | 'green' | 'blue';

type Subset = {
    amount: number,
    color: Color,
};

// This type looks insane, so in my defense: In each handful, the colors are
// listed in no particular order. This irregularity makes me concerned that
// their order is relevant in Part II.
type Handful = Subset[];

type Game = {
    id: number,
    handfuls: Handful[],
};

function parseHandful(text: string) : Subset[] {
    // `matchAll` seems to restart the DFA when it enters an accepting state on
    // the unconsumed input. This is best done using `String.split` but I'm
    // just playing around.
    const language = /(?<amount>\d+) (?<color>red|green|blue)(?:, )?/g;

    let subsets: Subset[] = [];
    for (const match of text.matchAll(language)) {
        assert(match.groups !== undefined);
        const groups = match.groups;
        subsets.push({
            amount: Number(groups["amount"]),
            color: groups["color"] as Color,
        });
    }
    return subsets;
}

function parseLine(line: string) : Game {
    const language = /Game (?<id>\d+): (?<handfuls>.+)/;

    const match = line.match(language);
    assert(match);
    assert(match.groups !== undefined);
    const groups = match.groups;

    return {
        id: Number(groups["id"]),
        handfuls: groups["handfuls"].split(/; /).map(parseHandful),
    };
}

function isImpossible(subset: Subset) : boolean {
    return (
        subset.color == 'red'   && subset.amount > 12 ||
        subset.color == 'green' && subset.amount > 13 ||
        subset.color == 'blue'  && subset.amount > 14
    );
}

function partOne(games: Game[]) : number {
    return games.reduce((ids, game) => {
        const possible = !game.handfuls.some(handful => handful.some(isImpossible));
        return ids + (possible ? game.id : 0);
    } , 0);
}

function samples(game: Game, color: Color) : number[] {
    return game.handfuls.map(handful => {
        for (const subset of handful) {
            if (subset.color == color) {
                return subset.amount;
            }
        }
        return 0;
    });
}

function partTwo(games: Game[]) : number {
    return games.reduce((powers, game) => (
        powers +
            Math.max(...samples(game, "red")) *
            Math.max(...samples(game, "green")) *
            Math.max(...samples(game, "blue"))
    ), 0);
}

function solve() : void {
    const lines = fs.readFileSync(process.argv[2], { encoding: "ascii" })
        // JavaScript is cute and doesn't have a standard library capable of
        // splitting end-of-line encodings. So we'll assume this is a UNIX
        // derivative.
        .split(/\n/);

    const games = lines.map(parseLine)
    console.log(`part one: ${partOne(games)}`);
    console.log(`part two: ${partTwo(games)}`);
}

solve();
