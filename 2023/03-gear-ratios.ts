import * as fs from "node:fs";
import * as process from "node:process";
import assert from "node:assert";

type Part = {
    row: number,
    column: number,
    width: number,
    number: number,
};

type Mark = {
    row: number,
    column: number,
    shape: string,
};

function showMark(mark: Mark) : string {
    return `(${mark.row}, ${mark.column})`;
}

function isDigit(digit: string) : boolean {
    return digit.match(/\d/) !== null;
}

function parts(schematic: string[]) : Part[] {
    let parts = [];
    for (let row = 0; row < schematic.length; ++row) {
        const tokens = schematic[row];
        for (let column = 0; column < tokens.length; ++column) {
            // Danger! UTF-16 string indexing, okay because ASCII.
            const maybeDigit = tokens.at(column)!;
            if (isDigit(maybeDigit)) {
                let end = column;
                while (end < tokens.length && isDigit(tokens.at(end)!)) {
                    ++end;
                }
                parts.push({
                    row, column,
                    width: end - column,
                    number: Number(tokens.slice(column, end))
                });
                column = end;
            }
        }
    }
    return parts;
}

function adjacentMarks(schematic: string[], part: Part) : Mark[] {
    let adjacent: [number, number][] = [];

    // Spaces above and below the part number.
    for (let column = part.column; column < part.column + part.width; ++column) {
        adjacent.push([part.row - 1, column], [part.row + 1, column]);
    }

    // Spaces to the left of the part number.
    adjacent.push([part.row, part.column - 1]);
    adjacent.push([part.row - 1, part.column - 1]);
    adjacent.push([part.row + 1, part.column - 1]);

    // Spaces to the right of the part number.
    adjacent.push([part.row, part.column + part.width]);
    adjacent.push([part.row - 1, part.column + part.width]);
    adjacent.push([part.row + 1, part.column + part.width]);

    const inSchematic = (space: [number, number]) : boolean => {
        const [row, column] = space;
        return 0 <= row && row < schematic.length &&
               0 <= column && column < schematic[part.row].length;
    }

    const isSymbol = (space: [number, number]) : boolean => {
        const [row, column] = space;
        const maybeSymbol = schematic[row].at(column)!;
        return !isDigit(maybeSymbol) && maybeSymbol !== ".";
    }

    return adjacent.filter(space => inSchematic(space) && isSymbol(space)).map(space => {
        const [row, column] = space;
        return { row, column, shape: schematic[row].at(column)! };
    });
}

function isPartNumber(schematic: string[], part: Part) : boolean {
    return adjacentMarks(schematic, part).length > 0;
}

type AdjacentParts = { mark: Mark, parts: Part[] };

function adjacentParts(schematic: string[], parts: Part[]) : AdjacentParts[] {
    const markToParts: Map<string, { mark: Mark, parts: Part[] }>  = new Map();

    for (const part of parts) {
        for (const mark of adjacentMarks(schematic, part)) {
            if (markToParts.has(showMark(mark))) {
                markToParts.get(showMark(mark))!.parts.push(part);
            } else {
                markToParts.set(showMark(mark), { mark, parts: [part] });
            }
        }
    }

    return [...markToParts.values()];
}

function solve() : void {
    const lines = fs.readFileSync(process.argv[2], { encoding: "ascii" })
        // JavaScript is cute and doesn't have a standard library capable of
        // splitting end-of-line encodings. So we'll assume this is a UNIX
        // derivative.
        .split(/\n/);

        const partNumberTotal = parts(lines)
            .filter(part => isPartNumber(lines, part))
            .map(part => part.number)
            .reduce((total, partNumber) => total + partNumber);

        console.log(`part one: ${partNumberTotal}`);

        const productGearRatios = adjacentParts(lines, parts(lines))
            .map(adjacent =>
                adjacent.mark.shape === "*" && adjacent.parts.length === 2
                ? adjacent.parts[0].number * adjacent.parts[1].number
                : 0)
            .reduce((total, gearRatio) => total + gearRatio);

        console.log(`part two: ${productGearRatios}`);
}

solve();
