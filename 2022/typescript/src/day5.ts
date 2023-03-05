import { promises as fs } from 'fs';

export const day5 = async (): Promise<void> => {
  const lines = (await fs.readFile('./src/day5.input.txt', 'utf8')).split('\n');
  const resultPart1 = day5Part1(lines);
  console.log({ day5part1: resultPart1 });
  // const resultPart2 = day5Part2(lines);
  // console.log({resultPart2});
};

const day5Part1 = (lines: string[]): string => {
  const crateStacks = CrateStack.fromString(getCratesSection(lines));
  const moves = getMovesSection(lines).map(move => new Move(move));
  const crane = new Crane(crateStacks);
  for (const move of moves) {
    crane.move(move);
  }

  return crateStacks
    .map(crateStack => crateStack.peek())
    .map(crate => crate?.id || '')
    .join('');
};

const getCratesSection = (lines: string[]): string[] => {
  return lines.slice(0, lines.indexOf('')).reverse();
};

const getLineCrates = (line: string): Record<number, Crate | null> => {
  const crates: Record<number, Crate | null> = {};
  for (let i = 0; i < line.length; i += 4) {
    const id = line.substring(i, i + 3);
    const crate = /\[.\]/.test(id) ? new Crate(id) : null;
    crates[i / 4 + 1] = crate;
  }
  return crates;
};

const getMovesSection = (lines: string[]): string[] => {
  return lines.slice(lines.indexOf('') + 1);
};

class Move {
  private _amount: number;
  private _from: number;
  private _to: number;

  constructor(description: string) {
    const match = description.match(/move (\d+) from (\d+) to (\d+)/);
    if (!match) throw new Error(`Invalid move description: ${description}`);
    this._amount = parseInt(match[1]);
    this._from = parseInt(match[2]);
    this._to = parseInt(match[3]);
  }

  get amount(): number {
    return this._amount;
  }

  get from(): number {
    return this._from;
  }

  get to(): number {
    return this._to;
  }
}

class Crane {
  private crateStacks: CrateStack[] = [];

  constructor(crateStacks: CrateStack[]) {
    this.crateStacks = crateStacks;
  }

  move({ amount, from, to }: Move): void {
    const fromStack = this.crateStacks[from - 1];
    const toStack = this.crateStacks[to - 1];
    for (let i = 0; i < amount; i += 1) {
      const crate = fromStack.unstack();
      if (!crate) throw new Error('Cannot move crate from empty stack');
      toStack.stack(crate);
    }
  }
}

class CrateStack {
  private crates: Crate[] = [];
  private _id: string;

  private constructor(id: string, crates: Crate[]) {
    this.crates = crates;
    this._id = id;
  }

  static fromString(lines: string[]): CrateStack[] {
    const crateStacks: Map<string, CrateStack> = new Map();
    for (let i = 1; i < lines.length; i += 1) {
      const crates = getLineCrates(lines[i]);
      for (const [stackId, crate] of Object.entries(crates)) {
        if (!crate) continue;
        const crateStack = crateStacks.get(stackId) || new CrateStack(stackId, []);
        crateStack.stack(crate);
        crateStacks.set(stackId, crateStack);
      }
    }

    return [...crateStacks.values()];
  }

  stack(crate: Crate): void {
    this.crates.push(crate);
  }

  unstack(): Crate | undefined {
    return this.crates.pop();
  }

  peek(): Crate | undefined {
    return this.crates[this.crates.length - 1];
  }

  toString(): string {
    return `${this._id}: ${this.crates.map(crate => crate.id).join(' ')}`;
  }
}

class Crate {
  private _id: string;
  constructor(id: string) {
    this._id = id;
  }

  get id(): string {
    return this._id.slice(1, -1);
  }
}
