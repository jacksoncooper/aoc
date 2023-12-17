import * as fs from "node:fs";
import * as process from "node:process";
import assert from "node:assert";

function partOne(lines: string[]) : number {
    // There don't seem to be any zeros in the calibration values!

    const numbers = lines.map(line => {
        const value = Array.from(line)
            .filter(value => value.match(/\d/))
            .join('');

        // Danger: UTF-16 code units!
        assert(value.length > 0);
        return Number(value.at(0)! + value.at(-1)!);
    });

    return numbers.reduce((rest, value) => rest + value);
}

function calibrationHalf(line: string, tokens: Map<string, number>) : number | null {
    type Value = { token: string, position: number };

    let value: Value | null = null;
    for (const token of tokens.keys()) {
        const position = line.indexOf(token);
        const candidate: Value = { token, position };
        if (candidate.position !== -1) {
            if (value === null || candidate.position < value.position) {
                value = { token: candidate.token, position: candidate.position };
            }
        }
    }
    if (value === null) {
        return null;
    }

    const digit = tokens.get(value.token);
    if (digit === undefined) {
        return null;
    }

    return digit;
}

function partTwo(lines: string[]) : number {
    const contents: [string, number][] = [
        ["1", 1], ["2", 2], ["3", 3],
        ["4", 4], ["5", 5], ["6", 6],
        ["7", 7], ["8", 8], ["9", 9],
        ["one", 1], ["two", 2], ["three", 3],
        ["four", 4], ["five", 5], ["six", 6],
        ["seven", 7], ["eight", 8], ["nine", 9],
    ];

    const tokens: Map<string, number> = new Map(contents);
    const backwardTokens = new Map(contents.map(
        ([token, value]) => [Array.from(token).toReversed().join(''), value]
    ));

    const numbers = lines.map(line => {
        const backward = Array.from(line).toReversed().join('');
        const firstHalf = calibrationHalf(line, tokens);
        assert(firstHalf !== null);
        const lastHalf =  calibrationHalf(backward, backwardTokens);
        assert(lastHalf !== null);
        return firstHalf * 10 + lastHalf;
    });

    return numbers.reduce((rest, value) => rest + value);
}

function solve() : void {
    const lines = fs.readFileSync(process.argv[2], { encoding: "ascii" })
        // JavaScript is cute and doesn't have a standard library capable of
        // splitting end-of-line encodings. So we'll assume this is a UNIX
        // derivative.
        .split(/\n/);

    console.log(`part one: ${partOne(lines)}`);
    console.log(`part two: ${partTwo(lines)}`);
}

solve();
