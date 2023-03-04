import { promises as fs } from 'fs';

export const day4 = async (): Promise<void> => {
  const lines = (await fs.readFile('./src/day4.input.txt', 'utf-8')).split('\n');
  const resultPart1 = day4Part1(lines);
  console.log({ day4part1: resultPart1, isCorrect: resultPart1 === 567 });
  const resultPart2 = day4Part2(lines);
  console.log({ day4part2: resultPart2, isCorrect: resultPart2 === 907 });
};

const day4Part1 = (lines: string[]): number => {
  return lines.reduce((acc, line) => {
    if (!line) return acc;
    const sectionPair = SectionPair.fromString(line);
    return sectionPair.hasFullyOverlappingSections() ? acc + 1 : acc;
  }, 0);
};

const day4Part2 = (lines: string[]): number => {
  return lines.reduce((acc, line) => {
    if (!line) return acc;
    const sectionPair = SectionPair.fromString(line);
    return sectionPair.hasPartiallyOverlappingSections() ? acc + 1 : acc;
  }, 0);
};

class SectionPair {
  private readonly a: Section;
  private readonly b: Section;

  private constructor(a: Section, b: Section) {
    this.a = a;
    this.b = b;
  }

  static fromString(line: string): SectionPair {
    const [a, b] = line.split(',').map(s => new Section(s));
    return new SectionPair(a, b);
  }

  hasFullyOverlappingSections(): boolean {
    return this.a.isFullyContainedIn(this.b) || this.b.isFullyContainedIn(this.a);
  }

  hasPartiallyOverlappingSections(): boolean {
    return this.a.isPartiallyContainedIn(this.b) || this.b.isPartiallyContainedIn(this.a);
  }
}

class Section {
  private start: number;
  private end: number;

  constructor(range: string) {
    const [start, end] = range.split('-').map(n => parseInt(n, 10));
    this.start = start;
    this.end = end;
  }

  *[Symbol.iterator](): IterableIterator<number> {
    for (let i = this.start; i <= this.end; i++) {
      yield i;
    }
  }

  isFullyContainedIn(other: Section): boolean {
    return this.start >= other.start && this.end <= other.end;
  }

  isPartiallyContainedIn(other: Section): boolean {
    return this.start <= other.end && this.end >= other.start;
  }
}
