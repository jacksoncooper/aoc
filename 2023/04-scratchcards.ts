import * as fs from "node:fs";
import * as process from "node:process";
import assert from "node:assert";

type Scratchcard = {
    id: number,
    numbers: Set<number>,
    winningNumbers: Set<number>,
};

function parseScratchcard(line: string) : Scratchcard | null {
    // Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    const language = /Card\s+(?<id>\d+):\s+(?<winningNumbers>(?:\d+\s*)+) \|\s+(?<numbers>(?:\d+\s*)+)/;
    const match = line.match(language);

    if (match === null) {
        return null;
    }

    const parseNumbers = (numbers: string) => numbers
        .split(/\s+/)
        .map(number => Number.parseInt(number));

    const id = Number.parseInt(match.groups!["id"]);
    const numbers = new Set(parseNumbers(match.groups!["numbers"]));
    const winningNumbers = new Set(parseNumbers(match.groups!["winningNumbers"]));

    return { id, numbers, winningNumbers };
}

function numberOfWins(card: Scratchcard) : number {
    return [...card.numbers]
        .filter(number => card.winningNumbers.has(number))
        .length;
}

function replicate(cards: Scratchcard[]) : number {
    const inventory = new Map<number, number>();

    for (const card of cards) {
        inventory.set(card.id, 1);
    }

    // The cards only replicate those with larger identifiers, so we are
    // guaranteed that once we process a card it is never possible for it to be
    // replicated.

    // Assumes `cards` is sorted by increasing identifier.

    for (const card of cards) {
        const wins = numberOfWins(card);
        for (let idOffset = 1; idOffset <= wins; ++idOffset) {
            const winId = card.id + idOffset;
            if (winId - 1 < cards.length) {
                inventory.set(
                    winId,
                    inventory.get(winId)! + inventory.get(card.id)!
                );
            }
        }
    }

    return [...inventory.values()]
        .reduce((total, count) => total + count);
}

function solve() : void {
    const lines = fs.readFileSync(process.argv[2], { encoding: "ascii" })
        // JavaScript is cute and doesn't have a standard library capable of
        // splitting end-of-line encodings. So we'll assume this is a UNIX
        // derivative.
        .split(/\n/);

    const cards = lines.map(line => {
        const card = parseScratchcard(line);
        assert(card !== null, line);
        return card;
    });

    const totalScores = cards
        .map(card => {
            const wins = numberOfWins(card);
            return wins > 0 ? Math.pow(2, wins - 1) : 0;
        })
        .reduce((total, score) => total + score);

    console.log(`part one: ${totalScores}`);
    console.log(`part two: ${replicate(cards)}`);
}

solve();
