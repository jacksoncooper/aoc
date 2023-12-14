import * as fs from "node:fs";
import * as process from "node:process";
import assert from "node:assert";

function partOne(lines: string[]) : Number {
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

function calibrationHalf(line: string, values: Map<string, number>) : number | null {
    let value: string | null = null;
    let valuePosition: number | null = null;

    for (const candidate of values.keys()) {
        const candidatePosition = line.indexOf(candidate);
        if (candidatePosition !== -1) {
            if (valuePosition === null || candidatePosition < valuePosition) {
                value = candidate;
                valuePosition = candidatePosition;
            }
        }
    }
    if (value === null) {
        return null;
    }

    const digit = values.get(value);
    if (digit === undefined) {
        return null;
    }

    return digit;
}

function partTwo(lines: string[]) : Number {
    const contents: [string, number][] = [
        ["1", 1], ["2", 2], ["3", 3],
        ["4", 4], ["5", 5], ["6", 6],
        ["7", 7], ["8", 8], ["9", 9],
        ["one", 1], ["two", 2], ["three", 3],
        ["four", 4], ["five", 5], ["six", 6],
        ["seven", 7], ["eight", 8], ["nine", 9],
    ];

    const values: Map<string, number> = new Map(contents);
    const backwardValues = new Map(contents.map(
        ([token, value]) => [Array.from(token).toReversed().join(''), value]
    ));

    const numbers = lines.map(line => {
        const backward = Array.from(line).toReversed().join('');
        const firstHalf = calibrationHalf(line, values);
        assert(firstHalf !== null);
        const lastHalf =  calibrationHalf(backward, backwardValues);
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
