import { promises as fs } from 'fs';

export const day2 = async (): Promise<void> => {
  const lines = (await fs.readFile('./src/day2.input.txt', 'utf8')).split('\n');
  const resultPart1 = day2Part1(lines);
  console.log({ day2part1: resultPart1, isCorrect: resultPart1 === 11475 });
  const resultPart2 = day2Part2(lines);
  console.log({ day2part2: resultPart2, isCorrect: resultPart2 === 16862 });
};

const day2Part1 = (strategyGuideSteps: string[]): number => {
  return analyseStrategyGuide(strategyGuideSteps, step => StrategyGuideStep.stepIsOpponentAndResponseMoves(step));
};

const day2Part2 = (strategyGuideSteps: string[]): number => {
  return analyseStrategyGuide(strategyGuideSteps, step => StrategyGuideStep.stepIsOpponentMoveAndDesiredOutcome(step));
};

const analyseStrategyGuide = (
  strategyGuideSteps: string[],
  strategyGuideStepProvider: (step: string) => StrategyGuideStep
): number => {
  const scores: number[] = strategyGuideSteps.map(line => {
    if (!line) return 0;
    const strategyGuide = strategyGuideStepProvider(line);
    return strategyGuide.score();
  });

  return scores.reduce((a, b) => a + b, 0);
};

class StrategyGuideStep {
  private readonly scoreCalculator: () => number;

  private constructor(scoreCalculator: () => number) {
    this.scoreCalculator = scoreCalculator;
  }

  static stepIsOpponentAndResponseMoves(step: string): StrategyGuideStep {
    const guide = new StrategyGuideStep(() => {
      const moves = step.split(' ');
      const [opponentMove, responseMove] = [new OpponentMove(moves[0]), new ResponseMove(moves[1])];
      let score = 0;
      if (responseMove.winsAgainst(opponentMove)) score += 6;
      if (responseMove.tiesAgainst(opponentMove)) score += 3;
      score += responseMove.score;

      return score;
    });

    return guide;
  }

  static stepIsOpponentMoveAndDesiredOutcome(step: string): StrategyGuideStep {
    const guide = new StrategyGuideStep(() => {
      const moves = step.split(' ');
      const [opponentMove, desiredOutcome] = [new OpponentMove(moves[0]), new DesiredOutcome(moves[1])];
      let score = 0;
      if (desiredOutcome.isWin()) score += 6;
      if (desiredOutcome.isTie()) score += 3;
      score += desiredOutcome.getResponseFor(opponentMove).score;

      return score;
    });

    return guide;
  }

  score(): number {
    return this.scoreCalculator();
  }
}

class OpponentMove {
  private readonly move: string;

  constructor(move: string) {
    if (!['A', 'B', 'C'].includes(move)) throw new Error(`Invalid opponent move: ${move}`);
    this.move = move;
  }

  get score(): number {
    if (this.isRock()) return 1;
    if (this.isPaper()) return 2;
    if (this.isScissors()) return 3;

    throw new Error(`Unknown move: ${this.move}`);
  }

  isRock(): boolean {
    return this.move === 'A';
  }

  isPaper(): boolean {
    return this.move === 'B';
  }

  isScissors(): boolean {
    return this.move === 'C';
  }

  winsAgainst(responseMove: ResponseMove): boolean {
    if (this.isRock()) return responseMove.isScissors();
    if (this.isPaper()) return responseMove.isRock();
    if (this.isScissors()) return responseMove.isPaper();

    throw new Error(`Unknown move: ${this.move}`);
  }

  losesAgainst(responseMove: ResponseMove): boolean {
    if (this.isRock()) return responseMove.isPaper();
    if (this.isPaper()) return responseMove.isScissors();
    if (this.isScissors()) return responseMove.isRock();

    throw new Error(`Unknown move: ${this.move}`);
  }

  tiesAgainst(responseMove: ResponseMove): boolean {
    if (this.isRock()) return responseMove.isRock();
    if (this.isPaper()) return responseMove.isPaper();
    if (this.isScissors()) return responseMove.isScissors();

    throw new Error(`Unknown move: ${this.move}`);
  }

  getWinningResponse(): ResponseMove {
    if (this.isRock()) return ResponseMove.paper();
    if (this.isPaper()) return ResponseMove.scissors();
    if (this.isScissors()) return ResponseMove.rock();

    throw new Error(`Unknown move: ${this.move}`);
  }

  getLosingResponse(): ResponseMove {
    if (this.isRock()) return ResponseMove.scissors();
    if (this.isPaper()) return ResponseMove.rock();
    if (this.isScissors()) return ResponseMove.paper();

    throw new Error(`Unknown move: ${this.move}`);
  }

  getTieResponse(): ResponseMove {
    if (this.isRock()) return ResponseMove.rock();
    if (this.isPaper()) return ResponseMove.paper();
    if (this.isScissors()) return ResponseMove.scissors();

    throw new Error(`Unknown move: ${this.move}`);
  }
}

class ResponseMove {
  private readonly move: string;

  constructor(move: string) {
    if (!['X', 'Y', 'Z'].includes(move)) throw new Error(`Invalid response move: ${move}`);
    this.move = move;
  }

  static rock(): ResponseMove {
    return new ResponseMove('X');
  }

  static paper(): ResponseMove {
    return new ResponseMove('Y');
  }

  static scissors(): ResponseMove {
    return new ResponseMove('Z');
  }

  get score(): number {
    if (this.isRock()) return 1;
    if (this.isPaper()) return 2;
    if (this.isScissors()) return 3;

    throw new Error(`Unknown move: ${this.move}`);
  }

  isRock(): boolean {
    return this.move === 'X';
  }

  isPaper(): boolean {
    return this.move === 'Y';
  }

  isScissors(): boolean {
    return this.move === 'Z';
  }

  winsAgainst(opponentMove: OpponentMove): boolean {
    if (this.isRock()) return opponentMove.isScissors();
    if (this.isPaper()) return opponentMove.isRock();
    if (this.isScissors()) return opponentMove.isPaper();

    throw new Error(`Unknown move: ${this.move}`);
  }

  losesAgainst(opponentMove: OpponentMove): boolean {
    if (this.isRock()) return opponentMove.isPaper();
    if (this.isPaper()) return opponentMove.isScissors();
    if (this.isScissors()) return opponentMove.isRock();

    throw new Error(`Unknown move: ${this.move}`);
  }

  tiesAgainst(opponentMove: OpponentMove): boolean {
    if (this.isRock()) return opponentMove.isRock();
    if (this.isPaper()) return opponentMove.isPaper();
    if (this.isScissors()) return opponentMove.isScissors();

    throw new Error(`Unknown move: ${this.move}`);
  }
}

class DesiredOutcome {
  private readonly outcome: string;

  constructor(outcome: string) {
    if (!['X', 'Y', 'Z'].includes(outcome)) throw new Error(`Invalid desired outcome: ${outcome}`);
    this.outcome = outcome;
  }

  isLose(): boolean {
    return this.outcome === 'X';
  }

  isTie(): boolean {
    return this.outcome === 'Y';
  }

  isWin(): boolean {
    return this.outcome === 'Z';
  }

  getResponseFor(opponentMove: OpponentMove): ResponseMove {
    if (this.isLose()) return opponentMove.getLosingResponse();
    if (this.isTie()) return opponentMove.getTieResponse();
    if (this.isWin()) return opponentMove.getWinningResponse();

    throw new Error(`Unknown outcome: ${this.outcome}`);
  }
}
