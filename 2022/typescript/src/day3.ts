import { promises as fs } from 'fs';

export const day3 = async (): Promise<void> => {
  const lines = (await fs.readFile('./src/day3.input.txt', 'utf-8')).split('\n');
  const resultPart1 = day3Part1(lines);
  console.log({ day3part1: resultPart1, isCorrect: resultPart1 === 7826 });
  const resultPart2 = day3Part2(lines);
  console.log({ day3part2: resultPart2, isCorrect: resultPart2 === 2577 });
};

const day3Part1 = (lines: string[]): number => {
  return lines.reduce((acc, rucksack) => {
    if (!rucksack) return acc;
    return acc + new Rucksack(rucksack).priotiry;
  }, 0);
};

const day3Part2 = (lines: string[]): number => {
  const rucksackGroups: RucksackGroup[] = [];
  for (let i = 0; i < lines.length; i += 3) {
    const rucksackGroup = new RucksackGroup([
      new Rucksack(lines[i]),
      new Rucksack(lines[i + 1]),
      new Rucksack(lines[i + 2])
    ]);
    rucksackGroups.push(rucksackGroup);
  }

  return rucksackGroups.reduce((acc, rucksackGroup) => {
    return acc + rucksackGroup.getPriority();
  }, 0);
};

class Rucksack {
  private readonly _items: string;

  constructor(items: string) {
    this._items = items;
  }

  get items(): string {
    return this._items;
  }

  get firstCompartment(): string {
    return this.items.slice(0, this.items.length / 2);
  }

  get secondCompartment(): string {
    return this.items.slice(this.items.length / 2);
  }

  get priotiry(): number {
    return letterToPriority(this.getCommonItem());
  }

  getCommonItem(): string {
    for (const item of this.firstCompartment) {
      if (this.secondCompartment.includes(item)) {
        return item;
      }
    }

    throw new Error('no common item found');
  }
}

class RucksackGroup {
  private readonly rucksacks: [Rucksack, Rucksack, Rucksack];

  constructor(rucksacks: [Rucksack, Rucksack, Rucksack]) {
    this.rucksacks = rucksacks;
  }

  getBadge(): string {
    const [a, b, c] = this.rucksacks.map(r => r.items.split(''));
    for (const item of a) {
      if (b.includes(item) && c.includes(item)) {
        return item;
      }
    }

    throw new Error('no common item found');
  }

  getPriority(): number {
    return letterToPriority(this.getBadge());
  }
}

function letterToPriority(letter: string): number {
  if (/[a-z]/.test(letter)) return letter.charCodeAt(0) - 'a'.charCodeAt(0) + 1;
  if (/[A-Z]/.test(letter)) return letter.charCodeAt(0) - 'A'.charCodeAt(0) + 27;

  throw new Error(`invalid letter: ${letter}`);
}
