const fs = require('fs');

const COLORS = ['red', 'green', 'blue'];

const BAG = {
  red: 12,
  green: 13,
  blue: 14,
};

const solution = solve(readLines(`file.txt`));
console.log(solution);

function readLines(inputFilePath) {
  const input = fs.readFileSync(inputFilePath, 'utf-8');
  return input.split('\n');
}

function solve(inputLines) {
  const games = getGames(inputLines);
  return {
    part1: solvePart1(games),
    part2: solvePart2(games),
  };
}

function getGames(inputLines) {
  return inputLines.reduce((acc, line) => {
    const [gameStr, setsStr] = line.split(':');
    const [, gameNr] = gameStr.split(' ');
    const sets = setsStr.split(';').map((set) =>
      set
        .split(',')
        .map((setPart) => setPart.trim().split(' '))
        .reduce(
          (acc, [countStr, color]) => {
            acc[color] = parseInt(countStr, 10);
            return acc;
          },
          { red: 0, green: 0, blue: 0 }
        )
    );

    acc[gameNr] = sets;

    return acc;
  }, {});
}

function solvePart1(games) {
  return Object.entries(games).reduce(
    (possibleGameNumbersSum, [gameNumber, game]) =>
      isGamePossible(game)
        ? possibleGameNumbersSum + parseInt(gameNumber, 10)
        : possibleGameNumbersSum,
    0
  );
}

function solvePart2(games) {
  return Object.values(games)
    .map((game) => getMinimumBag(game).reduce((acc, num) => acc * num, 1))
    .reduce((sum, num) => sum + num, 0);
}

function isGamePossible(sets) {
  return sets.every(isSetPossible);
}

function isSetPossible(set) {
  return COLORS.reduce(
    (result, color) => result && set[color] <= BAG[color],
    true
  );
}

function getMinimumBag(sets) {
  return COLORS.map((color) => Math.max(...sets.map((s) => s[color])));
}
