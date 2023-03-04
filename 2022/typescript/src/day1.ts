import { promises as fs } from 'fs';

export const day1 = async (): Promise<void> => {
  const lines = (await fs.readFile('./src/day1.input.txt', 'utf8')).split('\n');
  const resultPart1 = day1Part1(lines);
  console.log({ day1part1: resultPart1, isCorrect: resultPart1 === 71506 });
  const resultPart2 = day1Part2(lines);
  console.log({ day1part2: resultPart2, isCorrect: resultPart2 === 209603 });
};

const day1Part1 = (lines: string[]): number => {
  let maxCalories = Number.MIN_SAFE_INTEGER;
  let accumulatedCalories = 0;

  for (const line of lines) {
    if (line !== '') {
      accumulatedCalories += parseInt(line, 10);
      continue;
    }
    maxCalories = Math.max(maxCalories, accumulatedCalories);
    accumulatedCalories = 0;
  }

  return maxCalories;
};

const day1Part2 = (lines: string[]): number => {
  const caloriesPerElf: number[] = [];
  let accumulatedCalories = 0;

  for (const line of lines) {
    if (line !== '') {
      accumulatedCalories += parseInt(line, 10);
      continue;
    }
    caloriesPerElf.push(accumulatedCalories);
    accumulatedCalories = 0;
  }

  return caloriesPerElf
    .sort((a, b) => b - a)
    .slice(0, 3)
    .reduce((a, b) => a + b, 0);
};
