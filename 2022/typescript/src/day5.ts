import { promises as fs } from 'fs';

export const day5 = async (): Promise<void> => {
  const input = new Input((await fs.readFile('./src/day5.input.txt', 'utf8')).split('\n'));
  const resultPart1 = day5Part1(input);
  console.log({ day5part1: resultPart1, isCorrect: resultPart1 === 'DHBJQJCCW' });
  const resultPart2 = day5Part2(input);
  console.log({ day5Part2: resultPart2, isCorrect: resultPart2 === 'WJVRLSJJT' });
};

const day5Part1 = (input: Input): string => {
  const crateStacks = CrateStack.parse(input.getCratesSection());
  const moves = Move.parse(input.getMovesSection());
  const crane = Crane.getCrateMover9000(crateStacks);
  for (const move of moves) {
    crane.move(move);
  }

  return crateStacks
    .map(crateStack => crateStack.peek())
    .map(crate => crate?.id || '')
    .join('');
};

const day5Part2 = (input: Input): string => {
  const crateStacks = CrateStack.parse(input.getCratesSection());
  const moves = Move.parse(input.getMovesSection());
  const crane = Crane.getCrateMover9001(crateStacks);
  for (const move of moves) {
    crane.move(move);
  }

  return crateStacks
    .map(crateStack => crateStack.peek())
    .map(crate => crate?.id || '')
    .join('');
};

class Input {
  private _lines: string[];

  constructor(lines: string[]) {
    this._lines = lines;
  }

  get lines(): readonly string[] {
    return this._lines;
  }

  getCratesSection(): string[] {
    return this._lines.slice(0, this._lines.indexOf(''));
  }

  getMovesSection(): string[] {
    return this._lines.slice(this._lines.indexOf('') + 1);
  }
}

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

  static parse(lines: readonly string[]): Move[] {
    return lines.map(line => new Move(line));
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
  private _move: (move: Move) => void = () => {
    throw new Error('Crane not initialized');
  };

  private constructor(crateStacks: CrateStack[]) {
    this.crateStacks = crateStacks;
  }

  static getCrateMover9000(crateStacks: CrateStack[]): Crane {
    const crateMover9000 = new Crane(crateStacks);
    crateMover9000._move = ({ amount, from, to }: Move): void => {
      const fromStack = crateMover9000.crateStacks[from - 1];
      const toStack = crateMover9000.crateStacks[to - 1];
      for (let i = 0; i < amount; i += 1) {
        const crate = fromStack.pop();
        if (!crate) throw new Error('Cannot move crate from empty stack');
        toStack.stack(crate);
      }
    };

    return crateMover9000;
  }

  static getCrateMover9001(crateStacks: CrateStack[]): Crane {
    const crateMover9001 = new Crane(crateStacks);
    crateMover9001._move = ({ amount, from, to }: Move): void => {
      const fromStack = crateMover9001.crateStacks[from - 1];
      const toStack = crateMover9001.crateStacks[to - 1];
      toStack.stack(...fromStack.take(amount));
    };

    return crateMover9001;
  }

  move(move: Move): void {
    this._move(move);
  }
}

class CrateStack {
  private crates: Crate[] = [];
  private _id: string;

  private constructor(id: string, crates: Crate[]) {
    this.crates = crates;
    this._id = id;
  }

  static parse(lines: readonly string[]): CrateStack[] {
    const _lines = [...lines].reverse();
    const crateStacks: Map<string, CrateStack> = new Map();

    for (let i = 1; i < _lines.length; i += 1) {
      const crates = Crate.parse(_lines[i]);

      for (const [stackId, crate] of Object.entries(crates)) {
        if (!crate) continue;
        const crateStack = crateStacks.get(stackId) || new CrateStack(stackId, []);
        crateStack.stack(crate);
        crateStacks.set(stackId, crateStack);
      }
    }

    return [...crateStacks.values()];
  }

  stack(...crates: Crate[]): void {
    this.crates.push(...crates);
  }

  pop(): Crate | undefined {
    return this.crates.pop();
  }

  take(amount: number): Crate[] {
    return this.crates.splice(this.crates.length - amount, amount);
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

  static parse(line: string): Record<number, Crate | null> {
    const crates: Record<number, Crate | null> = {};
    for (let i = 0; i < line.length; i += 4) {
      const id = line.substring(i, i + 3);
      const crate = /\[.\]/.test(id) ? new Crate(id) : null;
      crates[i / 4 + 1] = crate;
    }
    return crates;
  }

  get id(): string {
    return this._id.slice(1, -1);
  }
}
